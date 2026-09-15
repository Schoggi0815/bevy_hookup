use bevy::prelude::*;

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[reflect(Default)]
pub struct ClientId(u64);

impl Default for ClientId {
    fn default() -> Self {
        Self(rand::random())
    }
}
