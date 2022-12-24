use super::super::Span;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{opt, value},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    pub base: Base,
    pub digits: Vec<u8>,
    pub sign: Sign,
}

impl Integer {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        let (input, sign) = Sign::parse(input)?;
        let (input, base) = Base::parse(input)?;
        let (input, digits) = base.parse_digits(input)?;

        Ok((input, Self { base, digits, sign }))
    }
}

#[test]
fn integer_parses() {
    use super::super::test;

    assert_eq!(
        test::strip_span(Integer::parse("42".into())),
        Ok((
            String::new(),
            Integer {
                base: Base::Decimal,
                digits: vec![4, 2],
                sign: Sign::Positive,
            }
        ))
    );
    assert_eq!(
        test::strip_span(Integer::parse("0xff".into())),
        Ok((
            String::new(),
            Integer {
                base: Base::Hexadecimal,
                digits: vec![0xf, 0xf],
                sign: Sign::Positive,
            }
        ))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Float {
    pub base: Base,
    pub whole: Vec<u8>,
    pub fractional: Vec<u8>,
    pub sign: Sign,
    pub exponent: Option<Exponent>,
}

impl Float {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        let (input, sign) = Sign::parse(input)?;
        let (input, base) = Base::parse(input)?;
        let (input, whole) = opt(|input| base.parse_digits(input))
            .map(Option::unwrap_or_default)
            .parse(input)?;

        let (input, _) = tag(".")(input)?;
        let (input, fractional) = base.parse_digits(input)?;

        let (input, exponent) = if base == Base::Decimal {
            Exponent::parse(input)?
        } else {
            (input, None)
        };

        Ok((
            input,
            Self {
                base,
                whole,
                fractional,
                sign,
                exponent,
            },
        ))
    }
}

#[test]
fn float_parses() {
    use super::super::test;

    assert_eq!(
        test::strip_span(Float::parse(".5".into())),
        Ok((
            String::new(),
            Float {
                base: Base::Decimal,
                whole: Vec::new(),
                fractional: vec![5],
                sign: Sign::Positive,
                exponent: None
            }
        ))
    );
    assert_eq!(
        test::strip_span(Float::parse("4.2".into())),
        Ok((
            String::new(),
            Float {
                base: Base::Decimal,
                whole: vec![4],
                fractional: vec![2],
                sign: Sign::Positive,
                exponent: None
            }
        ))
    );
    assert_eq!(
        test::strip_span(Float::parse("0xff.0".into())),
        Ok((
            String::new(),
            Float {
                base: Base::Hexadecimal,
                whole: vec![0xf, 0xf],
                fractional: vec![0],
                sign: Sign::Positive,
                exponent: None
            }
        ))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exponent {
    pub whole: Vec<u8>,
    pub fractional: Vec<u8>,
    pub sign: Sign,
}

impl Exponent {
    pub fn parse(input: Span) -> IResult<Span, Option<Self>> {
        let (input, Some(_)) = opt(alt((tag("e"), tag("E"))))(input)? else {
            return Ok((input, None))
        };

        let (input, sign) = Sign::parse(input)?;
        let (input, whole) = Base::Decimal.parse_digits(input)?;

        let (input, (_, fractional)) = opt(tag("."))
            .and(opt(|input| Base::Decimal.parse_digits(input)).map(Option::unwrap_or_default))
            .parse(input)?;

        Ok((
            input,
            Some(Self {
                whole,
                fractional,
                sign,
            }),
        ))
    }
}

#[test]
fn exponent_parses() {
    use super::super::test;

    assert_eq!(
        test::strip_span(Exponent::parse(";".into())),
        Ok((";".to_string(), None))
    );
    assert_eq!(
        test::strip_span(Exponent::parse("e1;".into())),
        Ok((
            ";".to_string(),
            Some(Exponent {
                whole: vec![1],
                fractional: Vec::new(),
                sign: Sign::Positive
            })
        ))
    );
    assert_eq!(
        test::strip_span(Exponent::parse("e-1.1;".into())),
        Ok((
            ";".to_string(),
            Some(Exponent {
                whole: vec![1],
                fractional: vec![1],
                sign: Sign::Negative
            })
        ))
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Base {
    Decimal,
    Hexadecimal,
    Binary,
    Octal,
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
fn base_parses() {
    use super::super::test;

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

#[test]
fn digits_parse() {
    use super::super::test;

    assert_eq!(
        test::strip_span(Base::Decimal.parse_digits("10".into())),
        Ok((String::new(), vec![1, 0]))
    );
    assert_eq!(
        test::strip_span(Base::Hexadecimal.parse_digits("FF".into())),
        Ok((String::new(), vec![0xF, 0xF]))
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
    use super::super::test;

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
