use crate::authenticator::Authenticator;
use crate::error::Error;

/// SheetsClient: Google Sheets APIへのHTTPリクエストを管理
pub struct SheetsClient {
    authenticator: Authenticator,
    // 必要に応じてベースURLや設定等を追加
}

impl SheetsClient {
    pub fn new(authenticator: Authenticator) -> Self {
        Self { authenticator }
    }

    pub fn ping_api(&self) -> Result<String, Error> {
        // TODO: HTTPクライアント実装
        // ひとまずダミー実装を返す
        Ok("pong".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Authenticator;

    #[test]
    fn test_ping_api_ok() {
        let auth = Authenticator::new(Some("test_key".to_string()));
        let client = SheetsClient::new(auth);
        let result = client.ping_api().unwrap();
        assert_eq!(result, "pong");
    }
}
