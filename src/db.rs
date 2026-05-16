use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};

use crate::entities::user;

/// 创建数据库连接并自动建表
///
/// Schema::create_table_from_entity 对标 GORM 的 AutoMigrate:
///   db.AutoMigrate(&User{})
pub async fn init_db(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db = Database::connect(database_url).await?;

    // 自动建表 (IF NOT EXISTS — 幂等)
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let mut stmt = schema.create_table_from_entity(user::Entity);
    stmt.if_not_exists();
    db.execute(builder.build(&stmt)).await?;

    tracing::info!("数据库已连接，表结构已同步");
    Ok(db)
}
