use bevy::{ecs::component::Component, log::error};
use bevy_hookup_core::connection::remote_action::RemoteAction;
use bevy_steamworks::{networking_sockets::NetConnection, networking_types::SendFlags};
use bincode::{
    config,
    serde::{decode_from_slice, encode_to_vec},
};
use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::steamworks_session::SteamworksSession;

#[derive(Component)]
pub struct SteamworksSessionHandler {
    connection: NetConnection,
    handler_receiver: Receiver<RemoteAction>,
    actions_sender: Sender<RemoteAction>,
}

impl SteamworksSessionHandler {
    pub fn new_pair(connection: NetConnection) -> (Self, SteamworksSession) {
        let (sender, receiver) = unbounded();

        let session = SteamworksSession::new(sender);

        let handler = Self {
            connection,
            handler_receiver: receiver,
            actions_sender: session.incoming_sender.clone(),
        };

        (handler, session)
    }

    pub fn send_actions(&self) {
        let data = self.handler_receiver.try_iter().collect::<Vec<_>>();

        if data.len() == 0 {
            return;
        }

        let bytes = match encode_to_vec(data, config::standard()) {
            Err(err) => {
                error!("Failed to decode data: {}", err);
                return;
            }
            Ok(bytes) => bytes,
        };

        match self.connection.send_message(&bytes, SendFlags::RELIABLE) {
            Err(err) => {
                error!("Failed to send data: {}", err);
            }
            _ => {}
        };
    }

    pub fn check_actions(&mut self) {
        let batch_size = 10;

        while let Ok(messages) = self.connection.receive_messages(batch_size) {
            let exhausted = messages.len() < batch_size;

            for message in messages {
                let data = message.data();
                let (data, _) =
                    match decode_from_slice::<Vec<RemoteAction>, _>(data, config::standard()) {
                        Ok(data) => data,
                        Err(err) => {
                            error!("Failed to decode message: {}", err);
                            continue;
                        }
                    };

                data.into_iter().for_each(|data| {
                    if let Err(err) = self.actions_sender.try_send(data) {
                        error!("Failed to send message into actions channel: {}", err);
                    }
                });
            }

            if exhausted {
                break;
            }
        }
    }
}
