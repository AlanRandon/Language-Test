pub mod expression;
pub mod function;
pub mod identifier;
pub mod let_in;
pub mod literal;
pub mod types;
pub mod whitespace;

pub mod prelude {
    #[cfg(test)]
    pub use super::test;
    pub use super::{
        expression::{self, Expression},
        function::Function,
        identifier::Identifier,
        let_in::LetIn,
        literal::{self, number, Literal},
        types::Type,
        whitespace,
    };
    pub use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while},
        character::complete::{multispace1, one_of, satisfy},
        combinator::{complete, consumed, not, opt, value},
        error::context,
        multi::{many0, many1, separated_list0},
        sequence::{delimited, pair, terminated, tuple},
        IResult, Parser,
    };
    pub use nom_locate::LocatedSpan;

    pub type Span<'a> = LocatedSpan<&'a str>;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use nom::{error::Error, IResult};

    pub fn strip_span<O>(result: IResult<Span, O>) -> IResult<String, O> {
        result
            .map(|(span, result)| (span.to_string(), result))
            .map_err(|err| err.map(|err| Error::new(err.input.to_string(), err.code)))
    }
}
