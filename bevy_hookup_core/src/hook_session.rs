use bevy::prelude::*;

use crate::{
    session::{Session, SessionChannels},
    session_action::SessionAction,
};

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[reflect(Default)]
pub struct SessionId(u64);

impl Default for SessionId {
    fn default() -> Self {
        Self(rand::random())
    }
}

pub trait SessionMessenger<TSendables> {
    fn to_session(self) -> Session<TSendables>;
    fn get_session_id(&self) -> SessionId;
    fn get_channels(&self) -> SessionChannels<TSendables>;
    fn handle_actions(&mut self, actions: &Vec<SessionAction<TSendables>>);
    fn pushes_to_same_world(&self) -> bool {
        false
    }
}
