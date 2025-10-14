use bevy::prelude::*;

use std::marker::PhantomData;

use crate::{from_session::FromSession, owner_component::Owner, session::Session, shared::Shared};

pub struct ReshareComponentPlugin<TSendables, TComponent>(
    PhantomData<TComponent>,
    PhantomData<TSendables>,
);

impl<TS, TC> Default for ReshareComponentPlugin<TS, TC> {
    fn default() -> Self {
        Self(PhantomData::default(), PhantomData::default())
    }
}

impl<TSendables, TComponent> Plugin for ReshareComponentPlugin<TSendables, TComponent>
where
    TComponent: Send + Sync + 'static + Clone,
    TSendables: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::reshare_components)
            .add_observer(Self::reshare_remove);
    }
}

impl<TSendables, TComponent> ReshareComponentPlugin<TSendables, TComponent> {
    fn reshare_components(
        changed_components: Query<
            (
                Entity,
                &Shared<TComponent>,
                Option<&mut Owner<TComponent>>,
                &FromSession,
            ),
            Changed<Shared<TComponent>>,
        >,
        sessions: Query<&Session<TSendables>>,
        mut commands: Commands,
    ) where
        TComponent: Send + Sync + 'static + Clone,
        TSendables: Send + Sync + 'static,
    {
        let same_world_sessions = sessions
            .iter()
            .filter(|s| s.pushes_to_same_world())
            .map(|s| s.get_session_id())
            .collect::<Vec<_>>();

        for (entity, shared_component, owned_component, from_session) in changed_components {
            let Some(mut owned_component) = owned_component else {
                if same_world_sessions.contains(&from_session.session_id) {
                    continue;
                }

                commands
                    .entity(entity)
                    .insert(Owner::new(shared_component.inner.clone()));

                continue;
            };

            owned_component.update_inner(shared_component.inner.clone());
        }
    }

    fn reshare_remove(event: On<Remove, Shared<TComponent>>, mut commands: Commands)
    where
        TComponent: Send + Sync + 'static,
    {
        commands.entity(event.entity).remove::<Owner<TComponent>>();
    }
}
