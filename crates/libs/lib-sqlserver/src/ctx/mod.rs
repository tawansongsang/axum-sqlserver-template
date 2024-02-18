mod error;

use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: Option<String>,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        let root_id = None;
        Ctx { user_id: root_id }
    }

    pub fn new(user_id: Option<String>) -> Result<Self> {
        if user_id.is_none() {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }
}
