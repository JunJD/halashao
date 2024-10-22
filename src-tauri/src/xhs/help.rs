use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use serde_json::json;
// use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

use num_bigint::{BigInt, Sign};
use num_traits::{Zero, One};

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
    let ie: [i32; 256] = [
        0, 1996959894, 3993919788, 2567524794, 124634137, 1886057615, 3915621685,
        2657392035, 249268274, 2044508324, 3772115230, 2547177864, 162941995,
        2125561021, 3887607047, 2428444049, 498536548, 1789927666, 4089016648,
        2227061214, 450548861, 1843258603, 4107580753, 2211677639, 325883990,
        1684777152, 4251122042, 2321926636, 335633487, 1661365465, 4195302755,
        2366115317, 997073096, 1281953886, 3579855332, 2724688242, 1006888145,
        1258607687, 3524101629, 2768942443, 901097722, 1119000684, 3686517206,
        2898065728, 853044451, 1172266101, 3705015759, 2882616665, 651767980,
        1373503546, 3369554304, 3218104598, 565507253, 1454621731, 3485111705,
        3099436303, 671266974, 1594198024, 3322730930, 2970347812, 795835527,
        1483230225, 3244367275, 3060149565, 1994146192, 31158534, 2563907772,
        4023717930, 1907459465, 112637215, 2680153253, 3904427059, 2013776290,
        251722036, 2517215374, 3775830040, 2137656763, 141376813, 2439277719,
        3865271297, 1802195444, 476864866, 2238001368, 4066508878, 1812370925,
        453092731, 2181625025, 4111451223, 1706088902, 314042704, 2344532202,
        4240017532, 1658658271, 366619977, 2362670323, 4224994405, 1303535960,
        984961486, 2747007092, 3569037538, 1256170817, 1037604311, 2765210733,
        3554079995, 1131014506, 879679996, 2909243462, 3663771856, 1141124467,
        855842277, 2852801631, 3708648649, 1342533948, 654459306, 3188396048,
        3373015174, 1466479909, 544179635, 3110523913, 3462522015, 1591671054,
        702138776, 2966460450, 3352799412, 1504918807, 783551873, 3082640443,
        3233442989, 3988292384, 2596254646, 62317068, 1957810842, 3939845945,
        2647816111, 81470997, 1943803523, 3814918930, 2489596804, 225274430,
        2053790376, 3826175755, 2466906013, 167816743, 2097651377, 4027552580,
        2265490386, 503444072, 1762050814, 4150417245, 2154129355, 426522225,
        1852507879, 4275313526, 2312317920, 282753626, 1742555852, 4189708143,
        2394877945, 397917763, 1622183637, 3604390888, 2714866558, 953729732,
        1340076626, 3518719985, 2797360999, 1068828381, 1219638859, 3624741850,
        2936675148, 906185462, 1090812512, 3747672003, 2825379669, 829329135,
        1181335161, 3412177804, 3160834842, 628085408, 1382605366, 3423369109,
        3138078467, 570562233, 1426400815, 3317316542, 2998733608, 733239954,
        1555261956, 3268935591, 3050360625, 752459403, 1541320221, 2607071920,
        3965973030, 1969922972, 40735498, 2617837225, 3943577151, 1913087877,
        83908371, 2512341634, 3803740692, 2075208622, 213261112, 2463272603,
        3855990285, 2094854071, 198958881, 2262029012, 4057260610, 1759359992,
        534414190, 2176718541, 4139329115, 1873836001, 414664567, 2282248934,
        4279200368, 1711684554, 285281116, 2405801727, 4167216745, 1634467795,
        376229701, 2685067896, 3608007406, 1308918612, 956543938, 2808555105,
        3495958263, 1231636301, 1047427035, 2932959818, 3654703836, 1088359270,
        936918000, 2847714899, 3736837829, 1202900863, 817233897, 3183342108,
        3401237130, 1404277552, 615818150, 3134207493, 3453421203, 1423857449,
        601450431, 3009837614, 3294710456, 1567103746, 711928724, 3020668471,
        3272380065, 1510334235, 755167117,
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

fn base36encode(number: BigInt, alphabet: Option<&str>) -> String {
    let alphabet_str = alphabet.unwrap_or("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let base = BigInt::from(alphabet_str.len());
    let mut n = number;
    let mut result = String::new();
    
    // 检查数字的符号，并处理负数
    let sign = if n.sign() == Sign::Minus {
        n = -n;
        "-"
    } else {
        ""
    };

    // 如果 n 在 0 到字母表长度之间，直接返回对应字符
    if n.is_zero() {
        return sign.to_owned() + &alphabet_str.chars().next().unwrap().to_string();
    }

    while !n.is_zero() {
        let (div, modulo) = n.div_rem(&base);
        result.push(
            alphabet_str
                .chars()
                .nth(modulo.to_usize().expect("Invalid modulo value"))
                .expect("Invalid index in alphabet"),
        );
        n = div;
    }

    sign.to_owned() + &result.chars().rev().collect::<String>()
}

fn get_search_id() -> String {
    let now = BigInt::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    let e = now << 64; // 这里不会溢出，因为 BigInt 可以自动扩展
    let t = BigInt::from(rand::thread_rng().gen_range(0..2147483647));
    let result = e + t; // 结果也是 BigInt

    // 将 BigInt 转换为 base36 字符串（需要实现 base36 编码）
    base36encode(result, None)
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
