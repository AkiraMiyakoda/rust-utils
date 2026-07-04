// Copyright © 2026 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::Write;

use compact_str::format_compact;
use rust_decimal::prelude::*;

pub struct WithCommas<T>(T)
where
    T: Copy + ToPrimitive;

impl<T> From<T> for WithCommas<T>
where
    T: Copy + ToPrimitive,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<&T> for WithCommas<T>
where
    T: Copy + ToPrimitive,
{
    fn from(value: &T) -> Self {
        Self(*value)
    }
}

impl<T> std::fmt::Display for WithCommas<T>
where
    T: Copy + ToPrimitive,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0.to_f64().unwrap_or(f64::NAN);
        if value.is_nan() {
            return write!(f, "NaN");
        }
        if value.is_infinite() {
            return if value.is_sign_negative() {
                write!(f, "-inf")
            } else if f.sign_plus() {
                write!(f, "+inf")
            } else {
                write!(f, "inf")
            };
        }

        let text = format_compact!("{value:.digits$}", digits = f.precision().unwrap_or(0));
        let point_index = text.find('.').unwrap_or(text.len());
        let min_index = if text.starts_with('-') { 1 } else { 0 };

        if value.is_sign_positive() && f.sign_plus() {
            f.write_char('+')?;
        }

        for (i, c) in text.chars().enumerate() {
            if i > min_index && i < point_index && (point_index - i).is_multiple_of(3) {
                f.write_char(',')?;
            }

            f.write_char(c)?;
        }

        Ok(())
    }
}

#[test]
fn test() {
    assert_eq!(
        format!(
            "{:.3}",
            WithCommas::from(Decimal::from_str_exact("-1234567.89").unwrap())
        ),
        "-1,234,567.890"
    );
    assert_eq!(
        format!(
            "{:+.0}",
            WithCommas::from(Decimal::from_str_exact("1234567.89").unwrap())
        ),
        "+1,234,568"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("1").unwrap())),
        "1"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("12").unwrap())),
        "12"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("123").unwrap())),
        "123"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("1234").unwrap())),
        "1,234"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("-1").unwrap())),
        "-1"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("-12").unwrap())),
        "-12"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("-123").unwrap())),
        "-123"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(&Decimal::from_str_exact("-123").unwrap())),
        "-123"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("-1234").unwrap())),
        "-1,234"
    );
    assert_eq!(
        format!("{:.0}", WithCommas::from(&Decimal::from_str_exact("-1234").unwrap())),
        "-1,234"
    );
    assert_eq!(format!("{:.0}", WithCommas::from(f64::INFINITY)), "inf");
    assert_eq!(format!("{:+.0}", WithCommas::from(f64::INFINITY)), "+inf");
    assert_eq!(format!("{:+.0}", WithCommas::from(f64::NEG_INFINITY)), "-inf");
}
