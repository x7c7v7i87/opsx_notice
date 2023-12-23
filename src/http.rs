use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

use thiserror::Error;
//https://github.com/tokio-rs/axum/discussions/1610
//https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/with_rejection.rs
// We derive `thiserror::Error`
#[derive(Debug, Error)]
pub enum ApiError {
    // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    QueryExtractorRejection(#[from] QueryRejection),
    #[error(transparent)]
    PathExtractorRejection(#[from] PathRejection),
}

// We implement `IntoResponse` so ApiError can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (msg, code) = match self {
            ApiError::JsonExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
            ApiError::QueryExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
            ApiError::PathExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
        };
        let payload = json!({
            "code":203,
            "message": msg,
            "result": ""
        });
        (code, Json(payload)).into_response()
    }
}

//常量
pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RespVO<T> {
    pub code: u16,
    pub msg: String,
    pub result: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &T) -> Self {
        Self {
            code: CODE_SUCCESS.as_u16(),
            msg: "success".to_string(),
            result: Some(arg.clone()),
        }
    }

    pub fn success() -> Self {
        Self {
            code: CODE_SUCCESS.as_u16(),
            msg: "success".to_string(),
            result: None,
        }
    }

    pub fn error(arg: &str) -> Self {
        Self {
            code: 201,
            msg: arg.to_string(),
            result: None,
        }
    }

    pub fn not(arg: &str) -> Self {
        Self {
            code: 201,
            msg: arg.to_string(),
            result: None,
        }
    }

    pub fn from_error_info(code: StatusCode, info: &str) -> Self {
        Self {
            code: code.as_u16(),
            msg: info.to_string(),
            result: None,
        }
    }

    pub fn to_json(&self) -> String {
        let arg = self;
        let john = json!(arg);
        john.to_string()
    }
}
