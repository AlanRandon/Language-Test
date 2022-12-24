use super::{literal::Literal, optional_whitespace};
use binary::Binary;
use nom::{branch::alt, error::context, sequence::delimited, IResult, Parser};

pub mod binary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
    Binary(Binary),
}

impl Expression {
    /// Parse an arbitary expression
    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(optional_whitespace, Binary::parse, optional_whitespace)(input)
    }

    /// Parse all non-binary terms (e.g. literals and identifiers)
    pub fn parse_term(input: &str) -> IResult<&str, Self> {
        delimited(
            optional_whitespace,
            alt((context("literal", Literal::parse.map(Self::Literal)),)),
            optional_whitespace,
        )(input)
    }
}

#[test]
fn expression_parses() {
    use super::literal::Boolean;

    assert_eq!(
        Expression::parse(" \n true "),
        Ok(("", Expression::Literal(Literal::Boolean(Boolean(true)))))
    );
}
