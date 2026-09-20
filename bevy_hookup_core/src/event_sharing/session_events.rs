use std::marker::PhantomData;

use bevy::prelude::*;

use crate::connection::connection_id::ConnectionId;

#[derive(EntityEvent)]
pub struct SessionAddedComponent<TComponent> {
    pub entity: Entity,
    pub connection_id: ConnectionId,
    pub(crate) phantom: PhantomData<TComponent>,
}

#[derive(EntityEvent)]
pub struct SessionRemovedComponent<TComponent> {
    pub entity: Entity,
    pub connection_id: ConnectionId,
    pub(crate) phantom: PhantomData<TComponent>,
}

#[derive(EntityEvent)]
pub struct SessionUpdatedComponent<TComponent> {
    pub entity: Entity,
    pub connection_id: ConnectionId,
    pub(crate) phantom: PhantomData<TComponent>,
}
