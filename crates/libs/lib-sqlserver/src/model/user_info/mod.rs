pub mod bmc;

use super::error::{Error, Result};
use lib_sqlserver_derive::TryFromRow;
use serde::{Deserialize, Serialize};
use tiberius::time::time::PrimitiveDateTime;
use tiberius::{Row, Uuid};

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub email_verified: PrimitiveDateTime,

    // -- pwd and token info
    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
    pub create_by: Uuid,
    pub create_on: PrimitiveDateTime,
    pub update_by: Uuid,
    pub update_on: PrimitiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoGet {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub email_verified: PrimitiveDateTime,
    pub name: String,
    pub create_by: Uuid,
    pub create_on: PrimitiveDateTime,
    pub update_by: Uuid,
    pub update_on: PrimitiveDateTime,
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

#[derive(Debug, Deserialize)]
pub struct UserInfoForLogin {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub password: Option<String>, // encrypted, #_scheme_id_#....
    pub password_salt: Uuid,
    pub token_salt: Uuid,
    pub role: String,
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
    pub id: Uuid,
}
