// use async_trait::async_trait;
use headless_chrome::Tab;
use std::sync::Arc;
use tokio::select;
use tokio::sync::oneshot;
// use serde_json::Value;
use anyhow::{anyhow, Result};
// use headless_chrome::protocol::cdp::Network;
use log::info;
use std::time::Duration;
use tokio::time::sleep;

// use crate::config::base_config;
// use crate::tools::{utils, crawler_util};
use crate::tools::crawler_util;
// use crate::cache::cache_factory::CacheFactory;

pub struct XiaoHongShuLogin {
    login_type: String,
    tab: Arc<Tab>
}

impl XiaoHongShuLogin {
    pub fn new(login_type: String, tab: Arc<Tab>) -> Self {
        Self {
            login_type,
            tab
        }
    }

    pub async fn begin(&mut self) -> Result<()> {
        info!("[XiaoHongShuLogin.begin] Begin login xiaohongshu ...");
        match self.login_type.as_str() {
            "qrcode" => self.login_by_qrcode().await,
            // "phone" => self.login_by_mobile().await?,
            // "cookie" => self.login_by_cookies().await?,
            _ => {
                Err(anyhow!(
                    "Invalid Login Type. Currently only supported qrcode or phone or cookies ..."
                ))
            }
        }
    }

    async fn check_login_state(tab: &Arc<Tab>, no_logged_in_session: &str) -> Result<bool> {
        let content = tab.get_content()?;
        if content.contains("请通过验证") {
            info!("[XiaoHongShuLogin.check_login_state] 登录过程中出现验证码，请手动验证");
        }

        let cookies = tab.get_cookies()?;
        let (_, cookie_dict) = crawler_util::convert_cookies(Some(cookies));
        let current_web_session = cookie_dict.get("web_session").cloned().unwrap_or_default();
        Ok(current_web_session != no_logged_in_session)
    }
    async fn login_by_qrcode(&self) -> Result<()> {
        info!("[XiaoHongShuLogin.login_by_qrcode] Begin login xiaohongshu by qrcode ...");
        let qrcode_img_selector = "img.qrcode-img";

        let _base64_qrcode_img =
            crawler_util::find_login_qrcode(&self.tab, qrcode_img_selector).await?;

        // TODO: Implement show_qrcode function
        // crawler_util::show_qrcode(&base64_qrcode_img)().map_err(|e| format!("Failed to create new tab: {}", e))?;

        let cookies = self.tab.get_cookies()?;
        let (_, cookie_dict) = crawler_util::convert_cookies(Some(cookies));
        let no_logged_in_session = cookie_dict.get("web_session").cloned().unwrap_or_default();

        info!("[XiaoHongShuLogin.login_by_qrcode] waiting for scan code login, remaining time is 120s");

        let (tx, rx) = oneshot::channel();
        let tab = self.tab.clone();
        let no_logged_in_session = no_logged_in_session.clone();

        tokio::spawn(async move {
            for _ in 0..120 {
                if XiaoHongShuLogin::check_login_state(&tab, &no_logged_in_session)
                    .await
                    .unwrap_or(false)
                {
                    let _ = tx.send(true);
                    return;
                }
                info!("[XiaoHongShuLogin.login_by_qrcode] Waiting for QR code scan...");
                sleep(Duration::from_secs(1)).await;
            }
            let _ = tx.send(false);
        });

        select! {
            _ = sleep(Duration::from_secs(120)) => {
                Err(anyhow!("[XiaoHongShuLogin.login_by_qrcode] Login timeout"))
            }
            result = rx => {
                match result {
                    Ok(true) => {
                        info!("[XiaoHongShuLogin.login_by_qrcode] Login successful then wait for 5 seconds redirect ...");
                        sleep(Duration::from_secs(5)).await;
                        info!("[XiaoHongShuLogin.login_by_qrcode] end");
                        Ok(())
                    }
                    _ => Err(anyhow!("[XiaoHongShuLogin.login_by_qrcode] Login failed"))
                }
            }
        }
    }
}
