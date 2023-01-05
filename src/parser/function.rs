use super::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<'a> {
    pub parameters: Parameters<'a>,
    pub return_type: Type<'a>,
    pub body: Box<Expression<'a>>,
}

impl<'a> Function<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span<'a>, Self> {
        let (input, _) = delimited(whitespace::optional, tag("fn"), whitespace::optional)(input)?;
        let (input, parameters) = delimited(
            pair(tag("("), whitespace::optional),
            Parameters::parse,
            pair(whitespace::optional, tag(")")),
        )(input)?;
        let (input, _) = tuple((whitespace::optional, tag("->"), whitespace::optional))(input)?;
        let (input, return_type) = Type::parse(input)?;
        let (input, body) = delimited(
            whitespace::optional,
            delimited(tag("{"), Expression::parse, tag("}")),
            whitespace::optional,
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
                body: Box::new(Expression::Literal(Literal::Integer(literal::Integer {
                    base: number::Base::Decimal,
                    digits: vec![1],
                    sign: number::Sign::Positive
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
                whitespace::optional,
                tag(","),
                whitespace::optional,
            ))(input)
        };
        terminated(
            separated_list0(
                parse_seperator,
                tuple((Type::parse, whitespace::required, Identifier::parse))
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
