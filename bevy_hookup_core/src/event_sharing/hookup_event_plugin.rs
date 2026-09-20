use std::marker::PhantomData;

use bevy::prelude::*;
use erased_serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{
    client_id::ClientId,
    connection::{Connection, remote_action::RemoteAction},
    event_sharing::{
        event_type_id::EventTypeId, receive_event_systems::ReceiveEventSystems,
        received_event::ReceivedEvent, send_event::SendEvent,
    },
    hookup_core_plugin::ReadIncomingSystems,
    resharing::reshare_events_plugin::ReshareEvent,
};

pub struct HookupEventPlugin<
    TEvent: Send + Sync + 'static + Serialize + DeserializeOwned,
    const EVENT_ID: u64,
>(PhantomData<TEvent>);

impl<TEvent: Send + Sync + 'static + Serialize + DeserializeOwned, const EVENT_ID: u64> Default
    for HookupEventPlugin<TEvent, EVENT_ID>
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<TEvent: Send + Sync + 'static + Serialize + DeserializeOwned, const EVENT_ID: u64> Plugin
    for HookupEventPlugin<TEvent, EVENT_ID>
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            Self::check_session_channels.in_set(ReceiveEventSystems::<TEvent>::default()),
        )
        .add_observer(Self::send_events)
        .configure_sets(
            FixedUpdate,
            ReceiveEventSystems::<TEvent>::default().after(ReadIncomingSystems),
        );
    }
}

impl<TEvent: Send + Sync + 'static + Serialize + DeserializeOwned, const EVENT_ID: u64>
    HookupEventPlugin<TEvent, EVENT_ID>
{
    fn send_events(
        event: On<SendEvent<TEvent>>,
        connections: Query<&mut Connection>,
        client_id: Res<ClientId>,
    ) -> Result {
        let event = event.event();
        for mut connection in connections {
            if !event
                .connection_filter
                .is_allowed(&connection.get_connection_id())
            {
                continue;
            }

            connection.send_event(
                EventTypeId(EVENT_ID),
                &event.event,
                *client_id,
                event.event_id,
                event.timestamp,
                event.client_filter.clone(),
            )?;
        }

        Ok(())
    }

    fn check_session_channels(
        connections: Query<&mut Connection>,
        mut commands: Commands,
        client_id: Res<ClientId>,
    ) {
        for connection in connections {
            for (
                event_data,
                origin_client_id,
                event_id,
                timestamp,
                client_filter,
                event_type_id,
                event_data_raw,
            ) in connection.messages().filter_map(|message| match message {
                RemoteAction::SendEvent {
                    event_type_id,
                    event_data_raw,
                    client_id,
                    client_filter,
                    event_id,
                    timestamp,
                } => {
                    if event_type_id.0 != EVENT_ID {
                        return None;
                    }

                    connection.get_data::<TEvent>(event_data_raw).map(|e| {
                        (
                            e,
                            *client_id,
                            event_id,
                            timestamp,
                            client_filter,
                            event_type_id,
                            event_data_raw,
                        )
                    })
                }
                _ => None,
            }) {
                if client_filter.is_allowed(&client_id) {
                    commands.trigger(ReceivedEvent {
                        event: event_data,
                        from_connection: connection.get_connection_id(),
                        from_client: origin_client_id,
                    });
                }

                commands.trigger(ReshareEvent {
                    event_data: event_data_raw.clone(),
                    event_type_id: *event_type_id,
                    event_id: *event_id,
                    timestamp: *timestamp,
                    client_filter: client_filter.clone(),
                    from_connection: connection.get_connection_id(),
                    from_client: origin_client_id,
                });
            }
        }
    }
}
