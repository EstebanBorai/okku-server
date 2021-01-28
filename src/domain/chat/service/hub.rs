use futures::StreamExt;
use std::default::Default;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::delay_for;
use uuid::Uuid;
use warp::ws::WebSocket;

use crate::application::service::UserService;
use crate::domain::chat::entity::{Input, Message, Output, Parcel, Proto};
use crate::domain::chat::ChatRepository;
use crate::domain::user::User;
use crate::error::Result;

use super::chat::ChatProvider;

pub struct HubService {
    pub output_tx: Sender<Proto<Output>>,
    pub chat_provider: ChatProvider,
    pub user_service: Arc<UserService>,
    users: Vec<User>,
}

impl HubService {
    pub fn new(chat_repository: ChatRepository, user_service: Arc<UserService>) -> Self {
        let (output_tx, _) = channel(16_usize);

        Self {
            output_tx,
            users: Vec::new(),
            chat_provider: ChatProvider::new(chat_repository),
            user_service,
        }
    }

    /// Registers a new client (User) to the `Hub` and forwards messages
    /// from the Hub's main channel to the client's WebSocket sink.
    pub async fn register(&self, user_id: &Uuid, web_socket: WebSocket) -> Result<()> {
        let output_rx = self.subscribe();
        let (sink, stream) = web_socket.split();
        let user = self.user_service.find_by_id(user_id).await?;

        todo!()
    }

    /// Subscribes a client (User) to the Hub's main `channel`
    /// retrieving a `Receiver` of the channel
    pub async fn subscribe(&self) -> Receiver<Proto<Output>> {
        self.output_tx.subscribe()
    }

    /// Initializes a `loop` to publish a `Proto<Output>` which wraps
    /// a `Poll` kind (Polling Interval Representation) which acts as an
    /// alive signal for Hub clients
    pub async fn poll(&self) {
        let five_seconds = Duration::from_secs(5);

        loop {
            delay_for(five_seconds).await;
            for user in self.users.iter() {
                self.publish(Proto::poll_interval());
            }
        }
    }

    /// Handles `Proto<Input>` instances coming from the WebSocket stream
    pub async fn handle_input_proto(&self, input_proto: Proto<Input>) {
        match input_proto.inner {
            Input::Message(inner) => {
                self.handle_input_message(inner);
            }
        }
    }

    /// Initializes polling process to send alive signals to
    /// subscribers and handles `Proto<Input>` incoming instances.
    ///
    /// `Proto<Input>` instances are expected from the `input_rx`.
    pub async fn init(&self, input_rx: UnboundedReceiver<Proto<Input>>) {
        let polling = self.poll();
        let input_handling = input_rx.for_each(|input_proto| self.handle_input_proto(input_proto));

        tokio::select! {
            _ = polling => {},
            _ = input_handling => {},
        }
    }

    /// Handles user incoming messages and publishes them to the
    /// respective chat.
    ///
    /// If the author doesn't belongs to the chat specified, then
    /// is the message is not published
    pub async fn handle_input_message(&self, incoming_message: IncomingMessage) {
        info!("Received input message: {:?}", incoming_message);
        if let Ok(message) = self
            .chat_provider
            .handle_incoming_message(incoming_message.message)
            .await
        {
            self.publish_to_chat(message).await;
            return;
        }

        // Handle error someway
    }

    /// Publishes an `Proto<Output>` to Hub's main channel receiver.
    /// If there's no receivers (subscribers) available then the `Proto`
    /// is never sent through the channel
    pub async fn publish(&self, proto: Proto<Output>) {
        if self.output_tx.receiver_count() == 0 {
            return;
        }

        // safe to unwrap due tot available receivers
        // are checked before sending
        self.output_tx.send(proto).unwrap();
    }

    /// Sends an `Proto<Output>` which wraps a `Message` through the
    /// Hub's main `channel` filtering the Author of the message
    pub async fn publish_to_chat(&self, message: Message) {
        let chat = message.chat.clone();
        let message_author_id = message.author.id.clone();

        if self.output_tx.receiver_count() > 0 {
            for participant_id in chat.participants_ids.iter() {
                if *participant_id != message_author_id {
                    self.output_tx
                        .send(Proto::new_output(Parcel::LocalMessage(message.clone())))
                        .unwrap();
                }
            }
        }
    }
}
