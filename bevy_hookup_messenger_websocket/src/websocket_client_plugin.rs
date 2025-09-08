use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_hookup_core::session::Session;
use serde::{Serialize, de::DeserializeOwned};

use crate::{session_message::SessionMessage, websocket_client::WebsocketClient};

pub struct WebsocketClientPlugin<
    TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone,
> {
    _phantom_sendable: PhantomData<TSendables>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone> Default
    for WebsocketClientPlugin<TSendables>
{
    fn default() -> Self {
        Self {
            _phantom_sendable: Default::default(),
        }
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone> Plugin
    for WebsocketClientPlugin<TSendables>
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::manage_client_sessions, Self::handle_client_states),
        );
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketClientPlugin<TSendables>
{
    fn manage_client_sessions(
        websocket_clients: Query<&WebsocketClient<TSendables>>,
        mut sessions: Query<&mut Session<TSendables>>,
        mut commands: Commands,
    ) {
        for session in websocket_clients
            .iter()
            .flat_map(|wc| wc.get_session_messages())
        {
            match session {
                SessionMessage::Add(session) => {
                    commands.spawn(session);
                }
                SessionMessage::Remove(session_id) => {
                    let session = sessions
                        .iter_mut()
                        .find(|s| s.get_session_id() == session_id);
                    if let Some(mut session) = session {
                        session.remove = true;
                    }
                }
            }
        }
    }

    fn handle_client_states(
        websocket_clients: Query<(Entity, &WebsocketClient<TSendables>)>,
        mut commands: Commands,
    ) {
        for (entity, client) in websocket_clients {
            for new_state in client.get_state_updates() {
                commands.entity(entity).insert(new_state);
            }
        }
    }
}
