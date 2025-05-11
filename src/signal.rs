use std::rc::Rc;
use std::cell::RefCell;

use crate::registry::{get_next_signal_id, register_dependency, mark_dependents_dirty};

pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
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
    pub fn new(value: T) -> Self {
        Signal {
            value: Rc::new(RefCell::new(value)),
            subscribers: Rc::new(RefCell::new(vec![])),
            id: get_next_signal_id(),
        }
    }

    pub fn get(&self) -> Rc<RefCell<T>> {
        register_dependency(self.id);
        self.value.clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        
        mark_dependents_dirty(self.id);
        
        self.notify();
    }

    pub fn subscribe(&self, callback: Box<dyn Fn()>) {
        self.subscribers.borrow_mut().push(callback);
    }

    pub fn notify(&self) {
        for sub in self.subscribers.borrow().iter() {
            sub();
        }
    }
    
    pub fn id(&self) -> usize {
        self.id
    }

    pub(crate) fn set_silent(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
    }
} 