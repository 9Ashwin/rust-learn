# rust-learn

基于 rustlings 24 个练习模块的 Rust 学习 Demo，附带 Axum + SeaORM Web 应用示例。

## 快速开始

```bash
# 安装依赖
cargo build

# 运行全部学习 demo
cargo run -- learn

# 启动 API 服务器
cargo run -- serve

# 运行测试
cargo test
```

## 项目结构

```
src/
├── main.rs          # 入口，CLI 子命令 (serve / learn)
├── config.rs        # 多层配置合并 (figment)
├── db.rs            # 数据库连接
├── error.rs         # 错误类型
├── openapi.rs       # OpenAPI 文档
├── router.rs        # 路由定义
├── dto/             # 请求/响应数据结构
├── entities/        # SeaORM 实体
├── handlers/        # HTTP 处理器
├── repositories/    # 数据访问层
├── services/        # 业务逻辑层
└── learn/           # Rust 学习 Demo 模块
    ├── demo_01_basics.rs          # 变量、函数、控制流
    ├── demo_02_ownership.rs       # 所有权、借用、生命周期
    ├── demo_03_types.rs           # 结构体、枚举、集合
    ├── demo_04_error_handling.rs  # Option、Result、错误传播
    ├── demo_05_generics_traits.rs # 泛型、Trait、动态分发
    ├── demo_06_advanced.rs        # 迭代器、智能指针、类型转换
    ├── demo_07_concurrency.rs     # 线程、Mutex、Channel
    └── demo_08_macros.rs          # 声明式宏
```

## 配置

| 文件 | 用途 |
|------|------|
| `config.toml` | 默认配置，提交到仓库 |
| `config.local.toml` | 本地覆盖，不提交 |
| `.env` | 环境变量覆盖 (`APP_*` 前缀) |

配置优先级：环境变量 > config.local.toml > config.toml
