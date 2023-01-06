use super::{Evaluates, Scope, Type, Value};
use crate::parser::literal::Literal;

mod float;

impl<'a> Evaluates<'a> for Literal {
    fn evaulate(self, _scope: Scope) -> Value {
        match self {
            Self::Boolean(value) => Value {
                type_defintion: Type::Boolean,
                data: vec![u8::from(value.0)],
            },
            Self::Character(value) => Value {
                type_defintion: Type::Character,
                data: (value.0 as u32).to_be_bytes().to_vec(),
            },
            Self::Float(value) => value.into(),
            _ => todo!(),
        }
    }
}
