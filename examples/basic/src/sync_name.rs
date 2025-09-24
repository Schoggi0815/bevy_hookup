use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Reflect, Serialize, Deserialize)]
pub struct SyncName {
    pub name: String,
}
