use anyhow::{Error, Result};
use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use uuid::Uuid;
use warp::filters::ws::WebSocket;

use crate::proto::input::Input;
use crate::proto::output::Output;
use crate::proto::parcel::Parcel;

#[derive(Clone, Copy, Default)]
pub struct Client {
    pub id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    /// Reads the incoming `Stream` and attempts to
    /// deserialize them into a `InputParcel`
    pub fn read_input(
        &self,
        stream: SplitStream<WebSocket>,
    ) -> impl Stream<Item = Result<Parcel<Input>>> {
        let client_id = self.id;

        stream
            .take_while(|message| {
                future::ready(if let Ok(message) = message {
                    message.is_text()
                } else {
                    false
                })
            })
            .map(move |message| match message {
                Err(err) => Err(Error::new(err)),
                Ok(message) => {
                    let input = serde_json::from_str(message.to_str().unwrap()).unwrap();
                    info!("Received: {:?}", input);

                    Ok(Parcel::new(client_id, input))
                }
            })
    }

    pub fn write_output<S, E>(&self, stream: S) -> impl Stream<Item = Result<warp::ws::Message>>
    where
        S: TryStream<Ok = Parcel<Output>, Error = E> + Stream<Item = Result<Parcel<Output>, E>>,
        E: std::error::Error,
    {
        let client_id = self.id;

        stream
            .try_filter(move |output_parcel| future::ready(output_parcel.id == client_id))
            .map_ok(|output_parcel| {
                let data = serde_json::to_string(&output_parcel.payload).unwrap();

                warp::ws::Message::text(data)
            })
            .map_err(|err| Error::msg(err.to_string()))
    }
}
