use nom::{IResult, Parser};

use super::{identifier::Identifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type<'a>(pub Identifier<'a>);

impl<'a> Type<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span<'a>, Self> {
        Identifier::parse.map(Type).parse(input)
    }
}

#[test]
fn type_parses() {
    use super::test;

    assert_eq!(
        test::strip_span(Type::parse("Int64".into())),
        Ok((String::new(), Type(Identifier(Span::new("Int64")))))
    );
}
