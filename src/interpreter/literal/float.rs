use super::super::{Type, Value};
use crate::parser::literal::{
    number::{float::Exponent, Sign},
    Float,
};

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
            .fold((1, 0.0), |(place_value, value), digit| {
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

impl From<Exponent> for f64 {
    fn from(value: Exponent) -> Self {
        let Exponent {
            whole,
            fractional,
            sign,
        } = value;

        let (_, whole_value) = whole
            .into_iter()
            .fold((1, 0.0), |(place_value, value), digit| {
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
