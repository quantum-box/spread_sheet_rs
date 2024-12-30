mod authenticator;
mod error;
mod sheets_client;
mod response;

pub use authenticator::Authenticator;
pub use error::Error;
pub use sheets_client::SheetsClient;
pub use response::Response;

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
