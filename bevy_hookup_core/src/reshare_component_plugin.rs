use bevy::prelude::*;

use std::marker::PhantomData;

use crate::{
    receive_component_systems::ReceiveComponentSystems,
    reshare_entity_component::ReshareEntityComponent,
    send_component_systems::SendComponentSystems,
    share_component::ShareComponent,
    sync_entity::{SyncEntity, SyncEntityOwner},
};

pub struct ReshareComponentPlugin<TComponent>(PhantomData<TComponent>);

impl<TC> Default for ReshareComponentPlugin<TC> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<TComponent> Plugin for ReshareComponentPlugin<TComponent>
where
    TComponent: Send + Sync + 'static + Clone + Component,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (Self::reshare_components, Self::add_reshare_marker))
            .add_observer(Self::reshare_remove)
            .configure_sets(
                FixedUpdate,
                ReceiveComponentSystems::<TComponent>::default()
                    .before(SendComponentSystems::<TComponent>::default()),
            );
    }
}

impl<TComponent> ReshareComponentPlugin<TComponent> {
    fn reshare_components(
        components_without_share: Query<
            Entity,
            (
                With<ReshareEntityComponent>,
                With<TComponent>,
                Without<ShareComponent<TComponent>>,
            ),
        >,
        mut commands: Commands,
    ) where
        TComponent: Send + Sync + 'static + Clone + Component,
    {
        for entity in components_without_share {
            commands
                .entity(entity)
                .insert(ShareComponent::<TComponent>::default());
        }
    }

    fn reshare_remove(event: On<Remove, TComponent>, mut commands: Commands)
    where
        TComponent: Send + Sync + 'static + Component,
    {
        commands
            .entity(event.entity)
            .remove::<ShareComponent<TComponent>>();
    }

    fn add_reshare_marker(
        mut commands: Commands,
        to_reshare: Query<
            Entity,
            (
                With<TComponent>,
                With<SyncEntity>,
                Without<SyncEntityOwner>,
                Without<ReshareEntityComponent>,
            ),
        >,
    ) where
        TComponent: Send + Sync + 'static + Component,
    {
        for entity in to_reshare {
            commands.entity(entity).insert(ReshareEntityComponent);
        }
    }
}
