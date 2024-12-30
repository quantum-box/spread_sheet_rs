pub mod authenticator;
pub mod error;
pub mod response;
pub mod sheets_client;
pub mod spreadsheet_reader;

// Re-export commonly used types
pub use authenticator::Authenticator;
pub use error::Error;
pub use response::Response;
pub use sheets_client::SheetsClient;
pub use spreadsheet_reader::SpreadsheetReader;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
