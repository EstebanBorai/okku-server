use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use warp::http::{self, StatusCode};
use warp::Filter;

use crate::application::service::Services;
use crate::domain::chat::{Input, Proto};
use crate::infrastructure::database::{get_db_pool, ping};
use crate::server::utils::Response;

use super::handler;
use super::middleware::{with_authorization, with_service};

const MAX_FILE_SIZE: u64 = 1_000_000;

/// Query parameters expected by the `/chat` WebSocket
/// endpoint
#[derive(Deserialize, Serialize)]
struct ChatQueryParams {
    pub token: String,
}

pub struct Http {
    pub port: u16,
}

impl Http {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn serve(&self) {
        ping().await.expect("Unable to PING Database");

        let db_pool = get_db_pool().await;
        let (chat_input_tx, chat_input_rx) = unbounded_channel::<Proto<Input>>();
        let services = Services::init(db_pool);
        let cors = warp::cors()
            .allow_any_origin()
            .allow_credentials(true)
            .allow_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .allow_methods(&[
                http::Method::GET,
                http::Method::OPTIONS,
                http::Method::POST,
                http::Method::PUT,
            ]);

        let api = warp::path("api");
        let api_v1 = api.and(warp::path("v1"));

        // API V1 Filters
        let auth = api_v1.and(warp::path("auth"));
        let chats = api_v1.and(warp::path("chats"));
        let files = api_v1.and(warp::path("files"));
        let profiles = api_v1.and(warp::path("profiles"));

        let chat_web_socket = chats
            .and(warp::ws::ws())
            .and(with_service(services.clone()))
            .and(warp::query())
            .and(warp::any().map(move || chat_input_tx.clone()))
            .map(
                move |ws: warp::ws::Ws,
                      services: Services,
                      qparams: ChatQueryParams,
                      chat_input_tx: UnboundedSender<Proto<Input>>| {
                    ws.on_upgrade(move |web_socket| async move {
                        if let Ok(claims) = services.auth_service.verify_token(&qparams.token).await
                        {
                            match services
                                .hub_service
                                .register_and_listen(&claims.user_id, web_socket, chat_input_tx)
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => {}
                            }
                        } else {
                            Response::message("Invalid token param provided".to_string())
                                .status_code(StatusCode::FORBIDDEN)
                                .reject();
                        }
                    })
                },
            );

        let signup = auth
            .and(warp::path("signup"))
            .and(warp::body::json())
            .and(with_service(services.clone()))
            .and_then(handler::auth::signup);

        let login = auth
            .and(warp::path("login"))
            .and(warp::header::<String>("authorization"))
            .and(with_service(services.clone()))
            .and_then(handler::auth::login);

        let me = auth
            .and(warp::path("me"))
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and_then(handler::auth::me);

        let upload_file = files
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::multipart::form().max_length(MAX_FILE_SIZE))
            .and_then(handler::files::upload);

        let download_file = files
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::path::param())
            .and_then(handler::files::download);

        let upload_avatar = profiles
            .and(warp::path("avatar"))
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::multipart::form().max_length(MAX_FILE_SIZE))
            .and_then(handler::profiles::upload_avatar);

        let create_chat = chats
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::body::json())
            .and_then(handler::chats::create_chat);

        let get_routes = warp::get().and(login.or(me.or(download_file)));
        let post_routes =
            warp::post().and(signup.or(upload_file).or(upload_avatar).or(create_chat));
        let routes = chat_web_socket.or(get_routes.or(post_routes));
        let routes = routes.recover(handler::rejection::handle_rejection);

        let serve_process = warp::serve(routes.with(cors)).bind(([127, 0, 0, 1], self.port));
        let chat_hub_process = services.hub_service.init(chat_input_rx);

        tokio::select! {
            _ = serve_process => {},
            _ = chat_hub_process => {},
        }
    }
}
