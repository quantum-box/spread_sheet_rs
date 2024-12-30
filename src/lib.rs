mod authenticator;
mod error;
mod response;
mod sheets_client;

pub use authenticator::Authenticator;
pub use error::Error;
pub use response::Response;
pub use sheets_client::SheetsClient;

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
