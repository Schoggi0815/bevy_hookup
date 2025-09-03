use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    hook_session::{SessionId, SessionMessenger},
    session::Session,
};

#[derive(Resource)]
pub struct SessionHandler<TSendables> {
    sessions: HashMap<SessionId, Session<TSendables>>,
}

impl<TSendables: Clone> SessionHandler<TSendables> {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn add_session<TSession>(&mut self, session: TSession)
    where
        TSession: SessionMessenger<TSendables> + Send + Sync + 'static,
    {
        let channels = session.get_channels();

        self.sessions.insert(
            session.get_session_id(),
            Session::new(Box::new(session), channels),
        );
    }

    pub fn get_sessions(&mut self) -> impl Iterator<Item = &mut Session<TSendables>> {
        self.sessions.values_mut()
    }
}
