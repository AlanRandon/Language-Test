use super::{
    identifier::Identifier, optional_whitespace, required_whitespace, types::Type, Expression, Span,
};
use nom::{
    bytes::streaming::tag,
    combinator::{complete, opt},
    multi::separated_list0,
    sequence::{delimited, pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<'a> {
    pub parameters: Parameters<'a>,
    pub return_type: Type<'a>,
    pub body: Box<Expression<'a>>,
}

impl<'a> Function<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span<'a>, Self> {
        let (input, _) = delimited(optional_whitespace, tag("fn"), optional_whitespace)(input)?;
        let (input, parameters) = delimited(
            pair(tag("("), optional_whitespace),
            Parameters::parse,
            pair(optional_whitespace, tag(")")),
        )(input)?;
        let (input, _) = tuple((optional_whitespace, tag("->"), optional_whitespace))(input)?;
        let (input, return_type) = Type::parse(input)?;
        let (input, body) = delimited(
            optional_whitespace,
            delimited(tag("{"), Expression::parse, tag("}")),
            optional_whitespace,
        )(input)?;
        Ok((
            input,
            Self {
                parameters,
                return_type,
                body: Box::new(body),
            },
        ))
    }
}

#[test]
fn function_parses() {
    use super::{
        literal::number::{Base, Sign},
        literal::Integer,
        test, Literal,
    };

    assert_eq!(
        test::strip_span(Function::parse(
            "fn(Int64 x, Int64 y) -> Int64 { 1 }".into()
        )),
        Ok((
            String::new(),
            Function {
                parameters: Parameters(vec![
                    (
                        Type(Identifier(unsafe {
                            Span::new_from_raw_offset(3, 1, "Int64", ())
                        })),
                        Identifier(unsafe { Span::new_from_raw_offset(9, 1, "x", ()) })
                    ),
                    (
                        Type(Identifier(unsafe {
                            Span::new_from_raw_offset(12, 1, "Int64", ())
                        })),
                        Identifier(unsafe { Span::new_from_raw_offset(18, 1, "y", ()) })
                    )
                ]),
                return_type: Type(Identifier(unsafe {
                    Span::new_from_raw_offset(24, 1, "Int64", ())
                })),
                body: Box::new(Expression::Literal(Literal::Integer(Integer {
                    base: Base::Decimal,
                    digits: vec![1],
                    sign: Sign::Positive
                })))
            }
        ))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameters<'a>(pub Vec<(Type<'a>, Identifier<'a>)>);

impl<'a> Parameters<'a> {
    fn parse(input: Span<'a>) -> IResult<Span<'a>, Self> {
        let parse_seperator = |input| {
            complete(delimited(
                optional_whitespace,
                tag(","),
                optional_whitespace,
            ))(input)
        };
        terminated(
            separated_list0(
                parse_seperator,
                tuple((Type::parse, required_whitespace, Identifier::parse))
                    .map(|(param_type, _, name)| (param_type, name)),
            ),
            opt(parse_seperator),
        )
        .map(Self)
        .parse(input)
    }
}

#[test]
fn parameters_parse() {
    use super::test;

    assert_eq!(
        test::strip_span(Parameters::parse("Int64 x, Int64 y".into())),
        Ok((
            String::new(),
            Parameters(vec![
                (
                    Type(Identifier(Span::new("Int64"))),
                    Identifier(unsafe { Span::new_from_raw_offset(6, 1, "x", ()) })
                ),
                (
                    Type(Identifier(unsafe {
                        Span::new_from_raw_offset(9, 1, "Int64", ())
                    })),
                    Identifier(unsafe { Span::new_from_raw_offset(15, 1, "y", ()) })
                )
            ])
        ))
    );
}
