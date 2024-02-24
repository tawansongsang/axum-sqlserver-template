use uuid::Uuid;

use crate::{
    convert::TryFromRow,
    ctx::Ctx,
    model::{conditions, task::TaskRecord, Error, ListOptions, ModelManager, Result},
};

use super::{Task, TaskBy, TaskFilter, TaskParamsForCreate, TaskParamsForUpdate};

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskParamsForCreate,
    ) -> Result<TaskRecord> {
        let mut client = mm.db().get().await?;
        let user_id = ctx.user_id().ok_or(Error::UserIdNotFound)?;

        {
            let sql = "BEGIN TRANSACTION;";
            let _stream = client.simple_query(sql).await?;
        }

        // TODO: Create sp for create task
        let sql = r#"
        INSERT INTO dbo.Task
            (Title, CreateBy , UpdateBy)
        OUTPUT Inserted.TaskID
        VALUES
            (@P1, @P2, @P3);
        "#;

        let row = client
            .query(sql, &[&task_c.Title, &user_id, &user_id])
            .await?
            .into_row()
            .await?
            .ok_or(Error::DataNotFoundFromCreated);
        let task_record = match row {
            Ok(r) => TaskRecord::try_from_row(r),
            Err(e) => {
                let sql = "ROLLBACK;";
                let _stream = client.simple_query(sql).await?;
                return Err(e);
            }
        };

        {
            let sql = "COMMIT;";
            let _stream = client.simple_query(sql).await?;
        }

        task_record
    }

    pub async fn get<E>(_ctx: &Ctx, mm: &ModelManager, task_id: &str) -> Result<E>
    where
        E: TaskBy,
    {
        let mut client = mm.db().get().await?;
        let sql = r#"
        SELECT *
        FROM dbo.Task
        WHERE TaskID = @P1;
        "#;

        let row = client
            .query(sql, &[&task_id])
            .await?
            .into_row()
            .await?
            .ok_or(Error::DataNotFoundFromCreated)?;

        let task_e = E::try_from_row(row);

        task_e
    }

    pub async fn list<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TaskFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<E>>
    where
        E: TaskBy,
    {
        let mut client = mm.db().get().await?;

        let conditions = conditions::gen_all_condition(filters, list_options);

        let sql = format!("SELECT * FROM dbo.Task {};", conditions);

        let row = client.simple_query(sql).await?.into_first_result().await?;

        let task_e: Result<Vec<E>> = row.into_iter().map(|r| E::try_from_row(r)).collect();

        task_e
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        task_id: &str,
        task_u: TaskParamsForUpdate,
    ) -> Result<()> {
        let mut client = mm.db().get().await?;
        let user_id = ctx.user_id().ok_or(Error::UserIdNotFound)?;

        {
            let sql = "BEGIN TRANSACTION;";
            let _stream = client.simple_query(sql).await?;
        }

        let sql = r#"
        UPDATE dbo.Task
        SET Title = @P2, Done = @P3, UpdateOn = GETDATE(), UpdateBy = @P4
        WHERE TaskID = @P1;
        "#;

        let rows_affected = client
            .execute(sql, &[&task_id, &task_u.Title, &task_u.Done, &user_id])
            .await;

        match rows_affected {
            Ok(r) => {
                if r.total() == 1 {
                    ()
                } else {
                    return Err(Error::UpdateError("TaskIDNotFoundForUpdate".to_string()));
                }
            }
            Err(e) => {
                let sql = "ROLLBACK;";
                let _stream = client.simple_query(sql).await?;
                return Err(Error::UpdateError(e.to_string()));
            }
        };

        {
            let sql = "COMMIT;";
            let _stream = client.simple_query(sql).await?;
        }
        Ok(())
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, task_id: &str) -> Result<()> {
        let mut client = mm.db().get().await?;

        {
            let sql = "BEGIN TRANSACTION;";
            let _stream = client.simple_query(sql).await?;
        }

        let sql = r#"
        DELETE FROM dbo.Task
        WHERE TaskID = @P1;
        "#;

        let rows_affected = client.execute(sql, &[&task_id]).await;

        match rows_affected {
            Ok(r) => {
                if r.total() == 1 {
                    ()
                } else {
                    return Err(Error::UpdateError("TaskIDNotFoundForUpdate".to_string()));
                }
            }
            Err(e) => {
                let sql = "ROLLBACK;";
                let _stream = client.simple_query(sql).await?;
                return Err(Error::UpdateError(e.to_string()));
            }
        };

        {
            let sql = "COMMIT;";
            let _stream = client.simple_query(sql).await?;
        }

        Ok(())
    }
}

// TODO: Create Unit for multi create and delete all and list task Test
// region:    --- Tests
// #[cfg(test)]
// mod tests {
//     pub type Result<T> = core::result::Result<T, Error>;
//     pub type Error = Box<dyn std::error::Error>; // For tests.

//     use crate::model;

//     use super::*;
//     use serial_test::serial;

//     #[serial]
//     #[tokio::test]
//     async fn test_create_delete_first_task_ok() -> Result<()> {
//         // -- Setup & Fixtures
//         let mm = model::ModelManager::new().await?;
//         let ctx = Ctx::new(Some("user_info:iR1f8i7Wg7jipR3uhDhJ".to_string())).unwrap();
//         let fx_task_for_create = TaskParamsForCreate {
//             title: "Task Test OK".to_string(),
//         };

//         // -- Exec
//         let task_id = TaskBmc::create(&ctx, &mm, fx_task_for_create)
//             .await
//             .unwrap()
//             .id
//             .id
//             .to_raw();

//         let deleted = TaskBmc::delete(&ctx, &mm, &task_id).await.unwrap();

//         // -- Check
//         assert_eq!(deleted, ());

//         Ok(())
//     }
// }
// // endregion: --- Tests
