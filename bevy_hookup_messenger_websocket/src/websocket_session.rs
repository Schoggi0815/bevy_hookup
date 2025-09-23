use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::{SessionId, SessionMessenger},
    session::{Session, SessionChannels},
    session_action::SessionAction,
};
use crossbeam::channel::unbounded;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::mpsc::UnboundedSender;

pub struct WebsocketSession<TSendables> {
    session_id: SessionId,
    channels: SessionChannels<TSendables>,
    websocket_sender: UnboundedSender<Vec<SessionAction<TSendables>>>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    WebsocketSession<TSendables>
{
    pub fn new(websocket_sender: UnboundedSender<Vec<SessionAction<TSendables>>>) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            websocket_sender,
            session_id: SessionId::default(),
            channels: SessionChannels { sender, receiver },
        }
    }

    fn send_data(&mut self, data: Vec<SessionAction<TSendables>>) {
        let _ = self.websocket_sender.send(data);
    }
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SessionMessenger<TSendables> for WebsocketSession<TSendables>
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
        self.send_data(actions.clone());
    }
}
