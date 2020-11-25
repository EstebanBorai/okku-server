use crate::client::Client;
use crate::hub::Hub;
use crate::proto::input::Input;
use crate::proto::parcel::Parcel;
use futures::{StreamExt, TryStreamExt};
use log::{error, info};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time::Duration;
use warp::ws::WebSocket;
use warp::Filter;

mod handler;
mod http_response;

/// Max size for Avatar file. 3 MB in bytes
const MAX_AVATAR_IMAGE_SIZE: u64 = 3_000_000;

pub struct Server {
    port: u16,
    hub: Arc<Hub>,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            hub: Arc::new(Hub::new(Some(Duration::from_secs(5)))),
        }
    }

    pub async fn run(&self) {
        let (input_sender, input_receiver) = mpsc::unbounded_channel::<Parcel<Input>>();
        let hub = self.hub.clone();

        let chat = warp::path("chat")
            .and(warp::ws())
            .and(warp::any().map(move || input_sender.clone()))
            .and(warp::any().map(move || hub.clone()))
            .map(
                move |ws: warp::ws::Ws,
                      input_sender: UnboundedSender<Parcel<Input>>,
                      hub: Arc<Hub>| {
                    ws.on_upgrade(move |web_socket| async move {
                        tokio::spawn(Server::process_client(hub, web_socket, input_sender));
                    })
                },
            );

        let health = warp::path("health").and(warp::get().and_then(handler::health::check));
        let auth = warp::path("auth").and(
            warp::path("signup")
                .and(warp::post())
                .and(warp::body::json())
                .and_then(handler::auth::signup)
                .or(warp::path("login")
                    .and(warp::get())
                    .and(warp::header::<String>("authorization"))
                    .and_then(handler::auth::login)),
        );

        let api = warp::path("api");
        let v1 = warp::path("v1");
        let users = api.and(v1).and(warp::path("users"));

        let users_avatar = users.and(warp::path("avatar"));

        let upload_avatar = users_avatar
            .and(warp::post())
            .and(warp::path::param())
            .and(warp::multipart::form().max_length(MAX_AVATAR_IMAGE_SIZE))
            .and_then(handler::user::upload_avatar);

        let update_avatar = users_avatar
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::multipart::form().max_length(MAX_AVATAR_IMAGE_SIZE))
            .and_then(handler::user::replace_avatar);

        let download_avatar = users_avatar.and(
            warp::get()
                .and(warp::path::param())
                .and_then(handler::user::download_avatar),
        );

        let routes = chat
            .or(auth)
            .or(health)
            .or(upload_avatar)
            .or(download_avatar)
            .or(update_avatar);

        let shutdown = async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");
        };

        let (_, serving) =
            warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), shutdown);

        let running_hub = self.hub.run(input_receiver);

        tokio::select! {
          _ = serving => {},
          _ = running_hub => {},
        }
    }

    async fn process_client(
        hub: Arc<Hub>,
        web_socket: WebSocket,
        input_sender: UnboundedSender<Parcel<Input>>,
    ) {
        let output_receiver = hub.subscribe();
        let (ws_sink, ws_stream) = web_socket.split();
        let client = Client::new();

        info!("Client {} connected", client.id);

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
            error!("Client connection error: {}", err);
        }

        hub.on_disconnect(client.id).await;
        info!("Client {} disconnected", client.id);
    }
}
