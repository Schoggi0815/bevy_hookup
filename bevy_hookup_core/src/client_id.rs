use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Resource, Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[reflect(Default)]
pub struct ClientId(u32);

impl Default for ClientId {
    fn default() -> Self {
        Self(rand::random())
    }
}
