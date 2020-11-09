use uuid::Uuid;

/// Container for a message either for
/// input or output
#[derive(Debug, Clone)]
pub struct Parcel<T>
where T: std::fmt::Debug + std::clone::Clone
{
  pub id: Uuid,
  pub payload: T,
}

impl<T> Parcel<T>
where T: std::fmt::Debug + std::clone::Clone
{
  pub fn new(id: Uuid, payload: T) -> Self {
    Self {
      id,
      payload,
    }
  }
}
