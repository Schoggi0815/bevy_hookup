use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncId(u64);

impl Default for SyncId {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncId {
    pub fn new() -> Self {
        Self(rand::random())
    }
}
