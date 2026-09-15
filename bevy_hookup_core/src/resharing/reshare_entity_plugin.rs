use bevy::prelude::*;

use std::marker::PhantomData;

use crate::{
    connection::connection_id::ConnectionId,
    entity_sharing::{
        receive_entity_systems::ReceiveEntitySystems,
        sync_entity::{SyncEntity, SyncEntityOwner},
    },
    filter::Filter,
    origin::Origin,
    resharing::reshare_entity_component::ReshareEntityComponent,
};

pub struct ReshareEntityPlugin<TSendables>(PhantomData<TSendables>);

impl<TS> Default for ReshareEntityPlugin<TS> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<TS> Plugin for ReshareEntityPlugin<TS>
where
    TS: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::reshare_entity.after(ReceiveEntitySystems::<TS>::default()),
        );
    }
}

impl<TS> ReshareEntityPlugin<TS> {
    fn reshare_entity(
        missing_owners: Query<
            (Entity, &Origin<ConnectionId>),
            (
                With<ReshareEntityComponent>,
                With<SyncEntity>,
                Without<SyncEntityOwner>,
            ),
        >,
        mut commands: Commands,
    ) where
        TS: Send + Sync + 'static,
    {
        for (entity, from_session) in missing_owners {
            commands.entity(entity).insert(
                SyncEntityOwner::new()
                    .with_read_filter(Filter::Blacklist(vec![from_session.0]))
                    .with_write_filter(Filter::Whitelist(vec![from_session.0])),
            );
        }
    }
}
