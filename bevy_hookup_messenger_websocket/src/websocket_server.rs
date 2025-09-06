use bevy::ecs::resource::Resource;
use bevy_hookup_core::hook_session::SessionMessenger;
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
    session_message::SessionMessage, websocket_data::WebsocketData,
    websocket_session::WebsocketSession,
};

#[derive(Resource)]
pub struct WebsocketServer<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone>
{
    session_receiver: Receiver<SessionMessage<TSendables>>,
}

impl<TSendables: Serialize + DeserializeOwned + Send + Sync + 'static + Clone + Sized>
    WebsocketServer<TSendables>
{
    pub fn new() -> Self {
        let (session_sender, session_receiver) = unbounded();

        tokio::spawn(async move {
            let server = TcpListener::bind("0.0.0.0:9001").await.unwrap();

            while let Ok((stream, _)) = server.accept().await {
                let session_sender = session_sender.clone();

                tokio::spawn(async move {
                    let mut websocket = accept_async(stream).await.unwrap();

                    let (ws_sender, mut ws_receiver) = mpsc::unbounded_channel();
                    let session = WebsocketSession::<TSendables>::new(ws_sender);
                    let channels = session.get_channels();
                    let session_id = session.get_session_id();

                    session_sender
                        .try_send(SessionMessage::Add(session.to_session()))
                        .unwrap();

                    loop {
                        tokio::select! {
                            msg = websocket.next() => {
                                let Some(Ok(msg)) = msg else {
                                    break;
                                };

                                if !msg.is_binary() {
                                    continue;
                                }

                                let (data, _) = decode_from_slice::<WebsocketData<TSendables>, _>(
                                    &msg.into_data(),
                                    config::standard(),
                                )
                                .unwrap();

                                data.send_into_channel(&channels);
                            }
                            data = ws_receiver.recv() => {
                                let Some(data) = data else {
                                    continue;
                                };

                                let bytes = encode_to_vec(data, config::standard()).unwrap();

                                let message = Message::binary(bytes);

                                websocket.send(message).await.unwrap();
                            }
                        }
                    }

                    session_sender
                        .try_send(SessionMessage::Remove(session_id))
                        .unwrap();
                });
            }
        });

        Self { session_receiver }
    }

    pub fn get_session_messages(&self) -> impl Iterator<Item = SessionMessage<TSendables>> {
        self.session_receiver.try_iter()
    }
}
