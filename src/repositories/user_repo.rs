use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::dto::common::Pagination;
use crate::entities::user;

/// 用户数据访问层
pub struct UserRepo;

impl UserRepo {
    /// 分页列表
    pub async fn list(
        db: &DatabaseConnection,
        pagination: &Pagination,
    ) -> Result<(Vec<user::Model>, u64), sea_orm::DbErr> {
        let paginator = user::Entity::find()
            .order_by_desc(user::Column::CreatedAt)
            .paginate(db, pagination.page_size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(pagination.page - 1).await?;
         Ok((items, total))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn insert(
        db: &DatabaseConnection,
        active: user::ActiveModel,
    ) -> Result<user::Model, sea_orm::DbErr> {
        active.insert(db).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        active: user::ActiveModel,
    ) -> Result<user::Model, sea_orm::DbErr> {
        active.update(db).await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        active: user::ActiveModel,
    ) -> Result<sea_orm::DeleteResult, sea_orm::DbErr> {
        active.delete(db).await
    }
}
