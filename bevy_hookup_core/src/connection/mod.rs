use crate::connection::connection_messenger::ConnectionMessenger;
use crate::connection::remote_action::RemoteAction;
use crate::{
    connection::connection_id::ConnectionId, entity_sharing::sync_entity_id::SyncEntityId,
};
use bevy::prelude::Component;
use crossbeam::channel::{Receiver, Sender};

pub mod connection_id;
pub mod connection_messenger;
pub mod remote_action;

#[derive(Component)]
pub struct Connection<TSendables> {
    messenger: Box<dyn ConnectionMessenger<TSendables> + Send + Sync>,
    message_collection: Vec<RemoteAction<TSendables>>,
    pub channels: ConnectionChannels<TSendables>,
}

impl<TSendables> Connection<TSendables> {
    pub fn new(
        messenger: Box<dyn ConnectionMessenger<TSendables> + Send + Sync>,
        channels: ConnectionChannels<TSendables>,
    ) -> Self {
        Self {
            messenger,
            channels,
            message_collection: Vec::new(),
        }
    }

    pub fn get_connection_id(&self) -> ConnectionId {
        self.messenger.get_connection_id()
    }

    pub fn entity_added(&mut self, sync_id: SyncEntityId) {
        self.message_collection
            .push(RemoteAction::AddEntity { id: sync_id });
    }

    pub fn entity_removed(&mut self, sync_id: SyncEntityId) {
        self.message_collection
            .push(RemoteAction::RemoveEntity { id: sync_id });
    }

    pub fn component_added(&mut self, entity_id: SyncEntityId, component_data: TSendables) {
        self.message_collection.push(RemoteAction::AddComponent {
            component_data,
            entity_id,
        });
    }

    pub fn componend_updated(&mut self, entity_id: SyncEntityId, component_data: TSendables) {
        self.message_collection.push(RemoteAction::UpdateComponent {
            component_data,
            entity_id,
        });
    }

    pub fn component_removed(&mut self, entity_id: SyncEntityId) {
        self.message_collection
            .push(RemoteAction::RemoveComponent { entity_id });
    }

    pub fn send_event(&mut self, event_data: TSendables) {
        self.message_collection
            .push(RemoteAction::SendEvent { event_data });
    }

    pub fn push_messages(&mut self) {
        if self.message_collection.is_empty() {
            return;
        }

        self.messenger.handle_actions(&self.message_collection);
        self.message_collection.clear();
    }
}

#[derive(Clone)]
pub struct ConnectionChannels<TSendables> {
    pub sender: Sender<RemoteAction<TSendables>>,
    pub receiver: Receiver<RemoteAction<TSendables>>,
}
