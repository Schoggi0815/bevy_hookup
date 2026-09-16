use erased_serde::Serialize;

use crate::connection::{Connection, connection_id::ConnectionId, remote_action::RemoteAction};

pub trait ConnectionMessenger {
    fn to_connection(self) -> Connection;
    fn get_connection_id(&self) -> ConnectionId;
    fn serialize(&self, data: Box<&dyn Serialize>) -> Result<Vec<u8>, anyhow::Error>;
    fn with_deserializer(
        &self,
        raw: &[u8],
        callback: &mut dyn FnMut(&mut dyn erased_serde::Deserializer) -> anyhow::Result<()>,
    ) -> anyhow::Result<()>;
    fn send_action(&self, action: RemoteAction) -> anyhow::Result<()>;
}
