use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::{
    external_component::ExternalComponent,
    hook_session::{SessionId, SessionMessenger},
    sync_id::SyncId,
};

#[derive(Component)]
pub struct Session<TSendables> {
    messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>,
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
        }
    }

    pub fn get_session_id(&self) -> SessionId {
        self.messenger.get_session_id()
    }

    pub fn entity_added(&mut self, sync_id: SyncId) {
        self.messenger.entity_added(&self.channels, sync_id);
    }

    pub fn entity_removed(&mut self, sync_id: SyncId) {
        self.messenger.entity_removed(&self.channels, sync_id);
    }

    pub fn component_added(
        &mut self,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        self.messenger
            .component_added(&self.channels, external_component, component_data);
    }

    pub fn componend_updated(
        &mut self,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        self.messenger
            .componend_updated(&self.channels, external_component, component_data);
    }

    pub fn component_removed(&mut self, external_component: ExternalComponent) {
        self.messenger
            .component_removed(&self.channels, external_component);
    }
}

#[derive(Clone)]
pub struct SessionChannels<TSendables> {
    pub entity: (Sender<EntityActions>, Receiver<EntityActions>),
    pub added: (
        Sender<AddedData<TSendables>>,
        Receiver<AddedData<TSendables>>,
    ),
    pub updated: (
        Sender<UpdatedData<TSendables>>,
        Receiver<UpdatedData<TSendables>>,
    ),
    pub removed: (Sender<RemovedData>, Receiver<RemovedData>),
}

#[derive(Serialize, Deserialize)]
pub enum EntityActions {
    Add(SyncId),
    Remove(SyncId),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct AddedData<TSendables> {
    pub component_data: TSendables,
    pub external_component: ExternalComponent,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UpdatedData<TSendables> {
    pub component_data: TSendables,
    pub external_component: ExternalComponent,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RemovedData {
    pub external_component: ExternalComponent,
}
