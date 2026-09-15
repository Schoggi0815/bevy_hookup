use bevy::prelude::*;

use crate::connection::connection_id::ConnectionId;

#[derive(Event, Debug, Deref, DerefMut)]
pub struct ReceivedEvent<T> {
    #[deref]
    pub event: T,
    pub from_connection: ConnectionId,
}
