use mobc_postgres::tokio_postgres::types::FromSql;
use mobc_postgres::tokio_postgres::types::Type;
use serde::Serialize;
use std::str::from_utf8;
use std::string::ToString;
use uuid::Uuid;

/// Supported Avatar MIME types
#[derive(Serialize)]
pub enum AvatarMIMEType {
    Png,
    Jpeg,
}

#[derive(Debug)]
pub struct InvalidMIMEError(pub String);

impl std::fmt::Display for InvalidMIMEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidMIMEError {}

impl ToString for AvatarMIMEType {
    fn to_string(&self) -> String {
        match self {
            AvatarMIMEType::Png => String::from("image/png"),
            AvatarMIMEType::Jpeg => String::from("image/jpeg"),
        }
    }
}

impl<'a> FromSql<'a> for AvatarMIMEType {
    fn from_sql(_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let as_string = from_utf8(raw).unwrap();

        match as_string {
            "image/png" => Ok(AvatarMIMEType::Png),
            "image/jpeg" => Ok(AvatarMIMEType::Jpeg),
            _ => Err(Box::new(InvalidMIMEError(as_string.to_string()))),
        }
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::VARCHAR
    }
}

#[derive(Serialize)]
pub struct Avatar {
    pub id: Uuid,
    pub image: Vec<u8>,
    pub mime_type: AvatarMIMEType,
}
