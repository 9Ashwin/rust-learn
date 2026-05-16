use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod dto;
mod entities;
mod error;
mod handlers;
mod learn;
mod openapi;
mod repositories;
mod router;
mod services;

/// Rust 学习 Demo — SeaORM + 三层架构 + DTO + 分页 + 优雅关闭
#[derive(Parser)]
#[command(name = "rust-learn")]
#[command(about = "Rust 技术栈学习 Demo")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// 启动 HTTP 服务器
    Serve,
    /// 运行 rustlings 全部学习 demo
    Learn,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_learn=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let config = config::AppConfig::load();
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Serve) {
        Command::Serve => {
            let db = db::init_db(&config.database.url).await?;

            let app = router::create_router(db);
            let listener = tokio::net::TcpListener::bind(&config.server.addr).await?;
            tracing::info!("服务器启动: http://{}", config.server.addr);
            tracing::info!("按 Ctrl+C 优雅关闭");

            // 优雅关闭: 收到 SIGTERM/SIGINT 后等待现有请求完成再退出
            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown_signal())
                .await?;

            tracing::info!("服务器已关闭");
        }
        Command::Learn => {
            learn::run_all();
        }
    }

    Ok(())
}

/// 监听 Ctrl+C (SIGINT) 和 SIGTERM
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("收到关闭信号, 开始优雅退出...");
}
