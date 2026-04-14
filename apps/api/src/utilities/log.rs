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
    ($($arg:tt)*) => {
        format!("[🚨 FATAL] {}", format_args!($($arg)*))
    };
}
