/// Response構造体: API呼び出し結果を管理
#[derive(Debug)]
pub struct Response<T> {
    pub is_success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> Response<T> {
    pub fn new_success(data: T) -> Self {
        Self {
            is_success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn new_error(msg: &str) -> Self {
        Self {
            is_success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.data, &self.error) {
            (Some(data), _) => write!(f, "{}", data),
            (None, Some(error)) => write!(f, "エラー: {}", error),
            (None, None) => write!(f, "レスポンスにデータがありません"),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("is_success", &self.is_success)
            .field("data", &self.data)
            .field("error", &self.error)
            .finish()
    }
}
