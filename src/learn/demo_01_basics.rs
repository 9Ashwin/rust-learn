//! # Demo 01: 基础语法
//! 覆盖 rustlings: 00_intro, 01_variables, 02_functions, 03_if, 04_primitive_types, 10_modules
//!
//! 核心知识点:
//! - `let` 声明变量，默认不可变
//! - `mut` 声明可变变量
//! - `fn` 声明函数，参数和返回值必须标注类型
//! - `if` 是表达式，可以赋值
//! - 数组大小是类型的一部分

pub fn run() {
    println!("── Demo 01: 变量、函数、控制流、基本类型 ──\n");
    demo_variables();
    demo_functions();
    demo_control_flow();
    demo_primitives();
    demo_modules();
    println!();
}

// ── 01_variables ────────────────────────────────────────────

fn demo_variables() {
    println!(">>> 变量 (对应 rustlings: 01_variables)");

    // let 绑定 — 默认不可变
    let x = 5;
    println!("  let x = 5;      // 不可变绑定, x = {}", x);

    // mut 声明可变变量
    let mut y = 10;
    y += 1;
    println!("  let mut y = 10;  // mut 声明可变, y = {}", y);

    // 显式类型标注
    let z: i64 = 42;
    println!("  let z: i64 = 42; // 显式类型标注, z = {}", z);

    // shadowing (遮蔽) — 用 let 重新绑定, 可以改变值和类型
    let val = "hello";
    let val = val.len(); // 同一个名字, 新类型
    println!("  shadowing: val = \"hello\" → val.len() = {} (类型已变)", val);

    // const — 必须标注类型
    const PI: f64 = std::f64::consts::PI;
    println!("  const PI: f64 = {:.5};    // const 必须标注类型", PI);
}

// ── 02_functions ────────────────────────────────────────────

fn demo_functions() {
    println!("\n>>> 函数 (对应 rustlings: 02_functions)");

    // 无参无返回值
    fn say_hello() {
        println!("    Hello from fn say_hello()!");
    }
    say_hello();

    // 有参有返回值 — 无分号结尾 = 隐式返回 (表达式)
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("    add(3, 4) = {}", add(3, 4));

    // return 关键字显式返回
    fn is_positive(n: i32) -> bool {
        return n > 0; // 带分号需要 return
    }
    println!("    is_positive(-1) = {}", is_positive(-1));
}

// ── 03_if ───────────────────────────────────────────────────

fn demo_control_flow() {
    println!("\n>>> 控制流 (对应 rustlings: 03_if)");

    let number = 6;

    // if 是表达式, 可以赋值
    let description = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };
    println!("    {} 是 {} (if 作为表达式赋值)", number, description);

    // match — 穷尽匹配所有分支
    let grade = 'B';
    let score = match grade {
        'A' => 4,
        'B' => 3,
        'C' => 2,
        'D' => 1,
        _ => 0, // _ 通配符 — 必须覆盖所有情况
    };
    println!("    等级 {} → 绩点 {} (match 穷尽匹配)", grade, score);

    // loop 无限循环 — break 可以带值
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            break count * 10;
        }
    };
    println!("    loop break 带值 = {}", result);
}

// ── 04_primitive_types ──────────────────────────────────────

fn demo_primitives() {
    println!("\n>>> 基本类型 (对应 rustlings: 04_primitive_types)");

    // bool 布尔类型
    let is_active: bool = true;
    println!("    bool: {}", is_active);

    // char — 4 字节 Unicode 标量值
    let c: char = '🦀';
    println!("    char: {} (4 字节 Unicode)", c);

    // 数组 — 大小是类型的一部分
    let arr: [i32; 3] = [1, 2, 3];
    println!("    array: {:?}, len = {} (大小是类型的一部分)", arr, arr.len());

    // 快速创建 — [值; 数量]
    let zeros = [0u8; 5];
    println!("    [0u8; 5] = {:?}", zeros);

    // 切片 — 引用数组的一部分
    let slice = &arr[1..3];
    println!("    &arr[1..3] = {:?} (切片)", slice);

    // 元组 — 多种类型组合
    let tup: (i32, f64, &str) = (42, std::f64::consts::PI, "hello");
    println!("    tuple: {:?}", tup);
    println!("    tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // 元组解构
    let (a, b, msg) = tup;
    println!("    解构后: a={}, b={}, msg={}", a, b, msg);
}

// ── 10_modules ──────────────────────────────────────────────

mod inner_module {
    // 默认私有 — 不加 pub 外部不可见
    fn private_fn() -> &'static str {
        "我是私有的"
    }

    pub fn public_fn() -> &'static str {
        "我是公开的 (pub)"
    }

    pub fn call_private() -> String {
        format!("通过 pub fn 访问: {}", private_fn())
    }
}

fn demo_modules() {
    println!("\n>>> 模块 (对应 rustlings: 10_modules)");
    println!("    {} ", inner_module::public_fn());
    println!("    {}", inner_module::call_private());
    // inner_module::private_fn(); // ← 编译错误: private function
    println!("    (private_fn 无法直接调用 — 默认私有)");
}
