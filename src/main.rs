use std::rc::Rc;
use std::cell::RefCell;

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            value: self.value.clone(),
            subscribers: self.subscribers.clone(),
        }
    }
}

struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>
}

impl<T> Signal<T> {
    fn new(value: T) -> Self {
        Signal {
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(vec![])),
        }
    }

    fn get(&self) -> Rc<RefCell<T>> {
        self.value.clone()
    }

    fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        self.notify();
    }

    fn subscribe(&self, callback: Box<dyn Fn()>) {
        self.subscribers.borrow_mut().push(callback);
    }

    fn notify(&self) {
        for sub in self.subscribers.borrow().iter() {
            sub();
        }
    }
}

fn create_effect<F: Fn() + 'static, T: 'static>(f: F, signal: &Signal<T>) {
    signal.subscribe(Box::new(f));
}

fn main() {
    let counter = Signal::new(1);

    let counter1 = counter.clone();
    create_effect(
        move || {
            println!("Effect 1: counter = {}", *counter1.get().borrow());
        },
        &counter,
    );

    let counter2 = counter.clone();
    create_effect(
        move || {
            println!("Effect 2: counter = {}", *counter2.get().borrow());
        },
        &counter,
    );

    println!("--- Initial state ---");
    counter.notify();

    println!("--- After counter.set(42) ---");
    counter.set(42);

    println!("--- After counter.set(100) ---");
    counter.set(100);
}
