use bevy::prelude::*;

use crate::{
    client_id::ClientId,
    connection::{
        connection_id::ConnectionId,
        remote_action::{EventId, EventTimestamp},
    },
    filter::Filter,
};

#[derive(Event, Debug)]
pub struct SendEvent<T> {
    pub event: T,
    pub event_id: EventId,
    pub timestamp: EventTimestamp,
    pub client_filter: Filter<ClientId>,
    pub connection_filter: Filter<ConnectionId>,
}

impl<T> SendEvent<T> {
    pub fn new(event: T) -> Self {
        Self {
            event,
            event_id: EventId::default(),
            timestamp: EventTimestamp::default(),
            client_filter: Filter::allow_all(),
            connection_filter: Filter::allow_all(),
        }
    }

    pub fn with_client_filter(self, client_filter: Filter<ClientId>) -> Self {
        Self {
            client_filter,
            ..self
        }
    }

    pub fn with_connection_filter(self, connection_filter: Filter<ConnectionId>) -> Self {
        Self {
            connection_filter,
            ..self
        }
    }
}
