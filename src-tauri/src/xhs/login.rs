// use async_trait::async_trait;
use headless_chrome::Tab;
use std::sync::Arc;
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

    pub async fn begin(&mut self) -> Result<(String, tokio::sync::mpsc::Receiver<bool>)> {
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

    async fn login_by_qrcode(&self) -> Result<(String, tokio::sync::mpsc::Receiver<bool>)> {
        info!("[XiaoHongShuLogin.login_by_qrcode] Begin login xiaohongshu by qrcode ...");
        let qrcode_img_selector = "img.qrcode-img";
        
        let base64_qrcode_img = crawler_util::find_login_qrcode(&self.tab, qrcode_img_selector).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let tab = self.tab.clone();

        tokio::spawn(async move {
            let result = Self::wait_for_login(&tab).await;
            let _ = tx.send(result).await;
        });

        Ok((base64_qrcode_img, rx))
    }

    async fn wait_for_login(tab: &Arc<Tab>) -> bool {
        for _ in 0..120 {  // 等待最多120秒
            if Self::check_login_state(tab).await.unwrap_or(false) {
                info!("[XiaoHongShuLogin.login_by_qrcode] Login successful");
                return true;
            }
            sleep(Duration::from_secs(1)).await;
        }
        info!("[XiaoHongShuLogin.login_by_qrcode] Login timeout");
        false
    }

    async fn check_login_state(tab: &Arc<Tab>) -> Result<bool> {
        let content = tab.get_content()?;
        if content.contains("请通过验证") {
            info!("[XiaoHongShuLogin.check_login_state] 登录过程中出现验证码，请手动验证");
        }

        let cookies = tab.get_cookies()?;
        let (_, cookie_dict) = crawler_util::convert_cookies(Some(cookies));
        let current_web_session = cookie_dict.get("web_session").cloned().unwrap_or_default();
        Ok(!current_web_session.is_empty())
    }
}
