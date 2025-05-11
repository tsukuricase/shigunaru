use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use crate::signal::Signal;
use crate::registry::{set_current_computed, register_dependent};

pub struct ComputedState {
    pub dirty: bool,
    pub dependencies: HashSet<usize>,
}

impl Default for ComputedState {
    fn default() -> Self {
        ComputedState {
            dirty: true,
            dependencies: HashSet::new(),
        }
    }
}

pub struct ComputedSignal<T, F>
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

        ComputedSignal {
            compute_fn,
            cached_value: RefCell::new(None),
            signal,
            state: state.clone(),
        }
    }

    pub fn value(&self) -> T {
        if self.state.borrow().dirty || self.cached_value.borrow().is_none() {
            // 设置当前计算上下文
            let prev = set_current_computed(Some(self.state.clone()));
            
            // 清除之前的依赖
            self.state.borrow_mut().dependencies.clear();
            
            // 计算新值
            let new_value = (self.compute_fn)();
            *self.cached_value.borrow_mut() = Some(new_value.clone());
            
            // 更新信号值
            self.signal.set_silent(new_value);
            
            self.state.borrow_mut().dirty = false;
            
            // 恢复之前的上下文
            set_current_computed(prev);
            
            // 订阅所有依赖
            let state_clone = self.state.clone();
            
            // 将此计算信号注册为每个依赖信号的依赖者
            for dep_id in self.state.borrow().dependencies.iter() {
                register_dependent(*dep_id, state_clone.clone());
            }
        }
        
        self.cached_value.borrow().clone().unwrap()
    }
    
    /// 获取底层信号的引用
    ///
    /// 可用于订阅此计算信号的变化
    pub fn signal(&self) -> &Signal<T> {
        &self.signal
    }
}

/// 创建计算信号
///
/// # 参数
///
/// * `f` - 计算函数，返回计算结果
///
/// # 示例
///
/// ```
/// use shigunaru::{Signal, create_computed};
///
/// let counter = Signal::new(1);
/// let counter_for_computed = counter.clone();
/// 
/// let doubled = create_computed(move || {
///     *counter_for_computed.get().borrow() * 2
/// });
///
/// assert_eq!(doubled.value(), 2);
/// counter.set(5);
/// assert_eq!(doubled.value(), 10);
/// ```
pub fn create_computed<T: Clone + 'static, F: Fn() -> T + 'static>(f: F) -> ComputedSignal<T, F> {
    let computed = ComputedSignal::new(f);
    
    // 初始化时执行一次以建立依赖关系
    computed.value();
    
    computed
} 