use bevy::reflect::Reflect;
use bevy_hookup_core::sendable_component::SendableComponent;

use crate::all_sendables::Sendables;

#[derive(Default, Clone, Copy, Reflect)]
pub struct TestComponent {
    pub test_field: i32,
}

impl SendableComponent<Sendables> for TestComponent {
    fn to_sendable(&self) -> Sendables {
        Sendables::TestComponent(*self)
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::TestComponent(test_component) => Some(test_component),
            _ => None,
        }
    }
}
