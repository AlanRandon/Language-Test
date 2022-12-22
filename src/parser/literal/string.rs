use nom::{bytes::streaming::tag, combinator::not, multi::many0, IResult, Parser};

use super::Character;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Str(pub String);

impl Str {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("\"")(input)?;
        let (input, chars) = many0(
            not(tag("\""))
                .and(Character::parse_char)
                .map(|result| result.1),
        )(input)?;
        let (input, _) = tag("\"")(input)?;
        dbg!(&chars);
        Ok((input, Self(chars.iter().collect())))
    }
}

#[test]
fn string_parses() {
    assert_eq!(
        Str::parse("\"hello world\""),
        Ok(("", Str(String::from("hello world"))))
    );
    assert_eq!(
        Str::parse("\"hello\\nworld\""),
        Ok(("", Str(String::from("hello\nworld"))))
    );
}
