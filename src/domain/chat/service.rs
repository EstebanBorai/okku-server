use futures::future;
use futures::stream::SplitStream;
use futures::{Stream, StreamExt, TryStream, TryStreamExt};
use serde_json::from_str as from_json_str;
use std::collections::HashMap;
use std::str::from_utf8;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::RwLock;
use tokio::time::delay_for;
use uuid::Uuid;
use warp::ws::WebSocket;

use crate::domain::chat::{Chat, IncomingMessageDTO, Kind, Parcel};
use crate::domain::user::User;
use crate::error::{Error, Result};
use crate::infrastructure::repository::chat::Repository;

use super::{ChatRepository, HistoryMessageDTO};

pub struct ChatService<R>
where
    R: ChatRepository,
{
    channel: Sender<Parcel>,
    chat_repository: R,
    chats: RwLock<HashMap<Uuid, Vec<Uuid>>>,
}

impl<R> ChatService<R>
where
    R: ChatRepository,
{
    pub fn new(tx: Sender<Parcel>, chat_repository: R) -> Self {
        Self {
            channel: tx,
            chat_repository,
            chats: RwLock::new(HashMap::new()),
        }
    }

    /// Initializes polling and handles every incoming message
    /// from the `Receiver` provider
    pub async fn run(&self, rx: Receiver<Parcel>) {
        let ticking_alive = self.poll();
        let processing = rx.for_each(|p| self.handle(p.unwrap()));

        tokio::select! {
            _ = ticking_alive => {},
            _ = processing => {},
        }
    }

    /// Registers a WebSocket into the chat along with the `user_id`.
    /// WebSocket's `Sink` and `Stream` are retrieved in order to forward
    /// and retrieve messages
    pub async fn register(&self, user_id: Uuid, web_socket: WebSocket) {
        let channel_rx = self.subscribe();
        let (sink, stream) = web_socket.split();
        let user_chats = self.retrieve_user_chats(&user_id).await.unwrap();
        let user_chats: Vec<Uuid> = user_chats.iter().map(|c| c.id).collect();

        self.chats.write().await.insert(user_id, user_chats.clone());

        info!("User({}): Registered", user_id);

        let read_process = ChatService::<Repository>::read_into_parcel(user_id, stream)
            .try_for_each(|p| async {
                info!("Parcel Received: {:?}", p);
                self.publish(p);
                Ok(())
            });

        let (utx, urx) = unbounded_channel();
        spawn(urx.forward(sink));

        let write_process = ChatService::<Repository>::write_into_ws_message(
            channel_rx
                .filter(|streamee| {
                    if let Ok(parcel) = streamee {
                        return match parcel.chat_id {
                            Some(parcel_chat_id) => {
                                if user_chats.iter().any(|cid| *cid == parcel_chat_id) {
                                    return future::ready(true);
                                }

                                future::ready(false)
                            }
                            None => future::ready(true),
                        };
                    }

                    future::ready(false)
                })
                .into_stream(),
        )
        .try_for_each(|message| async {
            utx.send(Ok(message)).unwrap();
            Ok(())
        });

        if let Err(e) = tokio::select! {
            result = read_process => result,
            result = write_process => result,
        } {
            error!("User({}): Connection Error! ({})", user_id, e.to_string());
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
        self.channel.send(parcel).expect("Unable to send parcel");
    }

    pub async fn handle(&self, parcel: Parcel) {
        match parcel.kind {
            Kind::Message => {
                let data = parcel.data.clone();
                if let Some(message_as_bytes) = data {
                    let message_json = from_utf8(message_as_bytes.as_slice()).unwrap();
                    let message: IncomingMessageDTO = from_json_str(message_json).unwrap();

                    info!("Im storing!");
                    self.store_message(&message).await.unwrap();
                    // self.publish(parcel)
                }

                return;
            }
            Kind::Ping => {}
        }
    }

    pub async fn store_message(&self, incoming_message: &IncomingMessageDTO) -> Result<()> {
        self.chat_repository
            .append_to_chat_history(incoming_message)
            .await
    }

    pub async fn create_chat(&self, participants: Vec<User>) -> Result<Chat> {
        let chat = self.chat_repository.create_chat().await?;

        for participant in participants.iter() {
            match self
                .chat_repository
                .append_user_to_chat(&chat.id, &participant.id)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e.to_string());
                }
            }
        }

        Ok(chat)
    }

    pub async fn retrieve_user_chats(&self, user_id: &Uuid) -> Result<Vec<Chat>> {
        self.chat_repository.fetch_user_chats(user_id).await
    }

    pub async fn retrieve_chat_history(&self, chat_id: &Uuid) -> Result<Vec<HistoryMessageDTO>> {
        self.chat_repository.retrieve_chat_history(chat_id).await
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

    pub fn write_into_ws_message<S, E>(stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = Parcel, Error = E> + Stream<Item = std::result::Result<Parcel, E>>,
        E: std::error::Error,
    {
        stream
            .map_ok(|parcel| {
                let data = serde_json::to_string(&parcel).unwrap();

                warp::ws::Message::text(data)
            })
            .map_err(|e| Error::WebSocketReadMessageError(e.to_string()))
    }
}
