use crate::error::Error;
use crate::response::Response;
use crate::sheets_client::SheetsClient;

pub struct SpreadsheetWriter {
    client: SheetsClient,
}

impl SpreadsheetWriter {
    pub fn new(client: SheetsClient) -> Self {
        Self { client }
    }

    /// 単一セルへの書き込み例
    pub fn write_cell(
        &self,
        sheet_id: &str,
        range: &str,
        value: &str,
    ) -> Result<Response<String>, Error> {
        // 実際には Google Sheets API の書き込み用エンドポイントを呼び出す処理を入れる
        // Mock 実装 or 例示のみとしておく
        Ok(Response {
            is_success: true,
            data: Some(format!("Wrote '{}' to range '{}'", value, range)),
            error: None,
        })
    }

    /// 範囲指定での書き込み例
    pub fn write_range(
        &self,
        sheet_id: &str,
        range: &str,
        values: Vec<Vec<String>>,
    ) -> Result<Response<String>, Error> {
        // 実際の書き込みロジックを入れる
        Ok(Response {
            is_success: true,
            data: Some(format!("Wrote {} rows to range '{}'", values.len(), range)),
            error: None,
        })
    }
}
