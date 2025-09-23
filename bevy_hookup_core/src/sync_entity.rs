use bevy::prelude::*;

use crate::{hook_session::SessionId, session_filter::SessionFilter, sync_entity_id::SyncEntityId};

#[derive(Reflect, Component, Clone, Default)]
pub struct SyncEntity {
    pub sync_id: SyncEntityId,
}

#[derive(Reflect, Component, Clone)]
#[require(SyncEntity)]
pub struct SyncEntityOwner {
    pub on_sessions: Vec<SessionId>,
    pub session_read_filter: SessionFilter,
    pub session_write_filter: SessionFilter,
    pub remove: bool,
}

impl SyncEntityOwner {
    pub fn new() -> Self {
        Self {
            remove: false,
            on_sessions: Vec::new(),
            session_read_filter: SessionFilter::AllowAll,
            session_write_filter: SessionFilter::AllowNone,
        }
    }

    pub fn with_read_filter(mut self, read_filter: SessionFilter) -> Self {
        self.session_read_filter = read_filter;
        self
    }

    pub fn with_write_filter(mut self, write_filter: SessionFilter) -> Self {
        self.session_write_filter = write_filter;
        self
    }
}

impl SyncEntity {
    pub fn new_from_id(sync_id: SyncEntityId) -> Self {
        Self { sync_id }
    }
}
