use bevy::{ecs::component::Component, log::info};
use bevy_hookup_core::{hook_session::SessionMessenger, session_action::SessionAction};
use bincode::{
    config,
    serde::{decode_from_slice, encode_to_vec},
};
use crossbeam::channel::{Receiver, unbounded};
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{net::TcpListener, sync::mpsc};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{
    session_message::SessionMessage, websocket_server_state::WebsocketServerState,
    websocket_session::WebsocketSession,
};

#[derive(Component)]
#[require(WebsocketServerState)]
pub struct WebsocketServer<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
{
    session_receiver: Receiver<SessionMessage<TSendables>>,
    state_receiver: Receiver<WebsocketServerState>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone + Sized>
    WebsocketServer<TSendables>
{
    pub fn new_with_port(port: u16) -> Self {
        let full_address = format!("0.0.0.0:{port}");
        Self::new(full_address)
    }

    pub fn new(address: String) -> Self {
        let (session_sender, session_receiver) = unbounded();
        let (state_sender, state_receiver) = unbounded();

        info!("Opening server at [{}]", address);

        tokio::spawn(async move {
            let Ok(server) = TcpListener::bind(address).await else {
                state_sender
                    .try_send(WebsocketServerState::Failed)
                    .expect("Unbounded");
                return;
            };

            state_sender
                .try_send(WebsocketServerState::Ready)
                .expect("Unbounded");

            while let Ok((stream, _)) = server.accept().await {
                let session_sender = session_sender.clone();

                tokio::spawn(async move {
                    let Ok(mut websocket) = accept_async(stream).await else {
                        return;
                    };

                    let (ws_sender, mut ws_receiver) = mpsc::unbounded_channel();
                    let session = WebsocketSession::<TSendables>::new(ws_sender);
                    let channels = session.get_channels();
                    let session_id = session.get_session_id();

                    session_sender
                        .try_send(SessionMessage::Add(session.to_session()))
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

                                let Ok((data, _)) = decode_from_slice::<Vec<SessionAction<TSendables>>, _>(
                                    &msg.into_data(),
                                    config::standard(),
                                ) else {
                                    break;
                                };

                                data.into_iter().for_each(|sa| channels.sender.try_send(sa).expect("unbounded"));
                            }
                            data = ws_receiver.recv() => {
                                let Some(data) = data else {
                                    continue;
                                };

                                let Ok(bytes) = encode_to_vec(data, config::standard()) else {
                                    break;
                                };

                                let message = Message::binary(bytes);

                                if let Err(_) = websocket.send(message).await {
                                    break;
                                }
                            }
                        }
                    }

                    session_sender
                        .try_send(SessionMessage::Remove(session_id))
                        .expect("Unbounded");
                });
            }

            state_sender
                .try_send(WebsocketServerState::Closed)
                .expect("Unbounded")
        });

        Self {
            session_receiver,
            state_receiver,
        }
    }

    pub fn get_session_messages(&self) -> impl Iterator<Item = SessionMessage<TSendables>> {
        self.session_receiver.try_iter()
    }

    pub fn get_state_updates(&self) -> impl Iterator<Item = WebsocketServerState> {
        self.state_receiver.try_iter()
    }
}
