use crate::model::Uuid;
use serde::de::DeserializeOwned;
use tiberius::{Query, Row};

use crate::{
    ctx::Ctx,
    model::{error::QueryError, Error, ModelManager, Result},
};

use super::{
    TryFromRow, UserInfo, UserInfoBy, UserInfoCreated, UserInfoForAuth, UserInfoForCreate,
    UserInfoRecord,
};

pub struct UserInfoBmc;

impl UserInfoBmc {
    pub async fn get<E>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
    where
        E: UserInfoBy,
    {
        let mut client = mm.db().get().await?;
        let sql = "SELECT TOP 1 * FROM dbo.UserInfo WHERE UserInfoID=@P1";
        let mut query = Query::new(sql);
        query.bind(id);
        let row = query
            .query(&mut client)
            .await?
            .into_row()
            .await?
            .ok_or(Error::UserInfo(QueryError::DataNotFound))?;

        let user_info_e = E::try_from_row(row)?;

        Ok(user_info_e)
    }

    pub async fn first_by_username<E>(_ctx: &Ctx, mm: &ModelManager, username: &str) -> Result<E>
    where
        E: UserInfoBy,
    {
        let mut client = mm.db().get().await?;
        let sql = "SELECT TOP 1 * FROM dbo.UserInfo WHERE Username=@P1";
        let mut query = Query::new(sql);
        query.bind(username);
        let row = query
            .query(&mut client)
            .await?
            .into_row()
            .await?
            .ok_or(Error::UserInfo(QueryError::DataNotFound))?;

        let user_info_e = E::try_from_row(row)?;

        Ok(user_info_e)
    }

    // pub async fn first_by_id<E>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
    // where
    //     E: UserInfoBy,
    // {
    //     let mut client = mm.db().get().await?;
    //     let sql = "SELECT TOP 1 UserInfoID FROM dbo.UserInfo WHERE UserInfoID=@P1";
    //     let mut query = Query::new(sql);
    //     query.bind(id);
    //     let row = query
    //         .query(&mut client)
    //         .await?
    //         .into_row()
    //         .await?
    //         .ok_or(Error::UserInfo(QueryError::DataNotFound))?;

    //     let user_info_e = E::try_from_row(row)?;

    //     Ok(user_info_e)
    // }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: Uuid, password: &str) -> Result<()> {
        // FIXME: change to sql server
        // let db = mm.db();
        // let sql =
        //     "UPDATE ONLY user_info:&id SET password = &password update_by = user_info:&update_by update_on = time::now();";
        // let mut result = db
        //     .query(sql)
        //     .bind(("id", id))
        //     .bind(("password", password))
        //     .bind(("update_by", ctx.user_id()))
        //     .await?;

        // let _user_info: Option<UserInfo> = result.take(0)?;

        // Ok(())
        todo!()
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        user_info_for_create: UserInfoForCreate,
    ) -> Result<UserInfoRecord> {
        // FIXME: change to sql server
        // // Verify Username in DB
        // let user_info = UserInfoBmc::first_by_username::<UserInfoRecord>(
        //     &ctx,
        //     mm,
        //     &user_info_for_create.username,
        // )
        // .await?;
        // if let Some(_) = user_info {
        //     return Err(Error::UsernameAlreadyExists);
        // }
        // let validate_username =
        //     UserInfoBmc::validate_username(mm, &user_info_for_create.username).await?;
        // if !validate_username {
        //     return Err(Error::UsernameNotValidFormat);
        // }

        // let db = mm.db();

        // let user_id_create = ctx.user_id_thing();

        // let user_info_created = UserInfoCreated {
        //     username: &user_info_for_create.username,
        //     email: &user_info_for_create.username,
        //     name: user_info_for_create.name,
        //     password: user_info_for_create.password,
        //     create_by: &user_id_create,
        //     update_by: &user_id_create,
        // };

        // let mut created: Vec<UserInfoRecord> =
        //     db.create("user_info").content(user_info_created).await?;

        // let user_info = created.pop().ok_or(Error::DataNotFound)?;

        // Ok(user_info)
        todo!()
    }

    pub async fn validate_password(mm: &ModelManager, hash: &str, password: &str) -> Result<bool> {
        // FIXME: change to sql server
        // let db = mm.db();

        // let sql = "RETURN crypto::argon2::compare($hash, $pass)";

        // let mut result = db
        //     .query(sql)
        //     .bind(("hash", hash))
        //     .bind(("pass", password))
        //     .await?;

        // result
        //     .take::<Option<bool>>(0)?
        //     .ok_or(Error::CannotComparePasswordFromDB)
        todo!()
    }

    pub async fn validate_username(mm: &ModelManager, username: &str) -> Result<bool> {
        // FIXME: change to sql server
        // let db = mm.db();

        // let sql = "RETURN string::is::email($username);";

        // let mut result = db.query(sql).bind(("username", username)).await?;

        // result
        //     .take::<Option<bool>>(0)?
        //     .ok_or(Error::CannotValidateUsernameFromDB)
        todo!()
    }
}

// FIXME: change to sql server
// // region:    --- Tests
// #[cfg(test)]
// mod tests {
//     pub type Result<T> = core::result::Result<T, Error>;
//     pub type Error = Box<dyn std::error::Error>; // For tests.
//     use crate::model::{self, user_info::UserInfoForAuth};

//     use super::*;
//     use serial_test::serial;

//     #[serial]
//     #[tokio::test]
//     async fn test_first_ok_demo1() -> Result<()> {
//         // -- Setup & Fixtures
//         let mm = model::ModelManager::new().await?;
//         let ctx = Ctx::root_ctx();
//         let fx_username = "demo1";

//         // -- Exec
//         let user = UserInfoBmc::first_by_username::<UserInfoForAuth>(&ctx, &mm, fx_username)
//             .await?
//             .ok_or("Should have user 'demo1'")?;

//         // -- Check
//         assert_eq!(user.username, fx_username);

//         Ok(())
//     }
// }
// // endregion: --- Tests
