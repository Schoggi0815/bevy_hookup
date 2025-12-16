use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Reflect, Debug, Serialize, Deserialize, Component)]
pub struct BufferObject<TComponent> {
    pub component: TComponent,
    pub index: u64,
    pub last_changed: bool,
}

impl<TComponent> BufferObject<TComponent> {
    pub fn new(component: TComponent) -> Self {
        Self {
            component,
            index: 0,
            last_changed: false,
        }
    }
}
