use bevy::prelude::*;

use crate::sync_id::SyncId;

#[derive(Reflect, Component, Clone, Default)]
pub struct SyncEntity {
    pub sync_id: SyncId,
}

#[derive(Reflect, Component, Clone)]
#[require(SyncEntity)]
pub struct SyncEntityOwner {
    pub remove: bool,
}

impl SyncEntityOwner {
    pub fn new() -> Self {
        Self { remove: false }
    }
}

impl SyncEntity {
    pub fn new_from_id(sync_id: SyncId) -> Self {
        Self { sync_id }
    }
}
