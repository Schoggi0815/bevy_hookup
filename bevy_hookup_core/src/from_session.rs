use bevy::prelude::*;

use crate::hook_session::SessionId;

#[derive(Reflect, Component, Clone, Default)]
pub struct FromSession {
    pub session_id: SessionId,
}
