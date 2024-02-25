use super::{
    error::{Error, Result},
    mw_auth::{mw_ctx_require, CtxW},
};

use axum::{extract::State, middleware, routing::get, Json, Router};

use lib_sqlserver::model::{
    user_info::{bmc::UserInfoBmc, UserInfoGet},
    ModelManager,
};
use serde_json::{json, Value};
use tracing::debug;
use uuid::Uuid;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/session", get(api_session_handler))
        .with_state(mm)
        .route_layer(middleware::from_fn(mw_ctx_require))
}

// region:    --- Session
async fn api_session_handler(ctx: CtxW, State(mm): State<ModelManager>) -> Result<Json<Value>> {
    debug!("{:<12} - api_session_handler", "HANLDER");
    let ctx = ctx.0;

    let user_id = match ctx.user_id() {
        Some(user_id) => user_id,
        None => return Err(Error::NoUserIdInCookies),
    };

    let user_info_id = Uuid::parse_str(user_id)?;

    let user_info = UserInfoBmc::first_by_id::<UserInfoGet>(&ctx, &mm, user_info_id).await?;

    // -- Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
        },
        "data": {
            "id": user_info_id,
            "email": user_info.Username,
            "name": user_info.Name,
            "role": user_info.Role,
            "active": user_info.Active,
            "deleted": user_info.Deleted,
            "image": null,
        }
    }));

    Ok(body)

    // endregion: --- Session
}
