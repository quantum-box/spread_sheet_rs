use crate::error::Error;
use crate::response::Response;
use crate::SheetsClient;

/// SpreadsheetReader: シートからデータを読み取るコンポーネント
pub struct SpreadsheetReader {
    // Note: このフィールドは実際のAPI実装時に使用予定
    client: SheetsClient,
}

impl SpreadsheetReader {
    /// コンストラクタ
    pub fn new(client: SheetsClient) -> Self {
        Self { client }
    }

    /// シート全体の読み込み
    pub async fn read_entire_sheet(
        &self,
        _spreadsheet_id: &str,
        _sheet_name: &str,
    ) -> Result<Response<String>, Error> {
        // TODO: シート全体を読み込むAPI呼び出し
        Ok(Response::new_success("DummyData".to_string()))
    }

    /// 指定範囲の読み込み
    pub async fn read_range(
        &self,
        _spreadsheet_id: &str,
        _range: &str,
    ) -> Result<Response<String>, Error> {
        // TODO: 指定範囲を読み込むAPI呼び出し
        Ok(Response::new_success("DummyRangeData".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authenticator::Authenticator;
    use tokio;

    #[tokio::test]
    #[ignore = "このテストは実際のAPIコールを必要とするため、CIでは実行しません"]
    async fn test_read_entire_sheet() {
        let auth = Authenticator::new(Some("test_key".to_string()));
        let client = SheetsClient::new(auth);
        let reader = SpreadsheetReader::new(client);
        let result = reader.read_entire_sheet("dummy_id", "Sheet1").await;
        assert!(result.is_ok());
        if let Ok(response) = result {
            assert!(response.is_success);
            assert!(response.data.is_some());
            assert!(response.error.is_none());
        }
    }

    #[tokio::test]
    #[ignore = "このテストは実際のAPIコールを必要とするため、CIでは実行しません"]
    async fn test_read_range() {
        let auth = Authenticator::new(Some("test_key".to_string()));
        let client = SheetsClient::new(auth);
        let reader = SpreadsheetReader::new(client);
        let result = reader.read_range("dummy_id", "Sheet1!A1:B2").await;
        assert!(result.is_ok());
        if let Ok(response) = result {
            assert!(response.is_success);
            assert!(response.data.is_some());
            assert!(response.error.is_none());
        }
    }
}