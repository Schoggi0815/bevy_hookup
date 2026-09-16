use std::marker::PhantomData;

use bevy::prelude::*;
use erased_serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{
    connection::{Connection, remote_action::RemoteAction},
    event_sharing::{
        event_type_id::EventTypeId, received_event::ReceivedEvent, send_event::SendEvent,
    },
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
        app.add_systems(Update, Self::check_session_channels)
            .add_observer(Self::send_events);
    }
}

impl<TEvent: Send + Sync + 'static + Serialize + DeserializeOwned, const EVENT_ID: u64>
    HookupEventPlugin<TEvent, EVENT_ID>
{
    fn send_events(event: On<SendEvent<TEvent>>, connections: Query<&mut Connection>) -> Result {
        for mut connection in connections {
            connection.send_event(EventTypeId(EVENT_ID), &event.event().event)?;
        }

        Ok(())
    }

    fn check_session_channels(connections: Query<&mut Connection>, mut commands: Commands) {
        for connection in connections {
            for event_data in connection.messages().filter_map(|message| match message {
                RemoteAction::SendEvent {
                    event_type_id,
                    event_data_raw,
                } => {
                    if event_type_id.0 != EVENT_ID {
                        return None;
                    }

                    connection.get_data::<TEvent>(event_data_raw)
                }
                _ => None,
            }) {
                commands.trigger(ReceivedEvent {
                    event: event_data,
                    from_connection: connection.get_connection_id(),
                });
            }
        }
    }
}
