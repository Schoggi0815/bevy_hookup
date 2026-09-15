use bevy::prelude::*;

#[derive(Reflect, Component, Deref, Clone, Default)]
pub struct Origin<T>(pub T);
