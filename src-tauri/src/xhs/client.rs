use headless_chrome::{Browser, Tab};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use reqwest::Client as ReqwestClient;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use log::info;

use crate::tools::utils;
use crate::xhs::help::{get_search_id, sign};
use crate::xhs::field::{SearchNoteType, SearchSortType};

pub struct XiaoHongShuClient {
    tab: Tab,
    headers: HashMap<String, String>,
    cookie_dict: HashMap<String, String>,
    host: String,
    domain: String,
    timeout: Duration,
    proxies: Option<String>,
    http_client: ReqwestClient,
}

#[async_trait]
impl XiaoHongShuClient {
    pub fn new(
        tab: Tab,
        headers: HashMap<String, String>,
        cookie_dict: HashMap<String, String>,
        timeout: Duration,
        proxies: Option<String>,
    ) -> Result<Self> {
        let http_client = ReqwestClient::builder()
            .timeout(timeout)
            .build()?;

        Ok(Self {
            tab,
            headers,
            cookie_dict,
            host: "https://edith.xiaohongshu.com".to_string(),
            domain: "https://www.xiaohongshu.com".to_string(),
            timeout,
            proxies,
            http_client,
        })
    }

    async fn pre_headers(&self, url: &str, data: Option<&Value>) -> Result<HashMap<String, String>> {
        let js = format!(
            r#"
            (() => {{
                const url = "{}";
                const data = {};
                return window._webmsxyw(url, data);
            }})()
            "#,
            url,
            data.map_or("null".to_string(), |d| d.to_string())
        );

        let encrypt_params: Value = self.tab.evaluate(&js, false)?.value.into();
        let local_storage: Value = self.tab.evaluate("window.localStorage", false)?.value.into();

        let signs = sign(
            self.cookie_dict.get("a1").unwrap_or(&"".to_string()),
            local_storage.get("b1").and_then(|v| v.as_str()).unwrap_or(""),
            encrypt_params.get("X-s").and_then(|v| v.as_str()).unwrap_or(""),
            encrypt_params.get("X-t").and_then(|v| v.as_str()).unwrap_or(""),
        );

        let mut headers = self.headers.clone();
        headers.insert("X-S".to_string(), signs.get("x-s").unwrap().to_string());
        headers.insert("X-T".to_string(), signs.get("x-t").unwrap().to_string());
        headers.insert("x-S-Common".to_string(), signs.get("x-s-common").unwrap().to_string());
        headers.insert("X-B3-Traceid".to_string(), signs.get("x-b3-traceid").unwrap().to_string());

        Ok(headers)
    }

    async fn request(&self, method: reqwest::Method, url: &str, headers: HashMap<String, String>, body: Option<Value>) -> Result<Value> {
        let mut request = self.http_client.request(method, url)
            .headers((&headers).try_into()?);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let data: Value = response.json().await?;

        if data["success"].as_bool().unwrap_or(false) {
            Ok(data["data"].clone())
        } else {
            Err(anyhow!("Request failed: {:?}", data["msg"]))
        }
    }

    pub async fn get(&self, uri: &str, params: Option<HashMap<String, String>>) -> Result<Value> {
        let url = format!("{}{}", self.host, uri);
        let headers = self.pre_headers(&url, None).await?;
        
        let url = if let Some(params) = params {
            format!("{}?{}", url, serde_urlencoded::to_string(&params)?)
        } else {
            url
        };

        self.request(reqwest::Method::GET, &url, headers, None).await
    }

    pub async fn post(&self, uri: &str, data: Value) -> Result<Value> {
        let url = format!("{}{}", self.host, uri);
        let headers = self.pre_headers(&url, Some(&data)).await?;

        self.request(reqwest::Method::POST, &url, headers, Some(data)).await
    }

    pub async fn get_note_media(&self, url: &str) -> Result<Option<Vec<u8>>> {
        let response = self.http_client.get(url).send().await?;
        if response.status().is_success() {
            Ok(Some(response.bytes().await?.to_vec()))
        } else {
            info!("[XiaoHongShuClient.get_note_media] request {} err, res:{}", url, response.text().await?);
            Ok(None)
        }
    }

    pub async fn pong(&self) -> Result<bool> {
        info!("[XiaoHongShuClient.pong] Begin to pong xhs...");
        match self.get_note_by_keyword("小红书", 1, 20, SearchSortType::GENERAL, SearchNoteType::ALL).await {
            Ok(note_card) => Ok(note_card.get("items").is_some()),
            Err(e) => {
                info!("[XiaoHongShuClient.pong] Ping xhs failed: {}, and try to login again...", e);
                Ok(false)
            }
        }
    }

    pub async fn get_note_by_keyword(
        &self,
        keyword: &str,
        page: u32,
        page_size: u32,
        sort: SearchSortType,
        note_type: SearchNoteType
    ) -> Result<Value> {
        let uri = "/api/sns/web/v1/search/notes";
        let data = json!({
            "keyword": keyword,
            "page": page,
            "page_size": page_size,
            "search_id": get_search_id(),
            "sort": sort.to_string(),
            "note_type": note_type as u32
        });
        self.post(uri, data).await
    }

    // Implement other methods (get_note_by_id, get_note_comments, etc.) similarly...
}
