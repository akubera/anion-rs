extern crate anion;
extern crate pest;
extern crate num_bigdecimal;

use pest::prelude::*;
use anion::AnionValue;
use num_bigdecimal::BigDecimal;
use std::str::FromStr;

mod common;

use common::{get_file_lines, good_filename, get_equivs};

fn parse_value(src: &String) -> AnionValue
{
  println!("> '{}'", src);
  let mut parser = anion::Rdp::new(StringInput::new(src.trim_left()));
  assert!(parser.decimal());
  let val = parser.decimal_value();
  match &val {
    &AnionValue::Decimal(Some(ref x)) => println!("  -> {}", x),
    _ => println!("  -> None"),
  }
  return val;
}


#[test]
fn good_decimal()
{

  let filename = good_filename("decimal_values.ion");

  for line in get_file_lines(filename) {
    let line = line.unwrap();

    let formatted_string = line.replace("d", "e").replace("D", "e");

    let line = line.as_str();


    let expected = BigDecimal::from_str(formatted_string.as_str()).unwrap();
    println!("'{}' => {}", line, expected);

    let mut parser = anion::Rdp::new(StringInput::new(line));
    assert!(parser.decimal());

    let a_val = parser.decimal_value();
    assert_eq!(a_val, AnionValue::Decimal(Some(expected)));

    match a_val {
      AnionValue::Decimal(_) => (),
      _ => assert!(false),
    }
  }
}


#[test]
fn test_equiv_decimals()
{
  let eqiv_vec = get_equivs("decimals.ion");
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
fn test_equiv_decimals_with_underscores()
{
  for a in get_equivs("decimalsWithUnderscores.ion") {
    let mut i = a.iter();
    let f = i.next().unwrap();
    let x = parse_value(f);
    for b in i {
      let y = parse_value(b);
      assert_eq!(x, y);
    }
  }
}

#[test]
fn test_zero_decimals()
{
  let eqiv_vec = get_equivs("zeroDecimals.ion");
  for a in eqiv_vec {
    let mut i = a.iter();
    let f = String::from(i.next().unwrap().trim_left());
    println!("=> '{}'", f);
    let x = parse_value(&f);
    for b in i {
      let b = String::from(b.trim_left());
      println!("~> '{}'", b);
      let y = parse_value(&b);
      println!("{} ?= {}", f, b);
      assert_eq!(x, y);
    }
  }
}
