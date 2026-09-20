use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Reflect)]
pub struct EventTypeId(pub(crate) u64);
