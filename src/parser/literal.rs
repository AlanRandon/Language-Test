use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, combinator::opt, IResult,
    Parser,
};

pub struct Number {
    base: Base,
    whole: Vec<u8>,
    fractional: Vec<u8>,
    exponent: Vec<u8>,
    sign: Sign,
}

pub enum Base {
    Decimal,
    Hexadecimal,
    Binary,
    Octal,
}

impl Base {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("0x").map(|_| Self::Hexadecimal),
            tag("0o").map(|_| Self::Octal),
            tag("0b").map(|_| Self::Binary),
            tag("").map(|_| Self::Decimal),
        ))(input)
    }
}

pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("-").map(|_| Self::Negative),
            opt(tag("+")).map(|_| Self::Positive),
        ))(input)
    }
}
