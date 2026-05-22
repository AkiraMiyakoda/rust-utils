// Copyright © 2026 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::Write;

use compact_str::format_compact;
use rust_decimal::prelude::*;

pub struct WithCommas(f64);

impl<T> From<T> for WithCommas
where
    T: ToPrimitive,
{
    fn from(value: T) -> Self {
        Self(value.to_f64().unwrap_or(f64::NAN))
    }
}

impl std::fmt::Display for WithCommas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_nan() {
            return write!(f, "NaN");
        }
        if self.0.is_infinite() {
            return write!(f, "inf");
        }

        let str = format_compact!("{value:.digits$}", value = self.0, digits = f.precision().unwrap_or(0));
        let point_index = str.find('.').unwrap_or(str.len());
        let min_index = if str.starts_with('-') || str.ends_with('+') {
            1
        } else {
            0
        };

        if self.0.is_sign_positive() && f.sign_plus() {
            f.write_char('+')?;
        }

        for (i, c) in str.chars().enumerate() {
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
        format!("{:.0}", WithCommas::from(Decimal::from_str_exact("-1234").unwrap())),
        "-1,234"
    );
    assert_eq!(format!("{:+.0}", WithCommas::from(f64::NAN)), "NaN");
    assert_eq!(format!("{:+.0}", WithCommas::from(f64::INFINITY)), "inf");
    assert_eq!(format!("{:+.0}", WithCommas::from(f64::NEG_INFINITY)), "inf");
}
