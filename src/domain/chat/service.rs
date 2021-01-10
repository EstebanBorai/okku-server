use futures::{Stream, StreamExt, TryStream, TryStreamExt};
use futures::future;
use futures::stream::{SplitStream, SplitSink};
use std::str::from_utf8;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::time::delay_for;
use uuid::Uuid;
use warp::ws::WebSocket;

use crate::domain::chat::{Kind, Parcel};
use crate::error::{Error, Result};

pub struct ChatService {
    channel: Sender<Parcel>,
}

impl ChatService {
    pub fn new() -> Self {
        let (tx, _) = channel(256_usize);

        Self {
            channel: tx,
        }
    }

    pub async fn register(&self, client_id: Uuid, web_socket: WebSocket) {
        let rx = self.subscribe().await;
        let (sink, stream) = web_socket.split();
        let (urx, utx) = unbounded_channel::<Parcel>();

        let read_process = ChatService::read_into_parcel(client_id, stream)
            .try_for_each(|parcel| async {
                // L77
            })
    }

    /// Publishes a `Ping` `Parcel` to use as singal of alive
    /// connection
    pub async fn poll(&self) {
        loop {
            delay_for(Duration::from_secs(5)).await;
            self.publish(Parcel::ping()).await;
        }
    }

    /// Iterates through every `Parcel` sent over a `UnboundedReceiver`
    /// and handles the `Parcel` for the given case
    pub async fn attach_rx(&self, rx: UnboundedReceiver<Parcel>) {
        let ticking_alive = self.poll();
        let dispatcher = rx.for_each(|p| self.handle(p));

        tokio::select! {
            _ = ticking_alive => {},
            _ = dispatcher => {},
        }
    }

    /// Subscribes to the Chat main channel and retrieves
    /// a `Receiver` of the channel to be consumed by the
    /// client in question
    pub async fn subscribe(&self) -> Receiver<Parcel> {
        self.channel.subscribe()
    }

    /// Publishes a `Parcel` to the Chat main channel
    pub async fn publish(&self, parcel: Parcel) {
        self.channel.send(parcel);
    }

    pub async fn handle(&self, parcel: Parcel) {
        match parcel.kind {
            Kind::Message => self.publish(parcel).await,
            _ => {},
        }
    }

    async fn read_into_parcel(client_id: Uuid, stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<Parcel>> {
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
                    let input = message.into_bytes();

                    Ok(Parcel::message(&client_id, &input))
                }
            })
    }

    pub fn write_into_ws_message<S, E>(client_id: Uuid, stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = Parcel, Error = E> + Stream<Item = std::result::Result<Parcel, E>>,
        E: std::error::Error,
    {
        stream
            // .try_filter(move |parcel| future::ready(parcel.client_id.unwrap() == client_id))
            .map_ok(|parcel| {
                let data = serde_json::to_string(&parcel.inner.unwrap()).unwrap();

                warp::ws::Message::text(data)
            })
            .map_err(|e| Error::WebSocketReadMessageError(e.to_string()))
    }

    fn bytes_as_utf8(&self, bytes: &[u8]) -> Result<String> {
        from_utf8(bytes).map(|s| s.to_string()).map_err(Error::from)
    }
}
