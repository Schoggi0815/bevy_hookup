use crate::component_sharing::component_type_id::ComponentTypeId;
use crate::connection::connection_messenger::ConnectionMessenger;
use crate::connection::remote_action::{ComponentAction, EntityAction, RemoteAction};
use crate::event_sharing::event_type_id::EventTypeId;
use crate::{
    connection::connection_id::ConnectionId, entity_sharing::sync_entity_id::SyncEntityId,
};
use bevy::log::warn;
use bevy::prelude::Component;
use crossbeam::channel::{Receiver, Sender};
use erased_serde::Serialize;
use itertools::Itertools;
use serde::de::DeserializeOwned;

pub mod connection_id;
pub mod connection_messenger;
pub mod remote_action;

#[derive(Component)]
pub struct Connection {
    messenger: Box<dyn ConnectionMessenger + Send + Sync>,
    current_messanges: Vec<RemoteAction>,
    pub(super) outgoing: Sender<RemoteAction>,
    pub(super) incoming: Receiver<RemoteAction>,
}

impl Connection {
    pub fn new(
        messenger: Box<dyn ConnectionMessenger + Send + Sync>,
        outgoing: Sender<RemoteAction>,
        incoming: Receiver<RemoteAction>,
    ) -> Self {
        Self {
            messenger,
            outgoing,
            incoming,
            current_messanges: Vec::new(),
        }
    }

    pub fn get_connection_id(&self) -> ConnectionId {
        self.messenger.get_connection_id()
    }

    pub fn entity_added(&mut self, sync_id: SyncEntityId) {
        self.outgoing.send(RemoteAction::Entity {
            action: EntityAction::Add,
            id: sync_id,
        });
    }

    pub fn entity_removed(&mut self, sync_id: SyncEntityId) {
        self.outgoing.send(RemoteAction::Entity {
            action: EntityAction::Remove,
            id: sync_id,
        });
    }

    fn serialize<T: 'static + Serialize>(&mut self, data: &T) -> Option<String> {
        let Ok(component_data_string) = self.messenger.serialize(Box::new(data)) else {
            warn!("Could not serialize data");
            return None;
        };

        Some(component_data_string)
    }

    pub fn component_added<T: 'static + Serialize>(
        &mut self,
        entity_id: SyncEntityId,
        component_type_id: ComponentTypeId,
        component_data: &T,
    ) {
        let Some(component_data_string) = self.serialize(component_data) else {
            return;
        };

        self.outgoing.send(RemoteAction::Component {
            action: ComponentAction::AddOrUpdate {
                component_data_string,
            },
            component_type_id,
            entity_id,
        });
    }

    pub fn componend_updated<T: 'static + Serialize>(
        &mut self,
        entity_id: SyncEntityId,
        component_type_id: ComponentTypeId,
        component_data: &T,
    ) {
        let Some(component_data_string) = self.serialize(component_data) else {
            return;
        };

        self.outgoing.send(RemoteAction::Component {
            action: ComponentAction::AddOrUpdate {
                component_data_string,
            },
            component_type_id,
            entity_id,
        });
    }

    pub fn component_removed(
        &mut self,
        entity_id: SyncEntityId,
        component_type_id: ComponentTypeId,
    ) {
        self.outgoing.send(RemoteAction::Component {
            action: ComponentAction::Remove,
            component_type_id,
            entity_id,
        });
    }

    pub fn send_event<T: 'static + Serialize>(
        &mut self,
        event_type_id: EventTypeId,
        event_data: &T,
    ) {
        let Some(event_data_string) = self.serialize(event_data) else {
            return;
        };

        self.outgoing.send(RemoteAction::SendEvent {
            event_type_id,
            event_data_string,
        });
    }

    pub fn messages(&self) -> impl Iterator<Item = &RemoteAction> {
        self.current_messanges.iter()
    }

    pub fn get_data<T: DeserializeOwned>(&self, string: String) -> Option<T> {
        let deserializer = self.messenger.get_deserializer(string);

        erased_serde::deserialize(deserializer).ok()
    }

    pub fn collect_messages(&mut self) {
        self.current_messanges = self.incoming.try_iter().collect_vec()
    }
}
