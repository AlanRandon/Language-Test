use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{complete, map, value},
    multi::{many0, many1},
    sequence::pair,
    IResult,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

impl Binary {
    // fn parse(input: &str) -> IResult<&str, Expression> {
    //     many1(alt())(input)
    // }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Add, tag("+")),
            value(Self::Subtract, tag("-")),
            value(Self::Multiply, tag("*")),
            value(Self::Divide, tag("/")),
            value(Self::Exponent, tag("^")),
        ))(input)
    }

    // The ability of a operator to 'bind' to a term
    fn binding_powers(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Subtract => (10, 15),
            Self::Multiply | Self::Divide => (20, 25),
            Self::Exponent => (35, 30),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Terms {
    left_term: Expression,
    right: Vec<(Operator, Expression)>,
}

impl Terms {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, left_term) = Expression::parse_term(input)?;
        let (input, right) = many0(complete(pair(Operator::parse, Expression::parse_term)))(input)?;

        Ok((input, Self { left_term, right }))
    }

    /// Reduces terms to one expression
    ///
    /// TODO: finish
    fn reduce(self) -> Expression {
        let Self {
            mut left_term,
            mut right,
        } = self;

        while let Some((operator, right_term)) = right.pop() {
            let (left_binding_power, right_binding_power) = operator.binding_powers();

            if left_binding_power > right_binding_power {
                let right_term = Self {
                    left_term: right_term,
                    right: right.clone(),
                }
                .reduce();
                left_term = Expression::Binary(Binary {
                    left: Box::new(left_term),
                    operator,
                    right: Box::new(right_term),
                });
            } else {
                left_term = Expression::Binary(Binary {
                    left: Box::new(left_term),
                    operator,
                    right: Box::new(right_term),
                });
            }
        }

        left_term
    }
}

#[test]
fn terms_parse() {
    use super::super::{literal::Character, Literal};

    assert_eq!(
        Terms::parse("'a' + 'b' - 'c'"),
        Ok((
            "",
            Terms {
                left_term: Expression::Literal(Literal::Character(Character('a'))),
                right: vec![
                    (
                        Operator::Add,
                        Expression::Literal(Literal::Character(Character('b')))
                    ),
                    (
                        Operator::Subtract,
                        Expression::Literal(Literal::Character(Character('c')))
                    )
                ]
            }
        ))
    );
}

#[test]
fn terms_reduce() {
    use super::super::{literal::Character, Literal};

    assert_eq!(
        dbg!(Terms::parse("'a' + 'b' * 'c'").unwrap().1.reduce()),
        Expression::Binary(Binary {
            left: Box::new(Expression::Literal(Literal::Character(Character('a')))),
            operator: Operator::Add,
            right: Box::new(Expression::Binary(Binary {
                left: Box::new(Expression::Literal(Literal::Character(Character('b')))),
                operator: Operator::Multiply,
                right: Box::new(Expression::Literal(Literal::Character(Character('c'))))
            }))
        })
    );
}
