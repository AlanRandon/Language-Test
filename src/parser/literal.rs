pub use character::Character;
use nom::{branch::alt, error::context, IResult, Parser};
pub use number::Number;

pub mod character;
pub mod number;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(Number),
    Character(Character),
}

impl Literal {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            context("number", Number::parse.map(Self::Number)),
            context("character", Character::parse.map(Self::Character)),
        ))(input)
    }
}
