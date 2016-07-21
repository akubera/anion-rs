//! Rust implementation of Amazon's Ion data format
//!
//! There you go

#![recursion_limit = "197"]
#![feature(custom_attribute)]
#![allow(unreachable_code)]

#[macro_use]
extern crate pest;

pub mod parser;


#[derive(Debug, PartialEq, Clone)]
pub enum AnionValue {
  Boolean(Option<bool>),
  Integer(Option<i32>),
  Float(Option<f64>),
  String(Option<String>),
}

pub use parser::Rdp;
