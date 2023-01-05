use super::super::prelude::*;

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
    assert_eq!(
        test::strip_span(Boolean::parse("true".into())),
        Ok((String::new(), Boolean(true)))
    );
    assert_eq!(
        test::strip_span(Boolean::parse("false".into())),
        Ok((String::new(), Boolean(false)))
    );
}
