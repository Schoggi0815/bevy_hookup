use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::sync_id::SyncId;

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalComponent {
    pub entity_id: SyncId,
    pub component_id: SyncId,
}

impl ExternalComponent {
    pub fn new(entity_id: SyncId, component_id: SyncId) -> Self {
        Self {
            entity_id,
            component_id,
        }
    }
}
