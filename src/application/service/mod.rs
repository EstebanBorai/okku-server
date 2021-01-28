use std::sync::Arc;
use tokio::sync::broadcast::Sender;

use crate::domain::chat::{HubService, Parcel};
use crate::infrastructure::database::DbPool;

mod auth;
mod avatar;
mod file;
mod hub;
mod profile;
mod secret;
mod user;

pub use auth::*;
pub use avatar::*;
pub use file::*;
pub use hub::*;
pub use profile::*;
pub use secret::*;
pub use user::*;

#[derive(Clone)]
pub struct Services {
    pub avatar_service: Arc<avatar::AvatarService>,
    pub hub_service: Arc<HubService>,
    pub user_service: Arc<user::UserService>,
    pub secret_service: Arc<secret::SecretService>,
    pub auth_service: Arc<auth::AuthService>,
    pub file_service: Arc<file::FileService>,
    pub profile_service: Arc<profile::ProfileService>,
}

impl Services {
    pub fn init(db_pool: &'static DbPool, _: Sender<Parcel>) -> Self {
        let hub_service = Arc::new(hub::make_hub_service(db_pool));
        let secret_service = Arc::new(secret::make_secret_service(db_pool));
        let file_service = Arc::new(file::make_file_service(db_pool));
        let profile_service = Arc::new(profile::make_profile_service(db_pool));
        let user_service = Arc::new(user::make_user_service(
            db_pool,
            profile_service.clone(),
            secret_service.clone(),
        ));
        let avatar_service = Arc::new(avatar::make_avatar_service(db_pool, file_service.clone()));
        let auth_service = Arc::new(auth::make_auth_service(secret_service.clone()));

        Self {
            avatar_service,
            hub_service,
            user_service,
            secret_service,
            auth_service,
            file_service,
            profile_service,
        }
    }
}
