use super::super::{Type, Value};
use crate::parser::prelude::*;
use literal::Float;
use number::{float::Exponent, Sign};

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Self {
            type_defintion: Type::Float,
            data: f64::from(value).to_be_bytes().to_vec(),
        }
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        let Float {
            base,
            whole,
            fractional,
            sign,
            exponent,
        } = value;

        let base = base as u8;

        let (_, whole_value) = whole
            .into_iter()
            .fold((0, 0.0), |(place_value, value), digit| {
                (
                    place_value + 1,
                    Self::from(digit).mul_add(Self::from(base).powi(place_value), value),
                )
            });

        let (_, fractional_value) =
            fractional
                .into_iter()
                .fold((-1, 0.0), |(place_value, value), digit| {
                    (
                        place_value - 1,
                        Self::from(digit).mul_add(Self::from(base).powi(place_value), value),
                    )
                });

        let exponent = exponent.map_or(1.0, |exponent| 10f64.powf(exponent.into()));

        (whole_value + fractional_value).copysign(match sign {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        }) * exponent
    }
}

#[test]
#[allow(clippy::float_cmp)]
fn float_evaulates() {
    assert_eq!(
        <Float as std::convert::Into<f64>>::into(Float {
            base: number::Base::Decimal,
            whole: vec![1],
            fractional: Vec::new(),
            sign: Sign::Positive,
            exponent: None,
        }),
        1.0f64
    );

    assert_eq!(
        <Float as std::convert::Into<f64>>::into(Float {
            base: number::Base::Hexadecimal,
            whole: vec![0xf, 0xf],
            fractional: vec![0x8],
            sign: Sign::Negative,
            exponent: None,
        }),
        -255.5f64
    );

    assert_eq!(
        <Float as std::convert::Into<f64>>::into(Float {
            base: number::Base::Hexadecimal,
            whole: vec![1],
            fractional: Vec::new(),
            sign: Sign::Positive,
            exponent: Some(Exponent {
                whole: vec![3],
                fractional: Vec::new(),
                sign: Sign::Positive
            }),
        }),
        1000f64
    );
}

impl From<Exponent> for f64 {
    fn from(value: Exponent) -> Self {
        let Exponent {
            whole,
            fractional,
            sign,
        } = value;

        let (_, whole_value) = whole
            .into_iter()
            .fold((0, 0.0), |(place_value, value), digit| {
                (
                    place_value + 1,
                    Self::from(digit).mul_add(10f64.powi(place_value), value),
                )
            });

        let (_, fractional_value) =
            fractional
                .into_iter()
                .fold((-1, 0.0), |(place_value, value), digit| {
                    (
                        place_value - 1,
                        Self::from(digit).mul_add(10f64.powi(place_value), value),
                    )
                });

        (whole_value + fractional_value).copysign(match sign {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        })
    }
}
