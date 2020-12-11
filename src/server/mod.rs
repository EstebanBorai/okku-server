use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time::Duration;
use warp::Filter;

use crate::service::Client;
use crate::database::get_db_conn;
use crate::hub::Hub;
use crate::middleware::{with_authorization, with_service};
use crate::proto::input::Input;
use crate::proto::parcel::Parcel;
use crate::service::{AuthService, Services};

mod handler;
pub mod http_response;

/// Max size for Avatar file. 3 MB in bytes
const MAX_AVATAR_IMAGE_SIZE: u64 = 3_000_000;

/// MSend server implementation
pub struct Server {
    port: u16,
    hub: Arc<Hub>,
}

/// Query parameters expected by the `/chat` WebSocket
/// endpoint
#[derive(Deserialize, Serialize)]
struct WebSocketQuery {
    pub token: String,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            hub: Arc::new(Hub::new(Some(Duration::from_secs(5)))),
        }
    }

    /// Initilizes application services, database connection and routes
    /// and serves the MSend server on the specified address
    pub async fn run(&self) {
        let (input_sender, input_receiver) = mpsc::unbounded_channel::<Parcel<Input>>();
        let hub = self.hub.clone();
        let db_conn = get_db_conn().await.unwrap();
        let services = Services::init(db_conn);

        let chat = warp::path("chat")
            .and(warp::ws())
            .and(warp::query())
            .and(warp::any().map(move || input_sender.clone()))
            .and(warp::any().map(move || hub.clone()))
            .map(
                move |ws: warp::ws::Ws,
                      query_params: WebSocketQuery,
                      input_sender: UnboundedSender<Parcel<Input>>,
                      hub: Arc<Hub>| {
                    ws.on_upgrade(move |web_socket| async move {
                        if let Ok(claims) = AuthService::verify_jwt_token(&query_params.token) {
                            tokio::spawn(Client::subscribe_client(hub, web_socket, input_sender, claims.user_id));
                        } else {
                            warp::reject::reject();
                        }
                    })
                },
            );

        let health = warp::path("health").and(warp::get().and_then(handler::health::check));
        let auth = warp::path("auth").and(
            warp::path("signup")
                .and(warp::post())
                .and(warp::body::json())
                .and(with_service(services.clone()))
                .and_then(handler::auth::signup)
                .or(warp::path("login")
                    .and(warp::get())
                    .and(warp::header::<String>("authorization"))
                    .and(with_service(services.clone()))
                    .and_then(handler::auth::login)),
        );

        let api = warp::path("api");
        let v1 = warp::path("v1");
        let users = api.and(v1).and(warp::path("users"));

        let users_avatar = users.and(warp::path("avatar"));

        let upload_avatar = users_avatar
            .and(warp::post())
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::path::param())
            .and(warp::multipart::form().max_length(MAX_AVATAR_IMAGE_SIZE))
            .and_then(handler::user::upload_avatar);

        let update_avatar = users_avatar
            .and(warp::put())
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::path::param())
            .and(warp::multipart::form().max_length(MAX_AVATAR_IMAGE_SIZE))
            .and_then(handler::user::upload_avatar);

        let download_avatar = users_avatar.and(
            warp::get()
                .and(with_authorization())
                .and(with_service(services.clone()))
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
}
