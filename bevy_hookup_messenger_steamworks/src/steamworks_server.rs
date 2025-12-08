use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use bevy_hookup_core::hook_session::SessionMessenger;
use bevy_steamworks::{
    Client,
    networking_sockets::{InvalidHandle, ListenSocket},
    networking_types::ListenSocketEvent,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    steam_reference::SteamReference, steamworks_session_handler::SteamworksSessionHandler,
};

#[derive(Component)]
pub struct SteamworksServer<TSendables> {
    socket: Arc<Mutex<ListenSocket>>,
    phamtom: PhantomData<TSendables>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone + Sized>
    SteamworksServer<TSendables>
{
    pub fn new(client: &Client) -> Result<Self, InvalidHandle> {
        let socket = client
            .networking_sockets()
            .create_listen_socket_p2p(0, [])?;

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            phamtom: Default::default(),
        })
    }

    pub fn handle_events(
        servers: Query<&SteamworksServer<TSendables>>,
        sessions: Query<(Entity, &SteamReference)>,
        mut commands: Commands,
    ) {
        for server in servers {
            let guard = server.socket.lock().expect("Unable to lock mutex");

            while let Some(event) = guard.try_receive_event() {
                match event {
                    ListenSocketEvent::Connecting(connection_request) => {
                        info!("Received connection request.");
                        if let Err(err) = connection_request.accept() {
                            error!("Failed to accept connection request: {}", err);
                        }
                    }
                    ListenSocketEvent::Connected(connected_event) => {
                        let steam_id = connected_event
                            .remote()
                            .steam_id()
                            .expect("SteamID not found");
                        let connection = connected_event.take_connection();

                        let (handler, session) =
                            SteamworksSessionHandler::<TSendables>::new_pair(connection);

                        commands.spawn((SteamReference(steam_id), session.to_session(), handler));
                    }
                    ListenSocketEvent::Disconnected(disconnected_event) => {
                        let steam_id = disconnected_event
                            .remote()
                            .steam_id()
                            .expect("Steam ID not found");
                        let sessions = sessions.iter().filter(|(_, s)| s.0 == steam_id);

                        for (entity, _) in sessions {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
    }
}
