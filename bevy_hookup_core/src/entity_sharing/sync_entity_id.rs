use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncEntityId(u64);

impl Default for SyncEntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncEntityId {
    pub fn new() -> Self {
        Self(rand::random())
    }
}
