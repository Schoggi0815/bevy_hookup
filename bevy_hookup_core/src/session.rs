use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    external_component::ExternalComponent,
    hook_session::{SessionId, SessionMessenger},
    session_action::SessionAction,
    sync_entity_id::SyncEntityId,
};

#[derive(Component)]
pub struct Session<TSendables> {
    messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>,
    message_collection: Vec<SessionAction<TSendables>>,
    pub channels: SessionChannels<TSendables>,
    pub remove: bool,
}

impl<TSendables> Session<TSendables> {
    pub fn new(
        messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>,
        channels: SessionChannels<TSendables>,
    ) -> Self {
        Self {
            messenger,
            channels,
            remove: false,
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

    pub fn component_added(
        &mut self,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        self.message_collection.push(SessionAction::AddComponent {
            component_data,
            external_component,
        });
    }

    pub fn componend_updated(
        &mut self,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        self.message_collection
            .push(SessionAction::UpdateComponent {
                component_data,
                external_component,
            });
    }

    pub fn component_removed(&mut self, external_component: ExternalComponent) {
        self.message_collection
            .push(SessionAction::RemoveComponent { external_component });
    }

    pub fn push_messages(&mut self) {
        self.messenger.handle_actions(&self.message_collection);
        self.message_collection.clear();
    }
}

#[derive(Clone)]
pub struct SessionChannels<TSendables> {
    pub sender: Sender<SessionAction<TSendables>>,
    pub receiver: Receiver<SessionAction<TSendables>>,
}
