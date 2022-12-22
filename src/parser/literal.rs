pub use boolean::Boolean;
pub use character::Character;
use nom::{branch::alt, error::context, IResult, Parser};
pub use number::{Float, Integer};
pub use string::Str;

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
    pub fn parse(input: &str) -> IResult<&str, Self> {
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
    use number::{Base, Sign};

    assert_eq!(
        Literal::parse("10"),
        Ok((
            "",
            Literal::Integer(Integer {
                base: Base::Decimal,
                digits: vec![1, 0],
                sign: Sign::Positive
            })
        ))
    );

    assert_eq!(
        Literal::parse("10.0"),
        Ok((
            "",
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
        Literal::parse("'a'"),
        Ok(("", Literal::Character(Character('a'))))
    );

    assert_eq!(
        Literal::parse("\"a\""),
        Ok(("", Literal::String(Str(String::from("a")))))
    );
}
