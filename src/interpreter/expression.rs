use super::{Evaluates, Value};
use crate::parser::{expression::Expression, literal::Literal};

impl<'a> Evaluates<'a> for Expression<'a> {
    fn evaulate(self) -> Value<'a> {
        match self {
            Self::Literal(literal) => literal.evaulate(),
            _ => todo!(),
        }
    }
}
