//! Rust 学习 Demo — 基于 rustlings 24 个练习模块
//!
//! 每个 demo_XX 对应一组 rustlings 练习，包含可运行示例
//! 运行: `cargo run -- learn` 执行全部 demo

pub mod demo_01_basics;
pub mod demo_02_ownership;
pub mod demo_03_types;
pub mod demo_04_error_handling;
pub mod demo_05_generics_traits;
pub mod demo_06_advanced;
pub mod demo_07_concurrency;
pub mod demo_08_macros;

/// 运行全部学习 demo
pub fn run_all() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║     Rust 学习 Demo — rustlings 全覆盖            ║");
    println!("║     Rust 语言特性全面覆盖                          ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    demo_01_basics::run();
    demo_02_ownership::run();
    demo_03_types::run();
    demo_04_error_handling::run();
    demo_05_generics_traits::run();
    demo_06_advanced::run();
    demo_07_concurrency::run();
    demo_08_macros::run();

    println!("══════════════════════════════════════════════════");
    println!("  全部 demo 执行完毕。运行 `cargo test` 查看更多。");
    println!("══════════════════════════════════════════════════");
}
