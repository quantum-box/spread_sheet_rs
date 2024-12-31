use crate::authenticator::Authenticator;
use crate::error::Error;
use crate::response::Response;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

/// SheetsClient: Google Sheets APIへのHTTPリクエストを管理
#[derive(Clone)]
pub struct SheetsClient {
    authenticator: Authenticator,
    client: reqwest::Client,
}

impl SheetsClient {
    pub fn new(authenticator: Authenticator) -> Self {
        Self {
            authenticator,
            client: reqwest::Client::new(),
        }
    }

    pub async fn ping_api(&self) -> Result<Response<String>, Error> {
        self.get("https://sheets.googleapis.com/$discovery/rest?version=v4")
            .await
    }

    /// 指定されたURLにGETリクエストを送信し、認証付きでデータを取得
    pub async fn get(&self, url: &str) -> Result<Response<String>, Error> {
        let token = self.authenticator.get_token().await?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|e| Error::ApiError(format!("Invalid token format: {}", e)))?,
        );

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to read response body: {}", e)))?;

        Ok(Response {
            is_success: status.is_success(),
            data: Some(body),
            error: if status.is_success() {
                None
            } else {
                Some(format!("HTTP Error: {}", status))
            },
        })
    }

    /// 指定されたURLにPUTリクエストを送信し、認証付きでデータを更新
    pub async fn put<T: serde::Serialize>(&self, url: &str, body: &T) -> Result<String, Error> {
        let token = self.authenticator.get_token().await?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|e| Error::ApiError(format!("Invalid token format: {}", e)))?,
        );

        let response = self
            .client
            .put(url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to read response body: {}", e)))?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(Error::ApiError(format!(
                "HTTP Error: {} - {}",
                status, body
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Authenticator;

    #[tokio::test]
    #[ignore = "このテストは実際のAPIコールを必要とするため、CIでは実行しません"]
    async fn test_ping_api_ok() {
        let auth = Authenticator::new(Some("test_key".to_string()));
        let client = SheetsClient::new(auth);
        // Note: このテストは適切な認証情報が必要です
        // 統合テストとして別途実装予定
        let result = client.ping_api().await;
        assert!(result.is_err());
    }
}
