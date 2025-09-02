use bevy::prelude::*;

use crate::external_entity::ExternalEntity;

#[derive(Reflect, Component, Deref, DerefMut)]
pub struct Shared<T> {
    #[deref]
    pub inner: T,
    pub external_entity: ExternalEntity,
}

impl<T> Shared<T> {
    pub fn new(inner: T, external_entity: ExternalEntity) -> Self {
        Self {
            inner,
            external_entity,
        }
    }

    pub fn update_inner(&mut self, updated: T) {
        self.inner = updated;
    }
}
