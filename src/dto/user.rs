use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 现有用户响应 — 不暴露内部字段名
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::entities::user::Model> for UserResponse {
    fn from(m: crate::entities::user::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            email: m.email,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

/// 创建用户请求 — 带自定义校验
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// 更新用户请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// 校验 trait — 请求数据校验
pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

impl Validate for CreateUserRequest {
    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("名称不能为空".into());
        }
        if self.email.trim().is_empty() || !self.email.contains('@') {
            return Err("邮箱格式无效".into());
        }
        Ok(())
    }
}

impl Validate for UpdateUserRequest {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("名称不能为空".into());
            }
        }
        if let Some(ref email) = self.email {
            if email.trim().is_empty() || !email.contains('@') {
                return Err("邮箱格式无效".into());
            }
        }
        Ok(())
    }
}
