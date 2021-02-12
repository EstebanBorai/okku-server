use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub participants_ids: Vec<Uuid>,
}
