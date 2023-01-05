use super::{super::super::prelude::*, Base, Sign};

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
