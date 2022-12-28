use super::Span;
use nom::{
    bytes::complete::take_while, character::complete::satisfy, combinator::consumed,
    sequence::pair, IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier<'a>(pub Span<'a>);

impl<'a> Identifier<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span, Self> {
        let (input, (identifier, _)) = consumed(pair(
            satisfy(|character: char| character.is_alphabetic() || character == '_'),
            take_while(|character: char| character.is_alphanumeric() || character == '_'),
        ))
        .parse(input)?;

        match identifier.to_string().as_str() {
            "in" => Err(nom::Err::Error(nom::error::Error {
                input: identifier,
                code: nom::error::ErrorKind::Fail,
            })),
            _ => Ok((input, Self(identifier))),
        }
    }
}

#[test]
fn identifiers_parse() {
    use super::test;

    assert_eq!(
        test::strip_span(Identifier::parse("abc".into())),
        Ok((String::new(), Identifier(Span::new("abc"))))
    );
    assert_eq!(
        test::strip_span(Identifier::parse("a_b_c;".into())),
        Ok((";".to_string(), Identifier(Span::new("a_b_c"))))
    );
    assert_eq!(
        test::strip_span(Identifier::parse("é".into())),
        Ok((String::new(), Identifier(Span::new("é"))))
    );
    assert!(test::strip_span(Identifier::parse(";".into())).is_err());
    assert!(test::strip_span(Identifier::parse("in".into())).is_err());
}
