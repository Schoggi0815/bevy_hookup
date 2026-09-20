use bevy::{ecs::component::Component, log::info};
use bevy_hookup_core::connection::{
    connection_messenger::ConnectionMessenger, remote_action::RemoteAction,
};
use crossbeam::channel::{Receiver, unbounded};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    session_message::SessionMessage, websocket_client_state::WebsocketClientState,
    websocket_session::WebsocketSession,
};

#[derive(Component)]
#[require(WebsocketClientState)]
pub struct WebsocketClient {
    session_receiver: Receiver<SessionMessage>,
    state_receiver: Receiver<WebsocketClientState>,
}

impl WebsocketClient {
    pub fn new_with_host_and_port(host: String, port: u16) -> Self {
        let full_address = format!("ws://{host}:{port}");
        Self::new(full_address)
    }

    pub fn new(address: String) -> Self {
        let (session_sender, session_receiver) = unbounded();
        let (state_sender, state_receiver) = unbounded();

        info!("trying to connect to server at [{}]", address);

        tokio::spawn(async move {
            let websocket = connect_async(address).await;

            let Ok((mut websocket, _)) = websocket else {
                state_sender
                    .send(WebsocketClientState::Failed)
                    .expect("Unbounded");
                return;
            };

            state_sender
                .send(WebsocketClientState::Connected)
                .expect("Unbounded");

            let (outgoing_sender, mut outgoing_receiver) = mpsc::unbounded_channel();
            let session = WebsocketSession::new(outgoing_sender);
            let session_id = session.get_connection_id();
            let incoming_sender = session.incoming_sender.clone();

            session_sender
                .try_send(SessionMessage::Add(session.to_connection()))
                .expect("Unbounded");

            loop {
                tokio::select! {
                    msg = websocket.next() => {
                        let Some(Ok(msg)) = msg else {
                            break;
                        };

                        if !msg.is_binary() {
                            continue;
                        }

                        let Ok(data) = postcard::from_bytes::<'_, RemoteAction>(&msg.into_data()) else {
                            break;
                        };

                        incoming_sender.try_send(data).expect("unbounded");
                    }
                    data = outgoing_receiver.recv() => {
                        let Some(data) = data else {
                            continue;
                        };

                        let bytes = Vec::new();
                        let Ok(bytes) = postcard::to_extend(&data, bytes) else {
                            break;
                        };

                        let message = Message::binary(bytes);

                        if let Err(_) = websocket.send(message).await {
                            break;
                        }
                    }
                }
            }

            state_sender
                .send(WebsocketClientState::Closed)
                .expect("Unbounded");

            session_sender
                .try_send(SessionMessage::Remove(session_id))
                .expect("Unbounded");
        });

        Self {
            session_receiver,
            state_receiver,
        }
    }

    pub fn get_session_messages(&self) -> impl Iterator<Item = SessionMessage> {
        self.session_receiver.try_iter()
    }

    pub fn get_state_updates(&self) -> impl Iterator<Item = WebsocketClientState> {
        self.state_receiver.try_iter()
    }
}
