use bevy_hookup_core::session::{
    AddedData, EntityActions, RemovedData, SessionChannels, UpdatedData,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum WebsocketData<TSendables> {
    EntityAction(EntityActions),
    ComponentAdded(AddedData<TSendables>),
    ComponentUpdated(UpdatedData<TSendables>),
    ComponentRemoved(RemovedData),
}

impl<TSendables> WebsocketData<TSendables> {
    pub fn send_into_channel(self, channels: &SessionChannels<TSendables>) {
        match self {
            WebsocketData::EntityAction(entity_actions) => {
                channels.entity.0.try_send(entity_actions).unwrap()
            }
            WebsocketData::ComponentAdded(added_data) => {
                channels.added.0.try_send(added_data).unwrap()
            }
            WebsocketData::ComponentUpdated(updated_data) => {
                channels.updated.0.try_send(updated_data).unwrap()
            }
            WebsocketData::ComponentRemoved(removed_data) => {
                channels.removed.0.try_send(removed_data).unwrap()
            }
        }
    }
}
