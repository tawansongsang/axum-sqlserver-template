use lib_sqlserver::{
    ctx::Ctx,
    model::{
        task::{bmc::TaskBmc, Task, TaskFilter, TaskParamsForCreate, TaskParamsForUpdate},
        ModelManager,
    },
};

use crate::{
    params::{ParamsForCreate, ParamsForUpdate, ParamsIded},
    router::RpcRouter,
    rpc_router, Error, ParamsList, Result,
};

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add..
        create_task,
        list_tasks,
        update_task,
        delete_task,
    )
}

pub async fn create_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<TaskParamsForCreate>,
) -> Result<Task> {
    let ParamsForCreate { data } = params;

    let task_id = TaskBmc::create(&ctx, &mm, data)
        .await?
        .TaskID
        .ok_or(Error::DataNotFound(format!(
            "TaskID not return from create"
        )))?;
    let task = TaskBmc::get(&ctx, &mm, &task_id.to_string()).await?;

    Ok(task)
}

pub async fn list_tasks(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<TaskFilter>,
) -> Result<Vec<Task>> {
    let tasks = TaskBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(tasks)
}

pub async fn update_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<TaskParamsForUpdate>,
) -> Result<Task> {
    let ParamsForUpdate { id, data } = params;

    TaskBmc::update(&ctx, &mm, &id, data).await?;

    let task = TaskBmc::get(&ctx, &mm, &id).await?;

    Ok(task)
}

pub async fn delete_task(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Task> {
    let ParamsIded { id } = params;

    let task = TaskBmc::get(&ctx, &mm, &id).await?;
    TaskBmc::delete(&ctx, &mm, &id).await?;

    Ok(task)
}
