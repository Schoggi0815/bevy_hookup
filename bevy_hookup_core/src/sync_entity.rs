use bevy::prelude::*;

use crate::{hook_session::SessionId, session_filter::SessionFilter, sync_id::SyncId};

#[derive(Reflect, Component, Clone, Default)]
pub struct SyncEntity {
    pub sync_id: SyncId,
}

#[derive(Reflect, Component, Clone)]
#[require(SyncEntity)]
pub struct SyncEntityOwner {
    pub on_sessions: Vec<SessionId>,
    pub session_filter: SessionFilter,
    pub remove: bool,
}

impl SyncEntityOwner {
    pub fn new() -> Self {
        Self {
            remove: false,
            on_sessions: Vec::new(),
            session_filter: SessionFilter::default(),
        }
    }
}

impl SyncEntity {
    pub fn new_from_id(sync_id: SyncId) -> Self {
        Self { sync_id }
    }
}
