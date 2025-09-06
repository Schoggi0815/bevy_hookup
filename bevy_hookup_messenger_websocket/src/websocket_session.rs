use bevy::prelude::*;
use bevy_hookup_core::{
    external_component::ExternalComponent,
    hook_session::{SessionId, SessionMessenger},
    session::{AddedData, EntityActions, RemovedData, Session, SessionChannels, UpdatedData},
    sync_entity_id::SyncEntityId,
};
use crossbeam::channel::unbounded;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::mpsc::UnboundedSender;

use crate::websocket_data::WebsocketData;

pub struct WebsocketSession<TSendables> {
    session_id: SessionId,
    channels: SessionChannels<TSendables>,
    websocket_sender: UnboundedSender<WebsocketData<TSendables>>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketSession<TSendables>
{
    pub fn new(websocket_sender: UnboundedSender<WebsocketData<TSendables>>) -> Self {
        Self {
            websocket_sender,
            session_id: SessionId::default(),
            channels: SessionChannels {
                added: unbounded(),
                entity: unbounded(),
                removed: unbounded(),
                updated: unbounded(),
            },
        }
    }

    fn send_data(&mut self, data: WebsocketData<TSendables>) {
        self.websocket_sender.send(data).unwrap();
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SessionMessenger<TSendables> for WebsocketSession<TSendables>
{
    fn to_session(self) -> Session<TSendables> {
        let channels = self.channels.clone();
        Session::new(Box::new(self), channels)
    }

    fn entity_added(&mut self, _channels: &SessionChannels<TSendables>, sync_id: SyncEntityId) {
        info!("Entity Added!");

        self.send_data(WebsocketData::<TSendables>::EntityAction(
            EntityActions::Add(sync_id),
        ));
    }

    fn entity_removed(&mut self, _channels: &SessionChannels<TSendables>, sync_id: SyncEntityId) {
        info!("Entity Removed!");

        self.send_data(WebsocketData::<TSendables>::EntityAction(
            EntityActions::Remove(sync_id),
        ));
    }

    fn component_added(
        &mut self,
        _channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        info!("Added client component!");

        self.send_data(WebsocketData::<TSendables>::ComponentAdded(AddedData {
            component_data,
            external_component,
        }));
    }

    fn componend_updated(
        &mut self,
        _channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        info!("Updated client component!");

        self.send_data(WebsocketData::<TSendables>::ComponentUpdated(UpdatedData {
            component_data,
            external_component,
        }));
    }

    fn component_removed(
        &mut self,
        _channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
    ) {
        info!("Removed client component!");

        self.send_data(WebsocketData::<TSendables>::ComponentRemoved(RemovedData {
            external_component,
        }));
    }

    fn get_session_id(&self) -> SessionId {
        self.session_id
    }

    fn get_channels(&self) -> SessionChannels<TSendables> {
        self.channels.clone()
    }
}
