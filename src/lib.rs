//! Rust implementation of Amazon's Ion data format
//!
//! There you go

#![recursion_limit = "197"]
#![feature(custom_attribute)]
#![allow(unreachable_code)]

#[macro_use]
extern crate pest;

extern crate num_bigint;
extern crate num_bigdecimal;
extern crate num_rational;

use num_bigint::BigInt;
use num_bigdecimal::BigDecimal;
use num_rational::Rational;

pub mod parser;

/// Enum of all possible types of elements in an ion document.
///
/// These are mapped to either rust literal types or other anion types.
/// All values are Options, with a None value corresponding to the
/// equivalent 'null' value of the ion document.
///
#[derive(Debug, PartialEq, Clone)]
pub enum AnionValue {
  /// Pure null type
  // Null,
  /// true, false, null.bool
  Boolean(Option<bool>),

  /// Bigint values (unlimited)
  Integer(Option<BigInt>),

  /// 64 bit floating point value
  Float(Option<f64>),

  /// Exact precision real number value
  Decimal(Option<BigDecimal>),

  /// string.
  String(Option<String>),
}

/// Variant of AnionValue enum that does not permit null values.
/// This includes the 'pure NULL' null value, though this may
/// be removed due to naming sillyness. This is much closer in type
/// to true JSON values.
///
#[derive(Debug, PartialEq, Clone)]
pub enum NonNullAnionValue {
  /// The NULL value - in the NonNullValue!
  Null,

  /// true, false
  Boolean(bool),

  /// Bigint values (unlimited)
  Integer(i32),

  /// 64 bit floating point value
  Float(f64),

  /// Exact precision real number value
  Decimal(Rational),

  /// String of utf8 characters
  String(String),
}

pub use parser::Rdp;

macro_rules! impl_int_conversion {
  ($int_type:ident) => {
    impl From<$int_type> for AnionValue {
      #[inline]
      fn from(int: $int_type) -> Self {
        AnionValue::Integer(Some(BigInt::from(int)))
      }
    }
  }
}

impl_int_conversion!(i8);
impl_int_conversion!(i16);
impl_int_conversion!(i32);
impl_int_conversion!(i64);
impl_int_conversion!(u8);
impl_int_conversion!(u16);
impl_int_conversion!(u32);
impl_int_conversion!(u64);
impl_int_conversion!(BigInt);


macro_rules! impl_float_conversion {
  ($float_type:ident) => {
    impl From<$float_type> for AnionValue {
      #[inline]
      fn from(float_val: $float_type) -> Self {
        AnionValue::Float(Some(float_val as f64))
      }
    }
  }
}

impl_float_conversion!(f32);
impl_float_conversion!(f64);



// impl From<i32> for AnionValue {
//   fn from(int: i32) -> AnionValue
//   {
//     AnionValue::Integer(Some(BigInt::from(int)))
//   }
// }
