extern crate anion;
extern crate pest;

use pest::prelude::*;
use anion::AnionValue;

mod common;

use common::{get_file_lines, good_filename, get_equivs};


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
  let eqiv_vec = get_equivs("floats.ion");
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
