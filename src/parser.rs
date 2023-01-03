pub use expression::Expression;
pub use literal::Literal;
use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::multispace1,
    combinator::{complete, value},
    multi::{many0, many1},
    sequence::{pair, tuple},
    IResult,
};
use nom_locate::LocatedSpan;

pub mod expression;
pub mod function;
pub mod identifier;
pub mod let_in;
pub mod literal;
pub mod types;

type Span<'a> = LocatedSpan<&'a str>;

fn optional_whitespace(input: Span) -> IResult<Span, ()> {
    value(
        (),
        many0(complete(alt((
            value((), multispace1),
            value((), tuple((tag("/*"), take_until("*/"), tag("*/")))),
            value((), pair(tag("//"), take_until("\n"))),
        )))),
    )(input)
}

fn required_whitespace(input: Span) -> IResult<Span, ()> {
    value(
        (),
        many1(complete(alt((
            value((), multispace1),
            value((), tuple((tag("/*"), take_until("*/"), tag("*/")))),
            value((), pair(tag("//"), take_until("\n"))),
        )))),
    )(input)
}

#[test]
fn whitespace_parses() {
    assert_eq!(
        optional_whitespace(LocatedSpan::new(" \n \n \r\n \t"))
            .unwrap()
            .0
            .to_string(),
        ""
    );
    assert_eq!(
        optional_whitespace(LocatedSpan::new(" // comment \n "))
            .unwrap()
            .0
            .to_string(),
        ""
    );
    assert_eq!(
        optional_whitespace(" /* comment */ abc".into())
            .unwrap()
            .0
            .to_string(),
        "abc"
    );
}

#[cfg(test)]
mod test {
    use super::Span;
    use nom::{error::Error, IResult};

    pub fn strip_span<O>(result: IResult<Span, O>) -> IResult<String, O> {
        result
            .map(|(span, result)| (span.to_string(), result))
            .map_err(|err| err.map(|err| Error::new(err.input.to_string(), err.code)))
    }
}
