use serde::{Deserialize, Serialize};

use crate::sync_entity_id::SyncEntityId;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SessionAction<TSendables> {
    AddEntity {
        id: SyncEntityId,
    },
    RemoveEntity {
        id: SyncEntityId,
    },
    AddComponent {
        component_data: TSendables,
        entity_id: SyncEntityId,
    },
    UpdateComponent {
        component_data: TSendables,
        entity_id: SyncEntityId,
    },
    RemoveComponent {
        entity_id: SyncEntityId,
    },
    SendEvent {
        event_data: TSendables,
    },
}
