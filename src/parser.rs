//! Parsing code for Anion project

#[cfg_attr(rustfmt, rustfmt_skip)]


use pest::prelude::*;
use super::AnionValue;

impl_rdp! {
  grammar! {

    whitespace = _{ [" "] | ["\t"] }

    plus_or_minus = {["-"] | ["+"]}
    digit = {['0'..'9']}
    bin_digit = {["0"] | ["1"]}
    oct_digit = {['0'..'7']}
    hex_digit = {['0'..'9'] | ['a'..'f'] | ['A'..'F']}
    digits = { digit+ }
    nz_digit = {['1'..'9']}

    // 2,4,8-digit hexadecimal Unicode code points
    unicode_2d_esc = @{["x"] ~ hex_digit ~ hex_digit}
    unicode_4d_esc = @{["u"] ~ hex_digit ~ hex_digit ~ hex_digit ~ hex_digit}
    unicode_8d_esc = @{["U"] ~ hex_digit ~ hex_digit ~ hex_digit ~ hex_digit
                             ~ hex_digit ~ hex_digit ~ hex_digit ~ hex_digit}
    // backslash followed by ...
    escape = { ["\\"] ~ ["\""] | ["\\"] | ["/"] | ["?"]
                      | ["a"] | ["b"] | ["t"] | ["n"]  | ["f"] | ["r"] | ["v"]
                      | unicode_2d_esc | unicode_4d_esc | unicode_8d_esc
                      | ["NL"] // nothing character - goes away...
                      }

    // Start with double quote, then multiple escaped values or any
    // character NOT a backslash or double quote, then end with double quote
    string = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }

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
          ~(
            // Ends with optional exponential
            ( ["e"] | ["E"] ) ~ digits
            )?
          }

    null_int = @{ ["null.int"] }
    int = @{
        ["-"]? // ints may start with optional minus
        ~(
            // non-zero digit, followed by multiple digits
            //  - optional single underscores may split digits
            nz_digit ~ (["_"] ~ digits | digits)*

            // single zero
         |  ["0"]
         )
        }
    hex_int = @{ ["0x"] ~ hex_digit+ ~ (["_"] ~ hex_digit | hex_digit)* }
    oct_int = @{ ["0o"] ~ oct_digit+ ~ (["_"] ~ oct_digit | oct_digit)* }
    bin_int = @{ ["0b"] ~ bin_digit+ ~ (["_"] ~ bin_digit | bin_digit)* }

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
      string_value(&self) -> AnionValue {
        (&s: string) => {
          let (start, stop) = (1, s.len() - 1);
          let unescaped_string = s[start..stop].replace("\\NL", "")
                                               .replace("\\\\", "\\")
                                               .replace("\\n", "\n")
                                               .replace("\\0", "\0")
                                               .replace("\\t", "\t");
          let result = String::from(unescaped_string);
          return AnionValue::String(Some(result));
        },
      }

      int_value(&self) -> AnionValue {

        (&hex: hex_int) => {
            let int_str = hex.replace("_", "");
            let result = i32::from_str_radix(&int_str[2..], 16).unwrap();
            return AnionValue::Integer(Some(result));
        },

        (&oct: oct_int) => {
            let int_str = oct.replace("_", "");
            let result = i32::from_str_radix(&int_str[2..], 8).unwrap();
            return AnionValue::Integer(Some(result));
        },

        (&binary: bin_int) => {
            let int_str = binary.replace("_", "");
            let result = i32::from_str_radix(&int_str[2..], 2).unwrap();
            return AnionValue::Integer(Some(result));
        },

        (&int_token: int) => {
            let int_str = int_token.replace("_", "");
            let result: i32 = int_str.parse().unwrap();
            return AnionValue::Integer(Some(result));
        },

        (_: null_int) => {
            // assert_eq!(null_int_token, "null.int");
            return AnionValue::Integer(None);
        }
      }

      float_value(&self) -> AnionValue {
        (&float_token: float) => {
            let foo = float_token.replace("_", "");
            let result = foo.parse().unwrap();
            return AnionValue::Float(Some(result));
        },

        (_: null_float) => {
            // assert_eq!(null_float_token, "null.float");
            return AnionValue::Float(None);
        }
      }

      boolean_value(&self) -> AnionValue {
        (&bool_token: boolean) => {
            let result =
              if bool_token != "null.bool" {
                  Some(bool_token.parse::<bool>().unwrap())
              } else {
                  None
              };
            return AnionValue::Boolean(result);
        }
      }
    }
}

pub fn parse_string(a_string: &str) -> Option<AnionValue>
{
  let mut parser = Rdp::new(StringInput::new(a_string));
  if parser.float() || parser.null_float() {
    Some(parser.float_value())
  } else if parser.hex_int() || parser.oct_int() || parser.bin_int() || parser.int() || parser.null_int() {
    Some(parser.int_value())
  } else if parser.boolean() {
    Some(parser.boolean_value())
  } else {
    None
  }

}

macro_rules! integer_test {
    (
        $src:expr, $ex:expr
    ) => {
        #[test]
        fn test_str_to_int_works() {
            let mut parser = Rdp::new(StringInput::new($src));
            let expected_value = AnionValue::Integer(Some($ex));
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
                let expected_value = AnionValue::Integer(Some(ex));
                assert!(parser.hex_int() || parser.oct_int() || parser.bin_int() || parser.int());
                assert_eq!(parser.int_value(), expected_value);
                assert!(parser.end());
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
    ("0x42", 66),
    ("0x10101", 65793),
    ("0o42", 34),
    ("0o1010_1", 4161),
    ("0b10101", 21),
    ("0b10_10", 10),
    ("0b1101_1111_0101", 3573),
]);

macro_rules! not_integer_tests {
    (
        $list:expr
    ) => {
        #[test]
        fn test_strs_not_ints() {
            for &src in $list.iter() {
                let mut parser = Rdp::new(StringInput::new(src));
                let is_int = parser.hex_int() || parser.oct_int() || parser.bin_int() || parser.int();
                assert!(!is_int || !parser.end());

            }
        }
    }
}

#[rustfmt_skip]
not_integer_tests!([
    "_42",
    "a",
    "-_90",
    "3_1__41",
    "12_",
    "00",
    "0x_42",
    "0o900",
    "0o190",
]);


macro_rules! float_tests {
    (
        $list:expr
    ) => {
        #[test]
        fn test_strs_to_floats_works() {
            for &(src, ex) in $list.iter() {
                let mut parser = Rdp::new(StringInput::new(src));
                let expected_value = AnionValue::Float(Some(ex));
                assert!(parser.float());
                assert_eq!(parser.float_value(), expected_value);
                assert!(parser.end());
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
    ("-12.21e1", -122.1),
]);



macro_rules! string_tests {
    (
        $list:expr
    ) => {
        #[test]
        fn test_strs_to_strs_works() {
            for &(src, ex) in $list.iter() {
                let mut parser = Rdp::new(StringInput::new(src));
                let expected_value = AnionValue::String(Some(String::from(ex)));
                assert!(parser.string());
                assert_eq!(parser.string_value(), expected_value);
                assert!(parser.end());
            }
        }
    }
}

#[rustfmt_skip]
string_tests!([
  ("\"\"", ""),
  ("\"a\"", "a"),
  ("\"a\\NLb\"", "ab"),
  ("\"a\\nb\"", "a\nb"),
]);
