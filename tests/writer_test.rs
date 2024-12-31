#[cfg(test)]
mod writer_test {
    use spread_sheet::{Authenticator, SheetsClient, SpreadsheetWriter, spreadsheet_writer::ValueInputOption};

    #[tokio::test]
    async fn test_write_cell() {
        let client = SheetsClient::new(Authenticator::new(None)); // ダミー or 本物
        let writer = SpreadsheetWriter::new(client);
        let result = writer.write_cell("test_sheet_id", "A1", "Hello", Some(ValueInputOption::Raw)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_success);
    }

    #[tokio::test]
    async fn test_write_range() {
        let client = SheetsClient::new(Authenticator::new(None)); // ダミー or 本物
        let writer = SpreadsheetWriter::new(client);
        let data = vec![vec!["A".to_string()], vec!["B".to_string()]];
        let result = writer.write_range("test_sheet_id", "A1:B2", data, Some(ValueInputOption::Raw)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_success);
    }
}
