use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{prelude::Deref, reflect::Reflect};
use serde::{Deserialize, Serialize};

use crate::{
    client_id::ClientId, component_sharing::component_type_id::ComponentTypeId,
    entity_sharing::sync_entity_id::SyncEntityId, event_sharing::event_type_id::EventTypeId,
    filter::Filter,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RemoteAction {
    Entity {
        action: EntityAction,
        id: SyncEntityId,
        client_id: ClientId,
    },
    Component {
        action: ComponentAction,
        component_type_id: ComponentTypeId,
        entity_id: SyncEntityId,
        client_id: ClientId,
    },
    SendEvent {
        event_type_id: EventTypeId,
        event_data_raw: Vec<u8>,
        client_id: ClientId,
        event_id: EventId,
        timestamp: EventTimestamp,
        client_filter: Filter<ClientId>,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EntityAction {
    AddOrUpdate {
        client_read_filter: Filter<ClientId>,
        client_write_filter: Filter<ClientId>,
    },
    Remove,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ComponentAction {
    AddOrUpdate { component_data_raw: Vec<u8> },
    Remove,
}

#[derive(Debug, Clone, Hash, Copy, Serialize, Deserialize, Reflect, PartialEq, Eq, Deref)]
pub struct EventId(pub u64);

impl Default for EventId {
    fn default() -> Self {
        Self(rand::random())
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Reflect, PartialEq, Eq, PartialOrd, Ord, Deref,
)]
pub struct EventTimestamp(pub u64);

impl Default for EventTimestamp {
    fn default() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Are you time traveling???")
                .as_secs(),
        )
    }
}
