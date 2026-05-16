#![allow(dead_code)]
//! # Demo 05: 泛型与 Trait
//! 覆盖 rustlings: 14_generics, 15_traits
//!
//! 核心知识点:
//! - 泛型通过单态化为每个具体类型生成代码，零运行时开销
//! - Trait 定义共享行为，需要显式实现
//! - Trait bound 约束泛型参数
//! - Trait object 实现动态分发

pub fn run() {
    println!("── Demo 05: 泛型与 Trait ──\n");
    demo_generics();
    demo_traits();
    demo_trait_objects();
    println!();
}

// ── 14_generics ──────────────────────────────────────────────

fn demo_generics() {
    println!(">>> 泛型 (对应 rustlings: 14_generics)");

    // 泛型结构体 — 单态化，零运行时开销
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
    }

    let w1 = Wrapper { value: 42i32 };
    let w2 = Wrapper { value: "hello" };
    println!("  Wrapper<i32> = {:?}", w1);
    println!("  Wrapper<&str> = {:?}", w2);

    // 泛型函数
    fn first<T>(slice: &[T]) -> Option<&T> {
        slice.first()
    }
    println!("  first(&[1,2,3]) = {:?}", first(&[1, 2, 3]));
    println!("  first(&[\"a\",\"b\"]) = {:?}", first(&["a", "b"]));

    // 多个泛型参数
    #[derive(Debug)]
    struct Pair<K, V> {
        key: K,
        value: V,
    }
    let p = Pair {
        key: "name",
        value: "Alice",
    };
    println!("  Pair<K,V> = {:?}", p);

    // impl<T> — 为泛型实现方法
    impl<T: std::fmt::Display> Wrapper<T> {
        fn display(&self) -> String {
            format!("Wrapper({})", self.value)
        }
    }
    println!("  {} (泛型方法)", w1.display());
}

// ── 15_traits ────────────────────────────────────────────────

fn demo_traits() {
    println!("\n>>> Trait (对应 rustlings: 15_traits)");

    // 定义 trait — 类似其他语言的 interface
    trait Describe {
        fn describe(&self) -> String;

        // 默认实现
        fn loud_describe(&self) -> String {
            format!("!!! {} !!!", self.describe())
        }
    }

    // 为类型实现 trait — 显式 impl 声明
    struct Book {
        title: String,
        author: String,
    }

    impl Describe for Book {
        fn describe(&self) -> String {
            format!("《{}》— {}", self.title, self.author)
        }
    }

    let book = Book {
        title: "Rust 入门".into(),
        author: "张三".into(),
    };
    println!("  {}", book.describe());
    println!("  {}", book.loud_describe()); // 使用默认实现

    // 为标准库类型实现 trait (孤儿规则: 只能在 trait 或类型的 crate 中实现)
    impl Describe for i32 {
        fn describe(&self) -> String {
            if *self >= 0 {
                format!("正数 {}", self)
            } else {
                format!("负数 {}", self)
            }
        }
    }
    println!("  42.describe() = {}", 42.describe());
    println!("  (-7).describe() = {}", (-7).describe());

    // impl Trait 作为参数 — 语法糖，等价于泛型约束
    fn print_description(item: &impl Describe) {
        println!("  (impl Trait 参数): {}", item.describe());
    }
    print_description(&book);
    print_description(&100);

    // 多 trait bound — 用 + 组合多个约束
    use std::fmt::Display;
    fn print_and_describe<T: Display + Describe>(item: &T) {
        println!("  Display: {}, Describe: {}", item, item.describe());
    }
    // Book 没有实现 Display, 这里不演示

    // where clause — 复杂约束的可读写法
    fn complex<T>(item: &T) -> String
    where
        T: Describe + Display,
    {
        format!("{} — {}", item, item.describe())
    }
    println!("  where 约束: {}", complex(&42));
}

// ── Trait Object ─────────────────────────────────────────────

fn demo_trait_objects() {
    println!("\n>>> Trait Object (动态分发)");

    trait Animal {
        fn sound(&self) -> &'static str;
    }

    struct Dog;
    impl Animal for Dog {
        fn sound(&self) -> &'static str {
            "汪汪"
        }
    }

    struct Cat;
    impl Animal for Cat {
        fn sound(&self) -> &'static str {
            "喵喵"
        }
    }

    // 静态分发 — 编译期单态化 (零运行时开销)
    fn static_sound(animal: &impl Animal) {
        println!("  静态分发: {}", animal.sound());
    }
    static_sound(&Dog);
    static_sound(&Cat);

    // 动态分发 — 运行时通过 vtable 调用
    fn dynamic_sound(animal: &dyn Animal) {
        println!("  动态分发: {}", animal.sound());
    }
    dynamic_sound(&Dog);
    dynamic_sound(&Cat);

    // 异构集合 — 不同类型的 vec
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in &animals {
        print!("{} ", animal.sound());
    }
    println!("(异构集合)");
    println!("  impl Trait = 静态分发 (每种类型生成一份代码, 更快)");
    println!("  dyn Trait = 动态分发 (通过 vtable 调用)");
}
