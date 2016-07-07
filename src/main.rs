#![recursion_limit = "80"]
#![feature(custom_attribute)]
#[cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
extern crate pest;
use pest::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum AzionValue {
  Boolean(Option<bool>),
  Integer(Option<i32>),
  Float(Option<f64>),
}

#[allow(unreachable_code)]
impl_rdp! {
  grammar! {

    whitespace = _{ [" "] | ["\t"] }

    plus_or_minus = {["-"] | ["+"]}
    digit = {['0'..'9']}
    digits = { digit+ }
    nz_digit = {['1'..'9']}

    // literal boolean values
    boolean = { ["true"] | ["false"] | ["null.bool"] }

    // null float value
    null_float = { ["null.float"] }

    float = @{
          plus_or_minus? // all floats may start with optional '+' or '-'
          ~(
              // non-zero followed by optional digits, non-optional
              // decimal point, and more optional digits
              nz_digit ~ digit* ~ ["."] ~ digit*

              // decimal followed by digits
           |  ["."] ~ digits

              // zero and decimal, followed by optional digits
           |  ["0."] ~ digit*
           )
          }

    null_int = @{ ["null.int"] }
    int = @{
        ["-"]? // ints may start with optional minus
        ~(
            // non-zero digit, followed by multiple digits
            //  - optional single underscores may split digits
            nz_digit ~ (["_"] ~ digits | digits)*

            // single zero
            // - force no decimal afterwards for help lexing floats
         |  ["0"] ~ !["."]
       )
      }


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
      int_value(&self) -> AzionValue {
          // capture int token
        (&int_token: int) => {
            let foo = int_token.replace("_", "");
            let result: i32 = foo.parse().unwrap();
            return AzionValue::Integer(Some(result));
        },

        (&null_int_token: null_int) => {
            assert_eq!(null_int_token, "null.int");
            return AzionValue::Integer(None);
        }
      }

      float_value(&self) -> AzionValue {
          // capture int token
        (&float_token: float) => {
            let foo = float_token.replace("_", "");
            let result = foo.parse().unwrap();
            return AzionValue::Float(Some(result));
        },

        (&null_float_token: null_float) => {
            assert_eq!(null_float_token, "null.float");
            return AzionValue::Float(None);
        }
      }

      boolean_value(&self) -> AzionValue {
          (&bool_token: boolean) => {
              let result = if bool_token != "null.bool" {
                  Some(bool_token.parse::<bool>().unwrap())
              } else {
                  None
              };
              return AzionValue::Boolean(result);
          }
      }
    }
}

fn parse_string(a_string: &str) -> Option<AzionValue>
{
  let mut parser = Rdp::new(StringInput::new(a_string));
  if parser.float() || parser.null_float() {
    Some(parser.float_value())
  } else if parser.int() || parser.null_int() {
    Some(parser.int_value())
  } else if parser.boolean() {
    Some(parser.boolean_value())
  } else {
    None
  }

}


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

macro_rules! integer_test {
    (
        $src:expr, $ex:expr
    ) => {
        #[test]
        fn test_str_to_int_works() {
            let mut parser = Rdp::new(StringInput::new($src));
            let expected_value = AzionValue::Integer(Some($ex));
            assert!(parser.int());
            assert_eq!(expected_value, parser.int_value());
        }
    }
}

macro_rules! integer_tests {
    (
        $list:expr
    ) => {
        #[test]
        fn test_strs_to_ints_works() {
            for &(src, ex) in $list.iter() {
                let mut parser = Rdp::new(StringInput::new(src));
                let expected_value = AzionValue::Integer(Some(ex));
                assert!(parser.int());
                assert_eq!(expected_value, parser.int_value());
            }
        }
    }
}

#[rustfmt_skip]
integer_tests!([
    ("42", 42),
    ("0", 0),
    ("-1000", -1000),
    ("3_141_592_6", 31415926),
    ("-101010101", -101010101),
]);

macro_rules! not_integer_tests {
    (
        $list:expr
    ) => {
        #[test]
        fn test_strs_not_ints() {
            for &src in $list.iter() {
                let mut parser = Rdp::new(StringInput::new(src));
                assert!(!parser.int());
            }
        }
    }
}

#[rustfmt_skip]
not_integer_tests!([
    "_42",
    "a",
    "-_90",
    // "3_1__41",
]);


macro_rules! float_tests {
    (
        $list:expr
    ) => {
        #[test]
        fn test_strs_to_floats_works() {
            for &(src, ex) in $list.iter() {
                let mut parser = Rdp::new(StringInput::new(src));
                let expected_value = AzionValue::Float(Some(ex));
                assert!(parser.float());
                assert_eq!(expected_value, parser.float_value());
            }
        }
    }
}

#[rustfmt_skip]
float_tests!([
    ("1.0", 1.0),
    ("0.", 0.0),
    ("0.0", 0.0),
    (".0", 0.0),
    ("-.0", 0.0),
    ("+.0", 0.0),
    (".012", 0.012),
    ("42.", 42.0),
    ("0.25", 0.25),
    ("+3.1415", 3.1415),
    ("-12.21", -12.21),
]);

// macro_rules! valid_int_test {
// ($($name:expr, $value:expr,)*) => {
// #[test]
// fn test_name_parses() {
// let mut parser = Rdp::new(StringInput::new($name));
//
// assert!(parser.int());
//
// assert!(parser.end());
// }
// }
// }
//
// valid_int_test!("9_0", 90);
//
