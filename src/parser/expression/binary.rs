use super::Expression;
use super::{super::Span, pratt::Terms};
use nom::{branch::alt, bytes::streaming::tag, combinator::value, IResult};

// A binary operation such as addition or subtraction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: Operator,
    pub right: Box<Expression<'a>>,
}

impl<'a> Binary<'a> {
    pub fn parse(input: Span) -> IResult<Span, Expression> {
        let (input, terms) = Terms::parse(input)?;
        Ok((input, terms.reduce()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,
    And,
    Or,
    Xor,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    NotEqual,
}

impl Operator {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        alt((
            value(Self::Add, tag("+")),
            value(Self::Subtract, tag("-")),
            value(Self::Multiply, tag("*")),
            value(Self::Divide, tag("/")),
            value(Self::Exponent, tag("**")),
            value(Self::Modulo, tag("%")),
            value(Self::And, tag("&&")),
            value(Self::Or, tag("||")),
            value(Self::Xor, tag("^")),
            value(Self::Equal, tag("==")),
            value(Self::LessThan, tag("<")),
            value(Self::LessThanOrEqual, tag("<=")),
            value(Self::GreaterThan, tag(">")),
            value(Self::GreaterThanOrEqual, tag(">=")),
            value(Self::NotEqual, tag("!=")),
        ))(input)
    }

    // The ability of a operator to 'bind' to a term
    pub const fn binding_powers(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Subtract => (10, 15),
            Self::Multiply | Self::Divide | Self::Modulo => (20, 25),
            Self::Exponent => (35, 30),
            Self::LessThan
            | Self::LessThanOrEqual
            | Self::GreaterThan
            | Self::GreaterThanOrEqual => (40, 45),
            Self::Equal | Self::NotEqual => (50, 55),
            Self::Xor => (60, 65),
            Self::And => (70, 75),
            Self::Or => (80, 85),
        }
    }
}
