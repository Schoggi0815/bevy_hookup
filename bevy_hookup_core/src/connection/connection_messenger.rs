use crate::connection::{
    Connection, ConnectionChannels, connection_id::ConnectionId, remote_action::RemoteAction,
};

pub trait ConnectionMessenger<TSendables> {
    fn to_connection(self) -> Connection<TSendables>;
    fn get_connection_id(&self) -> ConnectionId;
    fn get_channels(&self) -> ConnectionChannels<TSendables>;
    fn handle_actions(&mut self, actions: &Vec<RemoteAction<TSendables>>);
}
