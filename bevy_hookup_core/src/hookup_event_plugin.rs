use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    received_event::ReceivedEvent, send_event::SendEvent, session::Session,
    session_action::SessionAction,
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
    fn send_events(event: On<SendEvent<TEvent>>, sessions: Query<&mut Session<TSendables>>) {
        for mut session in sessions {
            session.send_event((&event.event().event).into());
        }
    }

    fn check_session_channels(sessions: Query<&mut Session<TSendables>>, mut commands: Commands) {
        for session in sessions {
            let mut unused_actions = Vec::new();
            for session_action in session.channels.receiver.try_iter() {
                match session_action {
                    SessionAction::SendEvent { ref event_data } => {
                        let Some(event_data) = Into::<Option<TEvent>>::into(event_data.clone())
                        else {
                            unused_actions.push(session_action);
                            continue;
                        };

                        commands.trigger(ReceivedEvent {
                            event: event_data,
                            from_session: session.get_session_id(),
                        });
                    }
                    _ => unused_actions.push(session_action),
                }
            }
            unused_actions
                .into_iter()
                .for_each(|sa| session.channels.sender.try_send(sa).expect("Unbounded"));
        }
    }
}
