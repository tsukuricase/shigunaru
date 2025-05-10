use std::rc::Rc;
use std::cell::RefCell;

use crate::registry::{get_next_signal_id, register_dependency, mark_dependents_dirty};

/// 响应式信号
///
/// 信号是响应式系统的基本组成部分，它包含一个值，并在值变化时通知订阅者。
pub struct Signal<T> {
    /// 信号的内部值
    value: Rc<RefCell<T>>,
    /// 订阅此信号的回调函数列表
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
    /// 此信号的唯一标识符
    id: usize,
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

impl<T> Signal<T> {
    /// 创建新的信号
    ///
    /// # 参数
    ///
    /// * `value` - 信号的初始值
    ///
    /// # 示例
    ///
    /// ```
    /// use shigunaru::Signal;
    ///
    /// let counter = Signal::new(0);
    /// ```
    pub fn new(value: T) -> Self {
        Signal {
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(vec![])),
            id: get_next_signal_id(),
        }
    }

    /// 获取信号值的引用
    ///
    /// 此方法会自动注册依赖关系，如果在计算信号的上下文中调用。
    pub fn get(&self) -> Rc<RefCell<T>> {
        // 注册依赖，如果在计算环境中
        register_dependency(self.id);
        
        self.value.clone()
    }

    /// 设置信号的新值并通知所有订阅者
    ///
    /// # 参数
    ///
    /// * `new_value` - 新的信号值
    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        
        // 通知所有依赖此信号的计算信号
        mark_dependents_dirty(self.id);
        
        self.notify();
    }

    /// 添加订阅者回调函数
    ///
    /// # 参数
    ///
    /// * `callback` - 当信号值改变时调用的回调函数
    pub fn subscribe(&self, callback: Box<dyn Fn()>) {
        self.subscribers.borrow_mut().push(callback);
    }

    /// 通知所有订阅者
    pub fn notify(&self) {
        for sub in self.subscribers.borrow().iter() {
            sub();
        }
    }
    
    /// 获取信号的唯一标识符
    pub fn id(&self) -> usize {
        self.id
    }

    /// 设置信号值但不通知订阅者
    /// 
    /// 这个方法用于内部实现，让其他模块可以更新值而不触发通知
    pub(crate) fn set_silent(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
    }
} 