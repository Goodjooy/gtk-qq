//! ```rust[ignore]
//! // init client
//! let client  = Client::init();
//! // send account and pwd to login
//! let mut  client = client.using_password()
//!
//! client.password_login(account,password).await
//!
//!
//! ```
//!

pub mod login_finish_cycles;
use std::sync::Arc;

use ricq::{Client, RQError};

use super::UnLoginClient;

pub struct PwdLoginClient {
    inner: Arc<Client>,
    state: Option<LoginStep>,
    finish: bool,
}

impl UnLoginClient {
    pub fn using_password(self) -> PwdLoginClient {
        PwdLoginClient {
            inner: self.inner,
            state: Some(LoginStep::Start),
            finish: false,
        }
    }
}

pub type VerifyURL = String;
pub type Message = String;
pub type Status = u8;

#[derive(Debug)]
pub enum LoginStep {
    Start,
    NeedCaptcha(VerifyURL),
    DeviceLock(Message, VerifyURL),
    Unknown(Message, Status),
    Deny(LoginDeny),
    Finish,
}

#[derive(Debug)]
pub enum LoginDeny {
    AccountFrozen,
    TooManySMSRequest,
    RQError(RQError),
}

#[cfg(test)]
mod test {
    use std::io::stdin;

    use ricq::device::Device;

    use crate::login::UnLoginClient;

    #[tokio::test]
    async fn name() {

    }
}
