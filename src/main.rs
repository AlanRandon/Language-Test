use parser::literal;

mod parser;

fn main() {
    println!("{:?}", literal::Number::parse("900"));
}
