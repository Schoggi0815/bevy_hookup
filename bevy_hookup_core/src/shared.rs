use bevy::prelude::*;

use crate::sync_id::SyncId;

#[derive(Reflect, Component, Deref, DerefMut)]
pub struct Shared<T> {
    #[deref]
    pub inner: T,
    pub component_id: SyncId,
}

impl<T> Shared<T> {
    pub fn new(inner: T, component_id: SyncId) -> Self {
        Self {
            inner,
            component_id,
        }
    }

    pub fn update_inner(&mut self, updated: T) {
        self.inner = updated;
    }
}
