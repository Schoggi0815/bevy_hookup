use std::marker::PhantomData;

use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::websocket_server::WebsocketServer;

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
        app.insert_resource(WebsocketServer::<TSendables>::new())
            .add_systems(Update, Self::add_server_sessions);
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketServerPlugin<TSendables>
{
    fn add_server_sessions(
        websocket_server: Res<WebsocketServer<TSendables>>,
        mut commands: Commands,
    ) {
        for session in websocket_server.get_new_sessions() {
            commands.spawn(session);
        }
    }
}
