use crate::signal::Signal;

/// 创建响应效果
///
/// 效果是响应系统的一部分，它在信号值变化时自动执行指定的函数。
///
/// # 参数
///
/// * `f` - 当依赖的信号变化时执行的函数
/// * `signal` - 要监听变化的信号
///
/// # 示例
///
/// ```
/// use shigunaru::{Signal, create_effect};
///
/// let counter = Signal::new(0);
/// let counter_effect = counter.clone();
///
/// create_effect(move || {
///     println!("Counter changed: {}", *counter_effect.get().borrow());
/// }, &counter);
///
/// // 更改信号值会触发效果
/// counter.set(42);
/// ```
pub fn create_effect<F: Fn() + 'static, T: 'static>(f: F, signal: &Signal<T>) {
    signal.subscribe(Box::new(f));
} 