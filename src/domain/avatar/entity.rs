use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::file::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub id: Uuid,
    pub file: File,
}
