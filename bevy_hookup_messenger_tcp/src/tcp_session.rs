use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use bevy::prelude::*;
use bevy_hookup_core::{
    external_component::ExternalComponent,
    hook_session::{SessionId, SessionMessenger},
    session::{AddedData, EntityActions, RemovedData, SessionChannels, UpdatedData},
    sync_id::SyncId,
};
use bincode::{
    config::{self},
    serde::{decode_from_std_read, encode_into_std_write},
};
use crossbeam::channel::unbounded;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub struct TcpSession<TSendables> {
    session_id: SessionId,
    channels: SessionChannels<TSendables>,
}

#[derive(Serialize, Deserialize)]
enum TcpData<TSendables> {
    EntityAction(EntityActions),
    ComponentAdded(AddedData<TSendables>),
    ComponentUpdated(UpdatedData<TSendables>),
    ComponentRemoved(RemovedData),
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    TcpSession<TSendables>
{
    pub fn new() -> Self {
        let channels = SessionChannels {
            added: unbounded(),
            entity: unbounded(),
            removed: unbounded(),
            updated: unbounded(),
        };
        let copy_channels = channels.clone();

        thread::spawn(move || {
            let listener = TcpListener::bind("localhost:8090").unwrap();

            for tcp_stream in listener.incoming() {
                let Ok(mut tcp_stream) = tcp_stream else {
                    continue;
                };

                let entity_sender = channels.entity.0.clone();
                let added_sender = channels.added.0.clone();
                let updated_sender = channels.updated.0.clone();
                let removed_sender = channels.removed.0.clone();

                thread::spawn(move || {
                    let result: Result<TcpData<TSendables>, _> =
                        decode_from_std_read(&mut tcp_stream, config::standard());

                    let Ok(result) = result else {
                        return;
                    };

                    match result {
                        TcpData::EntityAction(entity_actions) => {
                            entity_sender.try_send(entity_actions).expect("unbounded")
                        }
                        TcpData::ComponentAdded(added_data) => {
                            added_sender.try_send(added_data).expect("unbounded")
                        }
                        TcpData::ComponentUpdated(updated_data) => {
                            updated_sender.try_send(updated_data).expect("unbounded")
                        }
                        TcpData::ComponentRemoved(removed_data) => {
                            removed_sender.try_send(removed_data).expect("unbounded")
                        }
                    }
                });
            }
        });

        Self {
            session_id: SessionId::default(),
            channels: copy_channels,
        }
    }
}

fn create_tcp_stream() -> TcpStream {
    TcpStream::connect("localhost:8095").expect("Failed to connect!")
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
    SessionMessenger<TSendables> for TcpSession<TSendables>
{
    fn entity_added(&mut self, _channels: &SessionChannels<TSendables>, sync_id: SyncId) {
        info!("Entity Added!");
        let mut tcp_stream = create_tcp_stream();

        encode_into_std_write(
            TcpData::<TSendables>::EntityAction(EntityActions::Add(sync_id)),
            &mut tcp_stream,
            config::standard(),
        )
        .expect("Failed to write");
    }

    fn entity_removed(&mut self, _channels: &SessionChannels<TSendables>, sync_id: SyncId) {
        info!("Entity Removed!");
        let mut tcp_stream = create_tcp_stream();

        encode_into_std_write(
            TcpData::<TSendables>::EntityAction(EntityActions::Remove(sync_id)),
            &mut tcp_stream,
            config::standard(),
        )
        .expect("Failed to write");
    }

    fn component_added(
        &mut self,
        _channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        info!("Added client component!");
        let mut tcp_stream = create_tcp_stream();

        encode_into_std_write(
            TcpData::<TSendables>::ComponentAdded(AddedData {
                component_data,
                external_component,
            }),
            &mut tcp_stream,
            config::standard(),
        )
        .expect("Failed to write");
    }

    fn componend_updated(
        &mut self,
        _channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
        component_data: TSendables,
    ) {
        info!("Updated client component!");
        let mut tcp_stream = create_tcp_stream();

        encode_into_std_write(
            TcpData::<TSendables>::ComponentUpdated(UpdatedData {
                component_data,
                external_component,
            }),
            &mut tcp_stream,
            config::standard(),
        )
        .expect("Failed to write");
    }

    fn component_removed(
        &mut self,
        _channels: &SessionChannels<TSendables>,
        external_component: ExternalComponent,
    ) {
        info!("Removed client component!");
        let mut tcp_stream = create_tcp_stream();

        encode_into_std_write(
            TcpData::<TSendables>::ComponentRemoved(RemovedData { external_component }),
            &mut tcp_stream,
            config::standard(),
        )
        .expect("Failed to write");
    }

    fn get_session_id(&self) -> SessionId {
        self.session_id
    }

    fn get_channels(&self) -> SessionChannels<TSendables> {
        self.channels.clone()
    }
}
