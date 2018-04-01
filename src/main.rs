extern crate http_test;

use std::process;

fn main() {
    if let Err(e) = http_test::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}