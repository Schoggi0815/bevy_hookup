use bevy::prelude::*;

#[derive(Reflect, Component, Deref, DerefMut)]
pub struct SharedComponent<T> {
    #[deref]
    pub inner: T,
    pub external_entity: Entity,
}

impl<T> SharedComponent<T> {
    pub fn new(inner: T, external_entity: Entity) -> Self {
        Self {
            inner,
            external_entity,
        }
    }

    pub fn update_inner(&mut self, updated: T) {
        self.inner = updated;
    }
}
