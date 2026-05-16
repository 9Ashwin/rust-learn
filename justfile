# Rust 学习项目 — just 命令手册
# 安装: brew install just  (或 cargo install just)

# 默认: 列出所有命令
default:
    @just --list

# 编译 (debug)
build:
    cargo build

# 编译 (release)
release:
    cargo build --release

# 运行学习 demo
learn:
    cargo run -- learn

# 启动 API 服务器
serve:
    RUST_LOG=info cargo run -- serve

# 启动服务器 (debug 日志)
dev:
    RUST_LOG=debug cargo run -- serve

# 运行全部测试
test:
    cargo test

# 运行测试 + 显示输出
test-v:
    cargo test -- --nocapture

# Lint 检查
lint:
    cargo clippy -- -D warnings

# 格式化代码
fmt:
    cargo fmt
    @echo "代码已格式化"

# 检查格式 (CI 用)
fmt-check:
    cargo fmt --check

# 清理编译产物
clean:
    cargo clean

# 全量检查 (提交前运行)
check: lint test fmt-check
    @echo "全部通过 ✓"

# 更新依赖
update:
    cargo update
