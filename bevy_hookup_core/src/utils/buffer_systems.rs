use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet)]
pub struct BufferSystems<TComponent>(PhantomData<TComponent>);

impl<T> Default for BufferSystems<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for BufferSystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for BufferSystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BufferSystems").field(&self.0).finish()
    }
}

impl<T> Hash for BufferSystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for BufferSystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for BufferSystems<T> {}
