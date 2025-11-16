use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{hook_session::SessionId, session_filter::SessionFilter};

#[derive(Component)]
pub struct ShareComponent<TComponent> {
    phantom: PhantomData<TComponent>,
    pub on_sessions: Vec<SessionId>,
    pub read_filter: SessionFilter,
}

impl<T> Default for ShareComponent<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            on_sessions: Vec::<SessionId>::new(),
            read_filter: SessionFilter::AllowAll,
        }
    }
}

impl<T> ShareComponent<T> {
    pub fn with_read_filter(self, read_filter: SessionFilter) -> Self {
        Self {
            read_filter,
            ..self
        }
    }
}
