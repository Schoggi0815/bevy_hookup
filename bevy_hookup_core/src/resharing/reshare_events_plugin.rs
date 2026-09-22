use bevy::prelude::*;

use crate::{
    client_id::ClientId,
    connection::{
        Connection,
        connection_id::ConnectionId,
        remote_action::{EventId, EventTimestamp},
    },
    event_sharing::event_type_id::EventTypeId,
    filter::Filter,
};

pub struct ReshareEventsPlugin;

impl Plugin for ReshareEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_reshare_event);
    }
}

#[derive(Debug, Event, Reflect)]
pub struct ReshareEvent {
    pub event_data: Vec<u8>,
    pub event_type_id: EventTypeId,
    pub event_id: EventId,
    pub timestamp: EventTimestamp,
    pub client_filter: Filter<ClientId>,
    pub from_connection: ConnectionId,
    pub from_client: ClientId,
}

fn on_reshare_event(event: On<ReshareEvent>, connections: Query<&mut Connection>) {
    for mut connection in connections {
        if event.from_connection == connection.get_connection_id() {
            continue;
        }

        connection.send_event_raw(
            event.event_type_id,
            event.event_data.clone(),
            event.from_client,
            event.event_id,
            event.timestamp,
            event.client_filter.clone(),
        );
    }
}
