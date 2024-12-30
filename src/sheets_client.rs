use crate::authenticator::Authenticator;
use crate::error::Error;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

const SHEETS_API_BASE: &str = "https://sheets.googleapis.com/v4/spreadsheets";

/// SheetsClient: Google Sheets APIへのHTTPリクエストを管理
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

    pub async fn ping_api(&self) -> Result<String, Error> {
        let token = self.authenticator.get_token().await?;
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|e| Error::ApiError(format!("Invalid token format: {}", e)))?,
        );

        // Discovery APIエンドポイントを使用して疎通確認
        let response = self
            .client
            .get("https://sheets.googleapis.com/$discovery/rest?version=v4")
            .headers(headers)
            .send()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        if response.status().is_success() {
            Ok("pong".to_string())
        } else {
            Err(Error::ApiError(format!(
                "HTTP Error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Authenticator;

    #[tokio::test]
    async fn test_ping_api_ok() {
        let auth = Authenticator::new(Some("test_key".to_string()));
        let client = SheetsClient::new(auth);
        // Note: This test will fail without proper credentials
        // In practice, we should mock the HTTP client for testing
        let result = client.ping_api().await;
        assert!(result.is_ok());
    }
}
