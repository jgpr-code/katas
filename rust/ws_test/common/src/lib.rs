#![feature(lazy_cell)]
pub use num;
pub use regex;

pub fn print_module_name() {
    println!("module name is {}", module_path!());
}

#[macro_export]
macro_rules! foo {
    () => {
        println!("foo")
    };
}

#[macro_export]
macro_rules! re {
    ($re:literal) => {{
        static RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($re).unwrap());
        &RE
    }};
}
