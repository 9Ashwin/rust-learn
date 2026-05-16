use serde::Serialize;
use utoipa::ToSchema;

/// 统一 API 响应体
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { code: 0, data: Some(data), message: "ok".into() }
    }

    pub fn success_no_data() -> ApiResponse<()> {
        ApiResponse { code: 0, data: None, message: "ok".into() }
    }

    #[allow(dead_code)]
    pub fn error(code: i32, message: &str) -> ApiResponse<()> {
        ApiResponse { code, data: None, message: message.into() }
    }
}

/// 分页响应
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedData<T: Serialize> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

/// 分页请求参数
#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 { 1 }
fn default_page_size() -> u64 { 10 }
