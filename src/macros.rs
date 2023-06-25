#[macro_export]
#[cfg(feature = "wasm-logging")]
macro_rules! log {
    ($($arg:expr),+) => {
       gloo_console::log!($($arg)*);
    }
}
#[macro_export]
#[cfg(not(feature = "wasm-logging"))]
macro_rules! log {
    ($($arg:expr),+) => {
       println!("{}", $($arg)*);
    }
}