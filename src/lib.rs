//! Rust implementation of Amazon's Ion data format
//!
//! There you go

#![recursion_limit = "197"]
#![feature(custom_attribute)]
#![allow(unreachable_code)]

#[macro_use]
extern crate pest;

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
  Integer(Option<i32>),

  /// 64 bit floating point value
  Float(Option<f64>),

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

  /// String.
  String(String),
}

pub use parser::Rdp;


impl From<i32> for AnionValue {
  fn from(int: i32) -> AnionValue
  {
    AnionValue::Integer(Some(int))
  }
}
