use bevy::prelude::*;

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[reflect(Default)]
pub struct ConnectionId(u64);

impl Default for ConnectionId {
    fn default() -> Self {
        Self(rand::random())
    }
}
