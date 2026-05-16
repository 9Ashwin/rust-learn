use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dto::user::UserResponse;

/// 统一 API 响应体（泛型，运行时用）
#[derive(Debug, Serialize)]
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

/// 分页响应（泛型，运行时用）
#[derive(Debug, Serialize)]
pub struct PaginatedData<T: Serialize> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

/// 分页请求参数
#[derive(Debug, Deserialize, ToSchema)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 { 1 }
fn default_page_size() -> u64 { 10 }

// ── OpenAPI 专用具体类型（避免泛型 $ref 解析错误）──────────────

/// 用户响应 — OpenAPI schema
#[derive(ToSchema)]
pub struct UserApiResponse {
    pub code: i32,
    pub data: Option<UserResponse>,
    pub message: String,
}

/// 分页用户响应 — OpenAPI schema
#[derive(ToSchema)]
pub struct PaginatedUserApiResponse {
    pub code: i32,
    pub data: Option<PaginatedUserData>,
    pub message: String,
}

/// 空响应 — OpenAPI schema
#[derive(ToSchema)]
pub struct EmptyApiResponse {
    pub code: i32,
    pub data: Option<()>,
    pub message: String,
}

/// 分页用户数据 — OpenAPI schema
#[derive(ToSchema)]
pub struct PaginatedUserData {
    pub items: Vec<UserResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
