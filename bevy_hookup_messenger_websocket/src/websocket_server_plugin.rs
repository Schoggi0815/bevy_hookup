use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_hookup_core::session::Session;
use serde::{Serialize, de::DeserializeOwned};

use crate::{session_message::SessionMessage, websocket_server::WebsocketServer};

pub struct WebsocketServerPlugin<
    TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone,
> {
    _phantom_sendable: PhantomData<TSendables>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone> Default
    for WebsocketServerPlugin<TSendables>
{
    fn default() -> Self {
        Self {
            _phantom_sendable: Default::default(),
        }
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone> Plugin
    for WebsocketServerPlugin<TSendables>
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::manage_server_sessions, Self::handle_server_states),
        );
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketServerPlugin<TSendables>
{
    fn manage_server_sessions(
        websocket_servers: Query<&WebsocketServer<TSendables>>,
        sessions: Query<(Entity, &Session<TSendables>)>,
        mut commands: Commands,
    ) {
        for session in websocket_servers
            .iter()
            .flat_map(|ws| ws.get_session_messages())
        {
            match session {
                SessionMessage::Add(session) => {
                    commands.spawn(session);
                }
                SessionMessage::Remove(session_id) => {
                    let session = sessions
                        .iter()
                        .find(|(_, s)| s.get_session_id() == session_id);
                    if let Some((entity, _)) = session {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }

    fn handle_server_states(
        websocket_servers: Query<(Entity, &WebsocketServer<TSendables>)>,
        mut commands: Commands,
    ) {
        for (entity, server) in websocket_servers {
            for new_state in server.get_state_updates() {
                commands.entity(entity).insert(new_state);
            }
        }
    }
}
