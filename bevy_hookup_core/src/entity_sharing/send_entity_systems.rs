use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;

#[derive(SystemSet)]
pub struct SendEntitySystems<TSendables>(PhantomData<TSendables>);

impl<T> Default for SendEntitySystems<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for SendEntitySystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for SendEntitySystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HookupEntitySendSystems")
            .field(&self.0)
            .finish()
    }
}

impl<T> Hash for SendEntitySystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for SendEntitySystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for SendEntitySystems<T> {}
