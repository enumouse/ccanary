use std::env;

/// Calls into ccanary_driver for heavy lifting
fn main() {
    let args: Vec<String> = env::args().collect();
    ccanary_driver::main(&args);
}
