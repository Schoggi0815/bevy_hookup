use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component, Reflect, Deref, Clone, Default)]
pub struct ComponentOrigin<TComponent, T>(
    #[deref] pub T,
    #[reflect(ignore)] PhantomData<TComponent>,
);

impl<TComponent, T> ComponentOrigin<TComponent, T> {
    pub fn new(origin_id: T) -> Self {
        Self(origin_id, Default::default())
    }
}
