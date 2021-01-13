use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Kind {
  Message,
  Ping,
}

#[derive(Debug, Clone)]
pub struct Parcel {
  pub kind: Kind,
  pub inner: Option<Vec<u8>>,
  pub client_id: Option<Uuid>,
}

impl Parcel {
  pub fn ping() -> Self {
    Self {
      kind: Kind::Ping,
      inner: None,
      client_id: None,
    }
  }

  pub fn message(client_id: &Uuid, bytes: &[u8]) -> Self {
    Self {
      kind: Kind::Message,
      inner: Some(bytes.to_vec()),
      client_id: Some(client_id.to_owned()),
    }
  }
}
