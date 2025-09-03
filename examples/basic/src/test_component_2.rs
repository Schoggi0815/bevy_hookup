use bevy::reflect::Reflect;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::all_sendables::Sendables;

#[derive(Default, Clone, Copy, Reflect, Serialize, Deserialize)]
pub struct TestComponent2 {
    pub test_field: i32,
}

impl SendableComponent<Sendables> for TestComponent2 {
    fn to_sendable(&self) -> Sendables {
        Sendables::TestComponent2(*self)
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::TestComponent2(test_component) => Some(test_component),
            _ => None,
        }
    }
}
