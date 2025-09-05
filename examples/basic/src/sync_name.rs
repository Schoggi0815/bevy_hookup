use bevy::reflect::Reflect;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::all_sendables::Sendables;

#[derive(Default, Clone, Reflect, Serialize, Deserialize)]
pub struct SyncName {
    pub name: String,
}

impl SendableComponent<Sendables> for SyncName {
    fn to_sendable(&self) -> Sendables {
        Sendables::SyncName(self.clone())
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::SyncName(sync_name) => Some(sync_name),
            _ => None,
        }
    }
}
