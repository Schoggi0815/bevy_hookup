use serde::{Deserialize, Serialize};

use crate::entity_sharing::sync_entity_id::SyncEntityId;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RemoteAction<TSendables> {
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
