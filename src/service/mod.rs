use std::sync::Arc;

use crate::database::DbPool;

pub mod auth;
pub mod avatar;
pub mod chat;
pub mod image;
pub mod url;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub auth_service: Arc<auth::AuthService>,
    pub avatar_service: Arc<avatar::AvatarService>,
    pub image_service: Arc<image::ImageService>,
    pub url_service: Arc<url::UrlService>,
    pub user_service: Arc<user::UserService>,
}

pub type InjectedServices = Arc<Services>;

impl Services {
    pub fn init(db_pool: DbPool) -> Arc<Self> {
        let url_service = Arc::new(url::UrlService::new());
        let user_service = Arc::new(user::UserService::new(db_pool.clone()));
        let image_service = Arc::new(image::ImageService::new(
            db_pool.clone(),
            url_service.clone(),
        ));
        let auth_service = Arc::new(auth::AuthService::new(
            db_pool.clone(),
            user_service.clone(),
        ));
        let avatar_service = Arc::new(avatar::AvatarService::new(db_pool.clone(), image_service.clone()));

        Arc::new(Services {
            auth_service,
            avatar_service,
            image_service,
            url_service,
            user_service,
        })
    }
}
