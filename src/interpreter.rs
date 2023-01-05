use crate::parser::prelude::Expression;
use std::collections::HashMap;

mod expression;
mod literal;

pub trait Evaluates<'a> {
    fn evaulate(self) -> Value<'a>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Function {
        type_defintion: Type,
        expression: Expression<'a>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Function {
        return_type: Box<Type>,
        parameters: Vec<Type>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context<'a> {
    variables: HashMap<&'a str, Value<'a>>,
}
