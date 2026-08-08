// Copyright © 2026 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::Write;

use compact_str::CompactString;
use compact_str::ToCompactString;
use compact_str::format_compact;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

pub struct WithCommas<T>(T);

pub struct WithCommasPrimitive<T>(T)
where
    T: Copy + ToPrimitive;

impl<T> WithCommas<T> {
    pub fn primitive(value: T) -> WithCommasPrimitive<T>
    where
        T: Copy + ToPrimitive,
    {
        WithCommasPrimitive(value)
    }
}

impl<T> From<T> for WithCommas<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<&T> for WithCommas<T>
where
    T: Copy,
{
    fn from(value: &T) -> Self {
        Self(*value)
    }
}

impl<T> From<T> for WithCommasPrimitive<T>
where
    T: Copy + ToPrimitive,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<&T> for WithCommasPrimitive<T>
where
    T: Copy + ToPrimitive,
{
    fn from(value: &T) -> Self {
        Self(*value)
    }
}

impl std::fmt::Display for WithCommas<Decimal> {
    #[allow(clippy::cast_possible_truncation)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digits = f.precision().unwrap_or(0);
        let rounded = self.0.round_dp(digits as u32);
        let text = format_decimal(rounded, digits);

        write_commas(f, &text, rounded.is_sign_negative(), f.sign_plus())
    }
}

impl<T> std::fmt::Display for WithCommasPrimitive<T>
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

        write_commas(f, &text, value.is_sign_negative(), f.sign_plus())
    }
}

fn format_decimal(value: Decimal, digits: usize) -> CompactString {
    let mut text = value.to_compact_string();

    if digits == 0 {
        return text;
    }

    if let Some(point_index) = text.find('.') {
        let current_digits = text.len() - point_index - 1;

        for _ in current_digits..digits {
            text.push('0');
        }
    } else {
        text.push('.');

        for _ in 0..digits {
            text.push('0');
        }
    }

    text
}

fn write_commas(f: &mut std::fmt::Formatter<'_>, text: &str, is_negative: bool, sign_plus: bool) -> std::fmt::Result {
    let point_index = text.find('.').unwrap_or(text.len());
    let min_index = usize::from(text.starts_with('-'));

    let mut output = CompactString::with_capacity(text.len() + text.len() / 3 + 1);

    if !is_negative && sign_plus {
        output.push('+');
    }

    for (i, c) in text.chars().enumerate() {
        if i > min_index && i < point_index && (point_index - i).is_multiple_of(3) {
            output.push(',');
        }

        output.push(c);
    }

    let width = f.width().unwrap_or(0);
    let padding = width.saturating_sub(output.chars().count());

    match f.align() {
        Some(std::fmt::Alignment::Left) => {
            f.write_str(&output)?;
            for _ in 0..padding {
                f.write_char(f.fill())?;
            }
        }

        Some(std::fmt::Alignment::Center) => {
            let left = padding / 2;
            let right = padding - left;

            for _ in 0..left {
                f.write_char(f.fill())?;
            }

            f.write_str(&output)?;

            for _ in 0..right {
                f.write_char(f.fill())?;
            }
        }

        Some(std::fmt::Alignment::Right) | None => {
            for _ in 0..padding {
                f.write_char(f.fill())?;
            }

            f.write_str(&output)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::WithCommas;

    #[test]
    fn test_decimal() {
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

        assert_eq!(
            format!("{:.2}", WithCommas::from(Decimal::from_str_exact("0").unwrap())),
            "0.00"
        );

        assert_eq!(
            format!("{:+.2}", WithCommas::from(Decimal::from_str_exact("0").unwrap())),
            "+0.00"
        );

        assert_eq!(
            format!(
                "{:.0}",
                WithCommas::from(Decimal::from_str_exact("9007199254740993").unwrap())
            ),
            "9,007,199,254,740,993"
        );
    }

    #[test]
    fn test_primitive() {
        assert_eq!(format!("{:.2}", WithCommas::primitive(1234.5)), "1,234.50");

        assert_eq!(format!("{:.0}", WithCommas::primitive(1234567890)), "1,234,567,890");

        assert_eq!(format!("{:.0}", WithCommas::primitive(f64::NAN)), "NaN");
        assert_eq!(format!("{:.0}", WithCommas::primitive(f64::INFINITY)), "inf");
        assert_eq!(format!("{:+.0}", WithCommas::primitive(f64::INFINITY)), "+inf");
        assert_eq!(format!("{:+.0}", WithCommas::primitive(f64::NEG_INFINITY)), "-inf");
    }

    #[test]
    fn test_alignment() {
        let value = Decimal::from_str_exact("1234567.89").unwrap();

        assert_eq!(format!("{:>20.2}", WithCommas::from(value)), "        1,234,567.89");
        assert_eq!(format!("{:<20.2}", WithCommas::from(value)), "1,234,567.89        ");
        assert_eq!(format!("{:^20.2}", WithCommas::from(value)), "    1,234,567.89    ");
        assert_eq!(format!("{:*>20.2}", WithCommas::from(value)), "********1,234,567.89");
    }
}
