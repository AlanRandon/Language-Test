#![warn(clippy::pedantic, clippy::nursery)]

use parser::literal;

mod parser;

fn main() {
    println!("{:?}", literal::Literal::parse(include_str!("input.txt")));
}
