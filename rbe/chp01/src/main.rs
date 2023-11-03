//! Adding documentation for the items that contains this comment.
//! This type of comment is usually added in crate root main.rs and
//! lib.rs or module.rs

use chp01::print::print_plg;
use chp01::say_hello2;

// Single line comment in Rust.

/// Comment for generating library docs for the following main function
/// main is the entry point for the program.
pub fn main() {
    /* Block comment, usually for disabling chunks if code temporarily.

    let a = 2;

    /* And block comment can be nested.
     *
     * let b = "str";
     * let c = false;
     */

    */

    println!("Hello, world!");
    println!("{}", say_hello2("looper"));

    print_plg();
}
