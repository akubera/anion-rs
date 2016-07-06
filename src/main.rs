#![recursion_limit = "80"]

#[macro_use]
extern crate pest;
use pest::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AzionType {
  // Boolean(Option<bool>),
  Boolean(Option<bool>),
  Integer(Option<i32>),
}

#[allow(unreachable_code)]
impl_rdp! {
    grammar! {
        boolean  = { ["true"] | ["false"] | ["null.bool"] }
        int      = { ["-"]? ~ ["0"] | ["-"]? ~ ['1'..'9'] ~ ['0'..'9']* }
        null_int = { ["null.int"] }

/*
        json = { value ~ eoi }

        object = { ["{"] ~ pair ~ ([","] ~ pair)* ~ ["}"] | ["{"] ~ ["}"] }
        pair   = { string ~ [":"] ~ value }

        array = { ["["] ~ value ~ ([","] ~ value)* ~ ["]"] | ["["] ~ ["]"] }

        value = { boolean | string | number | object | array | null }
        null = @{ ["null.null"] | ["null"]}


        string  = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        escape  =  { ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["b"] | ["f"] | ["n"] | ["r"] | ["t"] | unicode) }
        unicode =  { ["u"] ~ hex ~ hex ~ hex ~ hex }
        hex     =  { ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }

        number = @{ ["-"]? ~ int ~ (["."] ~ ['0'..'9']+ ~ exp? | exp)? }


        exp    =  { (["E"] | ["e"]) ~ (["+"] | ["-"])? ~ int }

        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
*/    }

    process! {
      int_value(&self) -> AzionType {
          // capture int token
        (&int_token: int) => {
            let result: i32 = int_token.parse().unwrap();
            return AzionType::Integer(Some(result));
        },

        (&null_int_token: null_int) => {
            assert_eq!(null_int_token, "null.int");
            return AzionType::Integer(None);
        }
      }

      boolean_value(&self) -> AzionType {
          (&bool_token: boolean) => {
              let result = if bool_token != "null.bool" {
                  Some(bool_token.parse::<bool>().unwrap())
              } else {
                  None
              };
              return AzionType::Boolean(result);
          }
      }
    }
}


fn main()
{
  for s in std::env::args().skip(1) {
    let mut parser = Rdp::new(StringInput::new(s.as_str()));

    if parser.int() || parser.null_int() {
      match parser.int_value() {
        AzionType::Integer(Some(x)) => println!("> {} ", x),
        AzionType::Integer(None) => println!("> NULL Integer"),
        _ => {},
      }
    } else if parser.boolean() {
      match parser.boolean_value() {
        AzionType::Boolean(Some(x)) => println!("? {} ", x),
        AzionType::Boolean(None) => println!("? NULL boolean"),
        _ => {},
      }
    }
    println!("{:?}", parser.queue());

    // parser.boolean_value();
    // match parser.boolean_value() {
    //   AzionType::Boolean(Some(x)) => println!("Bool {}", x),
    //   AzionType::Boolean(None) => println!("Bool NULL!"),
    // }
    // println!("{}", );
    // println!(">> {}", parser.json())

  }
}

#[test]
fn parsing_int_works()
{
  let cases = [("90", 90), ("101010", 101010), ("-42", -42)];
  for &(s, ex) in cases.iter() {
    let mut parser = Rdp::new(StringInput::new(s));
    assert!(parser.int());
    match parser.int_value() {
      AzionType::Integer(Some(x)) => assert_eq!(x, ex),
      _ => unreachable!(),
    }
    assert!(parser.end());
  }
}

#[test]
fn parsing_invalid_values()
{
  let mut parser = Rdp::new(StringInput::new("_90"));

  assert!(!parser.int());
  // assert_eq!(parser.int_value(), Some(-90));
  // assert!(parser.end());
}

// macro_rules! valid_int_test {
// ($($name:expr, $value:expr,)*) => {
// #[test]
// fn test_name_parses() {
// let mut parser = Rdp::new(StringInput::new($name));
//
// assert!(parser.int());
// assert_eq!(parser.int_value(), Some($value));
// assert!(parser.end());
// }
// }
// }
//
// valid_int_test!("9_0", 90);
//
