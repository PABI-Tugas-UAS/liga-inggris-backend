use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl<T> BaseResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        BaseResponse {
            success: true,
            message: message.to_string(),
            data,
        }
    }

    pub fn failure(message: &str, data: T) -> Self {
        BaseResponse {
            success: false,
            message: message.to_string(),
            data,
        }
    }
}
