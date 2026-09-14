use bevy::prelude::*;

#[derive(Debug, Component, Deref, Reflect)]
pub struct Buffered<TComponent>(pub TComponent);
