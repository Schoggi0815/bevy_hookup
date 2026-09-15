use bevy::prelude::*;
use bevy_hookup_core::connection::{
    Connection, ConnectionChannels, connection_id::ConnectionId,
    connection_messenger::ConnectionMessenger, remote_action::RemoteAction,
};
use crossbeam::channel::unbounded;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::mpsc::UnboundedSender;

pub struct WebsocketSession<TSendables> {
    connection_id: ConnectionId,
    channels: ConnectionChannels<TSendables>,
    websocket_sender: UnboundedSender<Vec<RemoteAction<TSendables>>>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketSession<TSendables>
{
    pub fn new(websocket_sender: UnboundedSender<Vec<RemoteAction<TSendables>>>) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            websocket_sender,
            connection_id: ConnectionId::default(),
            channels: ConnectionChannels { sender, receiver },
        }
    }

    fn send_data(&mut self, data: Vec<RemoteAction<TSendables>>) {
        let _ = self.websocket_sender.send(data);
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    ConnectionMessenger<TSendables> for WebsocketSession<TSendables>
{
    fn to_connection(self) -> Connection<TSendables> {
        let channels = self.channels.clone();
        Connection::new(Box::new(self), channels)
    }

    fn get_connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    fn get_channels(&self) -> ConnectionChannels<TSendables> {
        self.channels.clone()
    }

    fn handle_actions(&mut self, actions: &Vec<RemoteAction<TSendables>>) {
        self.send_data(actions.clone());
    }
}
