#[macro_export]
macro_rules! audit {
    ($($arg:tt)*) => {
        println!("[📄 AUDIT] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("[⚠️  WARN] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("[❌ ERROR] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        println!("[🚨 FATAL] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! fatal_str {
    // If you pass multiple things: fatal_str!("{} - {}", a, b)
    ($fmt:expr, $($arg:tt)*) => {
        &format!("[🚨 FATAL] {}", format!($fmt, $($arg)*))
    };
    // If you pass one thing: fatal_str!("Error!") or fatal_str!(my_var)
    ($arg:expr) => {
        &format!("[🚨 FATAL] {}", $arg)
    };
}