use bevy::prelude::*;
use bevy_hookup_core::connection::Connection;

use crate::{session_message::SessionMessage, websocket_client::WebsocketClient};

pub struct WebsocketClientPlugin;

impl Plugin for WebsocketClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::manage_client_sessions, Self::handle_client_states),
        );
    }
}

impl WebsocketClientPlugin {
    fn manage_client_sessions(
        websocket_clients: Query<&WebsocketClient>,
        sessions: Query<(Entity, &Connection)>,
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
                        .iter()
                        .find(|(_, s)| s.get_connection_id() == session_id);
                    if let Some((entity, _)) = session {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }

    fn handle_client_states(
        websocket_clients: Query<(Entity, &WebsocketClient)>,
        mut commands: Commands,
    ) {
        for (entity, client) in websocket_clients {
            for new_state in client.get_state_updates() {
                commands.entity(entity).insert(new_state);
            }
        }
    }
}
