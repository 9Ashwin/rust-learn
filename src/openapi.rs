use utoipa::OpenApi;

use crate::dto::common::*;
use crate::dto::user::*;
use crate::handlers;

/// API 文档入口
///
/// /swagger-ui  — 交互式 Try it out
/// /api-docs/openapi.json — 原始 JSON
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::user::list_users,
        handlers::user::get_user,
        handlers::user::create_user,
        handlers::user::update_user,
        handlers::user::delete_user,
        crate::router::health_check,
    ),
    components(
        schemas(
            UserResponse,
            CreateUserRequest,
            UpdateUserRequest,
            Pagination,
            // 用 alias 替代泛型，Swagger UI 可正确解析 $ref
            UserApiResponse,
            PaginatedUserApiResponse,
            EmptyApiResponse,
            PaginatedUserData,
        )
    ),
    tags(
        (name = "用户管理", description = "用户 CRUD 接口"),
        (name = "系统", description = "健康检查"),
    )
)]
pub struct ApiDoc;
