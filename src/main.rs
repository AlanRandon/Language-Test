#![warn(clippy::pedantic, clippy::nursery)]

use parser::prelude::*;

use crate::interpreter::{Evaluates, Scope};

mod interpreter;
mod parser;

fn main() {
    println!(
        "{:?}",
        Expression::parse(include_str!("input.txt").into())
            .unwrap()
            .1
            .evaulate(Scope::default())
    );
}
