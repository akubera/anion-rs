
#[macro_use]
extern crate pest;
extern crate anion;

use anion::AnionValue;
use anion::parser::parse_string;

fn main()
{
  for s in std::env::args().skip(1) {
    let anion_value = parse_string(s.as_str());

    if let Some(val) = anion_value {
      match val {
        AnionValue::Boolean(Some(x)) => println!("Bool {}", x),
        AnionValue::Boolean(None) => println!("Bool NULL"),
        AnionValue::Integer(Some(x)) => println!("Int {}", x),
        AnionValue::Integer(None) => println!("Int NULL"),
        AnionValue::Float(Some(x)) => println!("Float {}", x),
        AnionValue::Float(None) => println!("Float NULL"),
        AnionValue::Decimal(Some(x)) => println!("Decimal {}", x),
        AnionValue::Decimal(None) => println!("Decimal NULL"),
        AnionValue::String(Some(x)) => println!("String {}", x),
        AnionValue::String(None) => println!("String NULL"),
      }
    }
  }
}
