use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncComponentId(u64);

impl Default for SyncComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncComponentId {
    pub fn new() -> Self {
        Self(rand::random())
    }
}
