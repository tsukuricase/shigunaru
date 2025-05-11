use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::registry::{register_dependent, set_current_computed};
use crate::signal::Signal;

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
            let prev = set_current_computed(Some(self.state.clone()));

            self.state.borrow_mut().dependencies.clear();

            let new_value = (self.compute_fn)();
            *self.cached_value.borrow_mut() = Some(new_value.clone());

            self.signal.set_silent(new_value);

            self.state.borrow_mut().dirty = false;

            set_current_computed(prev);

            let state_clone = self.state.clone();

            for dep_id in self.state.borrow().dependencies.iter() {
                register_dependent(*dep_id, state_clone.clone());
            }
        }

        self.cached_value.borrow().clone().unwrap()
    }

    pub fn signal(&self) -> &Signal<T> {
        &self.signal
    }
}

pub fn create_computed<T: Clone + 'static, F: Fn() -> T + 'static>(f: F) -> ComputedSignal<T, F> {
    let computed = ComputedSignal::new(f);

    computed.value();

    computed
}
