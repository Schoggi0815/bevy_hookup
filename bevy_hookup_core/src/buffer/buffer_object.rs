use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Reflect, Debug, Serialize, Deserialize, Component)]
pub struct BufferObject<TComponent> {
    pub component: TComponent,
    pub index: u64,
}

impl<TComponent> BufferObject<TComponent> {
    pub fn new(component: TComponent) -> Self {
        Self {
            component,
            index: 0,
        }
    }
}
