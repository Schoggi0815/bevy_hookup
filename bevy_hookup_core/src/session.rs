use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    hook_session::{SessionId, SessionMessenger},
    session_action::SessionAction,
    sync_entity_id::SyncEntityId,
};

#[derive(Component)]
pub struct Session<TSendables> {
    messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>,
    message_collection: Vec<SessionAction<TSendables>>,
    pub channels: SessionChannels<TSendables>,
}

impl<TSendables> Session<TSendables> {
    pub fn new(
        messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>,
        channels: SessionChannels<TSendables>,
    ) -> Self {
        Self {
            messenger,
            channels,
            message_collection: Vec::new(),
        }
    }

    pub fn get_session_id(&self) -> SessionId {
        self.messenger.get_session_id()
    }

    pub fn entity_added(&mut self, sync_id: SyncEntityId) {
        self.message_collection
            .push(SessionAction::AddEntity { id: sync_id });
    }

    pub fn entity_removed(&mut self, sync_id: SyncEntityId) {
        self.message_collection
            .push(SessionAction::RemoveEntity { id: sync_id });
    }

    pub fn component_added(&mut self, entity_id: SyncEntityId, component_data: TSendables) {
        self.message_collection.push(SessionAction::AddComponent {
            component_data,
            entity_id,
        });
    }

    pub fn componend_updated(&mut self, entity_id: SyncEntityId, component_data: TSendables) {
        self.message_collection
            .push(SessionAction::UpdateComponent {
                component_data,
                entity_id,
            });
    }

    pub fn component_removed(&mut self, entity_id: SyncEntityId) {
        self.message_collection
            .push(SessionAction::RemoveComponent { entity_id });
    }

    pub fn send_event(&mut self, event_data: TSendables) {
        self.message_collection
            .push(SessionAction::SendEvent { event_data });
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
pub struct SessionChannels<TSendables> {
    pub sender: Sender<SessionAction<TSendables>>,
    pub receiver: Receiver<SessionAction<TSendables>>,
}
