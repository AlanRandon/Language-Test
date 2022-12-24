pub use boolean::Boolean;
pub use character::Character;
use nom::{branch::alt, error::context, IResult, Parser};
pub use number::{Float, Integer};
pub use string::Str;

use super::Span;

pub mod boolean;
pub mod character;
pub mod number;
pub mod string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Character(Character),
    String(Str),
    Boolean(Boolean),
}

impl Literal {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        alt((
            context("boolean", Boolean::parse.map(Self::Boolean)),
            context("character", Character::parse.map(Self::Character)),
            context("string", Str::parse.map(Self::String)),
            context("float", Float::parse.map(Self::Float)),
            context("integer", Integer::parse.map(Self::Integer)),
        ))(input)
    }
}

#[test]
fn literal_parses() {
    use super::test;
    use number::{Base, Sign};

    assert_eq!(
        test::strip_span(Literal::parse("10".into())),
        Ok((
            String::new(),
            Literal::Integer(Integer {
                base: Base::Decimal,
                digits: vec![1, 0],
                sign: Sign::Positive
            })
        ))
    );

    assert_eq!(
        test::strip_span(Literal::parse("10.0".into())),
        Ok((
            String::new(),
            Literal::Float(Float {
                base: Base::Decimal,
                whole: vec![1, 0],
                fractional: vec![0],
                sign: Sign::Positive,
                exponent: None
            })
        ))
    );

    assert_eq!(
        test::strip_span(Literal::parse("'a'".into())),
        Ok((String::new(), Literal::Character(Character('a'))))
    );

    assert_eq!(
        test::strip_span(Literal::parse("\"a\"".into())),
        Ok((String::new(), Literal::String(Str(String::from("a")))))
    );
}
