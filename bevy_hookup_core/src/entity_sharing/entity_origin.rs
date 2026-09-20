use bevy::prelude::*;

#[derive(Reflect, Component, Deref, Clone, Default)]
pub struct EntityOrigin<T>(pub T);
