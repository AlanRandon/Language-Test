use super::super::Span;
use super::binary::Binary;
use super::{binary::Operator, Expression};
use nom::combinator::complete;
use nom::multi::many0;
use nom::sequence::pair;
use nom::{IResult, Parser};

#[derive(Debug, PartialEq, Eq)]
pub struct Terms {
    // The first term in a sequence of binary expressions (e.g. `1` in `1 + 2 * 3`)
    left_term: Expression,
    /// The operators and expressions to the right of `left_term`. stored in the reverse order to that which they appear in the expression
    right: Vec<(Operator, Expression)>,
}

impl Terms {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        let (input, left_term) = Expression::parse_term(input)?;
        let (input, right) = many0(complete(pair(Operator::parse, Expression::parse_term)))
            .map(|terms| terms.into_iter().rev().collect())
            .parse(input)?;

        Ok((input, Self { left_term, right }))
    }

    /// Reduces terms to one expression
    pub fn reduce(self) -> Expression {
        let Self {
            mut left_term,
            mut right,
        } = self;

        while let Some((operator, right_term)) = right.pop() {
            let left_bp = operator.binding_powers().1;
            let right_bp = right.get(0).map_or(255, |(op, _)| op.binding_powers().0);

            if left_bp < right_bp {
                return Expression::Binary(Binary {
                    left: Box::new(left_term),
                    operator,
                    right: Box::new(Self::reduce(Self {
                        left_term: right_term,
                        right,
                    })),
                });
            }

            left_term = Expression::Binary(Binary {
                left: Box::new(left_term),
                operator,
                right: Box::new(right_term),
            });
        }

        left_term
    }
}

#[test]
fn terms_parse() {
    use super::super::{literal::Character, test, Literal};

    assert_eq!(
        test::strip_span(Terms::parse("'a' + 'b' - 'c'".into())),
        Ok((
            String::new(),
            Terms {
                left_term: Expression::Literal(Literal::Character(Character('a'))),
                right: vec![
                    (
                        Operator::Subtract,
                        Expression::Literal(Literal::Character(Character('c')))
                    ),
                    (
                        Operator::Add,
                        Expression::Literal(Literal::Character(Character('b')))
                    ),
                ]
            }
        ))
    );
}

#[test]
fn terms_reduce() {
    use super::super::{literal::Character, Literal};

    assert_eq!(
        dbg!(Terms::parse("'a' + 'b' * 'c'".into()).unwrap().1.reduce()),
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
