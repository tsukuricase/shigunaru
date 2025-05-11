pub mod computed;
pub mod effect;
pub mod registry;
pub mod signal;

pub use computed::create_computed;
pub use computed::ComputedSignal;
pub use effect::create_effect;
pub use signal::Signal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let signal = Signal::new(10);
        assert_eq!(*signal.get().borrow(), 10);
    }
}
