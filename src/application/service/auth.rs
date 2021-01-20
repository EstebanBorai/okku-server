use std::sync::Arc;

use crate::domain::auth;
use crate::infrastructure::repository::secret::Repository;

use super::secret;

pub type AuthService = auth::AuthService<Repository>;

pub fn make_auth_service(secret_service: Arc<secret::SecretService>) -> AuthService {
    AuthService::new(secret_service)
}
