use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;

#[derive(SystemSet)]
pub struct ReceiveEventSystems<TComponent>(PhantomData<TComponent>);

impl<T> Default for ReceiveEventSystems<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for ReceiveEventSystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for ReceiveEventSystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReceiveComponentSet").field(&self.0).finish()
    }
}

impl<T> Hash for ReceiveEventSystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for ReceiveEventSystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for ReceiveEventSystems<T> {}
