use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;

#[derive(SystemSet)]
pub struct ReceiveEntitySystems<TSendables>(PhantomData<TSendables>);

impl<T> Default for ReceiveEntitySystems<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for ReceiveEntitySystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for ReceiveEntitySystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HookupEntitySystems").field(&self.0).finish()
    }
}

impl<T> Hash for ReceiveEntitySystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for ReceiveEntitySystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for ReceiveEntitySystems<T> {}
