//! Documentation for the current crate

#![allow(dead_code)]

pub mod print;

/// Say hello to name, library comment with markdown section: examples.
/// cargo test also run tests in the following code block
///
/// # Examples
///
/// ```
/// let greeting = chp01::say_hello2("looper");
///
/// assert_eq!("Hello, looper!", greeting);
/// ```
pub fn say_hello2(name: &str) -> String {
    format!("Hello, {}!", name)
}
