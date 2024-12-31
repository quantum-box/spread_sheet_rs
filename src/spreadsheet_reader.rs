use crate::error::Error;
use crate::response::Response;
use crate::SheetsClient;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct ValueRange {
    range: String,
    #[serde(default)]
    #[serde(rename = "majorDimension")]
    major_dimension: String,
    #[serde(default)]
    values: Vec<Vec<String>>,
}

/// SpreadsheetReader: シートからデータを読み取るコンポーネント
#[allow(dead_code)]
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
        spreadsheet_id: &str,
        sheet_name: &str,
    ) -> Result<Response<Vec<Vec<String>>>, Error> {
        // Google Sheets API V4のエンドポイントを構築
        // シート全体を読み込むために大きな範囲を指定（A1:ZZ1000）
        let range = format!("{}!A1:ZZ1000", sheet_name);
        // URLエンコードを行う
        let encoded_range = urlencoding::encode(&range);
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            spreadsheet_id, encoded_range
        );

        // APIリクエストを実行
        match self.client.get(&url).await {
            Ok(response) => {
                if response.is_success {
                    if let Some(data) = response.data {
                        // JSONレスポンスをパース
                        match serde_json::from_str::<ValueRange>(&data) {
                            Ok(value_range) => Ok(Response::new_success(value_range.values)),
                            Err(e) => Ok(Response::new_error(&format!(
                                "Failed to parse response: {}",
                                e
                            ))),
                        }
                    } else {
                        Ok(Response::new_error("No data received"))
                    }
                } else {
                    Ok(Response::new_error(
                        &response
                            .error
                            .unwrap_or_else(|| "Unknown error".to_string()),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// 指定範囲の読み込み
    pub async fn read_range(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<Response<Vec<Vec<String>>>, Error> {
        // Google Sheets API V4のエンドポイントを構築
        // URLエンコードを行う
        let encoded_range = urlencoding::encode(range);
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            spreadsheet_id, encoded_range
        );

        // APIリクエストを実行
        match self.client.get(&url).await {
            Ok(response) => {
                if response.is_success {
                    if let Some(data) = response.data {
                        // JSONレスポンスをパース
                        match serde_json::from_str::<ValueRange>(&data) {
                            Ok(value_range) => Ok(Response::new_success(value_range.values)),
                            Err(e) => Ok(Response::new_error(&format!(
                                "Failed to parse response: {}",
                                e
                            ))),
                        }
                    } else {
                        Ok(Response::new_error("No data received"))
                    }
                } else {
                    Ok(Response::new_error(
                        &response
                            .error
                            .unwrap_or_else(|| "Unknown error".to_string()),
                    ))
                }
            }
            Err(e) => Err(e),
        }
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
