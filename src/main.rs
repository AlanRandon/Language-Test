#![warn(clippy::pedantic, clippy::nursery)]

mod parser;

fn main() {
    println!("{:?}", parser::Expression::parse(include_str!("input.txt")));
}
