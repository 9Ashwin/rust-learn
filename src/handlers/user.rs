use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::dto::common::{ApiResponse, PaginatedData, Pagination};
use crate::dto::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::error::AppError;
use crate::services::user_service::UserService;

/// 获取用户列表
#[utoipa::path(
    get,
    path = "/api/users",
    params(
        ("page" = u64, Query, description = "页码，默认 1"),
        ("page_size" = u64, Query, description = "每页数量，默认 10"),
    ),
    responses(
        (status = 200, description = "分页用户列表", body = PaginatedUserApiResponse),
    ),
    tag = "用户管理"
)]
pub async fn list_users(
    State(db): State<Arc<DatabaseConnection>>,
    Query(pagination): Query<Pagination>,
) -> Json<ApiResponse<PaginatedData<UserResponse>>> {
    match UserService::list(&db, pagination).await {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => {
            let (code, msg) = err_body(&e);
            Json(ApiResponse { code, data: None, message: msg })
        }
    }
}

/// 获取单个用户
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = String, Path, description = "用户 ID"),
    ),
    responses(
        (status = 200, description = "用户详情", body = UserApiResponse),
        (status = 404, description = "用户不存在", body = EmptyApiResponse),
    ),
    tag = "用户管理"
)]
pub async fn get_user(
    State(db): State<Arc<DatabaseConnection>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    match UserService::get(&db, &id).await {
        Ok(user) => (StatusCode::OK, Json(ApiResponse::success(user))),
        Err(e) => {
            let (code, msg) = err_body(&e);
            (status_from_code(code), Json(ApiResponse { code, data: None, message: msg }))
        }
    }
}

/// 创建用户
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "创建成功", body = UserApiResponse),
        (status = 400, description = "参数校验失败", body = EmptyApiResponse),
        (status = 409, description = "邮箱已存在", body = EmptyApiResponse),
    ),
    tag = "用户管理"
)]
pub async fn create_user(
    State(db): State<Arc<DatabaseConnection>>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    match UserService::create(&db, req).await {
        Ok(user) => (StatusCode::CREATED, Json(ApiResponse::success(user))),
        Err(e) => {
            let (code, msg) = err_body(&e);
            (status_from_code(code), Json(ApiResponse { code, data: None, message: msg }))
        }
    }
}

/// 更新用户
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = String, Path, description = "用户 ID"),
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "更新成功", body = UserApiResponse),
        (status = 404, description = "用户不存在", body = EmptyApiResponse),
    ),
    tag = "用户管理"
)]
pub async fn update_user(
    State(db): State<Arc<DatabaseConnection>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateUserRequest>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    match UserService::update(&db, &id, req).await {
        Ok(user) => (StatusCode::OK, Json(ApiResponse::success(user))),
        Err(e) => {
            let (code, msg) = err_body(&e);
            (status_from_code(code), Json(ApiResponse { code, data: None, message: msg }))
        }
    }
}

/// 删除用户
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = String, Path, description = "用户 ID"),
    ),
    responses(
        (status = 204, description = "删除成功"),
        (status = 404, description = "用户不存在", body = EmptyApiResponse),
    ),
    tag = "用户管理"
)]
pub async fn delete_user(
    State(db): State<Arc<DatabaseConnection>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    match UserService::delete(&db, &id).await {
        Ok(()) => (StatusCode::NO_CONTENT, Json(ApiResponse::<()>::success_no_data())),
        Err(e) => {
            let (code, msg) = err_body(&e);
            (status_from_code(code), Json(ApiResponse { code, data: None, message: msg }))
        }
    }
}

/// AppError → (业务码, 消息)
fn err_body(e: &AppError) -> (i32, String) {
    match e {
        AppError::Database(err) => {
            tracing::error!(%err, "数据库错误");
            (50001, "内部服务器错误".into())
        }
        AppError::NotFound(msg) => (40400, msg.clone()),
        AppError::BadRequest(msg) => (40000, msg.clone()),
        AppError::Conflict(msg) => (40900, msg.clone()),
    }
}

/// 业务码 → HTTP 状态码
fn status_from_code(code: i32) -> StatusCode {
    match code / 100 {
        400 => StatusCode::BAD_REQUEST,
        404 => StatusCode::NOT_FOUND,
        409 => StatusCode::CONFLICT,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
