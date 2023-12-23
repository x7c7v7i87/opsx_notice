use reqwest;
use serde_json::json;
use tokio;

// 异步函数封装
async fn send_async_post_request(url: &str, message: &str, key: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/json; charset=utf-8")
        .json(&json!({
            "message": message,
            "key": key
        }))
        .send()
        .await?;

    let body = res.text().await?;
    Ok(body)
}

// 异步测试
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_async_post_request() {
        let url = "http://127.0.0.1:9191/notice/lark";
        let response = send_async_post_request(url, "test", "test").await;
        assert!(response.is_ok());
        // 这里可以根据您的实际响应内容进行更详细的断言
    }
}

// 同步函数封装
fn send_sync_post_request(url: &str, message: &str, key: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/json; charset=utf-8")
        .json(&json!({
            "message": message,
            "key": key
        }))
        .send()?;

    let body = res.text()?;
    Ok(body)
}

// 同步测试
#[cfg(test)]
mod sync_tests {
    use super::*;

    #[test]
    fn test_send_sync_post_request() {
        let url = "http://127.0.0.1:9191/notice/lark";
        let response = send_sync_post_request(url, "test", "test");
        assert!(response.is_ok());
        // 这里可以根据您的实际响应内容进行更详细的断言
    }
}

