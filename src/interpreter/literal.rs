use super::{Evaluates, Value};
use crate::parser::literal::Literal;

impl<'a> Evaluates<'a> for Literal {
    fn evaulate(self) -> Value<'a> {
        match self {
            Self::Boolean(value) => Value::Boolean(value.0),
            _ => todo!(),
        }
    }
}
