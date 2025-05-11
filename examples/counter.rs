use shigunaru::{create_computed, create_effect, Signal};

fn main() {
    // 创建一个计数器信号
    let counter = Signal::new(0);

    // 创建一个计算信号，计数器的平方
    let counter_for_squared = counter.clone();
    let squared = create_computed(move || {
        let value = *counter_for_squared.get().borrow();
        value * value
    });

    // 创建另一个计算信号，计数器+10
    let counter_for_plus10 = counter.clone();
    let plus10 = create_computed(move || {
        let value = *counter_for_plus10.get().borrow();
        value + 10
    });

    // 监听计数器变化
    let counter_for_effect = counter.clone();
    create_effect(
        move || {
            println!("Counter changed: {}", *counter_for_effect.get().borrow());
        },
        &counter,
    );

    // 监听计算信号变化
    let squared_for_effect = squared.signal().clone();
    create_effect(
        move || {
            println!("Squared value: {}", *squared_for_effect.get().borrow());
        },
        squared.signal(),
    );

    let plus10_for_effect = plus10.signal().clone();
    create_effect(
        move || {
            println!("Plus 10 value: {}", *plus10_for_effect.get().borrow());
        },
        plus10.signal(),
    );

    // 初始状态
    println!("\n--- Initial state ---");
    counter.notify();

    // 增加计数器的值
    for i in 1..5 {
        println!("\n--- Counter value: {} ---", i);
        counter.set(i);
    }
}
