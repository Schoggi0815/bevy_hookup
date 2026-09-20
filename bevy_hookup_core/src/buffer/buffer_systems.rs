use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet)]
pub struct RecieveBufferSystems<TComponent>(PhantomData<TComponent>);

impl<T> Default for RecieveBufferSystems<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for RecieveBufferSystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for RecieveBufferSystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BufferSystems").field(&self.0).finish()
    }
}

impl<T> Hash for RecieveBufferSystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for RecieveBufferSystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for RecieveBufferSystems<T> {}

#[derive(SystemSet)]
pub struct SendBufferSystems<TComponent>(PhantomData<TComponent>);

impl<T> Default for SendBufferSystems<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for SendBufferSystems<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for SendBufferSystems<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BufferSystems").field(&self.0).finish()
    }
}

impl<T> Hash for SendBufferSystems<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> PartialEq for SendBufferSystems<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for SendBufferSystems<T> {}
