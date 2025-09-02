use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::hook_session::SessionMessanger;

pub struct Session<TSendables> {
    messanger: Box<dyn SessionMessanger<TSendables> + Send + Sync>,
    pub channels: SessionChannels<TSendables>,
}

impl<TSendables> Session<TSendables> {
    pub fn new(messanger: Box<dyn SessionMessanger<TSendables> + Send + Sync>) -> Self {
        Self {
            messanger,
            channels: SessionChannels {
                added: unbounded(),
                updated: unbounded(),
                removed: unbounded(),
            },
        }
    }

    pub fn component_added(&self, entity: Entity, component_data: TSendables) {
        self.messanger
            .component_added(&self.channels, entity, component_data);
    }

    pub fn componend_updated(&self, entity: Entity, component_data: TSendables) {
        self.messanger
            .componend_updated(&self.channels, entity, component_data);
    }

    pub fn component_removed(&self, entity: Entity) {
        self.messanger.component_removed(&self.channels, entity);
    }
}

pub struct SessionChannels<TSendables> {
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

pub struct AddedData<TSendables> {
    pub component_data: TSendables,
    pub entity: Entity,
}

pub struct UpdatedData<TSendables> {
    pub component_data: TSendables,
    pub entity: Entity,
}

pub struct RemovedData {
    pub entity: Entity,
}
