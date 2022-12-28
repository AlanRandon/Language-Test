use super::{identifier::Identifier, let_in::LetIn, literal::Literal, optional_whitespace, Span};
use binary::Binary;
use nom::{
    branch::alt, bytes::streaming::tag, error::context, sequence::delimited, IResult, Parser,
};
use unary::Unary;

pub mod binary;
pub mod pratt;
pub mod unary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Literal(Literal),
    Binary(Binary<'a>),
    Unary(Unary<'a>),
    Identifier(Identifier<'a>),
    LetIn(LetIn<'a>),
}

impl<'a> Expression<'a> {
    /// Parse an arbitary expression
    pub fn parse(input: Span<'a>) -> IResult<Span, Self> {
        delimited(
            optional_whitespace,
            context("binary operations", Binary::parse),
            optional_whitespace,
        )(input)
    }

    /// Parse all non-binary terms (e.g. literals and identifiers)
    pub fn parse_term(input: Span<'a>) -> IResult<Span, Self> {
        delimited(
            optional_whitespace,
            context("unary operations", Unary::parse),
            optional_whitespace,
        )(input)
    }

    /// Parse all 'atoms' (e.g. literals and identifiers)
    pub fn parse_atom(input: Span<'a>) -> IResult<Span, Self> {
        delimited(
            optional_whitespace,
            alt((
                context("let-in", LetIn::parse.map(Self::LetIn)),
                context("literal", Literal::parse.map(Self::Literal)),
                context("group", delimited(tag("("), Self::parse, tag(")"))),
                context("identifier", Identifier::parse.map(Self::Identifier)),
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
