use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use rand::Rng;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

pub fn sign(a1: &str, b1: &str, x_s: &str, x_t: &str) -> serde_json::Value {
    let common = json!({
        "s0": 3,
        "s1": "",
        "x0": "1",
        "x1": "3.7.8-2",
        "x2": "Mac OS",
        "x3": "xhs-pc-web",
        "x4": "4.27.2",
        "x5": a1,
        "x6": x_t,
        "x7": x_s,
        "x8": b1,
        "x9": mrc(&format!("{}{}{}", x_t, x_s, b1)),
        "x10": 154,
    });

    let encode_str = encode_utf8(&common.to_string());
    let x_s_common = b64_encode(&encode_str);
    let x_b3_traceid = get_b3_trace_id();

    json!({
        "x-s": x_s,
        "x-t": x_t,
        "x-s-common": x_s_common,
        "x-b3-traceid": x_b3_traceid
    })
}

fn get_b3_trace_id() -> String {
    const CHARS: &[u8] = b"abcdef0123456789";
    let mut rng = rand::thread_rng();
    (0..16)
        .map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char)
        .collect()
}

fn mrc(e: &str) -> i32 {
    let ie: [u32; 256] = [
        0, 1996959894, 3993919788, 2567524794, 124634137, 1886057615, 3915621685,
        // ... (省略其余值以节省空间)
    ];

    let mut o: i32 = -1;

    for &byte in e.as_bytes() {
        o = ie[((o & 255) ^ (byte as u32)) as usize] ^ (o.unsigned_shr(8));
    }

    (o ^ -1 ^ 3988292384) as i32
}

fn b64_encode(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}

fn encode_utf8(e: &str) -> Vec<u8> {
    form_urlencoded::byte_serialize(e.as_bytes()).flat_map(|c| c.as_bytes().to_vec()).collect()
}

pub fn base36encode(mut number: i64) -> String {
    const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if number == 0 {
        return "0".to_string();
    }
    let mut base36 = String::new();
    while number > 0 {
        base36.insert(0, ALPHABET.chars().nth((number % 36) as usize).unwrap());
        number /= 36;
    }
    base36
}

pub fn get_search_id() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
    let e = now << 64;
    let t = rand::thread_rng().gen_range(0..2147483647);
    base36encode(e + t)
}

pub fn get_img_url_by_trace_id(trace_id: &str, format_type: &str) -> String {
    const IMG_CDNS: [&str; 4] = [
        "https://sns-img-qc.xhscdn.com",
        "https://sns-img-hw.xhscdn.com",
        "https://sns-img-bd.xhscdn.com",
        "https://sns-img-qn.xhscdn.com",
    ];
    let cdn = IMG_CDNS[rand::thread_rng().gen_range(0..IMG_CDNS.len())];
    format!("{}/{}?imageView2/format/{}", cdn, trace_id, format_type)
}

pub fn get_img_urls_by_trace_id(trace_id: &str, format_type: &str) -> Vec<String> {
    const IMG_CDNS: [&str; 4] = [
        "https://sns-img-qc.xhscdn.com",
        "https://sns-img-hw.xhscdn.com",
        "https://sns-img-bd.xhscdn.com",
        "https://sns-img-qn.xhscdn.com",
    ];
    IMG_CDNS
        .iter()
        .map(|&cdn| format!("{}/{}?imageView2/format/{}", cdn, trace_id, format_type))
        .collect()
}

pub fn get_trace_id(img_url: &str) -> String {
    let parts: Vec<&str> = img_url.split('/').collect();
    if img_url.contains("spectrum") {
        format!("spectrum/{}", parts.last().unwrap())
    } else {
        parts.last().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_img_url() {
        let img_url = "https://sns-img-bd.xhscdn.com/7a3abfaf-90c1-a828-5de7-022c80b92aa3";
        let trace_id = get_trace_id(img_url);
        let final_img_url = get_img_url_by_trace_id(&trace_id, "png");
        println!("{}", final_img_url);
        assert!(final_img_url.contains(&trace_id));
    }
}
