use bevy::reflect::Reflect;

use crate::hook_session::SessionId;

#[derive(Reflect, Clone)]
pub enum SessionFilter {
    AllowAll,
    AllowNone,
    Blacklist(Vec<SessionId>),
    Whitelist(Vec<SessionId>),
}

impl SessionFilter {
    pub fn allow_session(&self, session_id: &SessionId) -> bool {
        match self {
            Self::AllowAll => true,
            Self::AllowNone => false,
            Self::Blacklist(session_ids) => !session_ids.contains(session_id),
            Self::Whitelist(session_ids) => session_ids.contains(session_id),
        }
    }
}
