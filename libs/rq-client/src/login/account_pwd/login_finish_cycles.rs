use std::sync::Arc;

use ricq::{ext::common::after_login, Client, LoginResponse, RQResult};

use super::{LoginDeny, LoginStep, PwdLoginClient};

impl Iterator for PwdLoginClient {
    type Item = LoginStep;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.take()
    }
}

impl PwdLoginClient {
    async fn handle_resp(&mut self, resp: LoginResponse) {
        let next = match resp {
            LoginResponse::Success(_) => {
                self.finish = true;
                LoginStep::Finish
            }
            LoginResponse::NeedCaptcha(lnc) => {
                LoginStep::NeedCaptcha(lnc.verify_url.unwrap_or("<unknown>".into()))
            }
            LoginResponse::AccountFrozen => LoginStep::Deny(LoginDeny::AccountFrozen),
            LoginResponse::TooManySMSRequest => LoginStep::Deny(LoginDeny::TooManySMSRequest),
            LoginResponse::DeviceLocked(ldl) => LoginStep::DeviceLock(
                ldl.message.unwrap_or_default(),
                ldl.verify_url.unwrap_or("<unknonw>".into()),
            ),
            LoginResponse::DeviceLockLogin(_) => match self.inner.device_lock_login().await {
                Ok(_) => {
                    self.finish = true;
                    LoginStep::Finish
                }
                Err(err) => LoginStep::Deny(LoginDeny::RQError(err)),
            },
            LoginResponse::UnknownStatus(lus) => LoginStep::Unknown(lus.message, lus.status),
        };

        self.state.replace(next);
    }

    pub async fn password_login(
        &mut self,
        account: impl Into<i64>,
        password: impl AsRef<str>,
    ) -> RQResult<()> {
        let resp = self
            .inner
            .password_login(account.into(), password.as_ref())
            .await?;
        self.handle_resp(resp).await;
        Ok(())
    }

    pub async fn submit_ticket(&mut self, ticket: impl AsRef<str>) -> RQResult<()> {
        let resp = self.inner.submit_ticket(ticket.as_ref()).await?;
        self.handle_resp(resp).await;
        Ok(())
    }

    pub async fn device_confirm(&mut self) -> RQResult<()> {
        let resp = self.inner.device_lock_login().await?;
        self.handle_resp(resp).await;
        Ok(())
    }

    pub async fn finish(self) -> Arc<Client> {
        if self.finish {
            after_login(&self.inner).await;
            self.inner
        } else {
            panic!("Login not finish yet")
        }
    }
}
