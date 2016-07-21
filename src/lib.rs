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
  //Null,

  /// true, false, null.bool
  Boolean(Option<bool>),

  /// Bigint values (unlimited)  
  Integer(Option<i32>),

  /// 64 bit floating point value
  Float(Option<f64>),

  /// string.
  String(Option<String>),
}

pub use parser::Rdp;
