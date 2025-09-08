use bevy::prelude::*;

#[derive(Component, Default)]
pub enum WebsocketServerState {
    #[default]
    Initializing,
    Ready,
    Failed,
    Closed,
}
