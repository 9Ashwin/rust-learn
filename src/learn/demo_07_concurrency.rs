//! # Demo 07: 并发
//! 覆盖 rustlings: 20_threads
//!
//! 核心知识点:
//! - thread::spawn 创建线程
//! - Mutex<T> 包裹数据，编译器强制加锁后才能访问
//! - Arc 用于多线程共享所有权
//! - mpsc::channel 实现线程间消息传递

pub fn run() {
    println!("── Demo 07: 并发 ──\n");
    demo_threads();
    demo_mutex();
    demo_channels();
    println!();
}

// ── 线程 ─────────────────────────────────────────────────────

fn demo_threads() {
    println!(">>> 线程 (对应 rustlings: 20_threads)");

    // thread::spawn — 创建新线程
    let handle = std::thread::spawn(|| {
        // 闭包没有捕获变量, 所以是 'static
        42 // 返回值
    });

    // join 等待线程完成
    let result = handle.join().unwrap();
    println!("  thread::spawn + join = {}", result);

    // 捕获外部变量 — 需要 move 闭包
    let msg = String::from("来自主线程");
    let handle2 = std::thread::spawn(move || {
        // move: 所有权转移到线程
        println!("  线程内: {}", msg);
    });
    handle2.join().unwrap();
    // println!("{}", msg); // ← 编译错误: msg 已转移
    println!("  move 闭包: 所有权转移到线程");
}

// ── Mutex ────────────────────────────────────────────────────

fn demo_mutex() {
    println!("\n>>> Mutex (对应 rustlings: 20_threads)");

    // Mutex<T> — 数据被包裹, 编译器强制加锁后访问
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Arc = 多线程的 Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter); // 引用计数 +1
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 加锁
            *num += 1;
            // 自动释放锁 (MutexGuard 实现 Drop)
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("  counter = {} (5 线程各 +1)", *counter.lock().unwrap());

    println!("  Mutex 包裹数据 + MutexGuard 自动释放 = 安全性保障");
}

// ── Channel ──────────────────────────────────────────────────

fn demo_channels() {
    println!("\n>>> Channel (对应 rustlings: 20_threads)");

    use std::sync::mpsc; // multiple producer, single consumer
    use std::thread;

    // 创建 channel — mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // 多个 sender — Sender 可 clone
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // 生产者 1
    thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("P1-{}", i)).unwrap();
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    // 生产者 2
    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("P2-{}", i)).unwrap();
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    // 主 sender
    drop(tx); // 必须手动 drop 否则 rx 永远等待

    // 消费者
    for msg in rx {
        println!("  收到: {}", msg);
    }
    println!("  (tx 全部 drop 后 rx 自动停止)");
}
