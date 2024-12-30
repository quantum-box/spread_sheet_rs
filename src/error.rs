#[derive(Debug)]
pub enum Error {
    AuthenticationError(String),
    ApiError(String),
    NetworkError(String),
    ParseError(String),
    OtherError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthenticationError(msg) => write!(f, "認証エラー: {}", msg),
            Error::ApiError(msg) => write!(f, "APIエラー: {}", msg),
            Error::NetworkError(msg) => write!(f, "ネットワークエラー: {}", msg),
            Error::ParseError(msg) => write!(f, "パースエラー: {}", msg),
            Error::OtherError(msg) => write!(f, "その他のエラー: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
