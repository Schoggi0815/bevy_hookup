use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Reflect, Serialize, Deserialize)]
pub struct TestComponent2 {
    pub test_field: i32,
}
