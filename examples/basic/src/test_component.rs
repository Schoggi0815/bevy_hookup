use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Reflect, Serialize, Deserialize, PartialEq)]
pub struct TestComponent {
    pub test_field: i32,
}
