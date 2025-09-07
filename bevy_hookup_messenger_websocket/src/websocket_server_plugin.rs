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
        app.add_systems(Update, Self::manage_server_sessions);
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketServerPlugin<TSendables>
{
    fn manage_server_sessions(
        websocket_server: Res<WebsocketServer<TSendables>>,
        mut sessions: Query<&mut Session<TSendables>>,
        mut commands: Commands,
    ) {
        for session in websocket_server.get_session_messages() {
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
}
