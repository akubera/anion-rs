//! [\file]: # (tests/common/mod.rs)
//!
//! Common routines for test module


use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;
extern crate anion;
use pest::prelude::*;
use anion::AnionValue;

pub fn good_filename(filename: &str) -> String
{
  let filename = Path::new("ion-test-suite-data/iontestdata/good").join(filename);
  return String::from(filename.to_str().unwrap());
}

#[allow(dead_code)]
pub fn bad_filename(filename: &str) -> String
{
  let filename = Path::new("ion-test-suite-data/iontestdata/bad").join(filename);
  return String::from(filename.to_str().unwrap());
}

pub fn get_file_lines(filename: String) -> Lines<BufReader<File>>
{
  let f = File::open(filename.as_str()).unwrap();
  let reader = BufReader::new(f);
  return reader.lines();
}

/// Loads
pub fn get_equivs(filename: &str) -> Vec<Vec<String>>
{
  let mut result = Vec::new();
  let equiv_dir = Path::new("equivs").join(filename);
  let fname = equiv_dir.to_str().unwrap();
  let mut do_store = false;
  let lines = get_file_lines(good_filename(fname));

  for line in lines {
    let x = line.unwrap();
    if x == "(" {
      do_store = true;
      result.push(Vec::new());
    } else if x == ")" {
      do_store = false;
    } else if x == "" {
      continue;
    } else if do_store {
      result.last_mut().unwrap().push(String::from(x));
    }

  }
  return result;
}

#[allow(dead_code)]
pub fn parse_value(src: &String) -> AnionValue
{
  println!("> '{}'", src);
  let mut parser = anion::Rdp::new(StringInput::new(src));
  assert!(parser.ion());
  return parser.ion_value();
}
