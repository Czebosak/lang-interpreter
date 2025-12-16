use std::mem::size_of;

mod parser;
mod types;

fn main() {
    println!("{}", size_of::<Box<i64>>());
    println!("{}", size_of::<usize>());
    println!("{}", size_of::<types::Value>());
    println!("{}", size_of::<String>());
}
