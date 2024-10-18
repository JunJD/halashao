use base64::{engine::general_purpose, Engine as _};
use fantoccini::{Client, Locator};
use image::{ImageBuffer, Rgb};
use rand::seq::SliceRandom;
use regex::Regex;
use reqwest::Client as ReqwestClient;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;

use crate::tools::utils::LOGGER;

pub async fn find_login_qrcode(client: &Client, selector: &str) -> Result<String, Box<dyn std::error::Error>> {
    let element = client.find(Locator::Css(selector)).await?;
    let login_qrcode_img = element.attr("src").await?.unwrap_or_default();

    if login_qrcode_img.starts_with("http://") || login_qrcode_img.starts_with("https://") {
        let reqwest_client = ReqwestClient::new();
        log::info!("[find_login_qrcode] get qrcode by url:{}", login_qrcode_img);
        let resp = reqwest_client.get(&login_qrcode_img)
            .header("User-Agent", get_user_agent())
            .send()
            .await?;

        if resp.status().is_success() {
            let image_data = resp.bytes().await?;
            Ok(general_purpose::STANDARD.encode(image_data))
        } else {
            Err(format!("fetch login image url failed, response message:{}", resp.text().await?).into())
        }
    } else {
        Ok(login_qrcode_img)
    }
}

pub async fn find_qrcode_img_from_canvas(client: &Client, canvas_selector: &str) -> Result<String, Box<dyn std::error::Error>> {
    let canvas = client.find(Locator::Css(canvas_selector)).await?;
    let screenshot = canvas.screenshot().await?;
    Ok(general_purpose::STANDARD.encode(screenshot))
}

pub fn show_qrcode(qr_code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let qr_code = if qr_code.contains(',') {
        qr_code.split(',').nth(1).unwrap_or(qr_code)
    } else {
        qr_code
    };

    let qr_code = general_purpose::STANDARD.decode(qr_code)?;
    let img = image::load_from_memory(&qr_code)?;

    let (width, height) = img.dimensions();
    let mut new_image = ImageBuffer::new(width + 20, height + 20);

    // Fill the new image with white
    for pixel in new_image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Copy the original image to the center of the new image
    image::imageops::overlay(&mut new_image, &img, 10, 10);

    // Draw a black border
    for x in 0..width+20 {
        new_image.put_pixel(x, 0, Rgb([0, 0, 0]));
        new_image.put_pixel(x, height+19, Rgb([0, 0, 0]));
    }
    for y in 0..height+20 {
        new_image.put_pixel(0, y, Rgb([0, 0, 0]));
        new_image.put_pixel(width+19, y, Rgb([0, 0, 0]));
    }

    new_image.save("qrcode.png")?;
    Ok(())
}

pub fn get_user_agent() -> String {
    let ua_list = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
        // ... (其他 User-Agent 字符串)
    ];
    ua_list.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn get_mobile_user_agent() -> String {
    let ua_list = vec![
        "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Mobile/15E148 Safari/604.1",
        // ... (其他移动设备 User-Agent 字符串)
    ];
    ua_list.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn convert_cookies(cookies: Option<Vec<fantoccini::cookie::Cookie>>) -> (String, HashMap<String, String>) {
    match cookies {
        Some(cookies) => {
            let cookies_str = cookies.iter()
                .map(|c| format!("{}={}", c.name, c.value))
                .collect::<Vec<String>>()
                .join(";");
            let cookie_dict = cookies.into_iter()
                .map(|c| (c.name, c.value))
                .collect();
            (cookies_str, cookie_dict)
        },
        None => (String::new(), HashMap::new()),
    }
}

pub fn convert_str_cookie_to_dict(cookie_str: &str) -> HashMap<String, String> {
    cookie_str.split(';')
        .filter_map(|s| {
            let parts: Vec<&str> = s.trim().splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect()
}

pub fn match_interact_info_count(count_str: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    re.find(count_str)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0)
}

pub fn format_proxy_info(ip_proxy_info: &Value) -> (Option<HashMap<String, String>>, Option<HashMap<String, String>>) {
    let playwright_proxy = Some(HashMap::from([
        ("server".to_string(), format!("{}{}", ip_proxy_info["protocol"].as_str().unwrap_or(""), ip_proxy_info["ip"].as_str().unwrap_or(""))),
        ("username".to_string(), ip_proxy_info["user"].as_str().unwrap_or("").to_string()),
        ("password".to_string(), ip_proxy_info["password"].as_str().unwrap_or("").to_string()),
    ]));

    let httpx_proxy = Some(HashMap::from([
        (ip_proxy_info["protocol"].as_str().unwrap_or("").to_string(), 
         format!("http://{}:{}@{}:{}", 
                 ip_proxy_info["user"].as_str().unwrap_or(""),
                 ip_proxy_info["password"].as_str().unwrap_or(""),
                 ip_proxy_info["ip"].as_str().unwrap_or(""),
                 ip_proxy_info["port"].as_str().unwrap_or("")))
    ]));

    (playwright_proxy, httpx_proxy)
}

pub fn extract_text_from_html(html: &str) -> String {
    let script_re = Regex::new(r"<(script|style)[^>]*>.*?</\1>").unwrap();
    let tag_re = Regex::new(r"<[^>]+>").unwrap();
    
    let clean_html = script_re.replace_all(html, "");
    let clean_text = tag_re.replace_all(&clean_html, "");
    
    clean_text.trim().to_string()
}