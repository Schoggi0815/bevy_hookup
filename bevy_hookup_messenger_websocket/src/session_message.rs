use bevy_hookup_core::connection::{Connection, connection_id::ConnectionId};

pub enum SessionMessage<TSendables> {
    Add(Connection<TSendables>),
    Remove(ConnectionId),
}
