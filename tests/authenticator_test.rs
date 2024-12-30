use spread_sheet_rs::{Authenticator, Error};

#[test]
fn test_api_key_success() {
    let auth = Authenticator::new(Some("dummy_key".to_string()));
    let key = auth.get_api_key().unwrap();
    assert_eq!(key, "dummy_key");
}

#[test]
fn test_api_key_missing() {
    let auth = Authenticator::new(None);
    let result = auth.get_api_key();
    assert!(result.is_err());
    match result {
        Err(Error::AuthenticationError(msg)) => {
            assert_eq!(msg, "No API key provided.");
        }
        _ => panic!("Expected AuthenticationError"),
    }
}
