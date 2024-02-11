pub mod bmc;

use serde::{Deserialize, Serialize};
// use surrealdb::sql;
use tiberius::{time::DateTime2, Uuid};

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    // FIXME: change to sql server
    // pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    // pub email_verified: DateTime2,

    // -- pwd and token info
    pub password: String,
    // pub password_salt: Uuid,
    // pub token_salt: Uuid,
    // pub create_by: Uuid,
    // pub create_on: DateTime2,
    // pub update_by: Uuid,
    // pub update_on: Datetime2,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoGet {
    // FIXME: change to sql server
    // pub id: Uuid,
    // pub username: String,
    // pub email: String,
    // pub email_verified: Datetime2,
    // pub name: String,

    // pub create_by: Uuid,
    // pub create_on: DateTime2,
    // pub update_by: Uuid,
    // pub update_on: DateTime2,
}

#[derive(Debug, Serialize)]
pub struct UserInfoForCreate {
    pub username: String,
    // pub email: String,
    // pub email_verified: DateTime2,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoCreated<'a> {
    pub username: &'a str,
    pub email: &'a str,
    // pub email_verified: DateTime2,
    pub name: String,
    pub password: String,
    // FIXME: change to sql server
    // pub create_by: &'a Option<Uuid>,
    // pub update_by: &'a Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoForLogin {
    // FIXEME: change to sql server
    // pub id: Uuid,
    // pub username: String,
    // pub name: String,
    // pub password: Option<String>, // encrypted, #_scheme_id_#....
    // pub password_salt: Uuid,
    // pub token_salt: Uuid,
    // pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoForAuth {
    // FIXME: change to sql server
    // pub id: Uuid,
    // pub username: String,

    // // -- token info
    // pub token_salt: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoRecord {
    // FIXME: change to sql server
    // pub id: Uuid,
}
