# Shigunaru

A lightweight reactive signals library implemented in Rust. 

## Features

- Simple reactive signal implementation
- Computed signals with automatic dependency tracking
- Lazy evaluation and caching for computed values
- Subscription-based reactivity system
- Clean and modular API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shigunaru = "0.1.0"
```

## Basic Usage

```rust
use shigunaru::{Signal, create_computed, create_effect};

// Create a signal with initial value
let counter = Signal::new(0);

// Use it directly
counter.set(5);
let value = *counter.get().borrow();

// Subscribe to changes
let counter_effect = counter.clone();
create_effect(
    move || {
        println!("Counter changed: {}", *counter_effect.get().borrow());
    },
    &counter,
);

// Change the signal value - will trigger the effect
counter.set(10);
```

## Computed Signals

Computed signals derive their values from other signals and automatically update when dependencies change:

```rust
use shigunaru::{Signal, create_computed};

// Base signal
let count = Signal::new(1);

// Create a computed signal
let count_for_computed = count.clone();
let doubled = create_computed(move || {
    *count_for_computed.get().borrow() * 2
});

// Initial computed value
assert_eq!(doubled.value(), 2);

// Update the source signal
count.set(5);

// Computed value automatically updates
assert_eq!(doubled.value(), 10);
```

## Examples

See the `examples` directory for more usage examples:

- `counter.rs` - Basic counter with computed values

Run an example with:

```bash
cargo run --example counter
```

## Project Structure

```
shigunaru/
├── src/
│   ├── lib.rs         # Main API exports
│   ├── signal.rs      # Signal implementation
│   ├── computed.rs    # Computed signals
│   ├── effect.rs      # Effect system
│   ├── registry.rs    # Dependency tracking
│   └── tests/         # Unit tests
├── examples/          # Usage examples
└── tests/             # Integration tests
```

## License

MIT