use shigunaru::{Signal, create_computed, create_effect};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 如果有命令行参数，使用第一个参数作为计数器初始值
    let initial_value = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(1)
    } else {
        1
    };
    
    print_header();
    run_demo(initial_value);
}

fn print_header() {
    println!("Shigunaru - Reactive Signals Library Demo");
    println!("=========================================");
    println!("A lightweight reactive signals library in Rust");
    println!("");
}

fn run_demo(initial_value: i32) {
    println!("Starting demo with initial value: {}", initial_value);
    println!("");
    
    // 创建基础信号
    let counter = Signal::new(initial_value);
    
    // 创建计算信号
    let counter_for_computed = counter.clone();
    let doubled = create_computed(move || {
        let counter_value = *counter_for_computed.get().borrow();
        counter_value * 2
    });
    
    // 设置效果
    println!("Setting up effects...");
    
    let doubled_signal = doubled.signal().clone();
    create_effect(
        move || {
            println!("  [Effect] Computed value (doubled): {}", *doubled_signal.get().borrow());
        },
        doubled.signal(),
    );
    
    let counter_effect = counter.clone();
    create_effect(
        move || {
            println!("  [Effect] Counter value: {}", *counter_effect.get().borrow());
        },
        &counter,
    );

    // 初始状态
    println!("\n--- Initial state ---");
    counter.notify();
    
    // 修改信号值
    for value in [5, 10, 42, 100].iter() {
        println!("\n--- Setting counter to {} ---", value);
        counter.set(*value);
    }
    
    println!("\nDemo complete. Thanks for trying Shigunaru!");
    println!("For more examples, run: cargo run --example counter");
}
