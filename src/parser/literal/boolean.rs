use super::super::Span;
use nom::{branch::alt, bytes::streaming::tag, combinator::value, IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean(pub bool);

impl Boolean {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        alt((value(true, tag("true")), value(false, tag("false"))))
            .map(Self)
            .parse(input)
    }
}

#[test]
fn boolean_parses() {
    use super::super::test;

    assert_eq!(
        test::strip_span(Boolean::parse("true".into())),
        Ok((String::new(), Boolean(true)))
    );
    assert_eq!(
        test::strip_span(Boolean::parse("false".into())),
        Ok((String::new(), Boolean(false)))
    );
}
