use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

#[derive(SystemSet, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ReceiveEntitySystems;
