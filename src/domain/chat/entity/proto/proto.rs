use serde::{Deserialize, Serialize};

use super::{Output, Parcel};

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub fn new_output(parcel: Parcel) -> Self {
        Self {
            inner: Output { parcel },
        }
    }

    pub fn poll_interval() -> Self {
        Self {
            inner: Output {
                parcel: Parcel::Poll,
            },
        }
    }
}
