use bevy::prelude::*;
use bevy_hookup_macros::Sendable;
use serde::{Deserialize, Serialize};

use crate::{
    test_component::TestComponent, test_component_2::TestComponent2, test_event::TestEvent,
};

#[derive(Clone, Serialize, Deserialize, Sendable)]
pub enum Sendables {
    #[sendable]
    TestComponent(TestComponent),
    #[sendable]
    TestComponent2(TestComponent2),
    #[sendable]
    Name(Name),
    #[sendable]
    TestEvent(TestEvent),
}
