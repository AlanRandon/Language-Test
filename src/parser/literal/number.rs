use super::super::prelude::*;
pub use float::Float;
pub use integer::Integer;

pub mod float;
pub mod integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Base {
    Decimal = 10,
    Hexadecimal = 16,
    Binary = 2,
    Octal = 8,
}

impl Base {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        alt((
            value(Self::Hexadecimal, tag("0x")),
            value(Self::Octal, tag("0o")),
            value(Self::Binary, tag("0b")),
            value(Self::Decimal, tag("")),
        ))(input)
    }

    pub fn parse_digits<'a>(&self, input: Span<'a>) -> IResult<Span<'a>, Vec<u8>> {
        many1(
            terminated(
                match self {
                    Self::Binary => one_of("01"),
                    Self::Octal => one_of("01234567"),
                    Self::Decimal => one_of("0123456789"),
                    Self::Hexadecimal => one_of("0123456789abcdefABCDEF"),
                },
                opt(tag("_")),
            )
            .map(
                #[allow(clippy::cast_possible_truncation)]
                |char| char.to_digit(16).unwrap() as u8,
            ),
        )(input)
    }
}

#[test]
fn digits_parse() {
    assert_eq!(
        test::strip_span(Base::Decimal.parse_digits("10".into())),
        Ok((String::new(), vec![1, 0]))
    );
    assert_eq!(
        test::strip_span(Base::Hexadecimal.parse_digits("FF".into())),
        Ok((String::new(), vec![0xF, 0xF]))
    );
}

#[test]
fn base_parses() {
    assert_eq!(
        test::strip_span(Base::parse("0b1".into())),
        Ok(("1".to_string(), Base::Binary))
    );
    assert_eq!(
        test::strip_span(Base::parse("0o7".into())),
        Ok(("7".to_string(), Base::Octal))
    );
    assert_eq!(
        test::strip_span(Base::parse("9".into())),
        Ok(("9".to_string(), Base::Decimal))
    );
    assert_eq!(
        test::strip_span(Base::parse("".into())),
        Ok((String::new(), Base::Decimal))
    );
    assert_eq!(
        test::strip_span(Base::parse("0xff".into())),
        Ok(("ff".to_string(), Base::Hexadecimal))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        alt((
            value(Self::Negative, tag("-")),
            value(Self::Positive, opt(tag("+"))),
        ))(input)
    }
}

#[test]
fn sign_parses() {
    assert_eq!(
        test::strip_span(Sign::parse("+1".into())),
        Ok(("1".to_string(), Sign::Positive))
    );
    assert_eq!(
        test::strip_span(Sign::parse("-1".into())),
        Ok(("1".to_string(), Sign::Negative))
    );
    assert_eq!(
        test::strip_span(Sign::parse("1".into())),
        Ok(("1".to_string(), Sign::Positive))
    );
}
