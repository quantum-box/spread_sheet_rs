// SpreadsheetWriter provides functionality to write data to Google Sheets
use crate::error::Error;
use crate::response::Response;
use crate::sheets_client::SheetsClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum ValueInputOption {
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "USER_ENTERED")]
    UserEntered,
}

impl Default for ValueInputOption {
    fn default() -> Self {
        Self::UserEntered
    }
}

#[derive(Debug, Serialize)]
struct ValueRange {
    range: Option<String>,
    major_dimension: Option<String>,
    values: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Deserialize)]
struct UpdateValuesResponse {
    spreadsheet_id: String,
    updates: Option<UpdateResult>,
}

#[derive(Debug, Deserialize)]
struct UpdateResult {
    updated_range: String,
    updated_rows: i32,
    updated_columns: i32,
    updated_cells: i32,
}

pub struct SpreadsheetWriter {
    client: SheetsClient,
}

impl SpreadsheetWriter {
    pub fn new(client: SheetsClient) -> Self {
        Self { client }
    }

    /// 単一セルへの書き込み
    pub async fn write_cell(
        &self,
        sheet_id: &str,
        range: &str,
        value: &str,
        value_input_option: Option<ValueInputOption>,
    ) -> Result<Response<String>, Error> {
        let input_option = value_input_option.unwrap_or_default();
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption={}",
            sheet_id,
            range,
            serde_json::to_string(&input_option)
                .map_err(|e| Error::ParseError(e.to_string()))?
                .trim_matches('"')
        );

        let body = ValueRange {
            range: Some(range.to_string()),
            major_dimension: Some("ROWS".to_string()),
            values: Some(vec![vec![value.to_string()]]),
        };

        let response = self.client.put(&url, &body).await?;
        let update_response: UpdateValuesResponse =
            serde_json::from_str(&response).map_err(|e| Error::ParseError(e.to_string()))?;

        if let Some(updates) = update_response.updates {
            Ok(Response {
                is_success: true,
                data: Some(format!(
                    "Successfully wrote to range '{}'. Updated {} cells.",
                    updates.updated_range, updates.updated_cells
                )),
                error: None,
            })
        } else {
            Ok(Response {
                is_success: false,
                data: None,
                error: Some("No update information received".to_string()),
            })
        }
    }

    /// 範囲指定での書き込み
    pub async fn write_range(
        &self,
        sheet_id: &str,
        range: &str,
        values: Vec<Vec<String>>,
        value_input_option: Option<ValueInputOption>,
    ) -> Result<Response<String>, Error> {
        let input_option = value_input_option.unwrap_or_default();
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption={}",
            sheet_id,
            range,
            serde_json::to_string(&input_option)
                .map_err(|e| Error::ParseError(e.to_string()))?
                .trim_matches('"')
        );

        let body = ValueRange {
            range: Some(range.to_string()),
            major_dimension: Some("ROWS".to_string()),
            values: Some(values),
        };

        let response = self.client.put(&url, &body).await?;
        let update_response: UpdateValuesResponse =
            serde_json::from_str(&response).map_err(|e| Error::ParseError(e.to_string()))?;

        if let Some(updates) = update_response.updates {
            Ok(Response {
                is_success: true,
                data: Some(format!(
                    "Successfully wrote to range '{}'. Updated {} cells in {} rows and {} columns.",
                    updates.updated_range,
                    updates.updated_cells,
                    updates.updated_rows,
                    updates.updated_columns
                )),
                error: None,
            })
        } else {
            Ok(Response {
                is_success: false,
                data: None,
                error: Some("No update information received".to_string()),
            })
        }
    }
}
