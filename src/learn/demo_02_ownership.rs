//! # Demo 02: 所有权、借用、生命周期
//! 覆盖 rustlings: 06_move_semantics, 16_lifetimes
//!
//! 核心知识点:
//! - Rust 通过所有权系统管理内存，无需 GC
//! - 赋值/传参会移动所有权 (move)
//! - 引用 (&T) 是借用 (borrow)，不转移所有权
//! - 生命周期标注确保引用始终有效

pub fn run() {
    println!("── Demo 02: 所有权、借用、生命周期 ──\n");
    demo_ownership();
    demo_borrowing();
    demo_lifetimes();
    println!();
}

// ── 06_move_semantics: 所有权 ───────────────────────────────

fn demo_ownership() {
    println!(">>> 所有权 (对应 rustlings: 06_move_semantics)");

    // 简单类型的 Copy — 栈上复制, 原变量仍可用
    let x = 42;
    let y = x; // x 被复制, x 仍然有效
    println!("  i32 实现 Copy: x={}, y={} (都有效)", x, y);

    // String 的 Move — 堆上数据转移所有权
    let s1 = String::from("hello");
    let s2 = s1; // s1 所有权转移给 s2
                 // println!("{}", s1); // ← 编译错误! s1 已失效
    println!("  String 不实现 Copy: s2 = \"{}\" (s1 已失效)", s2);

    // clone 深拷贝 — s3 仍然有效
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("  clone() 深拷贝: s3={}, s4={} (都有效)", s3, s4);

    // 函数传参也是转移
    fn take_ownership(s: String) {
        println!("    函数接收了: {}", s);
    } // s 在此被 drop
    take_ownership(s4);
    // println!("{}", s4); // ← s4 已转移

    // 返回值转移所有权
    fn give_ownership() -> String {
        String::from("归还给你")
    }
    let received = give_ownership();
    println!("  从函数获取所有权: {}", received);
}

// ── 借用 ─────────────────────────────────────────────────────

fn demo_borrowing() {
    println!("\n>>> 借用 (对应 rustlings: 06_move_semantics)");

    let mut data = String::from("你好");

    // 不可变借用 &T — 可以有多个
    let ref1 = &data;
    let ref2 = &data;
    println!("  多个不可变借用: &ref1={}, &ref2={} (可共存)", ref1, ref2);
    // 不可变借用结束后才可写

    // 可变借用 &mut T — 同一时间只能有一个
    let ref_mut = &mut data;
    ref_mut.push_str("，世界");
    println!("  可变借用: 修改为 \"{}\" (独占写入)", ref_mut);
    // 注意: ref1, ref2 之后不能再用

    // 借用规则口诀:
    // - 任意时刻: N 个不可变借用 或 1 个可变借用 (不可共存)
    // - 借用必须不超出所有者的生命周期
    println!("  借用规则: 读共享 / 写独占，编译期强制检查");
}

// ── 16_lifetimes: 生命周期 ──────────────────────────────────

fn demo_lifetimes() {
    println!("\n>>> 生命周期 (对应 rustlings: 16_lifetimes)");

    // 生命周期的核心问题: 当函数返回引用时, 编译器需要知道
    // 这个引用来自哪个参数, 从而检查它不会悬垂

    // 泛型生命周期 'a — 标注输入和输出的关系
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = String::from("短");
    let s2 = String::from("长字符串");
    let result = longest(&s1, &s2);
    println!("  longest(\"{}\", \"{}\") = \"{}\"", s1, s2, result);

    // 带生命周期的结构体
    #[derive(Debug)]
    struct Excerpt<'a> {
        part: &'a str, // 这个引用不能比结构体本身活得更久
    }

    let novel = String::from("从前有座山。山里有个庙。");
    let excerpt = Excerpt {
        part: &novel[0..15],
    };
    println!("  Excerpt {{ part: \"{}\" }}", excerpt.part);

    // 生命周期省略规则 (绝大多数情况无需标注):
    // 1. 每个引用参数都有自己的生命周期
    // 2. 如果只有一个输入生命周期, 赋给所有输出
    // 3. 如果有 &self/&mut self, 它的生命周期赋给所有输出
    println!("  大多数情况编译器自动推断, 无需手动标注");
}
