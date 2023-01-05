use super::prelude::*;
use binary::Binary;
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
    Function(Function<'a>),
}

impl<'a> Expression<'a> {
    /// Parse an arbitary expression
    pub fn parse(input: Span<'a>) -> IResult<Span, Self> {
        delimited(
            whitespace::optional,
            context("binary operations", Binary::parse),
            whitespace::optional,
        )(input)
    }

    /// Parse all non-binary terms (e.g. literals and identifiers)
    pub fn parse_term(input: Span<'a>) -> IResult<Span, Self> {
        delimited(
            whitespace::optional,
            context("unary operations", Unary::parse),
            whitespace::optional,
        )(input)
    }

    /// Parse all 'atoms' (e.g. literals and identifiers)
    pub fn parse_atom(input: Span<'a>) -> IResult<Span, Self> {
        delimited(
            whitespace::optional,
            alt((
                context("let-in", LetIn::parse.map(Self::LetIn)),
                context("function", Function::parse.map(Self::Function)),
                context("literal", Literal::parse.map(Self::Literal)),
                context("group", delimited(tag("("), Self::parse, tag(")"))),
                context("identifier", Identifier::parse.map(Self::Identifier)),
            )),
            whitespace::optional,
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
