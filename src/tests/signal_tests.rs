#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    
    #[test]
    fn test_basic_signal() {
        let signal = Signal::new(10);
        assert_eq!(*signal.get().borrow(), 10);
        
        signal.set(20);
        assert_eq!(*signal.get().borrow(), 20);
    }
    
    #[test]
    fn test_signal_subscription() {
        use std::cell::RefCell;
        use std::rc::Rc;
        
        let signal = Signal::new(1);
        let counter = Rc::new(RefCell::new(0));
        
        let counter_clone = counter.clone();
        signal.subscribe(Box::new(move || {
            *counter_clone.borrow_mut() += 1;
        }));
        
        // 设置相同的值不应触发通知
        signal.set(1);
        assert_eq!(*counter.borrow(), 1);
        
        signal.set(2);
        assert_eq!(*counter.borrow(), 2);
        
        // 通过notify方法直接触发通知
        signal.notify();
        assert_eq!(*counter.borrow(), 3);
    }
    
    #[test]
    fn test_signal_cloning() {
        let signal1 = Signal::new(10);
        let signal2 = signal1.clone();
        
        // 更改第一个信号
        signal1.set(20);
        assert_eq!(*signal2.get().borrow(), 20);
        
        // 更改第二个信号
        signal2.set(30);
        assert_eq!(*signal1.get().borrow(), 30);
    }
} 