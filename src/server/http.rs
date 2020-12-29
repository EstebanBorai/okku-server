use warp::http;
use warp::Filter;

use crate::application::service::Services;
use crate::error::Result;
use crate::infrastructure::database::{get_db_pool, ping};

use super::handler;
use super::middleware::with_service;

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

        let signup = auth
            .and(warp::path("signup"))
            .and(warp::post())
            .and(warp::body::json())
            .and(with_service(services.clone()))
            .and_then(handler::auth::signup);

        let login = auth
            .and(warp::path("login"))
            .and(warp::get())
            .and(warp::header::<String>("authorization"))
            .and(with_service(services.clone()))
            .and_then(handler::auth::login);

        let routes = warp::any().and(signup.or(login)).with(cors);
        let routes = routes.recover(handler::rejection::handle_rejection);

        warp::serve(routes).bind(([127, 0, 0, 1], self.port)).await;

        Ok(())
    }
}
