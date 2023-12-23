pub mod msg;

extern crate hmac;
extern crate sha2;
extern crate base64;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use hmac::{Hmac, Mac, NewMac};
use serde::Serialize;
use sha2::Sha256;


#[derive(Serialize)]
struct Content {
    text: String,
}

#[derive(Serialize)]
struct RequestBody {
    timestamp: String,
    sign: String,
    msg_type: String,
    content: Content,
}

fn gen_sign(secret: &str, timestamp: i64) -> Result<String, &'static str> {
    let string_to_sign = format!("{}\n{}", timestamp, secret);

    type HmacSha256 = Hmac<Sha256>;

    // 使用 string_to_sign 作为 HMAC 的密钥
    let mut mac = HmacSha256::new_varkey(string_to_sign.as_bytes())
        .map_err(|_| "HMAC can't accept key")?;

    // 使用 空数据 作为 HMAC 的数据
    mac.update(&[]);
    let result = mac.finalize().into_bytes();

    Ok(base64::encode(&result))
}


#[derive(Serialize)]
pub struct LarkBot {
    url: String,
    secret: String,
    msg: String,
}


impl LarkBot {

    pub fn new(url: &str, secret: &str, msg: &str) -> Self {
        LarkBot {
            url: url.to_string(),
            secret: secret.to_string(),
            msg: msg.to_string(),
        }
    }

    pub async fn send(&self) -> Result<(), reqwest::Error> {
        let timestamp = chrono::Utc::now().timestamp();
        println!("timestamp: {}", timestamp);

        let sign = gen_sign(&self.secret, timestamp).expect("Failed to generate sign");
        println!("sign: {}", sign);

        let content = Content {
            text: self.msg.clone(),
        };

        let body = RequestBody {
            timestamp: timestamp.to_string(),
            sign: sign,
            msg_type: "text".to_string(),
            content: content,
        };

        println!(
            "Request Body: {:#?}",
            serde_json::to_string_pretty(&body).unwrap()
        );

        let client = reqwest::Client::new();
        let res = client.post(&self.url).json(&body).send().await?;

        let res_body = res.text().await?;
        println!("Request:{:#?}", res_body);

        Ok(())
    }
}

