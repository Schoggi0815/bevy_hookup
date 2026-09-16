use anyhow::Ok;
use bevy::ecs::component::Component;
use bevy_hookup_core::connection::{
    Connection, connection_id::ConnectionId, connection_messenger::ConnectionMessenger,
    remote_action::RemoteAction,
};
use bincode::{config, de::read::SliceReader, serde::encode_to_vec};
use crossbeam::channel::{Receiver, Sender, unbounded};

#[derive(Component)]
pub struct SteamworksSession {
    connection_id: ConnectionId,
    pub(crate) outgoing_sender: Sender<RemoteAction>,
    pub(crate) incoming_receiver: Receiver<RemoteAction>,
    pub(crate) incoming_sender: Sender<RemoteAction>,
}

impl SteamworksSession {
    pub fn new(outgoing_sender: Sender<RemoteAction>) -> Self {
        let (incoming_sender, incoming_receiver) = unbounded();

        Self {
            connection_id: ConnectionId::default(),
            incoming_receiver,
            incoming_sender,
            outgoing_sender,
        }
    }
}

impl ConnectionMessenger for SteamworksSession {
    fn to_connection(self) -> Connection {
        Connection::new(self.incoming_receiver.clone(), Box::new(self))
    }

    fn get_connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    fn serialize(&self, data: Box<&dyn erased_serde::Serialize>) -> Result<Vec<u8>, anyhow::Error> {
        let bytes = encode_to_vec(data, config::standard())?;
        Ok(bytes)
    }

    fn with_deserializer(
        &self,
        raw: &[u8],
        callback: &mut dyn FnMut(&mut dyn erased_serde::Deserializer) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let reader = SliceReader::new(raw);
        let mut decoder =
            bincode::serde::OwnedSerdeDecoder::from_reader(reader, config::standard());
        let deserializer = decoder.as_deserializer();
        let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
        callback(&mut erased)?;
        Ok(())
    }

    fn send_action(&self, action: RemoteAction) -> anyhow::Result<()> {
        self.outgoing_sender.send(action)?;
        Ok(())
    }
}
