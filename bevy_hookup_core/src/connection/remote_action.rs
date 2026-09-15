use serde::{Deserialize, Serialize};

use crate::{
    component_sharing::component_type_id::ComponentTypeId,
    entity_sharing::sync_entity_id::SyncEntityId, event_sharing::event_type_id::EventTypeId,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RemoteAction {
    Entity {
        action: EntityAction,
        id: SyncEntityId,
    },
    Component {
        action: ComponentAction,
        component_type_id: ComponentTypeId,
        entity_id: SyncEntityId,
    },
    SendEvent {
        event_type_id: EventTypeId,
        event_data_string: String,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EntityAction {
    Add,
    Remove,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ComponentAction {
    AddOrUpdate { component_data_string: String },
    Remove,
}
