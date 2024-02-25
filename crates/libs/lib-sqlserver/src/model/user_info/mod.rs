pub mod bmc;

use crate::convert::TryFromRow;
use lib_sqlserver_derive::TryFromRow;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tiberius::time::time::PrimitiveDateTime;
use tiberius::{Row, Uuid};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, TryFromRow)]
pub struct UserInfo {
    pub UserInfoID: Option<Uuid>,
    pub Username: Option<String>,
    pub Name: Option<String>,
    pub Email: Option<String>,
    pub EmailVerified: Option<PrimitiveDateTime>,

    // -- pwd and token info
    pub Password: Option<String>,
    pub PasswordSalt: Option<Uuid>,
    pub TokenSalt: Option<Uuid>,
    pub Role: Option<String>,
    pub CreateBy: Option<Uuid>,
    pub CreateOn: Option<PrimitiveDateTime>,
    pub Active: Option<String>,
    pub UpdateBy: Option<Uuid>,
    pub UpdateOn: Option<PrimitiveDateTime>,
    pub Deleted: Option<String>,
    pub DeleteBy: Option<Uuid>,
    pub DeleteOn: Option<PrimitiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, TryFromRow)]
pub struct UserInfoGet {
    pub UserInfoID: Option<Uuid>,
    pub Username: Option<String>,
    pub Name: Option<String>,
    pub Email: Option<String>,
    pub EmailVerified: Option<PrimitiveDateTime>,
    pub Role: Option<String>,
    pub CreateBy: Option<Uuid>,
    pub CreateOn: Option<PrimitiveDateTime>,
    pub Active: Option<String>,
    pub UpdateBy: Option<Uuid>,
    pub UpdateOn: Option<PrimitiveDateTime>,
    pub Deleted: Option<String>,
    pub DeleteBy: Option<Uuid>,
    pub DeleteOn: Option<PrimitiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct UserInfoForCreate {
    pub Username: String,
    pub Email: String,
    // pub email_verified: PrimitiveDateTime,
    pub Name: String,
    pub Password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoCreated {
    pub PasswordSalt: Option<Uuid>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoForLogin {
    pub UserInfoID: Option<Uuid>,
    pub Username: Option<String>,
    pub Name: Option<String>,
    pub Password: Option<String>, // encrypted, #_scheme_id_#....
    pub PasswordSalt: Option<Uuid>,
    pub TokenSalt: Option<Uuid>,
    pub Role: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoForAuth {
    pub UserInfoID: Option<Uuid>,
    pub Username: Option<String>,

    // -- token info
    pub TokenSalt: Option<Uuid>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, TryFromRow)]
pub struct UserInfoRecord {
    pub UserInfoID: Option<Uuid>,
}

/// Marker Trait
pub trait UserInfoBy: DeserializeOwned + TryFromRow<Row> {}

impl UserInfoBy for UserInfoForAuth {}
impl UserInfoBy for UserInfoGet {}
impl UserInfoBy for UserInfo {}
impl UserInfoBy for UserInfoForLogin {}
impl UserInfoBy for UserInfoRecord {}
impl UserInfoBy for UserInfoCreated {}
