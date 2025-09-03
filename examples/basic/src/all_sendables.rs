use crate::{test_component::TestComponent, test_component_2::TestComponent2};

pub enum Sendables {
    TestComponent(TestComponent),
    TestComponent2(TestComponent2),
}
