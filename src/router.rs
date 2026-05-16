use axum::{http::StatusCode, routing::get, Json, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::dto::common::ApiResponse;
use crate::handlers::user;
use crate::openapi::ApiDoc;

pub fn create_router(db: DatabaseConnection) -> Router {
    let shared_db = Arc::new(db);

    Router::new()
        // 用户 CRUD
        .route("/api/users", get(user::list_users).post(user::create_user))
        .route(
            "/api/users/:id",
            get(user::get_user)
                .put(user::update_user)
                .delete(user::delete_user),
        )
        // 健康检查
        .route("/health", get(health_check))
        // OpenAPI 文档 (SwaggerUi 自动注册 /api-docs/openapi.json)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // 兜底
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(shared_db)
}

/// 健康检查
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "服务正常", body = ApiResponse<serde_json::Value>),
    ),
    tag = "系统"
)]
pub async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(
        serde_json::json!({"status": "healthy"}),
    ))
}

async fn not_found() -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            code: 40400,
            data: None,
            message: "路径不存在".into(),
        }),
    )
}
