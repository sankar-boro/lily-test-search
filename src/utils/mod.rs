mod query;

use uuid::Uuid;
pub use query::{GetQueryResult};
use crate::error::Error;

pub trait ParseUuid {
    fn to_uuid(self) -> Result<Uuid, Error>;
}

impl ParseUuid for &String {
    fn to_uuid(self) -> Result<Uuid, Error> {
        Ok(Uuid::parse_str(self)?)
    }
}