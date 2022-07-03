use std::io::stdin;

use ricq::{device::Device, version::Protocol};
use rq_client::login::{account_pwd::LoginStep, UnLoginClient};

#[tokio::main]
async fn main() {
    let mut client = UnLoginClient::new(Device::random(), Protocol::MacOS)
        .await
        .unwrap()
        .using_password();

    client
        .password_login(3628862306i64, "wyq@qq.com020222")
        .await
        .expect("Error login ");

    while let Some(next) = client.next() {
        match next {
            LoginStep::NeedCaptcha(url) => {
                println!("captcha {url}");
                let mut tick = String::new();
                stdin().read_line(&mut tick).unwrap();
                client.submit_ticket(tick.trim()).await.unwrap();
            }
            LoginStep::DeviceLock(msg, verify) => {
                println!("device lock {msg}");
                println!("{verify}");
                let mut st = String::new();
                stdin().read_line(&mut st).unwrap();
                client.device_confirm().await.unwrap();
            }
            LoginStep::Unknown(msg, id) => {
                println!("unknown state [{id}] {msg}");
                client
                    .password_login(3628862306i64, "wyq@qq.com020222")
                    .await
                    .expect("Error login ");
            }
            LoginStep::Deny(e) => {
                println!("Error {e:?}");
                break;
            }
            LoginStep::Finish => break,
            _ => {}
        }
    }

    let _client = client.finish().await;

    println!("login success")
}
