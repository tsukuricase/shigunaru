#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;
    use crate::signal::Signal;
    use crate::computed::create_computed;
    
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
    
    #[test]
    fn test_nested_computed() {
        let base = Signal::new(1);
        
        // 第一级计算信号：值 * 2
        let base_for_doubled = base.clone();
        let doubled = create_computed(move || {
            *base_for_doubled.get().borrow() * 2
        });
        
        // 第二级计算信号：第一级的值 + 10
        let doubled_for_nested = doubled.signal().clone();
        let nested = create_computed(move || {
            *doubled_for_nested.get().borrow() + 10
        });
        
        // 初始值检查
        assert_eq!(doubled.value(), 2);
        assert_eq!(nested.value(), 12);
        
        // 更改基础信号，两级计算信号都应更新
        base.set(5);
        assert_eq!(doubled.value(), 10);
        assert_eq!(nested.value(), 20);
    }
} 