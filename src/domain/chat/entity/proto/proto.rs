use uuid::Uuid;

use super::{Output, Parcel};

#[derive(Clone, Debug)]
pub struct Proto<T>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    pub inner: T,
}

impl<T> Proto<T>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl Proto<Output> {
    pub fn new_output(parcel: Parcel, receiver: Uuid) -> Self {
        Self {
            inner: Output { parcel, receiver },
        }
    }

    pub fn poll_interval(receiver: Uuid) -> Self {
        Self {
            inner: Output {
                parcel: Parcel::Poll,
                receiver,
            },
        }
    }
}
