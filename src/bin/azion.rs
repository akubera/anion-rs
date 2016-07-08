
#[macro_use]
extern crate pest;
extern crate azion;

use azion::AzionValue;
use azion::parser::parse_string;

fn main()
{
  for s in std::env::args().skip(1) {
    let azion_value = parse_string(s.as_str());

    if let Some(val) = azion_value {
      match val {
        AzionValue::Boolean(Some(x)) => println!("Bool {}", x),
        AzionValue::Boolean(None) => println!("Bool NULL"),
        AzionValue::Integer(Some(x)) => println!("Int {}", x),
        AzionValue::Integer(None) => println!("Int NULL"),
        AzionValue::Float(Some(x)) => println!("Float {}", x),
        AzionValue::Float(None) => println!("Float NULL"),
      }
    }
  }
}
