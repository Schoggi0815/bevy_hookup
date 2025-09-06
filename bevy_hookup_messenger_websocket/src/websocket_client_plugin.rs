use std::marker::PhantomData;

use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::websocket_client::WebsocketClient;

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
        app.insert_resource(WebsocketClient::<TSendables>::new())
            .add_systems(Update, Self::add_client_sessions);
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketClientPlugin<TSendables>
{
    fn add_client_sessions(
        websocket_server: Res<WebsocketClient<TSendables>>,
        mut commands: Commands,
    ) {
        for session in websocket_server.get_new_sessions() {
            commands.spawn(session);
        }
    }
}
