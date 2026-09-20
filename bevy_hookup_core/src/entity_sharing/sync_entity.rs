use bevy::prelude::*;

use crate::{
    client_id::ClientId, connection::connection_id::ConnectionId,
    entity_sharing::sync_entity_id::SyncEntityId, filter::Filter,
};

#[derive(Debug, Clone, Deref, DerefMut, Component, Reflect)]
pub struct EntityReadFilter<T>(pub Filter<T>);

#[derive(Debug, Clone, Deref, DerefMut, Component, Reflect)]
pub struct EntityWriteFilter<T>(pub Filter<T>);

#[derive(Reflect, Component, Clone, Default)]
#[require(
    EntityReadFilter::<ClientId>(Filter::allow_all()),
    EntityWriteFilter::<ClientId>(Filter::allow_none()),
)]
pub struct SyncEntity {
    pub sync_id: SyncEntityId,
}

#[derive(Reflect, Component, Clone)]
#[require(
    SyncEntity,
    EntityReadFilter::<ConnectionId>(Filter::allow_all()),
    EntityWriteFilter::<ConnectionId>(Filter::allow_none()),
)]
pub struct SyncEntityOwner {
    pub on_connections: Vec<ConnectionId>,
}

impl Default for SyncEntityOwner {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncEntityOwner {
    pub fn new() -> Self {
        Self {
            on_connections: Vec::new(),
        }
    }
}

impl SyncEntity {
    pub fn new_from_id(sync_id: SyncEntityId) -> Self {
        Self { sync_id }
    }
}
