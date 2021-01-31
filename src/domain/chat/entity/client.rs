use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use uuid::Uuid;
use warp::filters::ws::WebSocket;

use crate::domain::chat::{Input, Output, Parcel, Proto};
use crate::domain::user::User;
use crate::error::{Error, Result};

/// A chat client which usually represents an
/// `User` but mostly represents any client which
/// is capable of send and receive messages
#[derive(Clone, Debug, Default)]
pub struct Client {
    pub user_id: Uuid,
    pub user_name: String,
}

impl From<User> for Client {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            user_name: user.name,
        }
    }
}

impl Client {
    pub fn read_input(
        &self,
        stream: SplitStream<WebSocket>,
    ) -> impl Stream<Item = Result<Proto<Input>>> {
        let user_id = self.user_id;

        stream
            .take_while(|message| {
                future::ready(if let Ok(message) = message {
                    message.is_text()
                } else {
                    false
                })
            })
            .map(move |message| match message {
                Err(err) => Err(Error::IO(err.to_string())),
                Ok(message) => {
                    let input: Proto<Input> =
                        serde_json::from_str(message.to_str().unwrap()).unwrap();

                    Ok(input)
                }
            })
    }

    pub fn write_output<S, E>(&self, stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = Proto<Output>, Error = E>
            + Stream<Item = std::result::Result<Proto<Output>, E>>,
        E: std::error::Error,
    {
        let user_id = self.user_id;

        stream
            .try_filter(
                move |output_proto| match output_proto.inner.parcel.clone() {
                    Parcel::LocalMessage(message) => {
                        if message.author.id == user_id {
                            return future::ready(false);
                        }

                        if message
                            .chat
                            .participants_ids
                            .iter()
                            .any(|pid| *pid == user_id)
                        {
                            return future::ready(true);
                        }

                        future::ready(false)
                    }
                    Parcel::Poll => future::ready(true),
                    _ => future::ready(false),
                },
            )
            .map_ok(|output_proto| {
                let data = serde_json::to_string(&output_proto.inner.parcel).unwrap();

                warp::ws::Message::text(data)
            })
            .map_err(|err| Error::IO(err.to_string()))
    }
}
