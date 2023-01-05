use super::{super::super::prelude::*, Base, Sign};

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
