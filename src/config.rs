use anyhow::Result;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotCfg {
    pub web_server_port: u16,
    pub telegram_bot_port: u16,
    pub telegram_bot_token: String,
    pub telegram_bot_uri: String,
    pub lark_cfg: Vec<LarkCfg>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LarkCfg {
    pub name: String,
    pub uri: String,
    pub token: String,
}

impl BotCfg {
    pub fn new(path: &str) -> Result<Self, serde_json::Error> {
        let path = Path::new(path);
        let mut file = File::open(&path).expect("Unable to open file");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let cfg = serde_json::from_str(&contents).expect("Unable to parse JSON");
        Ok(cfg)
    }
}
