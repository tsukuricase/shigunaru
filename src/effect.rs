use crate::signal::Signal;

pub fn create_effect<F: Fn() + 'static, T: 'static>(f: F, signal: &Signal<T>) {
    signal.subscribe(Box::new(f));
}
