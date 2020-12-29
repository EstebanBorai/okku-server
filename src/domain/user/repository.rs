use async_trait::async_trait;

use crate::error::Result;

use super::User;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, name: &str) -> Result<User>;
    async fn find_by_name(&self, name: &str) -> Result<User>;
    async fn find_all(&self) -> Result<Vec<User>>;
}
