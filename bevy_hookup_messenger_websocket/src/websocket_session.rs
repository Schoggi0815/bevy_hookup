use bevy::prelude::*;
use bevy_hookup_core::connection::{
    Connection, connection_id::ConnectionId, connection_messenger::ConnectionMessenger,
    remote_action::RemoteAction,
};
use crossbeam::channel::{Receiver, Sender, unbounded};
use tokio::sync::mpsc::UnboundedSender;

pub struct WebsocketSession {
    connection_id: ConnectionId,
    pub(crate) outgoing_sender: UnboundedSender<RemoteAction>,
    pub(crate) incoming_receiver: Receiver<RemoteAction>,
    pub(crate) incoming_sender: Sender<RemoteAction>,
}

impl WebsocketSession {
    pub fn new(outgoing_sender: UnboundedSender<RemoteAction>) -> Self {
        let (incoming_sender, incoming_receiver) = unbounded();
        Self {
            connection_id: ConnectionId::default(),
            outgoing_sender,
            incoming_receiver,
            incoming_sender,
        }
    }
}

impl ConnectionMessenger for WebsocketSession {
    fn to_connection(self) -> Connection {
        Connection::new(self.incoming_receiver.clone(), Box::new(self))
    }

    fn get_connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    fn serialize(&self, data: Box<&dyn erased_serde::Serialize>) -> Result<Vec<u8>, anyhow::Error> {
        let bytes = vec![];
        let bytes = postcard::to_extend(&data, bytes)?;

        Ok(bytes)
    }

    fn send_action(&self, action: RemoteAction) -> anyhow::Result<()> {
        self.outgoing_sender.send(action)?;
        Ok(())
    }

    fn with_deserializer(
        &self,
        raw: &[u8],
        callback: &mut dyn FnMut(&mut dyn erased_serde::Deserializer) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let mut deserializer = postcard::Deserializer::from_bytes(raw);

        let mut erased = <dyn erased_serde::Deserializer>::erase(&mut deserializer);

        callback(&mut erased)
    }
}
