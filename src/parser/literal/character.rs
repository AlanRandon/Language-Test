use super::super::Span;
use nom::{
    branch::alt, bytes::streaming::tag, character::complete::satisfy, combinator::value, IResult,
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character(pub char);

impl Character {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        let (input, _) = tag("'")(input)?;
        let (input, character) = Self::parse_char(input)?;
        let (input, _) = tag("'")(input)?;
        Ok((input, Self(character)))
    }

    pub fn parse_char(input: Span) -> IResult<Span, char> {
        let (input, result) = satisfy(|character| character != '\\')
            .or(tag("\\")
                .and(|input| {
                    alt((
                        value('\n', tag("n")),
                        value('\r', tag("r")),
                        value('\'', tag("'")),
                        value('\"', tag("\"")),
                        value('\0', tag("0")),
                    ))(input)
                })
                .map(|result| result.1))
            .parse(input)?;
        Ok((input, result))
    }
}

#[test]
fn character_parses() {
    use super::super::test;

    assert_eq!(
        test::strip_span(Character::parse("'a'".into())),
        Ok((String::new(), Character('a')))
    );
    assert_eq!(
        test::strip_span(Character::parse("'\n'".into())),
        Ok((String::new(), Character('\n')))
    );
}
