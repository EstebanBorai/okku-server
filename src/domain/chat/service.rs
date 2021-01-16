use futures::future;
use futures::stream::SplitStream;
use futures::{Stream, StreamExt, TryStream, TryStreamExt};
use std::time::Duration;
use tokio::spawn;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc::unbounded_channel;
use tokio::time::delay_for;
use uuid::Uuid;
use warp::ws::WebSocket;

use crate::domain::chat::{Kind, Parcel};
use crate::error::{Error, Result};

pub struct ChatService {
    channel: Sender<Parcel>,
}

impl ChatService {
    pub fn new(tx: Sender<Parcel>) -> Self {
        Self { channel: tx }
    }

    pub async fn run(&self, rx: Receiver<Parcel>) {
        let ticking_alive = self.poll();
        let processing = rx.for_each(|p| self.handle(p.unwrap()));

        tokio::select! {
            _ = ticking_alive => {},
            _ = processing => {},
        }
    }

    pub async fn register(&self, client_id: Uuid, web_socket: WebSocket) {
        let channel_rx = self.subscribe();
        let (sink, stream) = web_socket.split();

        info!("Client({}): Registered", client_id);

        let read_process =
            ChatService::read_into_parcel(client_id, stream).try_for_each(|p| async {
                info!("Parcel Received: {:?}", p);
                self.publish(p);
                Ok(())
            });

        let (utx, urx) = unbounded_channel();
        spawn(urx.forward(sink));

        let write_process = ChatService::write_into_ws_message(client_id, channel_rx.into_stream())
            .try_for_each(|message| async {
                utx.send(Ok(message)).unwrap();
                Ok(())
            });

        if let Err(e) = tokio::select! {
            result = read_process => result,
            result = write_process => result,
        } {
            error!(
                "Client({}): Connection Error! ({})",
                client_id,
                e.to_string()
            );
        }
    }

    /// Publishes a `Ping` `Parcel` to use as singal of alive
    /// connection
    pub async fn poll(&self) {
        loop {
            info!("Polling Tick");
            delay_for(Duration::from_secs(5)).await;
            self.publish(Parcel::ping());
        }
    }

    /// Subscribes to the Chat main channel and retrieves
    /// a `Receiver` of the channel to be consumed by the
    /// client in question
    pub fn subscribe(&self) -> Receiver<Parcel> {
        self.channel.subscribe()
    }

    /// Publishes a `Parcel` to the Chat main channel
    pub fn publish(&self, parcel: Parcel) {
        self.channel.send(parcel);
    }

    pub async fn handle(&self, parcel: Parcel) {
        match parcel.kind {
            Kind::Message => self.publish(parcel),
            _ => {}
        }
    }

    pub fn read_into_parcel(
        client_id: Uuid,
        stream: SplitStream<WebSocket>,
    ) -> impl Stream<Item = Result<Parcel>> {
        stream
            .take_while(|message| {
                future::ready(if let Ok(message) = message {
                    message.is_text()
                } else {
                    false
                })
            })
            .map(move |message| match message {
                Err(e) => Err(Error::WebSocketReadMessageError(e.to_string())),
                Ok(message) => {
                    let parcel: Parcel = serde_json::from_str(message.to_str().unwrap()).unwrap();
                    info!("Received(Client: {}): {:?}", client_id, parcel);

                    Ok(parcel)
                }
            })
    }

    pub fn write_into_ws_message<S, E>(
        client_id: Uuid,
        stream: S,
    ) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = Parcel, Error = E> + Stream<Item = std::result::Result<Parcel, E>>,
        E: std::error::Error,
    {
        stream
            .try_filter(move |parcel| match parcel.recipient_id {
                Some(recipient_id) => {
                    if recipient_id == client_id {
                        return future::ready(true);
                    }

                    future::ready(false)
                }
                None => future::ready(true),
            })
            .map_ok(|parcel| {
                let data = serde_json::to_string(&parcel).unwrap();

                warp::ws::Message::text(data)
            })
            .map_err(|e| Error::WebSocketReadMessageError(e.to_string()))
    }
}
