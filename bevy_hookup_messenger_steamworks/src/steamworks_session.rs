use bevy::ecs::component::Component;
use bevy_hookup_core::connection::{
    Connection, ConnectionChannels, connection_id::ConnectionId,
    connection_messenger::ConnectionMessenger, remote_action::RemoteAction,
};
use crossbeam::channel::{Sender, unbounded};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Component)]
pub struct SteamworksSession<TSendables> {
    connection_id: ConnectionId,
    channels: ConnectionChannels<TSendables>,
    handler_sender: Sender<Vec<RemoteAction<TSendables>>>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SteamworksSession<TSendables>
{
    pub fn new(handler_sender: Sender<Vec<RemoteAction<TSendables>>>) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            connection_id: ConnectionId::default(),
            channels: ConnectionChannels { sender, receiver },
            handler_sender,
        }
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    ConnectionMessenger<TSendables> for SteamworksSession<TSendables>
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
        self.handler_sender
            .try_send(actions.clone())
            .expect("Couldn't send actions to handler!");
    }
}
