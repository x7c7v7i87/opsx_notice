pub mod lark;
pub mod telegram;
pub mod config;
pub mod http;

use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct Data {
    pub key:String,
    pub message: String,
}

