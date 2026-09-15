use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    connection::{Connection, remote_action::RemoteAction},
    event_sharing::{received_event::ReceivedEvent, send_event::SendEvent},
};

pub struct HookupEventPlugin<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TEvent> + Into<Option<TEvent>>,
    TEvent: Send + Sync + 'static,
> {
    _phantom: PhantomData<TSendables>,
    _phantom_component: PhantomData<TEvent>,
}

impl<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TEvent> + Into<Option<TEvent>>,
    TEvent: Send + Sync + 'static,
> Default for HookupEventPlugin<TSendables, TEvent>
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
            _phantom_component: Default::default(),
        }
    }
}

impl<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TEvent> + Into<Option<TEvent>>,
    TEvent: Send + Sync + 'static,
> Plugin for HookupEventPlugin<TSendables, TEvent>
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::check_session_channels)
            .add_observer(Self::send_events);
    }
}

impl<
    TSendables: Send + Sync + 'static + Clone + for<'a> From<&'a TEvent> + Into<Option<TEvent>>,
    TEvent: Send + Sync + 'static,
> HookupEventPlugin<TSendables, TEvent>
{
    fn send_events(event: On<SendEvent<TEvent>>, connections: Query<&mut Connection<TSendables>>) {
        for mut connection in connections {
            connection.send_event((&event.event().event).into());
        }
    }

    fn check_session_channels(
        connections: Query<&mut Connection<TSendables>>,
        mut commands: Commands,
    ) {
        for connection in connections {
            let mut unused_actions = Vec::new();
            for session_action in connection.channels.receiver.try_iter() {
                match session_action {
                    RemoteAction::SendEvent { ref event_data } => {
                        let Some(event_data) = Into::<Option<TEvent>>::into(event_data.clone())
                        else {
                            unused_actions.push(session_action);
                            continue;
                        };

                        commands.trigger(ReceivedEvent {
                            event: event_data,
                            from_connection: connection.get_connection_id(),
                        });
                    }
                    _ => unused_actions.push(session_action),
                }
            }
            unused_actions
                .into_iter()
                .for_each(|sa| connection.channels.sender.try_send(sa).expect("Unbounded"));
        }
    }
}
