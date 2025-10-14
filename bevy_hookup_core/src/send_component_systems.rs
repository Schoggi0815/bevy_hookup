use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;

#[derive(SystemSet)]
pub struct SendComponentSystems<TComponent>(PhantomData<TComponent>);

impl<T> Default for SendComponentSystems<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for SendComponentSystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for SendComponentSystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SendComponentSystems")
            .field(&self.0)
            .finish()
    }
}

impl<T> Hash for SendComponentSystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for SendComponentSystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for SendComponentSystems<T> {}
