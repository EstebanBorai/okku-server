use futures::{StreamExt, TryStreamExt};
use std::default::Default;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::delay_for;
use uuid::Uuid;
use warp::ws::WebSocket;

use crate::application::service::UserService;
use crate::domain::chat::dto::InputProtoMessageDTO;
use crate::domain::chat::entity::{Client, Input, Message, Output, Parcel, Proto};
use crate::domain::chat::ChatRepository;
use crate::error::Result;

use super::chat::ChatProvider;

pub struct HubService {
    pub output_tx: Sender<Proto<Output>>,
    pub chat_provider: ChatProvider,
    pub user_service: Arc<UserService>,
    clients: Vec<Client>,
}

impl HubService {
    pub fn new(chat_repository: ChatRepository, user_service: Arc<UserService>) -> Self {
        let (output_tx, _) = channel(16_usize);

        Self {
            output_tx,
            clients: Vec::new(),
            chat_provider: ChatProvider::new(chat_repository),
            user_service,
        }
    }

    /// Registers a new client (User) to the `Hub` and forwards messages
    /// from the Hub's main channel to the client's WebSocket sink.
    pub async fn register_and_listen(
        &self,
        user_id: &Uuid,
        web_socket: WebSocket,
        input_tx: UnboundedSender<Proto<Input>>,
    ) -> Result<()> {
        let output_rx = self.subscribe();
        let (sink, stream) = web_socket.split();
        let user = self.user_service.find_by_id(user_id).await?;
        let client = Client::from(user.clone());

        let read_process = client.read_input(stream).try_for_each(|proto| async {
            input_tx.send(proto).unwrap();
            Ok(())
        });

        let (client_output_tx, client_output_rx) = unbounded_channel();

        tokio::spawn(client_output_rx.forward(sink));

        let write_process =
            client
                .write_output(output_rx.into_stream())
                .try_for_each(|proto| async {
                    client_output_tx.send(Ok(proto)).unwrap();
                    Ok(())
                });

        if let Err(err) = tokio::select! {
            res = read_process => res,
            res = write_process => res,
        } {}

        Ok(())
    }

    /// Subscribes a client (User) to the Hub's main `channel`
    /// retrieving a `Receiver` of the channel
    pub fn subscribe(&self) -> Receiver<Proto<Output>> {
        self.output_tx.subscribe()
    }

    /// Initializes a `loop` to publish a `Proto<Output>` which wraps
    /// a `Poll` kind (Polling Interval Representation) which acts as an
    /// alive signal for Hub clients
    pub async fn poll(&self) {
        let five_seconds = Duration::from_secs(5);

        loop {
            delay_for(five_seconds).await;
            for client in self.clients.iter() {
                self.publish(Proto::poll_interval());
            }
        }
    }

    /// Handles `Proto<Input>` instances coming from the WebSocket stream
    pub async fn handle_input_proto(&self, input_proto: Proto<Input>) {
        match input_proto.inner {
            Input(incoming_message_dto) => {
                self.handle_input_message(incoming_message_dto).await;
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
    pub async fn handle_input_message(&self, incoming_message: InputProtoMessageDTO) {
        match self
            .chat_provider
            .handle_incoming_message(incoming_message)
            .await
        {
            Ok(message) => {
                self.publish_to_chat(message).await;
                return;
            }
            Err(e) => {},
        }
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
