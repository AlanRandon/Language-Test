use nom::{branch::alt, bytes::streaming::tag, combinator::value, IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean(pub bool);

impl Boolean {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((value(true, tag("true")), value(false, tag("false"))))
            .map(Self)
            .parse(input)
    }
}

#[test]
fn boolean_parses() {
    assert_eq!(Boolean::parse("true"), Ok(("", Boolean(true))));
    assert_eq!(Boolean::parse("false"), Ok(("", Boolean(false))));
}
