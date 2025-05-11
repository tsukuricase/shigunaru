pub mod signal;
pub mod computed;
pub mod effect;
pub mod registry;

pub use signal::Signal;
pub use computed::ComputedSignal;
pub use computed::create_computed;
pub use effect::create_effect;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let signal = Signal::new(10);
        assert_eq!(*signal.get().borrow(), 10);
    }
} 