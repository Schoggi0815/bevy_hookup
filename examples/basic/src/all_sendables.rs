use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::{test_component::TestComponent, test_component_2::TestComponent2};

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub enum Sendables {
    TestComponent(TestComponent),
    TestComponent2(TestComponent2),
}
