use bevy::{ecs::entity::Entity, reflect::Reflect};

use crate::hook_session::SessionId;

#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
pub struct ExternalEntity {
    session_id: SessionId,
    entity: Entity,
}

impl ExternalEntity {
    pub fn new(entity: Entity, session_id: SessionId) -> Self {
        Self { session_id, entity }
    }
}
