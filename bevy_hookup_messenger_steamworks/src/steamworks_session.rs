use bevy::ecs::component::Component;
use bevy_hookup_core::{
    hook_session::{SessionId, SessionMessenger},
    session::{Session, SessionChannels},
    session_action::SessionAction,
};
use crossbeam::channel::{Sender, unbounded};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Component)]
pub struct SteamworksSession<TSendables> {
    session_id: SessionId,
    channels: SessionChannels<TSendables>,
    handler_sender: Sender<Vec<SessionAction<TSendables>>>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SteamworksSession<TSendables>
{
    pub fn new(handler_sender: Sender<Vec<SessionAction<TSendables>>>) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            session_id: SessionId::default(),
            channels: SessionChannels { sender, receiver },
            handler_sender,
        }
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SessionMessenger<TSendables> for SteamworksSession<TSendables>
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
        self.handler_sender
            .try_send(actions.clone())
            .expect("Couldn't send actions to handler!");
    }
}
