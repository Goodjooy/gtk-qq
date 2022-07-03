//! ```rust[ignore]
//! // init client
//! let client  = Client::init();
//! // send account and pwd to login
//! let client = client.login_with_account_password(account,&password).await;
//! // on captcha
//! let client = client.on_captcha()
//! // on device lock
//! let client = client.on_device_lock()
//!
//!
//!
//! ```
//!

use ricq::{Client, LoginResponse};

use super::UnLoginClient;

pub struct PwdLoginClient {
    inner: Client,
    resp: LoginResponse,
}

impl UnLoginClient {
    pub async fn login_with_account_password(
        self,
        account: impl Into<i64>,
        password: impl AsRef<str>,
    ) -> ricq::RQResult<PwdLoginClient> {
        let resp = self
            .inner
            .password_login(account.into(), password.as_ref())
            .await?;

        Ok(PwdLoginClient {
            inner: self.inner,
            resp,
        })
    }
}
