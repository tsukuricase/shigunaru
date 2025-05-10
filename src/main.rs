use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::HashMap;

// 全局信号注册表
thread_local! {
    static CURRENT_COMPUTED: RefCell<Option<Rc<RefCell<ComputedState>>>> = RefCell::new(None);
    static SIGNAL_REGISTRY: RefCell<HashMap<usize, Vec<Rc<RefCell<ComputedState>>>>> = RefCell::new(HashMap::new());
}

#[derive(Default)]
struct ComputedState {
    dirty: bool,
    dependencies: HashSet<usize>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            value: self.value.clone(),
            subscribers: self.subscribers.clone(),
            id: self.id,
        }
    }
}

struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
    id: usize,
}

static mut NEXT_SIGNAL_ID: usize = 0;

impl<T> Signal<T> {
    fn new(value: T) -> Self {
        let id = unsafe {
            let id = NEXT_SIGNAL_ID;
            NEXT_SIGNAL_ID += 1;
            id
        };
        
        Signal {
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(vec![])),
            id,
        }
    }

    fn get(&self) -> Rc<RefCell<T>> {
        // 注册依赖，如果在计算环境中
        CURRENT_COMPUTED.with(|current| {
            if let Some(computed) = current.borrow().clone() {
                computed.borrow_mut().dependencies.insert(self.id);
            }
        });
        
        self.value.clone()
    }

    fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        
        // 通知所有依赖此信号的计算信号
        SIGNAL_REGISTRY.with(|registry| {
            if let Some(dependents) = registry.borrow_mut().get(&self.id) {
                for dependent in dependents.iter() {
                    dependent.borrow_mut().dirty = true;
                }
            }
        });
        
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

struct ComputedSignal<T, F>
where
    F: Fn() -> T + 'static,
    T: Clone + 'static,
{
    compute_fn: F,
    cached_value: RefCell<Option<T>>,
    signal: Signal<T>,
    state: Rc<RefCell<ComputedState>>,
}

impl<T, F> ComputedSignal<T, F>
where
    F: Fn() -> T + 'static,
    T: Clone + 'static,
{
    fn new(compute_fn: F) -> Self {
        let state = Rc::new(RefCell::new(ComputedState {
            dirty: true,
            dependencies: HashSet::new(),
        }));
        
        let dummy_value = unsafe { std::mem::zeroed() };
        let signal = Signal::new(dummy_value);

        let computed = Self {
            compute_fn,
            cached_value: RefCell::new(None),
            signal,
            state: state.clone(),
        };
        
        computed
    }

    fn value(&self) -> T {
        if self.state.borrow().dirty || self.cached_value.borrow().is_none() {
            CURRENT_COMPUTED.with(|current| {
                let prev = current.borrow().clone();
                *current.borrow_mut() = Some(self.state.clone());
                
                self.state.borrow_mut().dependencies.clear();
                
                let new_value = (self.compute_fn)();
                *self.cached_value.borrow_mut() = Some(new_value.clone());
                
                *self.signal.value.borrow_mut() = new_value;
                
                self.state.borrow_mut().dirty = false;
                
                *current.borrow_mut() = prev;
            });
            
            // 订阅所有依赖
            let state_clone = self.state.clone();
            
            // 注册此计算信号为每个依赖信号的依赖者
            SIGNAL_REGISTRY.with(|registry| {
                for dep_id in self.state.borrow().dependencies.iter() {
                    let mut registry_ref = registry.borrow_mut();
                    let registry_entry = registry_ref.entry(*dep_id).or_insert_with(Vec::new);
                    registry_entry.push(state_clone.clone());
                }
            });
        }
        
        self.cached_value.borrow().clone().unwrap()
    }
    
    fn signal(&self) -> &Signal<T> {
        &self.signal
    }
}

fn create_effect<F: Fn() + 'static, T: 'static>(f: F, signal: &Signal<T>) {
    signal.subscribe(Box::new(f));
}

fn create_computed<T: Clone + 'static, F: Fn() -> T + 'static>(f: F) -> ComputedSignal<T, F> {
    let computed = ComputedSignal::new(f);
    
    // 初始化时执行一次以建立依赖关系
    computed.value();
    
    computed
}

fn main() {
    let counter = Signal::new(1);
    
    // Create a computed signal that depends on counter
    let counter_for_computed = counter.clone();
    let doubled = create_computed(move || {
        let counter_value = *counter_for_computed.get().borrow();
        counter_value * 2
    });
    
    // Create an effect that uses the computed value
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_basic_signal() {
        let signal = Signal::new(10);
        assert_eq!(*signal.get().borrow(), 10);
        
        signal.set(20);
        assert_eq!(*signal.get().borrow(), 20);
    }
    
    #[test]
    fn test_computed_signal() {
        let counter = Signal::new(5);
        
        // 创建计算信号：counter的值乘以2
        let counter_for_computed = counter.clone();
        let doubled = create_computed(move || {
            let counter_value = *counter_for_computed.get().borrow();
            counter_value * 2
        });
        
        // 初始值应该是10
        assert_eq!(doubled.value(), 10);
        
        // 更改counter后，doubled的值应自动更新
        counter.set(7);
        assert_eq!(doubled.value(), 14);
        
        // 再次更改counter
        counter.set(15);
        assert_eq!(doubled.value(), 30);
    }
    
    #[test]
    fn test_computed_with_dependencies() {
        let a = Signal::new(1);
        let b = Signal::new(2);
        
        // 创建依赖两个信号的计算信号
        let a_for_computed = a.clone();
        let b_for_computed = b.clone();
        let sum = create_computed(move || {
            let a_val = *a_for_computed.get().borrow();
            let b_val = *b_for_computed.get().borrow();
            a_val + b_val
        });
        
        // 检查初始值
        assert_eq!(sum.value(), 3);
        
        // 更改a的值
        a.set(5);
        assert_eq!(sum.value(), 7);
        
        // 更改b的值
        b.set(10);
        assert_eq!(sum.value(), 15);
        
        // 同时更改两个值
        a.set(20);
        b.set(30);
        assert_eq!(sum.value(), 50);
    }
    
    #[test]
    fn test_computed_caching() {
        let computation_count = Rc::new(Cell::new(0));
        let signal = Signal::new(1);
        
        let signal_for_computed = signal.clone();
        let computation_count_clone = computation_count.clone();
        let computed = create_computed(move || {
            computation_count_clone.set(computation_count_clone.get() + 1);
            *signal_for_computed.get().borrow() * 2
        });
        
        // 第一次访问应该计算值
        assert_eq!(computed.value(), 2);
        assert_eq!(computation_count.get(), 1);
        
        // 重复访问应该使用缓存值
        assert_eq!(computed.value(), 2);
        assert_eq!(computation_count.get(), 1); // 计数不变表示使用了缓存
        
        // 更改依赖值后应重新计算
        signal.set(10);
        assert_eq!(computed.value(), 20);
        assert_eq!(computation_count.get(), 2);
    }
}
