use super::super::prelude::*;

use super::Character;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String(pub std::string::String);

impl String {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        let (input, _) = tag("\"")(input)?;
        let (input, chars) = many0(
            not(tag("\""))
                .and(Character::parse_char)
                .map(|(_, result)| result),
        )(input)?;
        let (input, _) = tag("\"")(input)?;
        dbg!(&chars);
        Ok((input, Self(chars.iter().collect())))
    }
}

#[test]
fn string_parses() {
    assert_eq!(
        test::strip_span(String::parse("\"hello world\"".into())),
        Ok((
            std::string::String::new(),
            String(std::string::String::from("hello world"))
        ))
    );
    assert_eq!(
        test::strip_span(String::parse("\"hello\\nworld\"".into())),
        Ok((
            std::string::String::new(),
            String(std::string::String::from("hello\nworld"))
        ))
    );
}
