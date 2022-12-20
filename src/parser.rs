pub mod literal;

pub enum Literal {
    Number(literal::Number),
    None,
}
