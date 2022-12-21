use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{opt, value},
    multi::{many0, many1},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    pub base: Base,
    pub whole: Vec<u8>,
    pub fractional: Vec<u8>,
    pub sign: Sign,
    pub exponent: Option<Exponent>,
}

impl Number {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, sign) = Sign::parse(input)?;
        let (input, base) = Base::parse(input)?;
        let (input, whole) = base.parse_digits(input)?;

        let (input, (_, fractional)) = opt(tag("."))
            .and(|input| base.parse_digits(input))
            .parse(input)?;

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
fn number_parses() {
    assert_eq!(
        Number::parse("42"),
        Ok((
            "",
            Number {
                base: Base::Decimal,
                whole: vec![4, 2],
                fractional: Vec::new(),
                sign: Sign::Positive,
                exponent: None
            }
        ))
    );
    assert_eq!(
        Number::parse("4.2"),
        Ok((
            "",
            Number {
                base: Base::Decimal,
                whole: vec![4],
                fractional: vec![2],
                sign: Sign::Positive,
                exponent: None
            }
        ))
    );
    assert_eq!(
        Number::parse("0xFF"),
        Ok((
            "",
            Number {
                base: Base::Hexadecimal,
                whole: vec![0xf, 0xf],
                fractional: Vec::new(),
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
    pub fn parse(input: &str) -> IResult<&str, Option<Self>> {
        let (input, Some(_)) = opt(alt((tag("e"), tag("E"))))(input)? else {
            return Ok((input, None))
        };

        let (input, sign) = Sign::parse(input)?;
        let (input, whole) = Base::Decimal.parse_digits(input)?;

        let (input, (_, fractional)) = opt(tag("."))
            .and(|input| Base::Decimal.parse_digits(input))
            .parse(input)?;

        Ok((
            input,
            Some(Self {
                sign,
                whole,
                fractional,
            }),
        ))
    }
}

#[test]
fn exponent_parses() {
    assert_eq!(Exponent::parse(";"), Ok((";", None)));
    assert_eq!(
        Exponent::parse("e1;"),
        Ok((
            ";",
            Some(Exponent {
                whole: vec![1],
                fractional: Vec::new(),
                sign: Sign::Positive
            })
        ))
    );
    assert_eq!(
        Exponent::parse("e-1.1;"),
        Ok((
            ";",
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
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Hexadecimal, tag("0x")),
            value(Self::Octal, tag("0o")),
            value(Self::Binary, tag("0b")),
            value(Self::Decimal, tag("")),
        ))(input)
    }

    pub fn parse_digits<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<u8>> {
        many1(
            match self {
                Base::Binary => one_of("01"),
                Base::Octal => one_of("01234567"),
                Base::Decimal => one_of("0123456789"),
                Base::Hexadecimal => one_of("0123456789abcdefABCDEF"),
            }
            .map(|char| char.to_digit(16).unwrap() as u8),
        )(input)
    }
}

#[test]
fn base_parses() {
    assert_eq!(Base::parse("0b1"), Ok(("1", Base::Binary)));
    assert_eq!(Base::parse("0o7"), Ok(("7", Base::Octal)));
    assert_eq!(Base::parse("9"), Ok(("9", Base::Decimal)));
    assert_eq!(Base::parse("0xff"), Ok(("ff", Base::Hexadecimal)));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Negative, tag("-")),
            value(Self::Positive, opt(tag("+"))),
        ))(input)
    }
}

#[test]
fn sign_parses() {
    assert_eq!(Sign::parse("+1"), Ok(("1", Sign::Positive)));
    assert_eq!(Sign::parse("-1"), Ok(("1", Sign::Negative)));
    assert_eq!(Sign::parse("1"), Ok(("1", Sign::Positive)));
}
