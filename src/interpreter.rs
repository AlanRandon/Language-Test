use std::{collections::HashMap, rc::Rc};

use crate::parser::literal::Expression;

mod expression;
mod literal;

pub trait Evaluates<'a> {
    fn evaulate(self, scope: Scope<'a>) -> Value;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    type_defintion: Type,
    data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Character,
    Function {
        return_type: Box<Self>,
        parameters: Vec<Self>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scope<'a> {
    variables: HashMap<&'a str, Value>,
    enclosing_scope: Option<Rc<Scope<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    BinaryOperation {
        rhs_type: Type,
        lhs_type: Type,
        detail: String,
    },
    UnaryOperation {
        data_type: Type,
        detail: String,
    },
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BinaryOperation {
                rhs_type,
                lhs_type,
                detail,
            } => {
                write!(
                    f,
                    "Cannot perform binary operation on {:?} and {:?}: {}",
                    rhs_type, lhs_type, detail
                )
            }
            _ => unimplemented!(),
        }
    }
}
