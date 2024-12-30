#[cfg(test)]
mod writer_test {
    use spread_sheet_rs::{SheetsClient, SpreadsheetWriter};

    #[test]
    fn test_write_cell() {
        let client = SheetsClient::default(); // ダミー or 本物
        let writer = SpreadsheetWriter::new(client);
        let result = writer.write_cell("test_sheet_id", "A1", "Hello");
        assert!(result.is_ok());
        assert!(result.unwrap().is_success);
    }

    #[test]
    fn test_write_range() {
        let client = SheetsClient::default(); // ダミー or 本物
        let writer = SpreadsheetWriter::new(client);
        let data = vec![vec!["A".to_string()], vec!["B".to_string()]];
        let result = writer.write_range("test_sheet_id", "A1:B2", data);
        assert!(result.is_ok());
        assert!(result.unwrap().is_success);
    }
}
