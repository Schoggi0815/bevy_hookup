use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Reflect, Debug, Serialize, Deserialize, Component)]
pub struct BufferObject<TComponent> {
    pub component: TComponent,
    pub index: u64,
    last_changed: bool,
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

pub fn update_buffer_objects<TComponent: Component + Eq + Clone>(
    buffer_objects: Query<(&mut BufferObject<TComponent>, &TComponent)>,
) {
    for (mut update_buffer, component) in buffer_objects {
        if update_buffer.component == *component && !update_buffer.last_changed {
            continue;
        }

        update_buffer.last_changed = update_buffer.component != *component;
        update_buffer.component = component.clone();
        update_buffer.index += 1;
    }
}
