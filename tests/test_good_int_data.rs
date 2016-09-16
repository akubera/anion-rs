extern crate anion;
extern crate pest;

use anion::AnionValue;
use pest::prelude::*;

mod common;
use common::{get_file_lines, good_filename, get_equivs};

fn parse_value(src: &String) -> AnionValue
{
  println!("> '{}'", src);
  let mut parser = anion::Rdp::new(StringInput::new(src.trim_left()));
  assert!(parser.bin_int() || parser.hex_int() || parser.int());
  let int_val = parser.int_value();
  match &int_val {
    &AnionValue::Integer(Some(ref x)) => println!("  -> {}", x),
    _ => println!("  -> None"),
  }
  return int_val;
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
  let eqiv_vec = get_equivs("ints.ion");
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


#[test]
#[allow(non_snake_case)]
fn test_equiv_bigInts()
{
  let eqiv_vec = get_equivs("bigInts.ion");
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
  let eqiv_vec = get_equivs("intsWithUnderscores.ion");
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
