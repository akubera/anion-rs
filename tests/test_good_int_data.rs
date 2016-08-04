extern crate anion;
extern crate pest;

use pest::prelude::*;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::path::{Path, PathBuf};

use anion::AnionValue;

const DATA_PATH_GOOD: &'static str = "ion-test-suite-data/iontestdata/good";
const DATA_PATH_BAD: &'static str = "ion-test-suite-data/iontestdata/good";

fn get_file_lines(filename: &str) -> Lines<BufReader<File>>
{
  let f = File::open(filename).unwrap();
  let mut reader = BufReader::new(f);
  return reader.lines();
}

#[test]
// #[ignore]
fn binary()
{
  println!("");

  let filename = Path::new(DATA_PATH_GOOD).join("intBinary.ion");

  let expected: [i32; 3] = [240, 21, -15];

  for (e, line) in expected.iter().zip(get_file_lines(filename.to_str().unwrap())) {
    let line = line.unwrap();
    println!(">> {}", line);
    let line = line.as_str();
    let mut parser = anion::Rdp::new(StringInput::new(line));
    assert!(parser.bin_int());
    let a_val = parser.int_value();
    assert_eq!(a_val, AnionValue::Integer(Some(*e)));
    match a_val {
      AnionValue::Integer(_) => (),
      _ => assert!(false),
    }
  }
  // let len = reader.read_line(&mut line).unwrap();
  //

  // for line in iterate_through_file().lines() {
  //   println!(">> {}", line.unwrap());
  // }
  // assert!(false);
  // let mut buffer = String::new();
  // try!(f.read_to_string(&mut buffer));
}
