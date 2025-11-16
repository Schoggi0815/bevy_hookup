use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Reflect, Serialize, Deserialize, PartialEq, Component)]
pub struct TestComponent {
    pub test_field: i32,
}
