use crate::config::LarkCfg;
use crate::http::RespVO;
use crate::Data;
use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn notice(
    Extension(lark): Extension<Arc<Vec<LarkCfg>>>,
    Json(from): Json<Data>,
) -> impl IntoResponse {
    if lark.is_empty() {
        return Json(RespVO::<String>::error("lark cfg is empty"));
    }

    for v in lark.iter() {
        if from.key == v.name {
            let bot = crate::lark::LarkBot::new(
                &v.uri,   // 假设这里是配置中的 URI
                &v.token, // 假设这里是配置中的安全密钥
                &from.message,
            );
            match bot.send().await {
                Ok(_) => return Json(RespVO::<String>::success()),
                Err(e) => return Json(RespVO::<String>::error(&e.to_string())),
            }
        }
    }

    Json(RespVO::<String>::error("key not found"))
}
