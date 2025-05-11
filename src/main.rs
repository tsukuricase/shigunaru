use shigunaru::{Signal, create_computed, create_effect};

fn main() {
    let counter = Signal::new(1);
    
    let counter_for_computed = counter.clone();
    let doubled = create_computed(move || {
        let counter_value = *counter_for_computed.get().borrow();
        counter_value * 2
    });
    
    let doubled_signal = doubled.signal().clone();
    create_effect(
        move || {
            println!("Computed value (doubled): {}", *doubled_signal.get().borrow());
        },
        doubled.signal(),
    );
    
    let counter1 = counter.clone();
    create_effect(
        move || {
            println!("Effect 1: counter = {}", *counter1.get().borrow());
        },
        &counter,
    );

    println!("--- Initial state ---");
    counter.notify();
    
    println!("Doubled value: {}", doubled.value());

    println!("--- After counter.set(42) ---");
    counter.set(42);
    println!("Doubled value: {}", doubled.value());

    println!("--- After counter.set(100) ---");
    counter.set(100);
    println!("Doubled value: {}", doubled.value());
}
