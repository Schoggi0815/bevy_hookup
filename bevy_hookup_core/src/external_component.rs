use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::{sync_component_id::SyncComponentId, sync_entity_id::SyncEntityId};

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalComponent {
    pub entity_id: SyncEntityId,
    pub component_id: SyncComponentId,
}

impl ExternalComponent {
    pub fn new(entity_id: SyncEntityId, component_id: SyncComponentId) -> Self {
        Self {
            entity_id,
            component_id,
        }
    }

    pub fn counterpart(&self) -> Self {
        Self {
            entity_id: self.entity_id.counterpart(),
            component_id: self.component_id,
        }
    }
}
