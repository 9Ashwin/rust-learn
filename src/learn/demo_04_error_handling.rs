#![allow(dead_code, unused_must_use)]
//! # Demo 04: 错误处理
//! 覆盖 rustlings: 12_options, 13_error_handling
//!
//! 核心知识点:
//! - Option<T> 替代空值，编译器强制检查
//! - Result<T, E> 表示可恢复的错误
//! - `?` 操作符简化错误传播
//! - thiserror 自动生成错误类型及其 Display 实现

pub fn run() {
    println!("── Demo 04: 错误处理 ──\n");
    demo_option();
    demo_result();
    demo_custom_error();
    demo_error_propagation();
    println!();
}

// ── 12_options ───────────────────────────────────────────────

fn demo_option() {
    println!(">>> Option (对应 rustlings: 12_options)");

    // Option<T> — Rust 的 null 替代品
    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some(String::from("Alice")),
            2 => Some(String::from("Bob")),
            _ => None,
        }
    }

    // unwrap — 取出值, None 会 panic
    println!("  find_user(1).unwrap() = {}", find_user(1).unwrap());

    // unwrap_or — 默认值
    println!("  find_user(99).unwrap_or(\"Guest\") = {}", find_user(99).unwrap_or("Guest".to_string()));

    // if let — 只处理 Some 分支
    if let Some(name) = find_user(1) {
        println!("  if let Some(name) = ... → 找到: {}", name);
    }

    // map — 链式处理
    let upper = find_user(2).map(|n| n.to_uppercase());
    println!("  find_user(2).map(|n| n.to_uppercase()) = {:?}", upper);

    // and_then — flat map
    fn get_first_char(s: &str) -> Option<char> {
        s.chars().next()
    }
    let first = find_user(1).and_then(|n| get_first_char(&n));
    println!("  and_then 链式处理 → {:?}", first);
}

// ── 13_error_handling: Result ────────────────────────────────

fn demo_result() {
    println!("\n>>> Result (对应 rustlings: 13_error_handling)");

    // Result<T, E> — 可恢复的错误
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>() // 返回 Result<i32, ParseIntError>
    }

    // match 处理 — 分别处理 Ok 和 Err 分支
    match parse_number("42") {
        Ok(n) => println!("  parse(\"42\") = {} (成功)", n),
        Err(e) => println!("  错误: {}", e),
    }

    match parse_number("not a number") {
        Ok(n) => println!("  {}", n),
        Err(e) => println!("  parse(\"not a number\") = Err({}) (失败)", e),
    }

    // unwrap — 快速取出, 失败就 panic (仅测试/原型用)
    let n = parse_number("100").unwrap();
    println!("  parse(\"100\").unwrap() = {}", n);

    // expect — unwrap + 自定义 panic 信息
    let n = parse_number("200")
        .expect("这应该总是合法的数字");
    println!("  expect() = {}", n);

    // 组合子方法
    let doubled = parse_number("21")
        .map(|n| n * 2)               // Ok 时转换
        .unwrap_or(-1);                // Err 时给默认值
    println!("  map + unwrap_or: {}", doubled);
}

// ── 自定义错误 ───────────────────────────────────────────────

fn demo_custom_error() {
    println!("\n>>> 自定义错误 (对应 rustlings: 13_error_handling)");

    // 用 thiserror 定义错误枚举
    #[derive(thiserror::Error, Debug)]
    enum MyAppError {
        #[error("无效输入: {0}")]
        InvalidInput(String),

        #[error("未找到: id={0}")]
        NotFound(u32),

        #[error("权限不足")]
        PermissionDenied,
    }

    // 使用自定义错误
    fn get_user(id: u32) -> Result<&'static str, MyAppError> {
        match id {
            0 => Err(MyAppError::InvalidInput("ID 不能为 0".into())),
            1 => Ok("Alice"),
            _ => Err(MyAppError::NotFound(id)),
        }
    }

    println!("  get_user(1) = {:?}", get_user(1));
    println!("  get_user(0) = {:?}", get_user(0));
    println!("  get_user(99) = {:?}", get_user(99));
}

// ── ? 操作符: 错误传播 ──────────────────────────────────────

fn demo_error_propagation() {
    println!("\n>>> ? 操作符 (对应 rustlings: 13_error_handling)");

    // ? — 如果 Err, 立即返回; 如果 Ok, 取出值
    // 类似于 if let Err(e) = result { return Err(e.into()) }

    fn process_file(path: &str) -> Result<String, std::io::Error> {
        let content = std::fs::read_to_string(path)?; // ← 出错立即返回
        Ok(content.to_uppercase())
    }

    // ? 还能自动类型转换 (需要 From trait)
    #[derive(thiserror::Error, Debug)]
    enum AppError {
        #[error("IO 错误: {0}")]
        Io(#[from] std::io::Error), // #[from] 自动生成类型转换
        #[error("解析错误: {0}")]
        Parse(#[from] std::num::ParseIntError),
    }

    fn complex_operation() -> Result<i32, AppError> {
        let content = std::fs::read_to_string("/tmp/data.txt")?; // Io → AppError
        let num: i32 = content.trim().parse()?; // ParseInt → AppError
        Ok(num * 2)
    }
    // 注意: /tmp/data.txt 不存在, 所以这里会返回 Err
    println!("  complex_operation() = {:?}", complex_operation());

    // main 函数中的 ?
    println!("  ? 操作符 = 编译期自动错误传播, 配合 #[from] 自动类型转换");
}
