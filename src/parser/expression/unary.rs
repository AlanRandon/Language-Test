use super::{
    super::{optional_whitespace, Span},
    Expression,
};
use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{complete, value},
    multi::many0,
    sequence::{delimited, tuple},
    IResult, Parser,
};

/// A unary prefix operation, for example arithmetic or binary negation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unary<'a> {
    pub operator: Operator<'a>,
    pub expression: Box<Expression<'a>>,
}

impl<'a> Unary<'a> {
    /// Parses a 'term' - an expression with N prefix and postfix operators
    pub fn parse(input: Span) -> IResult<Span, Expression> {
        let (input, term) = Term::parse(input)?;
        Ok((input, term.reduce()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator<'a> {
    Negate,
    Not,
    Access(Box<Expression<'a>>),
    Call(Vec<Expression<'a>>),
}

impl<'a> Operator<'a> {
    pub fn parse_prefix(input: Span) -> IResult<Span, Self> {
        let (input, result) =
            alt((value(Self::Negate, tag("-")), value(Self::Not, tag("!"))))(input)?;
        let (input, _) = optional_whitespace(input)?;
        Ok((input, result))
    }

    pub fn parse_postfix(input: Span<'a>) -> IResult<Span, Self> {
        let (input, _) = optional_whitespace(input)?;
        complete(alt((
            delimited(tag("["), Expression::parse, tag("]"))
                .map(|expression| Self::Access(Box::new(expression))),
            delimited(tag("("), many0(Expression::parse), tag(")")).map(Self::Call),
        )))(input)
    }

    pub const fn binding_power(&self) -> u8 {
        match self {
            Self::Negate | Self::Not => 10,
            Self::Access(_) | Self::Call(_) => 20,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Term<'a> {
    prefix_operators: Vec<Operator<'a>>,
    expression: Expression<'a>,
    postfix_operators: Vec<Operator<'a>>,
}

impl<'a> Term<'a> {
    fn parse(input: Span<'a>) -> IResult<Span, Self> {
        let (input, (prefix_operators, expression, postfix_operators)) = tuple((
            many0(Operator::parse_prefix),
            Expression::parse_atom,
            many0(Operator::parse_postfix),
        ))(input)?;

        Ok((
            input,
            Self {
                prefix_operators,
                expression,
                postfix_operators,
            },
        ))
    }

    fn reduce(self) -> Expression<'a> {
        let Self {
            mut prefix_operators,
            mut expression,
            mut postfix_operators,
        } = self;

        postfix_operators.reverse();

        loop {
            match (prefix_operators.pop(), postfix_operators.pop()) {
                (None, None) => break,
                (None, Some(operator)) | (Some(operator), None) => {
                    expression = Expression::Unary(Unary {
                        expression: Box::new(expression),
                        operator,
                    });
                }
                (Some(prefix), Some(postfix)) => {
                    expression = Expression::Unary(Unary {
                        expression: Box::new(expression),
                        operator: if prefix.binding_power() > postfix.binding_power() {
                            postfix_operators.push(postfix);
                            prefix
                        } else {
                            prefix_operators.push(prefix);
                            postfix
                        },
                    });
                }
            }
        }

        expression
    }
}

#[test]
fn term_parses() {
    use super::super::{
        literal::{number, Integer, Str},
        test, Literal,
    };

    assert_eq!(
        test::strip_span(Term::parse("!\"a\"[1]".into())),
        Ok((
            String::new(),
            Term {
                prefix_operators: vec![Operator::Not],
                expression: Expression::Literal(Literal::String(Str("a".to_string()))),
                postfix_operators: vec![Operator::Access(Box::new(Expression::Literal(
                    Literal::Integer(Integer {
                        base: number::Base::Decimal,
                        digits: vec![1],
                        sign: number::Sign::Positive
                    })
                )))]
            }
        ))
    );
}

#[test]
fn term_reduces() {
    use super::super::{
        literal::{number, Integer, Str},
        test, Literal,
    };

    assert_eq!(
        test::strip_span(Term::parse("!\"a\"[1]".into()))
            .map(|(input, result)| (input, Term::reduce(result))),
        Ok((
            String::new(),
            Expression::Unary(Unary {
                operator: Operator::Not,
                expression: Box::new(Expression::Unary(Unary {
                    operator: Operator::Access(Box::new(Expression::Literal(Literal::Integer(
                        Integer {
                            base: number::Base::Decimal,
                            digits: vec![1],
                            sign: number::Sign::Positive
                        }
                    )))),
                    expression: Box::new(Expression::Literal(Literal::String(
                        Str("a".to_string())
                    )))
                }))
            })
        ))
    );
}
