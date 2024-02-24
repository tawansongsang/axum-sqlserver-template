// use std::borrow::Cow;

use crate::model::conditions::{
    create_eq_con, create_str_contain_con, create_time_le_con, create_time_me_con,
};

use crate::convert::TryFromRow;
use lib_sqlserver_derive::TryFromRow;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tiberius::Row;
use time::PrimitiveDateTime;
use uuid::Uuid;
// use surrealdb::sql::{Datetime, Thing};

use super::Filter;

pub mod bmc;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, TryFromRow)]
pub struct Task {
    pub TaskID: Option<Uuid>,
    pub Title: Option<String>,
    pub Done: Option<bool>,
    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub CreateBy: Option<Uuid>,
    pub CreateOn: Option<PrimitiveDateTime>,
    pub UpdateBy: Option<Uuid>,
    pub UpdateOn: Option<PrimitiveDateTime>,
    pub Deleted: Option<String>,
    pub DeleteOn: Option<PrimitiveDateTime>,
    pub DeleteBy: Option<Uuid>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TaskParamsForCreate {
    pub Title: String,
}

// #[allow(non_snake_case)]
// #[derive(Serialize)]
// pub struct TaskForCreate {
//     pub Title: String,
//     pub CreateBy: Uuid,
//     pub UpdateBy: Uuid,
// }

#[allow(non_snake_case)]
#[derive(Deserialize, Default, TryFromRow)]
pub struct TaskParamsForUpdate {
    pub Title: Option<String>,
    pub Done: Option<bool>,
    // pub UpdateOn: Option<PrimitiveDateTime>,
    // pub UpdateBy: Uuid,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, TryFromRow)]
pub struct TaskRecord {
    pub TaskID: Option<Uuid>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct TaskFilter {
    pub TaskID: Option<String>, //Option<Uuid>
    pub Title: Option<String>,
    pub Done: Option<bool>,

    pub CreateBy: Option<String>,      //Option<Uuid>
    pub StartCreateOn: Option<String>, //Option<PrimitiveDateTime>
    pub EndCreateOn: Option<String>,   //Option<PrimitiveDateTime>
    pub UpdateBy: Option<String>,      //Option<Uuid>
    pub StartUpdateOn: Option<String>, //Option<PrimitiveDateTime>
    pub EndUpdateOn: Option<String>,   //Option<PrimitiveDateTime>
}

#[allow(non_snake_case)]
impl Filter for TaskFilter {
    fn gen_condition(&self) -> Vec<String> {
        let mut conditions = Vec::new();

        create_eq_con!(self, conditions, TaskID, TaskID);
        create_str_contain_con!(self, conditions, Title, Title);
        create_eq_con!(self, conditions, Done, Done);
        create_eq_con!(self, conditions, CreateBy, CreateBy);
        // create_time_me_con!(self, conditions, CreateOn, StartCreateOn);
        // create_time_le_con!(self, conditions, CreateOn, EndCreateOn);
        create_eq_con!(self, conditions, UpdateBy, UpdateBy);
        // create_time_me_con!(self, conditions, UpdateOn, StartUpdateOn);
        // create_time_le_con!(self, conditions, UpdateOn, EndUpdateOn);

        conditions
    }
}

/// Marker Trait
pub trait TaskBy: DeserializeOwned + TryFromRow<Row> {}

impl TaskBy for TaskRecord {}
impl TaskBy for Task {}
impl TaskBy for TaskParamsForUpdate {}
