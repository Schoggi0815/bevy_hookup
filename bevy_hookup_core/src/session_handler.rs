use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    hook_session::{SessionId, SessionMessanger},
    session::Session,
};

#[derive(Resource)]
pub struct SessionHandler<TSendables> {
    sessions: HashMap<SessionId, Session<TSendables>>,
    current_id: i32,
}

impl<TSendables> SessionHandler<TSendables> {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn add_session<F, TSession>(&mut self, session_builder: F)
    where
        F: Fn(SessionId) -> TSession,
        TSession: SessionMessanger<TSendables> + Send + Sync + 'static,
    {
        self.sessions.insert(
            SessionId(self.current_id),
            Session::new(Box::new(session_builder(SessionId(self.current_id)))),
        );
        self.current_id += 1;
    }

    pub fn get_sessions(&self) -> impl Iterator<Item = &Session<TSendables>> {
        self.sessions.values()
    }
}
