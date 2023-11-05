use scratty::r_hello;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    r_hello(&args[1]);
}
