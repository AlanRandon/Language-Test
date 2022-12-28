use super::{identifier::Identifier, optional_whitespace, Expression, Span};
use nom::{bytes::streaming::tag, multi::many1, sequence::delimited, IResult};

/// A let-in expression, for example
///
/// ```!
/// let
///     x = 2
/// in
///     x ^ x
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LetIn<'a> {
    pub assignments: Vec<Assignment<'a>>,
    pub expression: Box<Expression<'a>>,
}

impl<'a> LetIn<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span, Self> {
        let (input, _) = delimited(optional_whitespace, tag("let"), optional_whitespace)(input)?;
        let (input, assignments) = many1(Assignment::parse)(input)?;
        let (input, _) = delimited(optional_whitespace, tag("in"), optional_whitespace)(input)?;
        let (input, expression) = Expression::parse(input)?;

        Ok((
            input,
            Self {
                assignments,
                expression: Box::new(expression),
            },
        ))
    }
}

#[test]
fn let_in_parses() {
    use super::{
        literal::{
            number::{Base, Sign},
            Integer,
        },
        test, Literal,
    };

    assert_eq!(
        test::strip_span(LetIn::parse("let a_useless_value = 1 in 1".into())),
        Ok((
            String::new(),
            LetIn {
                assignments: vec![Assignment {
                    identifier: Identifier(unsafe {
                        Span::new_from_raw_offset(4, 1, "a_useless_value", ())
                    }),
                    value: Expression::Literal(Literal::Integer(Integer {
                        base: Base::Decimal,
                        digits: vec![1],
                        sign: Sign::Positive
                    }))
                }],
                expression: Box::new(Expression::Literal(Literal::Integer(Integer {
                    base: Base::Decimal,
                    digits: vec![1],
                    sign: Sign::Positive
                })))
            }
        ))
    );
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Assignment<'a> {
    pub identifier: Identifier<'a>,
    pub value: Expression<'a>,
}

impl<'a> Assignment<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span, Self> {
        let (input, identifier) = Identifier::parse(input)?;
        let (input, _) = delimited(optional_whitespace, tag("="), optional_whitespace)(input)?;
        let (input, value) = Expression::parse(input)?;

        Ok((input, Self { identifier, value }))
    }
}

#[test]
fn assignment_parses() {
    use super::{
        literal::{
            number::{Base, Sign},
            Integer,
        },
        test, Literal,
    };

    assert_eq!(
        test::strip_span(Assignment::parse("a = 1".into())),
        Ok((
            String::new(),
            Assignment {
                identifier: Identifier(Span::new("a")),
                value: Expression::Literal(Literal::Integer(Integer {
                    base: Base::Decimal,
                    digits: vec![1],
                    sign: Sign::Positive
                }))
            }
        ))
    );
}
