use uuid::Uuid;
use warp::http;
use warp::Filter;

use crate::application::service::Claims;
use crate::application::service::Services;
use crate::error::Result;
use crate::infrastructure::database::{get_db_pool, ping};

use super::handler;
use super::middleware::{with_authorization, with_service};

const MAX_FILE_SIZE: u64 = 1_000_000;

pub struct Http {
    pub(crate) port: u16,
}

impl Http {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn serve(&self) -> Result<()> {
        ping().await?;

        let db_pool = get_db_pool().await;
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
        let auth = api_v1.and(warp::path("auth"));

        let chat = api_v1.and(warp::path("chat"))
            .and(warp::ws())
            // .and(with_authorization())
            .and(with_service(services.clone()))
            .map(move |ws: warp::ws::Ws, services: Services| {
                ws.on_upgrade(move |web_socket| async move {
                    services.chat_service.register(Uuid::new_v4(), web_socket).await;
                })
            });

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

        let files = api_v1.and(warp::path("files"));

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

        let profiles = api_v1.and(warp::path("profiles"));

        let create_profile = profiles
            .and(with_authorization())
            .and(warp::body::json())
            .and(with_service(services.clone()))
            .and_then(handler::profiles::create);

        let upload_avatar = profiles
            .and(warp::path("avatar"))
            .and(with_authorization())
            .and(with_service(services.clone()))
            .and(warp::multipart::form().max_length(MAX_FILE_SIZE))
            .and_then(handler::profiles::upload_avatar);

        let get_routes = warp::get()
                .and(login.or(download_file));

        let post_routes = warp::post()
                .and(signup
                .or(create_profile)
                .or(upload_file)
                .or(upload_avatar));

        let routes = chat.or(get_routes).or(post_routes).with(cors);
        let routes = routes.recover(handler::rejection::handle_rejection);

        warp::serve(routes).bind(([127, 0, 0, 1], self.port)).await;

        Ok(())
    }
}
