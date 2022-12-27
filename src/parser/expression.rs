use self::unary::Unary;

use super::{literal::Literal, optional_whitespace, Span};
use binary::Binary;
use nom::{
    branch::alt, bytes::streaming::tag, error::context, sequence::delimited, IResult, Parser,
};

pub mod binary;
pub mod pratt;
pub mod unary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
    Binary(Binary),
    Unary(Unary),
}

impl Expression {
    /// Parse an arbitary expression
    pub fn parse(input: Span) -> IResult<Span, Self> {
        delimited(
            optional_whitespace,
            context("binary operations", Binary::parse),
            optional_whitespace,
        )(input)
    }

    /// Parse all non-binary terms (e.g. literals and identifiers)
    pub fn parse_term(input: Span) -> IResult<Span, Self> {
        delimited(
            optional_whitespace,
            context("unary operations", Unary::parse),
            optional_whitespace,
        )(input)
    }

    /// Parse all 'atoms' (e.g. literals and identifiers)
    pub fn parse_atom(input: Span) -> IResult<Span, Self> {
        delimited(
            optional_whitespace,
            alt((
                context("literal", Literal::parse.map(Self::Literal)),
                context("group", delimited(tag("("), Self::parse, tag(")"))),
            )),
            optional_whitespace,
        )(input)
    }
}

#[test]
fn expression_parses() {
    use super::{literal::Boolean, test};

    assert_eq!(
        test::strip_span(Expression::parse(" \n true ".into())),
        Ok((
            String::new(),
            Expression::Literal(Literal::Boolean(Boolean(true)))
        ))
    );
}
