use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{connection::connection_id::ConnectionId, filter::Filter};

#[derive(Component, Reflect, Clone)]
pub struct ShareComponent<TComponent> {
    #[reflect(ignore)]
    phantom: PhantomData<TComponent>,
    pub on_sessions: Vec<ConnectionId>,
    pub read_filter: Filter<ConnectionId>,
}

impl<T> Default for ShareComponent<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            on_sessions: Vec::<ConnectionId>::new(),
            read_filter: Filter::AllowAll,
        }
    }
}

impl<T> ShareComponent<T> {
    pub fn with_read_filter(self, read_filter: Filter<ConnectionId>) -> Self {
        Self {
            read_filter,
            ..self
        }
    }
}
