use nom::{
    branch::alt, bytes::streaming::tag, character::complete::satisfy, combinator::value, IResult,
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character(pub char);

impl Character {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("'")(input)?;
        let (input, character) = Self::parse_char(input)?;
        let (input, _) = tag("'")(input)?;
        Ok((input, Self(character)))
    }

    pub fn parse_char(input: &str) -> IResult<&str, char> {
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
    assert_eq!(Character::parse("'a'"), Ok(("", Character('a'))));
    assert_eq!(Character::parse("'\n'"), Ok(("", Character('\n'))));
}
