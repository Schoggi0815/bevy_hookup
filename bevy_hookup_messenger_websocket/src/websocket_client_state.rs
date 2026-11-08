use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub enum WebsocketClientState {
    #[default]
    Connecting,
    Connected,
    Failed,
    Closed,
}
