use bevy::{ecs::entity_disabling::Disabled, prelude::*};

use crate::{
    connection::connection_id::ConnectionId,
    entity_sharing::{
        entity_origin::EntityOrigin,
        receive_entity_systems::ReceiveEntitySystems,
        sync_entity::{EntityReadFilter, EntityWriteFilter, SyncEntity, SyncEntityOwner},
    },
    filter::Filter,
};

pub struct ReshareEntityPlugin;

impl Plugin for ReshareEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::reshare_entity.after(ReceiveEntitySystems));
    }
}

impl ReshareEntityPlugin {
    fn reshare_entity(
        missing_owners: Query<
            (Entity, &EntityOrigin<ConnectionId>),
            (With<SyncEntity>, Without<SyncEntityOwner>, Allow<Disabled>),
        >,
        mut commands: Commands,
    ) {
        for (entity, from_session) in missing_owners {
            commands.entity(entity).insert((
                SyncEntityOwner::new(),
                EntityReadFilter(Filter::Blacklist(vec![from_session.0])),
                EntityWriteFilter(Filter::Whitelist(vec![from_session.0])),
            ));
        }
    }
}
