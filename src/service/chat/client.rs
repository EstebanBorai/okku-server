use anyhow::{Error, Result};
use futures::stream::SplitStream;
use futures::{future, Stream, StreamExt, TryStream, TryStreamExt};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;
use warp::filters::ws::WebSocket;

use crate::hub::Hub;
use crate::proto::input::Input;
use crate::proto::output::Output;
use crate::proto::parcel::Parcel;

#[derive(Clone, Copy, Default)]
pub struct Client {
    pub id: Uuid,
}

impl Client {
    /// Creates a new chat `Client` with the provided UUID.
    /// The provided UUID must belong to an existent `User`
    pub fn new(id: Uuid) -> Self {
        Self { id }
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

    /// Subscribes a `Client` to the `Hub` in order to handle I/O
    /// operations on the `Client`
    pub async fn subscribe_client(
      hub: Arc<Hub>,
      web_socket: WebSocket,
      input_sender: UnboundedSender<Parcel<Input>>,
      user_id: Uuid,
  ) {
      let output_receiver = hub.subscribe();
      let (ws_sink, ws_stream) = web_socket.split();
      let client = Client::new(user_id);

      info!("Client (User ID: {}) connected", client.id);

      let reading = client
          .read_input(ws_stream)
          .try_for_each(|input_parcel| async {
              input_sender.send(input_parcel).unwrap();
              Ok(())
          });

      let (tx, rx) = mpsc::unbounded_channel();

      tokio::spawn(rx.forward(ws_sink));

      let writing = client
          .write_output(output_receiver.into_stream())
          .try_for_each(|message| async {
              tx.send(Ok(message)).unwrap();
              Ok(())
          });

      if let Err(err) = tokio::select! {
        result = reading => result,
        result = writing => result,
      } {
          error!("Client (User ID: {}) had an error {}", client.id, err);
      }

      hub.on_disconnect(client.id).await;
      info!("Client (User ID: {}) disconnected", client.id);
  }
}
