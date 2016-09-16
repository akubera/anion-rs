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
  println!("> '{}'", src);
  let mut parser = anion::Rdp::new(StringInput::new(src.trim_left()));
  assert!(parser.float());
  let val = parser.float_value();
  match &val {
    &AnionValue::Float(Some(ref x)) => println!("  -> {}", x),
    _ => println!("  -> None"),
  }
  return val;
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
fn good_floats()
{

  let filename = good_filename("float_values.ion");

  // let expected: [i32; 3] = [240, 21, -15];

  for line in get_file_lines(filename) {
    let line = line.unwrap();
    let line = line.as_str();


    let expected: f64 = line.parse().unwrap();
    println!("'{}' => {}", line, expected);

    let mut parser = anion::Rdp::new(StringInput::new(line));
    assert!(parser.float());

    let a_val = parser.float_value();
    assert_eq!(a_val, AnionValue::from(expected));

    match a_val {
      AnionValue::Float(_) => (),
      _ => assert!(false),
    }
  }
}


#[test]
fn test_equiv_floats()
{
  let eqiv_vec = get_eqivs("floats.ion");
  for a in eqiv_vec {
    let mut i = a.iter();
    let f = i.next().unwrap();
    println!(">> {}", f);
    let x = parse_value(f);
    for b in i {
      let y = parse_value(b);
      println!("{} ?= {}", f, b);
      assert_eq!(x, y);
    }
  }
}
