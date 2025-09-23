use bevy::prelude::*;

use crate::{
    hook_session::SessionId, session_filter::SessionFilter, sync_component_id::SyncComponentId,
};

#[derive(Reflect, Component, Deref, DerefMut)]
pub struct Owner<T> {
    #[deref]
    pub inner: T,
    pub component_id: SyncComponentId,
    pub remove: bool,
    pub on_sessions: Vec<SessionId>,
    pub session_read_filter: SessionFilter,
}

impl<T> Owner<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            component_id: SyncComponentId::new(),
            remove: false,
            on_sessions: Vec::new(),
            session_read_filter: SessionFilter::AllowAll,
        }
    }

    pub fn with_read_filter(mut self, read_filter: SessionFilter) -> Self {
        self.session_read_filter = read_filter;
        self
    }

    pub fn get_inner(&self) -> &T {
        &self.inner
    }

    pub fn update_inner(&mut self, updated: T) {
        self.inner = updated;
    }
}
