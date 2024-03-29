use async_trait::async_trait;
use lib_auth::token::{validate_web_token, Token};
use lib_sqlserver::{
    ctx::Ctx,
    model::{
        user_info::{bmc::UserInfoBmc, UserInfoForAuth},
        ModelManager,
    },
};
use serde::Serialize;
use uuid::Uuid;

use super::error::{Error, Result};
use crate::web::{set_token_cookie, AUTH_TOKEN};
use axum::{
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_require(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = inner_ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // -- Store the ctx_ext_result in the request extension
    // (for Ctx extractor)

    let _ctxw = req.extensions_mut().insert(ctx_ext_result);

    let response = next.run(req).await;

    Ok(response)
}

async fn inner_ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    // -- Get Token String
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // -- Parse Token
    let token = token
        .parse::<Token>()
        .map_err(|_| CtxExtError::TokenWrongFormat)?;

    // -- Get uuid from token string
    let user_info_id =
        Uuid::parse_str(token.ident.as_str()).map_err(|_| CtxExtError::TokenWrongFormat)?;

    // -- Get UserInfoForAuth
    let user = UserInfoBmc::first_by_id::<UserInfoForAuth>(&Ctx::root_ctx(), &mm, user_info_id)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?;

    // -- Get Token Salt
    let token_salt = user.TokenSalt.ok_or(CtxExtError::TokenSaltNotFound)?;

    // -- Validate Token
    validate_web_token(&token, token_salt).map_err(|_| CtxExtError::FailValidate)?;

    // -- Update Token
    let user_id = user_info_id.to_string();
    set_token_cookie(cookies, user_id.as_str(), token_salt)
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    // -- Create CtxExtResult
    Ctx::new(Some(user_id))
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// region:    --- Ctx Extractor
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        let part = parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt);

        part
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = std::result::Result<CtxW, CtxExtError>;

#[derive(Debug, Serialize, Clone)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,
    TokenSaltNotFound,

    ModelAccessError(String),
    // UserNotFound,
    FailValidate,
    CannotSetTokenCookie,
    CtxCreateFail(String),
    CtxNotInRequestExt,
}
// endregion: --- Ctx Extractor Result/Error
