use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SendEvent<T> {
    pub event: T,
}

impl<T> SendEvent<T> {
    pub fn new(event: T) -> Self {
        Self { event }
    }
}
