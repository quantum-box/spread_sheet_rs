use crate::error::Error;

pub struct Authenticator {
    api_key: Option<String>,
    // OAuth 2.0 のトークン等も後で追加
}

impl Authenticator {
    // コンストラクタ
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    // とりあえず API キーを返すダミーメソッド
    pub fn get_api_key(&self) -> Result<String, Error> {
        match &self.api_key {
            Some(key) => Ok(key.clone()),
            None => Err(Error::AuthenticationError(String::from(
                "No API key provided.",
            ))),
        }
    }
}
