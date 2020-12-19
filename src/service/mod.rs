use crate::database::DbConn;
use std::sync::Arc;

mod auth;
mod chat;
mod image;
mod user;

pub use auth::*;
pub use chat::*;
pub use image::*;
pub use user::*;

#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<user::UserService>,
    pub auth_service: Arc<auth::AuthService>,
    pub image_service: image::ImageService,
}

pub type InjectedServices = Arc<Services>;

impl Services {
    pub fn init(db_conn: DbConn) -> Arc<Self> {
        let db_conn = Arc::new(db_conn);
        let user_service = user::UserService::new(db_conn.clone());
        let auth_service = auth::AuthService::new(db_conn.clone(), user_service.clone());

        Arc::new(Services {
            user_service: Arc::new(user_service),
            auth_service: Arc::new(auth_service),
            image_service: ImageService::new(),
        })
    }
}
