use bevy::prelude::*;

use crate::{
    connection::connection_id::ConnectionId, entity_sharing::sync_entity_id::SyncEntityId,
    filter::Filter,
};

#[derive(Reflect, Component, Clone, Default)]
pub struct SyncEntity {
    pub sync_id: SyncEntityId,
}

#[derive(Reflect, Component, Clone)]
#[require(SyncEntity)]
pub struct SyncEntityOwner {
    pub on_sessions: Vec<ConnectionId>,
    pub session_read_filter: Filter<ConnectionId>,
    pub session_write_filter: Filter<ConnectionId>,
}

impl Default for SyncEntityOwner {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncEntityOwner {
    pub fn new() -> Self {
        Self {
            on_sessions: Vec::new(),
            session_read_filter: Filter::AllowAll,
            session_write_filter: Filter::AllowNone,
        }
    }

    pub fn with_read_filter(mut self, read_filter: Filter<ConnectionId>) -> Self {
        self.session_read_filter = read_filter;
        self
    }

    pub fn with_write_filter(mut self, write_filter: Filter<ConnectionId>) -> Self {
        self.session_write_filter = write_filter;
        self
    }
}

impl SyncEntity {
    pub fn new_from_id(sync_id: SyncEntityId) -> Self {
        Self { sync_id }
    }
}
