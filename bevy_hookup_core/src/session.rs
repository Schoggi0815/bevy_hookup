use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::{
    external_component::ExternalComponent, hook_session::SessionMessenger, sync_id::SyncId,
};

pub struct Session<TSendables> {
    messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>,
    pub channels: SessionChannels<TSendables>,
}

impl<TSendables> Session<TSendables> {
    pub fn new(messenger: Box<dyn SessionMessenger<TSendables> + Send + Sync>) -> Self {
        Self {
            messenger,
            channels: SessionChannels {
                entity: unbounded(),
                added: unbounded(),
                updated: unbounded(),
                removed: unbounded(),
            },
        }
    }

    pub fn entity_added(&self, sync_id: SyncId) {
        self.messenger.entity_added(&self.channels, sync_id);
    }

    pub fn entity_removed(&self, sync_id: SyncId) {
        self.messenger.entity_removed(&self.channels, sync_id);
    }

    pub fn component_added(
        &self,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        self.messenger
            .component_added(&self.channels, external_component, component_data);
    }

    pub fn componend_updated(
        &self,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        self.messenger
            .componend_updated(&self.channels, external_component, component_data);
    }

    pub fn component_removed(&self, external_component: ExternalComponent) {
        self.messenger
            .component_removed(&self.channels, external_component);
    }
}

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

pub enum EntityActions {
    Add(SyncId),
    Remove(SyncId),
}

pub struct AddedData<TSendables> {
    pub component_data: TSendables,
    pub external_component: ExternalComponent,
}

pub struct UpdatedData<TSendables> {
    pub component_data: TSendables,
    pub external_component: ExternalComponent,
}

pub struct RemovedData {
    pub external_component: ExternalComponent,
}
