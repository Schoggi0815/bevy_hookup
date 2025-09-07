use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::{SessionId, SessionMessenger},
    session::{Session, SessionChannels},
    session_action::SessionAction,
};
use crossbeam::channel::unbounded;

pub struct SelfSession<TSendables: Clone> {
    session_id: SessionId,
    channels: SessionChannels<TSendables>,
}

impl<TSendables: Clone> SelfSession<TSendables> {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            session_id: SessionId::default(),
            channels: SessionChannels { receiver, sender },
        }
    }
}

impl<TSendables: Clone + Send + Sync + 'static> SessionMessenger<TSendables>
    for SelfSession<TSendables>
{
    fn to_session(self) -> Session<TSendables> {
        let channels = self.channels.clone();
        Session::new(Box::new(self), channels)
    }

    fn get_session_id(&self) -> SessionId {
        self.session_id
    }

    fn get_channels(&self) -> SessionChannels<TSendables> {
        self.channels.clone()
    }

    fn handle_actions(&mut self, actions: &Vec<SessionAction<TSendables>>) {
        actions.iter().for_each(|action| {
            self.channels
                .sender
                .try_send(action.clone().to_counterpart())
                .expect("Unbounded")
        });
    }
}
