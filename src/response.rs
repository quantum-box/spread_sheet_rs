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
