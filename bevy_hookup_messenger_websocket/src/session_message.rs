use bevy_hookup_core::{hook_session::SessionId, session::Session};

pub enum SessionMessage<TSendables> {
    Add(Session<TSendables>),
    Remove(SessionId),
}
