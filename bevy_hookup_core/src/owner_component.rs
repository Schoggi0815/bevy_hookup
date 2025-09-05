use bevy::prelude::*;

use crate::{
    hook_session::SessionId, session_filter::SessionFilter, sync_component_id::SyncComponentId,
};

#[derive(Reflect, Component, Deref, DerefMut, Default)]
pub struct Owner<T> {
    #[deref]
    inner: T,
    pub component_id: SyncComponentId,
    pub remove: bool,
    pub on_sessions: Vec<SessionId>,
    pub session_filter: SessionFilter,
}

impl<T> Owner<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            component_id: SyncComponentId::new(),
            remove: false,
            on_sessions: Vec::new(),
            session_filter: SessionFilter::default(),
        }
    }

    pub fn new_with_filter(inner: T, filter: SessionFilter) -> Self {
        Self {
            inner,
            component_id: SyncComponentId::new(),
            remove: false,
            on_sessions: Vec::new(),
            session_filter: filter,
        }
    }

    pub fn get_inner(&self) -> &T {
        &self.inner
    }
}
