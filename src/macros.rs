#[macro_export]
#[cfg(feature = "wasm-logging")]
macro_rules! log_wasm {
    ($($arg:expr),+) => {
       gloo_console::log!($($arg)*);
    }
}
#[macro_export]
#[cfg(not(feature = "wasm-logging"))]
macro_rules! log_wasm {
    ($($arg:expr),+) => {
       println!("{}", $($arg)*);
    }
}