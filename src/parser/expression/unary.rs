use super::{super::Span, Expression};
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
pub struct Unary {
    pub operator: Operator,
    pub expression: Box<Expression>,
}

impl Unary {
    /// Parses a 'term' - an expression with N prefix and postfix operators
    pub fn parse(input: Span) -> IResult<Span, Expression> {
        let (input, term) = Term::parse(input)?;
        Ok((input, term.reduce()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Negate,
    Not,
    Access(Box<Expression>),
    Call(Vec<Expression>),
}

impl Operator {
    pub fn parse_prefix(input: Span) -> IResult<Span, Self> {
        alt((value(Self::Negate, tag("-")), value(Self::Not, tag("!"))))(input)
    }

    pub fn parse_postfix(input: Span) -> IResult<Span, Self> {
        complete(alt((
            delimited(tag("["), Expression::parse_atom, tag("]"))
                .map(|expression| Self::Access(Box::new(expression))),
            delimited(tag("("), many0(Expression::parse_atom), tag(")")).map(Self::Call),
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
struct Term {
    prefix_operators: Vec<Operator>,
    expression: Expression,
    postfix_operators: Vec<Operator>,
}

impl Term {
    fn parse(input: Span) -> IResult<Span, Self> {
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

    fn reduce(self) -> Expression {
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
