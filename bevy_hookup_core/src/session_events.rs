use std::marker::PhantomData;

use bevy::prelude::*;

use crate::hook_session::SessionId;

#[derive(EntityEvent)]
pub struct SessionAddedComponent<TComponent> {
    pub entity: Entity,
    pub session_id: SessionId,
    pub(super) phantom: PhantomData<TComponent>,
}

#[derive(EntityEvent)]
pub struct SessionRemovedComponent<TComponent> {
    pub entity: Entity,
    pub session_id: SessionId,
    pub(super) phantom: PhantomData<TComponent>,
}

#[derive(EntityEvent)]
pub struct SessionUpdatedComponent<TComponent> {
    pub entity: Entity,
    pub session_id: SessionId,
    pub(super) phantom: PhantomData<TComponent>,
}
