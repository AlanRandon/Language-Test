pub use expression::Expression;
pub use literal::Literal;
use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::multispace1,
    combinator::{complete, value},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub mod expression;
pub mod literal;

fn optional_whitespace(input: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(complete(alt((
            value((), multispace1),
            value((), tuple((tag("/*"), take_until("*/"), tag("*/")))),
            value((), tuple((tag("//"), take_until("\n"), tag("\n")))),
        )))),
    )(input)
}

#[test]
fn whitespace_parses() {
    assert_eq!(optional_whitespace(" \n \n \r\n \t"), Ok(("", ())));
    assert_eq!(optional_whitespace(" // comment \n "), Ok(("", ())));
    assert_eq!(optional_whitespace(" /* comment */ "), Ok(("", ())));
}
