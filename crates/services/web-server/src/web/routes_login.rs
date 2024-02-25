use crate::web::{self, remove_token_cookie};

use super::error::{Error, Result};

use axum::{extract::State, routing::post, Json, Router};
use lib_auth::pwd::{self, scheme::SchemeStatus, ContentToHash};
use lib_sqlserver::{
    ctx::Ctx,
    model::{
        user_info::{
            bmc::UserInfoBmc, UserInfoForCreate, UserInfoForLogin, UserInfoGet, UserInfoRecord,
        },
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logout", post(api_logout_handler))
        .route("/api/register", post(api_register_handler))
        .with_state(mm)
}

// region:    --- Login
async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANLDER");

    let LoginPayload { username, password } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user.
    let user = UserInfoBmc::first_by_username::<UserInfoForLogin>(&root_ctx, &mm, &username)
        .await
        .map_err(|e| match e {
            lib_sqlserver::model::Error::UserInfo(
                lib_sqlserver::model::QueryError::DataNotFound,
            ) => Error::LoginFailUsernameNotFound,
            _ => Error::Model(e),
        })?;

    let user_id = user.UserInfoID.ok_or(Error::LoginFailUserInfoIDNotFound)?;

    // -- Validate the password.
    let Some(password_hash) = user.Password else {
        return Err(Error::LoginFailUserHasNoPwd {
            user_id: user_id.to_string(),
        });
    };

    let Some(password_salt) = user.PasswordSalt else {
        return Err(Error::LoginFailPwdSaltNotFound {
            user_id: user_id.to_string(),
        });
    };

    let scheme_status = pwd::validate_pwd(
        ContentToHash {
            salt: password_salt,
            content: password.clone(),
        },
        password_hash,
    )
    .await
    .map_err(|_| Error::LoginFailPwdNotMatching {
        user_id: user_id.to_string(),
    })?;

    // -- Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("pwd encrypt scheme outdated, upgrading.");
        UserInfoBmc::update_pwd(&root_ctx, &mm, user_id, password.as_str()).await?;
    }

    let token_salt = user.TokenSalt.ok_or(Error::LoginFailtTokenSaltNotFound {
        user_id: user_id.to_string(),
    })?;

    // -- Set web token
    web::set_token_cookie(&cookies, user_id.to_string().as_str(), token_salt)?;

    // -- Create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        },
        "data": {
            "id": user_id,
            "email": user.Username,
            "name": user.Name,
            "role": user.Role,
            "image": null,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
// endregion: --- Login

// region:    --- Logout
async fn api_logout_handler(
    cookies: Cookies,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logout_handler", "HANLDER");

    let should_logout = payload.logout;

    if should_logout {
        remove_token_cookie(&cookies)?;
    }

    // -- Create the success body.
    let body = Json(json!({
        "resutl": {
            "logout": should_logout
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    logout: bool,
}
// endregion: --- Logout

// endregion: --- Register
async fn api_register_handler(
    State(mm): State<ModelManager>,
    _cookies: Cookies,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_register_handler", "HANLDER");
    let root_ctx = Ctx::root_ctx();

    let RegisterPayload {
        username,
        name,
        password,
    } = payload;

    let user_info_record_checked =
        UserInfoBmc::first_by_username::<UserInfoRecord>(&root_ctx, &mm, &username).await;

    if let Ok(_user_info_record) = user_info_record_checked {
        return Err(Error::RegisterUsernameAlreadyExist);
    }

    let user_info_for_create = UserInfoForCreate {
        Username: username.clone(),
        Email: username,
        Name: name,
        Password: password,
    };

    let user_info_record = UserInfoBmc::create(&root_ctx, &mm, user_info_for_create).await?;
    let user_info_id = user_info_record
        .UserInfoID
        .ok_or(Error::RegisterUserInfoRecordNotFound)?;

    let user_info = UserInfoBmc::first_by_id::<UserInfoGet>(&root_ctx, &mm, user_info_id).await?;

    let body = Json(json!({
        "result": {
            "success": true
        },
        "data": user_info,
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    name: String,
    password: String,
}
// region:    --- Register
