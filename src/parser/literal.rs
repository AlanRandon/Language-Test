pub use super::prelude::*;
pub use boolean::Boolean;
pub use character::Character;
pub use number::{Float, Integer};
pub use string::String;

pub mod boolean;
pub mod character;
pub mod number;
pub mod string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Character(Character),
    String(String),
    Boolean(Boolean),
}

impl Literal {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        alt((
            context("boolean", Boolean::parse.map(Self::Boolean)),
            context("character", Character::parse.map(Self::Character)),
            context("string", String::parse.map(Self::String)),
            context("float", Float::parse.map(Self::Float)),
            context("integer", Integer::parse.map(Self::Integer)),
        ))(input)
    }
}

#[test]
fn literal_parses() {
    assert_eq!(
        test::strip_span(Literal::parse("10".into())),
        Ok((
            std::string::String::new(),
            Literal::Integer(Integer {
                base: number::Base::Decimal,
                digits: vec![1, 0],
                sign: number::Sign::Positive
            })
        ))
    );

    assert_eq!(
        test::strip_span(Literal::parse("10.0".into())),
        Ok((
            std::string::String::new(),
            Literal::Float(Float {
                base: number::Base::Decimal,
                whole: vec![1, 0],
                fractional: vec![0],
                sign: number::Sign::Positive,
                exponent: None
            })
        ))
    );

    assert_eq!(
        test::strip_span(Literal::parse("'a'".into())),
        Ok((
            std::string::String::new(),
            Literal::Character(Character('a'))
        ))
    );

    assert_eq!(
        test::strip_span(Literal::parse("\"a\"".into())),
        Ok((
            std::string::String::new(),
            Literal::String(String(std::string::String::from("a")))
        ))
    );
}
