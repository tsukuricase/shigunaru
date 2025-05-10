# Shigunaru

A lightweight reactive signals library implemented in Rust. Shigunaru (シグナル) means "signal" in Japanese, reflecting the core concept of this library.

## Features

- Simple reactive signal implementation
- Effect system that automatically tracks dependencies
- Subscription-based reactivity

## Example

```rust
use shigunaru::Signal;

fn main() {
    // Create a signal with initial value
    let counter = Signal::new(1);
    
    // Create reactive effects that track the counter
    let counter1 = counter.clone();
    create_effect(
        move || {
            println!("Effect 1: counter = {}", *counter1.get().borrow());
        },
        &counter,
    );
    
    // Effect updates automatically when signal changes
    println!("--- Initial state ---");
    counter.notify();
    
    println!("--- After counter.set(42) ---");
    counter.set(42);
}
```

## Implementation Details

Shigunaru uses Rust's `Rc` and `RefCell` to provide interior mutability while maintaining memory safety. The signal system allows for efficient updates with automatic subscription tracking.

## License

MIT