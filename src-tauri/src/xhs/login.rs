use async_trait::async_trait;
use fantoccini::{Client, Locator};
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error};

use crate::config::base_config;
use crate::tools::{utils, crawler_util};
// use crate::cache::cache_factory::CacheFactory;

pub struct XiaoHongShuLogin {
    login_type: String,
    browser_context: Client,
    login_phone: Option<String>,
    cookie_str: String,
}

#[async_trait]
impl XiaoHongShuLogin {
    pub fn new(
        login_type: String,
        browser_context: Client,
        login_phone: Option<String>,
        cookie_str: String,
    ) -> Self {
        unsafe { base_config::LOGIN_TYPE = &login_type.clone() };
        Self {
            login_type,
            browser_context,
            login_phone,
            cookie_str,
        }
    }

    async fn check_login_state(&self, no_logged_in_session: &str) -> bool {
        let content = self.browser_context.source().await.unwrap();
        if content.contains("请通过验证") {
            info!("登录过程中出现验证码，请手动验证");
        }

        let current_cookie = self.browser_context.get_all_cookies().await.unwrap();
        let cookie_dict = crawler_util::convert_cookies(&current_cookie);
        let current_web_session = cookie_dict.get("web_session").cloned().unwrap_or_default();
        current_web_session != no_logged_in_session
    }

    pub async fn begin(&self) {
        info!("Begin login xiaohongshu ...");
        match self.login_type.as_str() {
            "qrcode" => self.login_by_qrcode().await,
            "phone" => self.login_by_mobile().await,
            "cookie" => self.login_by_cookies().await,
            _ => panic!("Invalid Login Type. Currently only supported qrcode or phone or cookies ..."),
        }
    }

    async fn login_by_mobile(&self) {
        info!("Begin login xiaohongshu by mobile ...");
        sleep(Duration::from_secs(1)).await;

        // 尝试点击登录按钮
        match self.browser_context.find(Locator::XPath("//*[@id='app']/div[1]/div[2]/div[1]/ul/div[1]/button")).await {
            Ok(element) => {
                element.click().await.unwrap();
                // 尝试切换到手机登录
                if let Ok(element) = self.browser_context.find(Locator::XPath("//div[@class=\"login-container\"]//div[@class=\"other-method\"]/div[1]")).await {
                    element.click().await.unwrap();
                }
            }
            Err(_) => info!("have not found mobile button icon and keep going ..."),
        }

        sleep(Duration::from_secs(1)).await;

        // 输入手机号和验证码
        let login_container = self.browser_context.find(Locator::Css("div.login-container")).await.unwrap();
        let input_ele = login_container.find(Locator::Css("label.phone > input")).await.unwrap();
        input_ele.send_keys(&self.login_phone.clone().unwrap()).await.unwrap();
        sleep(Duration::from_millis(500)).await;

        let send_btn_ele = login_container.find(Locator::Css("label.auth-code > span")).await.unwrap();
        send_btn_ele.click().await.unwrap();

        let sms_code_input_ele = login_container.find(Locator::Css("label.auth-code > input")).await.unwrap();
        let submit_btn_ele = login_container.find(Locator::Css("div.input-container > button")).await.unwrap();

        // let cache_client = CacheFactory::create_cache(base_config::CACHE_TYPE_MEMORY);
        let mut max_get_sms_code_time = 60 * 2;
        let no_logged_in_session = self.browser_context.get_all_cookies().await.unwrap()
            .iter()
            .find(|c| c.name == "web_session")
            .map(|c| c.value.clone())
            .unwrap_or_default();

        // while max_get_sms_code_time > 0 {
        //     utils::logger::info(&format!("get sms code from redis remaining time {}s ...", max_get_sms_code_time));
        //     sleep(Duration::from_secs(1)).await;
        //     let sms_code_key = format!("xhs_{}", self.login_phone.clone().unwrap());
        //     if let Some(sms_code_value) = cache_client.get(&sms_code_key) {
        //         sms_code_input_ele.send_keys(&sms_code_value).await.unwrap();
        //         sleep(Duration::from_millis(500)).await;
                
        //         let agree_privacy_ele = self.browser_context.find(Locator::XPath("//div[@class='agreements']//*[local-name()='svg']")).await.unwrap();
        //         agree_privacy_ele.click().await.unwrap();
        //         sleep(Duration::from_millis(500)).await;

        //         submit_btn_ele.click().await.unwrap();
        //         break;
        //     }
        //     max_get_sms_code_time -= 1;
        // }

        // 检查登录状态
        for _ in 0..600 {
            if self.check_login_state(&no_logged_in_session).await {
                info!("Login successful then wait for 5 seconds redirect ...");
                sleep(Duration::from_secs(5)).await;
                return;
            }
            sleep(Duration::from_secs(1)).await;
        }

        info!("Login xiaohongshu failed by mobile login method ...");
        std::process::exit(1);
    }

    async fn login_by_qrcode(&self) {
        // 实现二维码登录逻辑
        // ...
    }

    async fn login_by_cookies(&self) {
        info!("Begin login xiaohongshu by cookie ...");
        let cookie_dict = crawler_util::convert_str_cookie_to_dict(&self.cookie_str);
        for (key, value) in cookie_dict {
            if key != "web_session" {
                continue;
            }
            self.browser_context.add_cookie(fantoccini::cookie::Cookie::new(key, value))
                .domain(".xiaohongshu.com")
                .path("/")
                .finish()
                .await
                .unwrap();
        }
    }
}
