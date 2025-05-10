use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::computed::ComputedState;

// 全局信号注册表，用于跟踪信号和它们的依赖关系
thread_local! {
    pub static CURRENT_COMPUTED: RefCell<Option<Rc<RefCell<ComputedState>>>> = RefCell::new(None);
    pub static SIGNAL_REGISTRY: RefCell<HashMap<usize, Vec<Rc<RefCell<ComputedState>>>>> = RefCell::new(HashMap::new());
}

/// 为信号生成唯一ID
static mut NEXT_SIGNAL_ID: usize = 0;

/// 获取新的唯一信号ID
pub fn get_next_signal_id() -> usize {
    unsafe {
        let id = NEXT_SIGNAL_ID;
        NEXT_SIGNAL_ID += 1;
        id
    }
}

/// 将计算信号状态注册为信号的依赖者
pub fn register_dependent(signal_id: usize, state: Rc<RefCell<ComputedState>>) {
    SIGNAL_REGISTRY.with(|registry| {
        let mut registry_ref = registry.borrow_mut();
        let registry_entry = registry_ref.entry(signal_id).or_insert_with(Vec::new);
        registry_entry.push(state);
    });
}

/// 标记所有依赖指定信号的计算信号为脏状态
pub fn mark_dependents_dirty(signal_id: usize) {
    SIGNAL_REGISTRY.with(|registry| {
        if let Some(dependents) = registry.borrow().get(&signal_id) {
            for dependent in dependents.iter() {
                dependent.borrow_mut().dirty = true;
            }
        }
    });
}

/// 设置当前正在计算的计算信号状态
pub fn set_current_computed(state: Option<Rc<RefCell<ComputedState>>>) -> Option<Rc<RefCell<ComputedState>>> {
    CURRENT_COMPUTED.with(|current| {
        let prev = current.borrow().clone();
        *current.borrow_mut() = state;
        prev
    })
}

/// 向当前活动的计算信号注册依赖
pub fn register_dependency(signal_id: usize) {
    CURRENT_COMPUTED.with(|current| {
        if let Some(computed) = current.borrow().clone() {
            computed.borrow_mut().dependencies.insert(signal_id);
        }
    });
} 