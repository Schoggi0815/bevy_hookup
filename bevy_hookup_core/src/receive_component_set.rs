use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;

#[derive(SystemSet)]
pub struct ReceiveComponentSet<TComponent>(PhantomData<TComponent>);

impl<T> Default for ReceiveComponentSet<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for ReceiveComponentSet<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for ReceiveComponentSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReceiveComponentSet").field(&self.0).finish()
    }
}

impl<T> Hash for ReceiveComponentSet<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for ReceiveComponentSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for ReceiveComponentSet<T> {}
