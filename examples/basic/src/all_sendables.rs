use crate::test_component::TestComponent;

#[derive(Default)]
pub enum Sendables {
    #[default]
    None,
    TestComponent(TestComponent),
}
