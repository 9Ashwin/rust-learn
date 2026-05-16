use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

/// 服务器配置
#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /// 监听地址，例如 "127.0.0.1:3000"
    pub addr: String,
}

/// 数据库配置
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    /// 连接字符串，例如 "sqlite:data.db?mode=rwc"
    pub url: String,
}

/// 应用配置 — 多层配置合并加载
#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    /// 多层配置合并加载
    ///
    /// 优先级（后者覆盖前者）：
    /// 1. config.toml        — 默认值，提交到 git
    /// 2. config.local.toml  — 本地覆盖，不提交 git
    /// 3. APP_* 环境变量    — 生产环境注入
    pub fn load() -> Self {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Toml::file("config.local.toml"))
            .merge(Env::prefixed("APP_"))
            .extract()
            .expect("加载配置失败，检查 config.toml 是否存在及格式正确")
    }
}
