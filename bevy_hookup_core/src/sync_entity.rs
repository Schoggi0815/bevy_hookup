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
}

impl SyncEntity {
    pub fn new_from_id(sync_id: SyncEntityId) -> Self {
        Self { sync_id }
    }
}
