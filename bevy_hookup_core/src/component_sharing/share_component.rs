use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{connection::connection_id::ConnectionId, filter::Filter};

#[derive(Debug, Clone, Deref, DerefMut, Component, Reflect)]
pub struct ComponentReadFilter<TComponent, T>(
    #[deref] pub Filter<T>,
    #[reflect(ignore)] PhantomData<TComponent>,
);

impl<TComponent, T> ComponentReadFilter<TComponent, T> {
    pub fn new(filter: Filter<T>) -> Self {
        Self(filter, Default::default())
    }
}

#[derive(Component, Reflect, Clone)]
#[require(
    ComponentReadFilter::<TComponent, ConnectionId>::new(Filter::allow_all()),
)]
pub struct ShareComponent<TComponent: Component> {
    #[reflect(ignore)]
    phantom: PhantomData<TComponent>,
    pub on_sessions: Vec<ConnectionId>,
}

impl<T: Component> Default for ShareComponent<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            on_sessions: Vec::<ConnectionId>::new(),
        }
    }
}
