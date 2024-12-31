use spread_sheet::{Authenticator, Error};

mod test_authenticator {
    use super::*;

    #[test]
    fn test_from_env_missing_var() {
        std::env::remove_var("GOOGLE_CRED");
        let result = Authenticator::from_env();
        assert!(result.is_err());
        match result {
            Err(Error::AuthenticationError(msg)) => {
                assert_eq!(msg, "GOOGLE_CRED環境変数が設定されていません");
            }
            _ => panic!("AuthenticationErrorが期待されます"),
        }
    }

    #[test]
    fn test_from_env_invalid_json() {
        std::env::set_var("GOOGLE_CRED", "invalid json");
        let result = Authenticator::from_env();
        assert!(result.is_err());
        match result {
            Err(Error::AuthenticationError(msg)) => {
                assert!(msg.contains("環境変数から認証情報JSONのパースに失敗しました"));
            }
            _ => panic!("AuthenticationErrorが期待されます"),
        }
    }

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
}
