use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("my.txt").
        expect("failed to read file");
    println!("read result: {}", contents);
}
