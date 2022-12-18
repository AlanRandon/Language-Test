pub struct Number {
    base: Base,
    whole: Vec<u8>,
    fractional: Vec<u8>,
}

pub enum Base {
    Decimal,
    Hexadecimal,
    Binary,
    Octal,
}
