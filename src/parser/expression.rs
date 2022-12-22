use super::{literal::Literal, optional_whitespace};
use nom::{branch::alt, error::context, sequence::delimited, IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
}

impl Expression {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(
            optional_whitespace,
            alt((context("literal", Literal::parse.map(Self::Literal)),)),
            optional_whitespace,
        )(input)
    }
}
