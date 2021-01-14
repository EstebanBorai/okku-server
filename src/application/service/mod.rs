use std::sync::Arc;

use crate::domain::chat::ChatService;
use crate::infrastructure::database::DbPool;

mod auth;
mod avatar;
mod chat;
mod file;
mod profile;
mod secret;
mod user;

pub use auth::*;
pub use avatar::*;
pub use chat::*;
pub use file::*;
pub use profile::*;
pub use secret::*;
pub use user::*;

#[derive(Clone)]
pub struct Services {
    pub chat_service: Arc<ChatService>,
    pub avatar_service: Arc<avatar::AvatarService>,
    pub user_service: Arc<user::UserService>,
    pub secret_service: Arc<secret::SecretService>,
    pub auth_service: Arc<auth::AuthService>,
    pub file_service: Arc<file::FileService>,
    pub profile_service: Arc<profile::ProfileService>,
}

impl Services {
    pub fn init(db_pool: &'static DbPool) -> Self {
        let chat_service = Arc::new(chat::make_chat_service());
        let secret_service = Arc::new(secret::make_secret_service(db_pool));
        let file_service = Arc::new(file::make_file_service(db_pool));
        let profile_service = Arc::new(profile::make_profile_service(db_pool));
        let user_service = Arc::new(user::make_user_service(db_pool, profile_service.clone()));
        let avatar_service = Arc::new(avatar::make_avatar_service(db_pool, file_service.clone()));
        let auth_service = Arc::new(auth::make_auth_service(secret_service.clone()));

        Self {
            chat_service,
            avatar_service,
            user_service,
            secret_service,
            auth_service,
            file_service,
            profile_service,
        }
    }
}
