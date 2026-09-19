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
