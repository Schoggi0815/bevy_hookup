use crate::client_id::ClientId;
use crate::component_sharing::component_type_id::ComponentTypeId;
use crate::connection::connection_messenger::ConnectionMessenger;
use crate::connection::remote_action::{
    ComponentAction, EntityAction, EventId, EventTimestamp, RemoteAction,
};
use crate::event_map::EventMap;
use crate::event_sharing::event_type_id::EventTypeId;
use crate::filter::Filter;
use crate::{
    connection::connection_id::ConnectionId, entity_sharing::sync_entity_id::SyncEntityId,
};
use bevy::ecs::error::Result;
use bevy::log::warn;
use bevy::prelude::Component;
use crossbeam::channel::Receiver;
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
    pub(super) incoming: Receiver<RemoteAction>,
}

impl Connection {
    pub fn new(
        incoming: Receiver<RemoteAction>,
        messenger: Box<dyn ConnectionMessenger + Send + Sync>,
    ) -> Self {
        Self {
            messenger,
            incoming,
            current_messanges: Vec::new(),
        }
    }

    pub fn get_connection_id(&self) -> ConnectionId {
        self.messenger.get_connection_id()
    }

    pub fn entity_added(
        &mut self,
        sync_id: SyncEntityId,
        client_id: ClientId,
        client_read_filter: Filter<ClientId>,
        client_write_filter: Filter<ClientId>,
    ) -> Result {
        self.messenger.send_action(RemoteAction::Entity {
            action: EntityAction::AddOrUpdate {
                client_read_filter,
                client_write_filter,
            },
            id: sync_id,
            client_id,
        })?;
        Ok(())
    }

    pub fn entity_updated(
        &mut self,
        sync_id: SyncEntityId,
        client_id: ClientId,
        client_read_filter: Filter<ClientId>,
        client_write_filter: Filter<ClientId>,
    ) -> Result {
        self.messenger.send_action(RemoteAction::Entity {
            action: EntityAction::AddOrUpdate {
                client_read_filter,
                client_write_filter,
            },
            id: sync_id,
            client_id,
        })?;
        Ok(())
    }

    pub fn entity_removed(&mut self, sync_id: SyncEntityId, client_id: ClientId) -> Result {
        self.messenger.send_action(RemoteAction::Entity {
            action: EntityAction::Remove,
            id: sync_id,
            client_id,
        })?;
        Ok(())
    }

    fn serialize<T: 'static + Serialize>(&mut self, data: &T) -> Option<Vec<u8>> {
        match self.messenger.serialize(Box::new(data)) {
            Ok(data_raw) => Some(data_raw),
            Err(error) => {
                warn!("Could not serialize data, error: [{}]", error);
                None
            }
        }
    }

    pub fn component_added<T: 'static + Serialize>(
        &mut self,
        entity_id: SyncEntityId,
        component_type_id: ComponentTypeId,
        component_data: &T,
        client_id: ClientId,
    ) -> Result {
        let Some(component_data_raw) = self.serialize(component_data) else {
            return Ok(());
        };
        let component_data_raw = component_data_raw.to_vec();

        self.messenger.send_action(RemoteAction::Component {
            action: ComponentAction::AddOrUpdate { component_data_raw },
            component_type_id,
            entity_id,
            client_id,
        })?;
        Ok(())
    }

    pub fn componend_updated<T: 'static + Serialize>(
        &mut self,
        entity_id: SyncEntityId,
        component_type_id: ComponentTypeId,
        component_data: &T,
        client_id: ClientId,
    ) -> Result {
        let Some(component_data_raw) = self.serialize(component_data) else {
            return Ok(());
        };
        let component_data_raw = component_data_raw.to_vec();

        self.messenger.send_action(RemoteAction::Component {
            action: ComponentAction::AddOrUpdate { component_data_raw },
            component_type_id,
            entity_id,
            client_id,
        })?;
        Ok(())
    }

    pub fn component_removed(
        &mut self,
        entity_id: SyncEntityId,
        component_type_id: ComponentTypeId,
        client_id: ClientId,
    ) -> Result {
        self.messenger.send_action(RemoteAction::Component {
            action: ComponentAction::Remove,
            component_type_id,
            entity_id,
            client_id,
        })?;
        Ok(())
    }

    pub fn send_event<T: 'static + Serialize>(
        &mut self,
        event_type_id: EventTypeId,
        event_data: &T,
        client_id: ClientId,
        event_id: EventId,
        event_timestamp: EventTimestamp,
        client_filter: Filter<ClientId>,
    ) -> Result {
        let Some(event_data_raw) = self.serialize(event_data) else {
            return Ok(());
        };
        let event_data_raw = event_data_raw.to_vec();

        self.messenger.send_action(RemoteAction::SendEvent {
            event_type_id,
            event_data_raw,
            client_id,
            event_id,
            timestamp: event_timestamp,
            client_filter,
        })?;
        Ok(())
    }

    pub fn send_event_raw(
        &mut self,
        event_type_id: EventTypeId,
        event_data_raw: Vec<u8>,
        client_id: ClientId,
        event_id: EventId,
        event_timestamp: EventTimestamp,
        client_filter: Filter<ClientId>,
    ) -> Result {
        self.messenger.send_action(RemoteAction::SendEvent {
            event_type_id,
            event_data_raw,
            client_id,
            event_id,
            timestamp: event_timestamp,
            client_filter,
        })?;
        Ok(())
    }

    pub fn messages(&self) -> impl Iterator<Item = &RemoteAction> {
        self.current_messanges.iter()
    }

    pub fn get_data<T: DeserializeOwned>(&self, raw: &[u8]) -> Option<T> {
        let mut result: Option<T> = None;

        self.messenger
            .with_deserializer(raw, &mut |deserializer| {
                result = Some(erased_serde::deserialize::<T>(deserializer)?);
                Ok(())
            })
            .ok()?;

        result
    }

    pub fn collect_messages(&mut self, event_map: &mut EventMap) {
        self.current_messanges = self
            .incoming
            .try_iter()
            .filter(|ra| match ra {
                RemoteAction::SendEvent {
                    event_type_id: _,
                    event_data_raw: _,
                    client_id: _,
                    client_filter: _,
                    event_id,
                    timestamp,
                } => {
                    if event_map.valid(event_id, timestamp) {
                        event_map.insert(*event_id, *timestamp);
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            })
            .collect_vec()
    }
}
