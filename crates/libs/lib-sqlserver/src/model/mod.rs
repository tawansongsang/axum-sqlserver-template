mod conditions;
mod error;
mod store;
pub mod task;
pub mod user_info;

pub use self::conditions::*;
use tracing::log::info;

pub use self::error::{Error, Result};

use self::store::{new_db_pool, Db};

use serde::Deserialize;
use tiberius::time::time::PrimitiveDateTime;
use tiberius::{Row, Uuid};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Contructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    // pub(in crate::model) fn db(&self) -> &Db {
    pub fn db(&self) -> &Db {
        &self.db
    }

    pub async fn test_connection(&self) {
        info!(
            "{:<12} - test_connection - {}",
            "STARTUP", "trying to use pool for connecting to sql server"
        );

        let mut client = self.db().get().await.unwrap();

        let stream = client.simple_query("SELECT @@VERSION;").await.unwrap();

        let result = stream
            .into_first_result()
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let val: &str = row.get(0).unwrap();
                String::from(val)
            })
            .collect::<Vec<_>>();

        info!(
            "{:<12} - test_connection {}: \n{:?}",
            "STARTUP", "Sql Server Version: ", result
        );

        let stream = client
            .simple_query(
                "SELECT GETDATE() as CreateOn, NEWID() as ID, CAST(NULL as varchar) as Foo;",
            )
            .await
            .unwrap();

        let row = stream.into_row().await.unwrap().unwrap();
        println!("{:?}", row);

        let data = row.get::<PrimitiveDateTime, usize>(0);
        let date_time = data.unwrap();
        let data = row.get::<Uuid, usize>(1);
        let uuid = data.unwrap();

        println!("{:?}", date_time);
        println!("{:?}", uuid);

        let test = Test::try_from(row);
        println!("{:?}", test);
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, lib_sqlserver_derive::TryFromRow)]
pub struct Test {
    pub CreateOn: Option<PrimitiveDateTime>,
    pub ID: Option<Uuid>,
    pub Food: Option<String>,
}

// impl TryFrom<Row> for Test {
//     type Error = Error;

//     fn try_from(row: Row) -> Result<Self> {
//         Ok(Self {
//             CreateOn: row.try_get("CreateOn")?,
//             ID: row.try_get("ID")?,
//             Foo: row.try_get("Foo")?.map(str::to_string),
//         })
//     }
// }
