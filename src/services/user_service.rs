use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection};
use uuid::Uuid;

use crate::dto::common::{PaginatedData, Pagination};
use crate::dto::user::{
    CreateUserRequest, UpdateUserRequest, UserResponse, Validate,
};
use crate::entities::user;
use crate::error::{AppError, AppResult};
use crate::repositories::user_repo::UserRepo;

pub struct UserService;

impl UserService {
    /// 分页获取用户列表
    pub async fn list(
        db: &DatabaseConnection,
        pagination: Pagination,
    ) -> AppResult<PaginatedData<UserResponse>> {
        let (items, total) = UserRepo::list(db, &pagination).await?;

        Ok(PaginatedData {
            items: items.into_iter().map(UserResponse::from).collect(),
            total,
            page: pagination.page,
            page_size: pagination.page_size,
        })
    }

    /// 获取单个用户
    pub async fn get(db: &DatabaseConnection, id: &str) -> AppResult<UserResponse> {
        let user = UserRepo::find_by_id(db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {id} 不存在")))?;
        Ok(UserResponse::from(user))
    }

    /// 创建用户
    pub async fn create(
        db: &DatabaseConnection,
        req: CreateUserRequest,
    ) -> AppResult<UserResponse> {
        // 校验
        req.validate()
            .map_err(AppError::BadRequest)?;

        // 邮箱唯一性
        if UserRepo::find_by_email(db, &req.email).await?.is_some() {
            return Err(AppError::Conflict(format!("邮箱 {} 已被使用", req.email)));
        }

        let now = Utc::now();
        let active = user::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4().to_string()),
            name: ActiveValue::Set(req.name),
            email: ActiveValue::Set(req.email),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };

        let model = UserRepo::insert(db, active).await?;
        Ok(UserResponse::from(model))
    }

    /// 更新用户
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        req: UpdateUserRequest,
    ) -> AppResult<UserResponse> {
        req.validate()
            .map_err(AppError::BadRequest)?;

        let existing = UserRepo::find_by_id(db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {id} 不存在")))?;

        let mut active: user::ActiveModel = existing.into();

        if let Some(name) = req.name {
            active.name = ActiveValue::Set(name);
        }
        if let Some(email) = req.email {
            if let Some(other) = UserRepo::find_by_email(db, &email).await? {
                if other.id != id {
                    return Err(AppError::Conflict(format!("邮箱 {email} 已被使用")));
                }
            }
            active.email = ActiveValue::Set(email);
        }

        active.updated_at = ActiveValue::Set(Utc::now());
        let model = UserRepo::update(db, active).await?;
        Ok(UserResponse::from(model))
    }

    /// 删除用户
    pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResult<()> {
        let existing = UserRepo::find_by_id(db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {id} 不存在")))?;

        let active: user::ActiveModel = existing.into();
        let result = UserRepo::delete(db, active).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("用户 {id} 不存在")));
        }
        Ok(())
    }
}
