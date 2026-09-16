use bevy::prelude::*;

use crate::{
    connection::connection_id::ConnectionId,
    entity_sharing::{
        entity_origin::EntityOrigin,
        receive_entity_systems::ReceiveEntitySystems,
        sync_entity::{SyncEntity, SyncEntityOwner},
    },
    filter::Filter,
    resharing::reshare_entity_component::ReshareEntityComponent,
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
            (
                With<ReshareEntityComponent>,
                With<SyncEntity>,
                Without<SyncEntityOwner>,
            ),
        >,
        mut commands: Commands,
    ) {
        for (entity, from_session) in missing_owners {
            commands.entity(entity).insert(
                SyncEntityOwner::new()
                    .with_read_filter(Filter::Blacklist(vec![from_session.0]))
                    .with_write_filter(Filter::Whitelist(vec![from_session.0])),
            );
        }
    }
}
