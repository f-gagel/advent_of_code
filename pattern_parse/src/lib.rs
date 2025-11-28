//! A crate to provide a macro to simplify parsing certain values from a larger string
//!
//! There currently only exists the [`parse_fn`] macro.

use std::{borrow::Cow, convert::Infallible, fmt::Display, num::ParseIntError, str::FromStr};

pub use pattern_parse_macros::parse_fn;

/// Core trait for parsable items
pub trait PatternParse: Sized {
    type Error: std::error::Error;

    /// Parses a string `s` to return a value of this type and the number of characters consumed.
    ///
    /// If the string is ill-formatted return an error specific to the
    /// inside [`Err`]. The error type is specific to the implementation of the trait.
    fn parse(input: &str) -> Result<(Self, usize), Self::Error>;
}

macro_rules! impl_uint_parse {
    ($($type:ty), *) => {
        $(
            impl PatternParse for $type {
                type Error = ParseIntError;
                fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
                    let len = input.chars().take_while(|c| c.is_numeric()).count();
                    let value = Self::from_str(&input[..len])?;
                    Ok((value, len))
                }
            }
        )*
    };
}

macro_rules! impl_int_parse {
    ($($type:ty), *) => {
        $(
            impl PatternParse for $type {
                type Error = ParseIntError;
                fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
                    let len = input.chars().enumerate().take_while(|(i,c)| {
                        if *i == 0 && (*c == '+' || *c == '-') {
                            true
                        } else {
                            c.is_numeric()
                        }
                    }).count();
                    let value = Self::from_str(&input[..len])?;
                    Ok((value, len))
                }
            }
        )*
    };
}

impl_uint_parse!(u8, u16, u32, u64, u128, usize);
impl_int_parse!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_float_parse {
    ($($type:ty), *) => {
        $(
            impl PatternParse for $type {
                type Error = <$type as FromStr>::Err;
                fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
                    let len = input.chars().take_while(|c| c.is_numeric()).count();
                    let value = Self::from_str(&input[..len])?;
                    Ok((value, len))
                }
            }
        )*
    };
}

impl_float_parse!(f32, f64);

impl PatternParse for char {
    type Error = Infallible;

    fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
        debug_assert!(input.is_ascii());

        Ok((input.chars().next().unwrap(), 1))
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub error: Box<dyn 'static + std::error::Error>,
    pub position: usize,
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.error)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parsing error at position {}: {}",
            self.position, self.error
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralMismatch {
    pub expected: Cow<'static, str>,
    pub got: String,
}

impl Display for LiteralMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mismatched literal expected:\"{}\" got:\"{}\"",
            self.expected, self.got
        )
    }
}

impl std::error::Error for LiteralMismatch {}
