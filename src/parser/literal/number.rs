use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{opt, value},
    multi::many0,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    base: Base,
    whole: Vec<char>,
    fractional: Vec<char>,
    sign: Sign,
    exponent: Vec<char>,
}

impl Number {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, sign) = Sign::parse(input)?;
        let (input, base) = Base::parse(input)?;
        let (input, whole) = base.parse_digits(input)?;

        let (input, (_, fractional)) = opt(tag("."))
            .and(|input| base.parse_digits(input))
            .parse(input)?;

        let (input, has_exponent) = if base != Base::Hexadecimal {
            opt(tag("e"))(input)?
        } else {
            (input, None)
        };

        let (input, exponent) = if has_exponent.is_some() {
            base.parse_digits(input)?
        } else {
            (input, ['1'].to_vec())
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

    pub fn parse_digits<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<char>> {
        (match self {
            Base::Binary => many0(one_of("01")),
            Base::Octal => many0(one_of("01234567")),
            Base::Decimal => many0(one_of("0123456789")),
            Base::Hexadecimal => many0(one_of("0123456789abcdefABCDEF")),
        })(input)
    }
}

#[test]
fn base_works() {
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
fn sign_works() {
    assert_eq!(Sign::parse("+1"), Ok(("1", Sign::Positive)));
    assert_eq!(Sign::parse("-1"), Ok(("1", Sign::Negative)));
    assert_eq!(Sign::parse("1"), Ok(("1", Sign::Positive)));
}
