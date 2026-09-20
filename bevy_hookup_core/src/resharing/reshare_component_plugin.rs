use bevy::{ecs::entity_disabling::Disabled, prelude::*};

use std::marker::PhantomData;

use crate::{
    component_sharing::{
        component_origin::ComponentOrigin, receive_component_systems::ReceiveComponentSystems,
        send_component_systems::SendComponentSystems, share_component::ShareComponent,
    },
    connection::connection_id::ConnectionId,
    entity_sharing::sync_entity::SyncEntityOwner,
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
        app.add_systems(Update, Self::reshare_components)
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
                With<ComponentOrigin<TComponent, ConnectionId>>,
                With<TComponent>,
                Without<ShareComponent<TComponent>>,
                With<SyncEntityOwner>,
                Allow<Disabled>,
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
}
