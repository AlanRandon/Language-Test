use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{map, value},
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
    fn parse(input: &str) -> IResult<&str, Expression> {
        many1(alt())(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrecLevel {
    AddOrSubtract,
    MultiplyOrDivide,
    Exponent,
}

impl PrecLevel {
    fn parser<'a>(&self) -> fn(&'a str) -> IResult<&'a str, Operator> {
        match self {
            Self::AddOrSubtract => {
                (|input| {
                    alt((
                        value(Operator::Add, tag("+")),
                        value(Operator::Subtract, tag("-")),
                    ))(input)
                }) as fn(_) -> _
            }
            Self::MultiplyOrDivide => {
                (|input| {
                    alt((
                        value(Operator::Multiply, tag("*")),
                        value(Operator::Divide, tag("/")),
                    ))(input)
                }) as fn(_) -> _
            }
            Self::Exponent => (|input| value(Operator::Exponent, tag("^"))(input)) as fn(_) -> _,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terms {
    first: Expression,
    others: Vec<(Operator, Expression)>,
}

impl Terms {
    fn parse(input: &str, level: PrecLevel) -> IResult<&str, Self> {
        let (input, first) = Expression::parse_term(input)?;
        let (input, others) = many0(pair(level.parser(), Expression::parse_term))(input)?;

        Ok((input, Self { first, others }))
    }

    fn fold(self) -> Expression {
        self.others
            .into_iter()
            .fold(self.first, |left, (operator, right)| {
                Expression::Binary(Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                })
            })
    }

    fn rfold(self) -> Expression {
        self.others
            .into_iter()
            .rev()
            .rfold(self.first, |left, (operator, right)| {
                Expression::Binary(Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                })
            })
    }
}

#[test]
fn terms_parse() {
    use super::super::{literal::Character, Literal};

    assert_eq!(
        Terms::parse("'a' + 'b' - 'c' * 'd'", PrecLevel::AddOrSubtract),
        Ok((
            "* 'd'",
            Terms {
                first: Expression::Literal(Literal::Character(Character('a'))),
                others: vec![
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
fn terms_fold() {
    use super::super::{literal::Character, Literal};

    assert_eq!(
        Terms::parse("'a' + 'b' - 'c' * 'd'", PrecLevel::AddOrSubtract)
            .unwrap()
            .1
            .fold(),
        Expression::Binary(Binary {
            left: Box::new(Expression::Binary(Binary {
                left: Box::new(Expression::Literal(Literal::Character(Character('a')))),
                operator: Operator::Add,
                right: Box::new(Expression::Literal(Literal::Character(Character('b'))))
            })),
            operator: Operator::Subtract,
            right: Box::new(Expression::Literal(Literal::Character(Character('c'))))
        })
    );

    assert_eq!(
        Terms::parse("'a' + 'b' - 'c' + 'd' ^ 'e'", PrecLevel::AddOrSubtract)
            .unwrap()
            .1
            .rfold(),
        Expression::Binary(Binary {
            left: Box::new(Expression::Binary(Binary {
                left: Box::new(Expression::Binary(Binary {
                    left: Box::new(Expression::Literal(Literal::Character(Character('a')))),
                    operator: Operator::Add,
                    right: Box::new(Expression::Literal(Literal::Character(Character('b'))))
                })),
                operator: Operator::Subtract,
                right: Box::new(Expression::Literal(Literal::Character(Character('c'))))
            })),
            operator: Operator::Add,
            right: Box::new(Expression::Literal(Literal::Character(Character('d'))))
        })
    );
}
