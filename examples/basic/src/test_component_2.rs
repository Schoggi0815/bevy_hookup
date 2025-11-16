use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Reflect, Serialize, Deserialize, Component)]
pub struct TestComponent2 {
    pub test_field: i32,
}
