use bevy::prelude::*;

use crate::sync_id::SyncId;

#[derive(Reflect, Component, Deref, DerefMut, Default)]
pub struct Owner<T> {
    #[deref]
    inner: T,
    pub component_id: SyncId,
    pub remove: bool,
}

impl<T> Owner<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            component_id: SyncId::new(),
            remove: false,
        }
    }

    pub fn get_inner(&self) -> &T {
        &self.inner
    }
}
