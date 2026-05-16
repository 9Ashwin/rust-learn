#![allow(dead_code)]
//! # Demo 06: 高级特性
//! 覆盖 rustlings: 18_iterators, 19_smart_pointers, 23_conversions
//!
//! 核心知识点:
//! - 迭代器是惰性的，支持链式组合
//! - Box 用于堆分配和递归类型
//! - From/Into 提供类型转换 trait
//! - Rc/Arc 提供引用计数共享

pub fn run() {
    println!("── Demo 06: 迭代器、智能指针、类型转换 ──\n");
    demo_iterators();
    demo_smart_pointers();
    demo_conversions();
    println!();
}

// ── 18_iterators ─────────────────────────────────────────────

fn demo_iterators() {
    println!(">>> 迭代器 (对应 rustlings: 18_iterators)");

    let numbers = vec![1, 2, 3, 4, 5, 6];

    // 创建迭代器 — 迭代器是惰性的, 不消费就不执行
    let iter = numbers.iter();

    // 消费: .collect() 收集结果
    let doubled: Vec<i32> = iter.map(|n| n * 2).collect();
    println!("  map + collect: {:?}", doubled);

    // 链式调用
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)  // 过滤偶数
        .map(|n| n * 10)            // 乘 10
        .collect();
    println!("  偶数 * 10: {:?}", result);

    // fold — 累加器模式
    let sum: i32 = numbers.iter().fold(0, |acc, &n| acc + n);
    println!("  fold 求和: {}", sum);

    // enumerate — 带索引遍历
    for (i, val) in numbers.iter().enumerate() {
        print!("  [{}]={} ", i, val);
    }
    println!();

    // flat_map — 扁平化
    let words = vec!["hello world", "rust demo"];
    let chars: Vec<char> = words
        .iter()
        .flat_map(|w| w.chars())
        .filter(|c| c.is_alphabetic())
        .collect();
    println!("  flat_map 字符: {:?}", chars);

    // product — 阶乘
    let factorial: u64 = (1..=5).product();
    println!("  5! = {} (product)", factorial);
}

// ── 19_smart_pointers ───────────────────────────────────────

fn demo_smart_pointers() {
    println!("\n>>> 智能指针 (对应 rustlings: 19_smart_pointers)");

    // Box<T> — 堆分配，拥有数据的所有权
    let b = Box::new(42);
    println!("  Box<i32> = {} (堆分配)", b);

    // Box 用于递归类型 — 让大小在编译期确定
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // Box 让大小固定
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("  {:?} (递归枚举, Box 让大小固定)", list);

    // Rc<T> — 单线程引用计数, 共享所有权
    use std::rc::Rc;
    let s = Rc::new(String::from("共享数据"));
    let s1 = Rc::clone(&s); // 引用计数 +1
    let s2 = Rc::clone(&s); // 引用计数 +1
    println!("  Rc: count = {} (共享所有权)", Rc::strong_count(&s));
    drop(s1);
    println!("  drop s1: count = {}", Rc::strong_count(&s));
    drop(s2);

    // Arc<T> — 原子引用计数, 用于多线程
    println!("  Arc = 多线程版本的 Rc (下个 demo 演示)");

    // Cow<T> — Clone-on-Write
    use std::borrow::Cow;
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    let mut owned: Cow<str> = Cow::Borrowed("hello");
    owned.to_mut().push_str(" world"); // 此时才 clone
    println!("  Cow: 借读 = \"{}\", 写时克隆 = \"{}\"", borrowed, owned);
    println!("  Cow = 写时复制，仅在需要修改时才分配内存");
}

// ── 23_conversions ───────────────────────────────────────────

fn demo_conversions() {
    println!("\n>>> 类型转换 (对应 rustlings: 23_conversions)");

    // as — 基本类型转换
    let pi: f64 = std::f64::consts::PI;
    let pi_approx: i32 = pi as i32;
    println!("  pi as i32 = {} (截断转换)", pi_approx);

    // From<T> / Into<T> — 标准类型转换 trait
    #[derive(Debug, PartialEq)]
    struct Celsius(f64);

    #[derive(Debug, PartialEq)]
    struct Fahrenheit(f64);

    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Self {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }

    let temp = Celsius(37.0);
    // Into 只要实现了 From 就自动可用
    let f: Fahrenheit = temp.into(); // 自动推导
    println!("  37°C = {}°F (From 自动获得 Into)", f.0);

    // TryFrom — 可失败的转换
    #[derive(Debug)]
    struct Age(u8);

    impl std::convert::TryFrom<i32> for Age {
        type Error = &'static str;
        fn try_from(n: i32) -> Result<Self, Self::Error> {
            if n < 0 || n > 150 {
                Err("年龄必须在 0-150 之间")
            } else {
                Ok(Age(n as u8))
            }
        }
    }

    let a: Result<Age, _> = (25i32).try_into();
    println!("  TryFrom 25 = {:?}", a.unwrap());
    let a: Result<Age, _> = (-5i32).try_into();
    println!("  TryFrom -5 = {:?}", a);

    // parse() — 字符串解析 (FromStr trait)
    let n: i32 = "42".parse().unwrap();
    println!("  \"42\".parse::<i32>() = {}", n);

    // AsRef — 廉价引用转换 (如 String → &str)
    fn print_str(s: impl AsRef<str>) {
        println!("  AsRef<str>: \"{}\" (String 和 &str 都能传)", s.as_ref());
    }
    print_str("hello");
    print_str(String::from("world"));
}
