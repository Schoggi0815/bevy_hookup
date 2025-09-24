use bevy_hookup_macros::Sendable;
use serde::{Deserialize, Serialize};

use crate::{sync_name::SyncName, test_component::TestComponent, test_component_2::TestComponent2};

#[derive(Clone, Serialize, Deserialize, Sendable)]
pub enum Sendables {
    #[sendable]
    TestComponent(TestComponent),
    #[sendable]
    TestComponent2(TestComponent2),
    #[sendable]
    SyncName(SyncName),
}
