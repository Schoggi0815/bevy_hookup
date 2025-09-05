use bevy::prelude::*;

use crate::sync_component_id::SyncComponentId;

#[derive(Reflect, Component, Deref, DerefMut)]
pub struct Shared<T> {
    #[deref]
    pub inner: T,
    pub component_id: SyncComponentId,
}

impl<T> Shared<T> {
    pub fn new(inner: T, component_id: SyncComponentId) -> Self {
        Self {
            inner,
            component_id,
        }
    }

    pub fn update_inner(&mut self, updated: T) {
        self.inner = updated;
    }
}
