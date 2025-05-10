// 导出公共模块
pub mod signal;
pub mod computed;
pub mod effect;
pub mod registry;

// 重新导出主要类型，构建公共API
pub use signal::Signal;
pub use computed::ComputedSignal;
pub use computed::create_computed;
pub use effect::create_effect;

/// Shigunaru 是一个轻量级的 Rust 响应式信号库。
/// 它允许你创建响应式状态并自动跟踪依赖关系。
///
/// # 示例
///
/// ```
/// use shigunaru::{Signal, create_computed, create_effect};
///
/// // 创建信号
/// let counter = Signal::new(1);
///
/// // 创建计算信号
/// let counter_for_computed = counter.clone();
/// let doubled = create_computed(move || {
///     *counter_for_computed.get().borrow() * 2
/// });
///
/// // 使用响应效果
/// let counter_effect = counter.clone();
/// create_effect(move || {
///     println!("Counter: {}", *counter_effect.get().borrow());
/// }, &counter);
///
/// // 更新信号值，自动触发效果
/// counter.set(42);
/// ```
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let signal = Signal::new(10);
        assert_eq!(*signal.get().borrow(), 10);
    }
} 