#![allow(dead_code)]
//! # Demo 03: 结构体、枚举、集合
//! 覆盖 rustlings: 07_structs, 08_enums, 05_vecs, 09_strings, 11_hashmaps
//!
//! 核心知识点:
//! - struct 定义数据结构，通过 impl 块添加方法
//! - enum 是带数据的标签联合体，配合 match 做模式匹配
//! - Vec 是动态数组，HashMap 是键值对集合
//! - String 是堆分配可变字符串，&str 是不可变字符串切片

pub fn run() {
    println!("── Demo 03: 结构体、枚举、集合 ──\n");
    demo_structs();
    demo_enums();
    demo_collections();
    println!();
}

// ── 07_structs ───────────────────────────────────────────────

fn demo_structs() {
    println!(">>> 结构体 (对应 rustlings: 07_structs)");

    // 命名结构体
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
        active: bool,
    }

    // 创建实例
    let mut user = User {
        name: String::from("张三"),
        age: 30,
        active: true,
    };
    println!("  命名结构体: {:?}", user);

    // 修改字段 (需要 mut)
    user.age += 1;
    println!("  age += 1 → {:?}", user);

    // 结构体更新语法
    let user2 = User {
        name: String::from("李四"),
        ..user // 其余字段从 user 复制
    };
    println!("  struct update: {:?}", user2);

    // 元组结构体 — 有名字的元组
    #[derive(Debug)]
    struct Point(i32, i32);
    let p = Point(10, 20);
    println!("  元组结构体: Point({}, {})", p.0, p.1);

    // 单元结构体 — 无字段, 用作标记
    #[derive(Debug)]
    struct AlwaysValid;
    println!("  单元结构体: AlwaysValid (无字段, 用作标记类型)");

    // impl 块 — 为类型定义方法
    impl User {
        // 关联函数 (无 self) — 通常用作构造函数
        fn new(name: &str, age: u8) -> Self {
            Self {
                name: name.to_string(),
                age,
                active: true,
            }
        }

        // 方法 &self — 借用实例，不获取所有权
        fn greet(&self) -> String {
            format!("我叫 {}, {} 岁", self.name, self.age)
        }
    }

    let u = User::new("王五", 25);
    println!("  User::new() → {}", u.greet());
}

// ── 08_enums ─────────────────────────────────────────────────

fn demo_enums() {
    println!("\n>>> 枚举 (对应 rustlings: 08_enums)");

    // 带数据的枚举 — 代数数据类型
    #[derive(Debug)]
    enum WebEvent {
        Click { x: i64, y: i64 },  // 命名字段
        KeyPress(char),             // 元组变体
        Paste(String),              // 元组变体
        PageLoad,                   // 无数据变体
    }

    let click = WebEvent::Click { x: 100, y: 200 };
    let key = WebEvent::KeyPress('a');
    let load = WebEvent::PageLoad;

    // match 模式匹配
    fn describe(event: &WebEvent) -> String {
        match event {
            WebEvent::Click { x, y } => format!("点击 ({}, {})", x, y),
            WebEvent::KeyPress(c) => format!("按键 '{}'", c),
            WebEvent::Paste(s) => format!("粘贴: {}", s),
            WebEvent::PageLoad => "页面加载".to_string(),
        } // 必须覆盖所有变体, 否则编译错误
    }

    println!("  {:?} → {}", click, describe(&click));
    println!("  {:?} → {}", key, describe(&key));
    println!("  {:?} → {}", load, describe(&load));

    // Option<T> — Rust 的 null 替代品
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }
    println!("\n  divide(10.0, 2.0) = {:?}", divide(10.0, 2.0));
    println!("  divide(10.0, 0.0) = {:?} (没有 nil panic!)", divide(10.0, 0.0));

    // if let — 只匹配一个分支
    if let Some(result) = divide(10.0, 2.0) {
        println!("  if let Some = {} (只处理 Some 分支)", result);
    }
}

// ── 05_vecs, 09_strings, 11_hashmaps ────────────────────────

fn demo_collections() {
    println!("\n>>> 集合类型 (对应 rustlings: 05_vecs, 09_strings, 11_hashmaps)");

    // Vec — 动态数组
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("  Vec: {:?}", v);

    // vec! 宏快速创建
    let v2 = vec![4, 5, 6];
    println!("  vec![4, 5, 6] = {:?}", v2);

    // 遍历 — 带索引
    for (i, val) in v2.iter().enumerate() {
        print!("  [{}]={} ", i, val);
    }
    println!();

    // String vs &str
    // String — 堆分配、可变
    let mut s = String::from("Hello");
    s.push_str(" Rust");
    println!("  String: \"{}\" (堆分配, 可变)", s);

    // &str — 字符串切片, 不可变
    let slice: &str = &s[0..5];
    println!("  &str: \"{}\" (切片, 不可变)", slice);

    // HashMap — 键值对集合
    let mut scores = std::collections::HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 20);

    // entry 模式 — 只在键不存在时插入
    scores.entry("Yellow").or_insert(30);
    scores.entry("Blue").or_insert(99); // 已存在, 不覆盖
    println!("  HashMap: {:?}", scores);
    println!("  entry().or_insert() — 只在键不存在时插入");
}
