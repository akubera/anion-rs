extern crate anion;
extern crate pest;

use pest::prelude::*;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::path::Path;

use anion::AnionValue;

fn parse_value(src: &String) -> AnionValue
{
  let mut parser = anion::Rdp::new(StringInput::new(src));
  assert!(parser.bin_int() || parser.hex_int() || parser.int());
  return parser.int_value();
}

fn good_filename(filename: &str) -> String
{
  let filename = Path::new("ion-test-suite-data/iontestdata/good").join(filename);
  return String::from(filename.to_str().unwrap());
}

fn get_file_lines(filename: String) -> Lines<BufReader<File>>
{
  let f = File::open(filename.as_str()).unwrap();
  let reader = BufReader::new(f);
  return reader.lines();
}

/// Loads
fn get_eqivs(filename: &str) -> Vec<Vec<String>>
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

#[test]
fn good_binary()
{

  let filename = good_filename("intBinary.ion");

  let expected: [i32; 3] = [240, 21, -15];

  for (e, line) in expected.iter().zip(get_file_lines(filename)) {
    let line = line.unwrap();
    println!(">> {}", line);
    let line = line.as_str();
    let mut parser = anion::Rdp::new(StringInput::new(line));
    assert!(parser.bin_int());
    let a_val = parser.int_value();
    assert_eq!(a_val, AnionValue::from(*e));
    match a_val {
      AnionValue::Integer(_) => (),
      _ => assert!(false),
    }
  }
}


#[test]
fn test_equiv_ints()
{
  let eqiv_vec = get_eqivs("ints.ion");
  for a in eqiv_vec {
    let mut i = a.iter();
    let f = i.next().unwrap();
    let x = parse_value(f);
    for b in i {
      let y = parse_value(b);
      println!("{} ?= {}", f, b);
      assert_eq!(x, y);
    }
  }
}


#[test]
#[allow(non_snake_case)]
fn test_equiv_bigInts()
{
  let eqiv_vec = get_eqivs("bigInts.ion");
  for a in eqiv_vec {
    let mut i = a.iter();
    let f = i.next().unwrap();
    let x = parse_value(f);

    for b in i {
      println!("{} ?= {}", b, f);
      let y = parse_value(b);
      assert_eq!(x, y);
    }
  }
}

#[test]
#[allow(non_snake_case)]
fn test_equiv_intsWithUnderscores()
{
  let eqiv_vec = get_eqivs("intsWithUnderscores.ion");
  for a in eqiv_vec {
    let mut i = a.iter();
    let f = i.next().unwrap();
    let x = parse_value(f);

    for b in i {
      println!("{} ?= {}", b, f);
      let y = parse_value(b);
      assert_eq!(x, y);
    }
  }
}
