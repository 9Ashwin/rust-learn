/// 应用错误 — 仅内部使用
///
/// handler 层通过 `map_error` 函数映射为 HTTP 状态码 + ApiResponse
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("未找到: {0}")]
    NotFound(String),

    #[error("参数错误: {0}")]
    BadRequest(String),

    #[error("冲突: {0}")]
    Conflict(String),
}

pub type AppResult<T> = Result<T, AppError>;
