use bevy::prelude::*;

#[derive(Reflect, Component, Deref, DerefMut, Default)]
pub struct Owner<T> {
    #[deref]
    inner: T,
}

impl<T> Owner<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn get_inner(&self) -> &T {
        &self.inner
    }
}
