pub mod bmc;

use crate::convert::TryFromRow;
use lib_sqlserver_derive::TryFromRow;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tiberius::time::time::PrimitiveDateTime;
use tiberius::{Row, Uuid};

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<PrimitiveDateTime>,

    // -- pwd and token info
    pub password: Option<String>,
    pub password_salt: Option<Uuid>,
    pub token_salt: Option<Uuid>,
    pub create_by: Option<Uuid>,
    pub create_on: Option<PrimitiveDateTime>,
    pub update_by: Option<Uuid>,
    pub update_on: Option<PrimitiveDateTime>,
}

#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoGet {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<PrimitiveDateTime>,
    pub name: Option<String>,
    pub create_by: Option<Uuid>,
    pub create_on: Option<PrimitiveDateTime>,
    pub update_by: Option<Uuid>,
    pub update_on: Option<PrimitiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct UserInfoForCreate {
    pub username: String,
    pub email: String,
    pub email_verified: PrimitiveDateTime,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoCreated {
    pub username: String,
    pub email: String,
    pub email_verified: PrimitiveDateTime,
    pub name: String,
    pub password: String,
    pub create_by: Option<Uuid>,
    pub update_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoForLogin {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>, // encrypted, #_scheme_id_#....
    pub password_salt: Option<Uuid>,
    pub token_salt: Option<Uuid>,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoForAuth {
    pub id: Option<Uuid>,
    pub username: Option<String>,

    // -- token info
    pub token_salt: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoRecord {
    pub id: Option<Uuid>,
}

/// Marker Trait
pub trait UserInfoBy: DeserializeOwned + TryFromRow<Row> {}

impl UserInfoBy for UserInfoForAuth {}
impl UserInfoBy for UserInfoGet {}
impl UserInfoBy for UserInfoForLogin {}
