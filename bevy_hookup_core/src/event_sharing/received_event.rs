use bevy::prelude::*;

use crate::{client_id::ClientId, connection::connection_id::ConnectionId};

#[derive(Event, Debug, Deref, DerefMut)]
pub struct ReceivedEvent<T> {
    #[deref]
    pub event: T,
    pub from_connection: ConnectionId,
    pub from_client: ClientId,
}
