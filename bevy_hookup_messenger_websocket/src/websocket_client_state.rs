use bevy::prelude::*;

#[derive(Component, Default)]
pub enum WebsocketClientState {
    #[default]
    Connecting,
    Connected,
    Failed,
    Closed,
}
