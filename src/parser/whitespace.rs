use super::prelude::*;

pub fn optional(input: Span) -> IResult<Span, ()> {
    value(
        (),
        many0(complete(alt((
            value((), multispace1),
            value((), tuple((tag("/*"), take_until("*/"), tag("*/")))),
            value((), pair(tag("//"), take_until("\n"))),
        )))),
    )(input)
}

pub fn required(input: Span) -> IResult<Span, ()> {
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
        optional(LocatedSpan::new(" \n \n \r\n \t"))
            .unwrap()
            .0
            .to_string(),
        ""
    );
    assert_eq!(
        optional(LocatedSpan::new(" // comment \n "))
            .unwrap()
            .0
            .to_string(),
        ""
    );
    assert_eq!(
        optional(" /* comment */ abc".into()).unwrap().0.to_string(),
        "abc"
    );
}
