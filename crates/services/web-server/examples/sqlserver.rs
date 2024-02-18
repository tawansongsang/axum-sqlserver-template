use std::str::FromStr;

use lib_sqlserver::{
    config::core_config,
    ctx::Ctx,
    model::{
        user_info::{bmc::UserInfoBmc, UserInfo, UserInfoCreated, UserInfoForLogin},
        ModelManager, Query,
    },
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("hello from example");

    let mm = ModelManager::new().await.unwrap();
    mm.test_connection().await;
    let ctx = Ctx::root_ctx();
    let username = "demo1";

    let db = mm.db();
    let config = core_config();
    // config.print_env();
    let mut client = db.get().await.unwrap();

    let trans_id = Uuid::from_str("ae20d4cf-2806-40f8-887b-a3b6aef03fd0")
        .unwrap()
        .to_string();
    // {
    let sql = format!("BEGIN TRANSACTION;");
    let row = client
        .simple_query(sql)
        .await
        .unwrap()
        .into_row()
        .await
        .unwrap();
    println!("begin transaction: {:?}", row);
    // }
    // {
    let sql = "SELECT * FROM dbo.UserInfo;";
    let row = client.simple_query(sql).await.unwrap().into_row().await;
    println!("in transaction: {:?}", row);
    // }
    {
        let sql = "COMMIT;";
        let row = client.simple_query(sql).await;
        println!("commit transaction: {:?}", row);
    }

    // let mut client3 = db.get().await.unwrap();
    // let sql = "COMMIT TRANSACTION Test;";
    // let row = client3.simple_query(sql).await;
    // println!("commit transaction: {:?}", row);
    // let mut client = db.get().await.unwrap();
    // let sql = "COMMIT TRANSACTION @P1";
    // let row2 = client.query(sql, &[&trans_id]).await.unwrap();
    // println!("commit transaction: {:?}", row2);
}
