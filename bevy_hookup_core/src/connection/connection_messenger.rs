use erased_serde::{Deserializer, Serialize};

use crate::connection::{Connection, connection_id::ConnectionId};

pub trait ConnectionMessenger {
    fn to_connection(self) -> Connection;
    fn get_connection_id(&self) -> ConnectionId;
    fn serialize(&mut self, data: Box<&dyn Serialize>) -> Result<String, anyhow::Error>;
    fn get_deserializer(&self, string: String) -> &mut dyn Deserializer;
}
