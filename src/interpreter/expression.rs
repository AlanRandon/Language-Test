use super::{Evaluates, Scope, Value};
use crate::parser::{expression::Expression, literal::Literal};

impl<'a> Evaluates<'a> for Expression<'a> {
    fn evaulate(self, scope: Scope) -> Value {
        match self {
            Self::Literal(literal) => literal.evaulate(scope),
            _ => todo!(),
        }
    }
}
