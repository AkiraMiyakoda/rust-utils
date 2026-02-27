// Copyright © 2026 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use core::f64;

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
        let mut str = format!(
            "{value:.digits$}",
            value = self.0,
            digits = f.precision().unwrap_or(0)
        );
        let mut index = str.find('.').unwrap_or(str.len());
        let min_index = if str.starts_with('-') || str.starts_with('+') {
            4
        } else {
            3
        };

        loop {
            if index <= min_index {
                break;
            }

            index -= 3;
            str.insert(index, ',');
        }
        if self.0.is_sign_positive() && f.sign_plus() {
            str.insert(0, '+');
        }

        write!(f, "{str}")
    }
}

#[test]
fn test() {
    assert_eq!(
        format!(
            "{:.3}",
            WithCommas::from(Decimal::from_str_exact("-12345.67").unwrap())
        ),
        "-12,345.670"
    );
}
