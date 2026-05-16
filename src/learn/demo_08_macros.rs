//! # Demo 08: 宏
//! 覆盖 rustlings: 21_macros
//!
//! 核心知识点:
//! - 宏在编译期展开，生成代码
//! - macro_rules! 通过模式匹配进行代码替换
//! - println!、vec!、assert_eq! 都是声明式宏
//! - 宏可以接受可变数量的参数，生成重复代码

pub fn run() {
    println!("── Demo 08: 宏 ──\n");
    demo_builtin_macros();
    demo_custom_macro();
    println!();
}

// ── 内置宏 ───────────────────────────────────────────────────

fn demo_builtin_macros() {
    println!(">>> 内置宏");

    // println! — 编译期检查格式串
    let name = "Rust";
    println!("  println!(\"Hello, {}!\") — 编译期检查格式串", name);

    // vec! — 快速创建 Vec
    let v = vec![1, 2, 3, 4, 5];
    println!("  vec![1,2,3,4,5] = {:?}", v);

    // format! — 格式化字符串，返回 String
    let s = format!("{}-{}", v.len(), name);
    println!("  format!() = {}", s);

    // assert! / assert_eq! — 测试断言，失败则 panic
    let expected = 15;
    let actual: i32 = v.iter().sum();
    assert_eq!(expected, actual, "求和应该等于 15");
    println!("  assert_eq!(15, sum) ✓ (编译期通过, 否则 panic)");

    // dbg! — 快速调试打印，输出文件:行号和表达式值
    let x = 42;
    let y = dbg!(x * 2); // 打印文件:行号 + 表达式 + 值
    println!("  dbg! 返回: {} (同时打印到 stderr)", y);

    // todo! / unimplemented! — 标记未完成代码
    // println!("  todo!() 会 panic, 这里不演示");
}

// ── 自定义宏 ─────────────────────────────────────────────────

fn demo_custom_macro() {
    println!("\n>>> 自定义宏 (对应 rustlings: 21_macros)");

    // macro_rules! — 声明式宏
    // 语法: (pattern) => { expansion }
    macro_rules! my_vec {
        // 空 vec
        () => {
            Vec::new()
        };
        // 一个元素的 vec — $( ) 重复模式
        ( $( $x:expr ),* $(,)? ) => {
            {
                let mut v = Vec::new();
                $(
                    v.push($x);
                )*
                v
            }
        };
    }

    let v1: Vec<i32> = my_vec!();
    let v2 = my_vec![1, 2, 3];
    let v3 = my_vec!["a", "b", "c"];

    println!("  my_vec!() = {:?}", v1);
    println!("  my_vec![1,2,3] = {:?}", v2);
    println!("  my_vec![\"a\",\"b\",\"c\"] = {:?}", v3);

    // 实用的自定义宏 — 快速构建 HashMap
    macro_rules! hashmap {
        ( $( $key:expr => $val:expr ),* $(,)? ) => {
            {
                let mut m = std::collections::HashMap::new();
                $(
                    m.insert($key, $val);
                )*
                m
            }
        };
    }

    let m = hashmap! {
        "Alice" => 30,
        "Bob" => 25,
    };
    println!("  hashmap! 宏: {:?}", m);

    // 宏的关键概念:
    println!("\n  宏 vs 函数的区别:");
    println!("  - 宏在编译期展开, 函数在运行时调用");
    println!("  - 宏可以接受可变数量的参数");
    println!("  - 宏可以生成重复代码");
    println!("  - 宏调用末尾加 ! 是 Rust 的约定");
}
