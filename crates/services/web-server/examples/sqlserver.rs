use lib_sqlserver::{
    config::core_config,
    ctx::Ctx,
    model::{
        user_info::{bmc::UserInfoBmc, UserInfo, UserInfoCreated, UserInfoForLogin},
        ModelManager,
    },
};

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
    // let mut conn = db.get().await.unwrap();
    // let res = conn
    //     .simple_query("SELECT @@version")
    //     .await
    //     .unwrap()
    //     .into_first_result()
    //     .await
    //     .unwrap()
    //     .into_iter()
    //     .map(|row| {
    //         let val: &str = row.get(0).unwrap();
    //         String::from(val)
    //     })
    //     .collect::<Vec<_>>();

    // let res = conn
    //     .simple_query("SELECT GETDATE() as date_time;")
    //     .await
    //     .unwrap()
    //     .into_row()
    //     .await
    //     .unwrap()
    //     .unwrap();

    // println!("{:?}", &res);
}

// fn main() {
//     println!("Hello from sqlserveq");
// }
