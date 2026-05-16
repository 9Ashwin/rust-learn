//! 集成测试 — 演示 rustlings 17_tests 的概念
//!
//! 核心知识点:
//! - #[test] 标记测试函数
//! - assert_eq! 用于值比较
//! - #[should_panic] 标记预期 panic 的测试
//! - cargo test 运行全部测试

use std::collections::HashMap;

// ── 基本断言 (对应 rustlings: tests1) ───────────────────────

#[test]
fn test_basic_assertions() {
    // assert! — 断言条件为真
    assert!(true);
    assert!(1 + 1 == 2);

    // assert_eq! — 断言两个值相等
    let got = 2 + 3;
    let want = 5;
    assert_eq!(got, want, "2 + 3 应该等于 5");

    // assert_ne! — 不等于
    assert_ne!(6, 7);
}

// ── 幂运算测试 (对应 rustlings: tests2) ────────────────────

fn power_of_two(n: u32) -> u64 {
    1u64 << n
}

#[test]
fn test_power_of_two() {
    assert_eq!(power_of_two(0), 1);
    assert_eq!(power_of_two(1), 2);
    assert_eq!(power_of_two(2), 4);
    assert_eq!(power_of_two(3), 8);
    assert_eq!(power_of_two(10), 1024);
}

// ── 测试 panic (对应 rustlings: tests3) ─────────────────────

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        if width <= 0.0 || height <= 0.0 {
            panic!("宽和高必须为正数, 收到: {} x {}", width, height);
        }
        Self { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[test]
fn test_rectangle_valid() {
    let rect = Rectangle::new(3.0, 4.0);
    assert_eq!(rect.area(), 12.0);
}

#[test]
#[should_panic(expected = "宽和高必须为正数")]
fn test_rectangle_negative_width() {
    let _rect = Rectangle::new(-1.0, 4.0);
}

#[test]
#[should_panic(expected = "宽和高必须为正数")]
fn test_rectangle_negative_height() {
    let _rect = Rectangle::new(3.0, -2.0);
}

// ── Result 测试 ──────────────────────────────────────────────

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

#[test]
fn test_divide_ok() -> Result<(), String> {
    let result = divide(10.0, 2.0)?; // 测试函数也可以用 ?
    assert_eq!(result, 5.0);
    Ok(())
}

#[test]
fn test_divide_err() {
    assert!(divide(10.0, 0.0).is_err());
    assert_eq!(
        divide(10.0, 0.0).unwrap_err(),
        "除数不能为零"
    );
}

// ── 迭代器测试 (对应 rustlings: 18_iterators) ───────────────

#[test]
fn test_iterator_sum() {
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    assert_eq!(sum, 15);
}

#[test]
fn test_filter_map_collect() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let even_doubled: Vec<i32> = v
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|n| n * 10)
        .collect();
    assert_eq!(even_doubled, vec![20, 40, 60]);
}

// ── HashMap 测试 (对应 rustlings: 11_hashmaps) ──────────────

#[test]
fn test_hashmap_entry() {
    let mut scores = HashMap::new();
    scores.entry("Blue").or_insert(10);
    scores.entry("Red").or_insert(20);
    scores.entry("Blue").or_insert(99); // 不覆盖

    assert_eq!(scores.get("Blue"), Some(&10));
    assert_eq!(scores.get("Red"), Some(&20));
    assert_eq!(scores.len(), 2);
}

// ── String 测试 (对应 rustlings: 09_strings) ────────────────

#[test]
fn test_string_operations() {
    let mut s = String::from("hello");
    s.push_str(" world");
    assert_eq!(s, "hello world");
    assert_eq!(s.trim(), "hello world");
    assert_eq!(s.replace("hello", "goodbye"), "goodbye world");
}

// ── 泛型测试 (对应 rustlings: 14_generics) ──────────────────

fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

#[test]
fn test_largest_i32() {
    assert_eq!(largest(&[1, 5, 3, 9, 2]), Some(9));
}

#[test]
fn test_largest_empty() {
    assert_eq!(largest::<i32>(&[]), None);
}

#[test]
fn test_largest_char() {
    assert_eq!(largest(&['a', 'z', 'b']), Some('z'));
}
