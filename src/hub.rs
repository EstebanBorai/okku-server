use crate::model::{Feed, Message, User};
use crate::proto::input::{Input, JoinInput, PostInput};
use crate::proto::output::{
    JoinedOutput, MessageOutput, Output, OutputError, PostedOutput, UserJoinedOutput,
    UserLeftOutput, UserOutput, UserPostedOutput,
};
use crate::proto::parcel::Parcel;
use chrono::Utc;
use futures::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{broadcast, RwLock};
use tokio::time;
use uuid::Uuid;

const MAX_MESSAGE_BODY_SIZE: usize = 256;
const OUTPUT_CHANNEL_SIZE: usize = 16;

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new("[A-Za-z\\s{4,24}]").unwrap();
}

pub struct Hub {
    pub alive_interval: Option<Duration>,
    pub output_sender: broadcast::Sender<Parcel<Output>>,
    pub users: RwLock<HashMap<Uuid, User>>,
    pub feed: RwLock<Feed>,
}

impl Hub {
    pub fn new(alive_interval: Option<Duration>) -> Self {
        let (output_sender, _) = broadcast::channel(OUTPUT_CHANNEL_SIZE);

        Self {
            alive_interval,
            output_sender,
            users: Default::default(),
            feed: Default::default(),
        }
    }

    /// Sends an `Output` to all joined users
    pub async fn send(&self, output: Output) {
        if self.output_sender.receiver_count() == 0 {
            return;
        }

        self.users.read().await.keys().for_each(|user_id| {
            self.output_sender
                .send(Parcel::new(*user_id, output.clone()))
                .unwrap();
        });
    }

    /// Sends an `Output` to one `User`
    pub fn send_targeted(&self, client_id: Uuid, output: Output) {
        if self.output_sender.receiver_count() > 0 {
            self.output_sender
                .send(Parcel::new(client_id, output))
                .unwrap();
        }
    }

    /// Sends an `Output` to all `Users` but the one ignored
    pub async fn send_ignored(&self, ignore: Uuid, output: Output) {
        if self.output_sender.receiver_count() == 0 {
            return;
        }

        self.users
            .read()
            .await
            .values()
            .filter(|user| user.id != ignore)
            .for_each(|user| {
                self.output_sender
                    .send(Parcel::new(user.id, output.clone()))
                    .unwrap();
            });
    }

    pub fn send_error(&self, client_id: Uuid, error: OutputError) {
        self.send_targeted(client_id, Output::Error(error));
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Parcel<Output>> {
        self.output_sender.subscribe()
    }

    /// Sends an `Output` to all the `Users` notifying an `User`
    /// disconnected
    pub async fn on_disconnect(&self, client_id: Uuid) {
        if self.users.write().await.remove(&client_id).is_some() {
            self.send_ignored(client_id, Output::UserLeft(UserLeftOutput::new(client_id)))
                .await;
        }
    }

    /// If an alive interval is defined will initialize a loop to
    /// broadcast an `Alive` `Output` on every `n` time
    pub async fn tick_alive(&self) {
        if let Some(alive_interval) = self.alive_interval {
            loop {
                time::delay_for(alive_interval).await;
                self.send(Output::Alive).await;
            }
        }
    }

    /// Creates futures for both, `tick_alive` and processes every
    /// item from `receiver`.
    ///
    /// This function will await for one of the created futures to
    /// finish, either `ticking_alive` and `process` and will delegate
    /// to `process` every `Input` received
    pub async fn run(&self, receiver: UnboundedReceiver<Parcel<Input>>) {
        let ticking_alive = self.tick_alive();
        let processing = receiver.for_each(|input_parcel| self.process(input_parcel));

        tokio::select! {
            _ = ticking_alive => {},
            _ = processing => {},
        }
    }

    /// Processes `Input` values and derive them to its handlers
    async fn process(&self, input_parcel: Parcel<Input>) {
        match input_parcel.payload {
            Input::Join(input) => self.process_join(input_parcel.id, input).await,
            Input::Post(input) => self.process_post(input_parcel.id, input).await,
        }
    }

    async fn process_join(&self, client_id: Uuid, input: JoinInput) {
        let user_name = input.name.trim();

        if self
            .users
            .read()
            .await
            .values()
            .any(|user| user.name == user_name)
        {
            // check for the provided username not to be taken
            self.send_error(client_id, OutputError::NameTaken);
            return;
        }

        if !USERNAME_REGEX.is_match(user_name) {
            self.send_error(client_id, OutputError::InvalidName);
            return;
        }

        let user = User::new(client_id, user_name);

        self.users.write().await.insert(client_id, user.clone());

        let user_output = UserOutput::new(client_id, user_name);
        let other_users = self
            .users
            .read()
            .await
            .values()
            .filter_map(|user| {
                if user.id != client_id {
                    Some(UserOutput::new(user.id, &user.name))
                } else {
                    None
                }
            })
            .collect();

        let messages = self
            .feed
            .read()
            .await
            .messages_iter()
            .map(|message| {
                MessageOutput::new(
                    message.id,
                    UserOutput::new(message.id, &message.user.name),
                    &message.body,
                    message.created_at,
                )
            })
            .collect();

        self.send_targeted(
            client_id,
            Output::Joined(JoinedOutput::new(
                user_output.clone(),
                other_users,
                messages,
            )),
        );

        self.send_ignored(
            client_id,
            Output::UserJoined(UserJoinedOutput::new(user_output)),
        )
        .await;
    }

    /// Verifies an `User` exists, validates the `PostInput`
    /// and then proceeds to broadcast the `PostInput`
    async fn process_post(&self, client_id: Uuid, input: PostInput) {
        let user = if let Some(user) = self.users.read().await.get(&client_id) {
            user.clone()
        } else {
            self.send_error(client_id, OutputError::NotJoined);
            return;
        };

        if input.body.is_empty() || input.body.len() > MAX_MESSAGE_BODY_SIZE {
            self.send_error(client_id, OutputError::InvalidMessageBody);
            return;
        }

        let message = Message::new(Uuid::new_v4(), user.clone(), &input.body, Utc::now());

        self.feed.write().await.add_message(message.clone());

        let message_output = MessageOutput::new(
            message.id,
            UserOutput::new(user.id, &user.name),
            &message.body,
            message.created_at,
        );

        self.send_targeted(
            client_id,
            Output::Posted(PostedOutput::new(message_output.clone())),
        );
        self.send_ignored(
            client_id,
            Output::UserPosted(UserPostedOutput::new(message_output)),
        )
        .await;
    }
}
