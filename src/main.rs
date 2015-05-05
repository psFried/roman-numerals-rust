#![feature(collections)]
#![feature(exit_status)]

use std::env;
use std::slice::SliceConcatExt;

struct Conversion {
    a: u64,
    r: String
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let output: String = get_output(args);
    println!("{}", output);
}

fn get_output(args: Vec<String>) -> String {
    let num_opt = args[1].parse::<u64>().ok();

    num_opt.map(|input| convert(input))
        .unwrap_or_else(|| get_error_msg(args))
}

fn get_error_msg(args: Vec<String>) -> String {
    std::env::set_exit_status(1);
   "invalid input".to_string()
}

fn create_arabic_to_roman_conversions() -> Vec<Conversion> {
    vec![
      Conversion {a: 1000u64, r: "M".to_string()},
      Conversion {a: 900u64, r: "CM".to_string()},
      Conversion {a: 500u64, r: "D".to_string()},
      Conversion {a: 400u64, r: "CD".to_string()},
      Conversion {a: 100u64, r: "C".to_string()},
      Conversion {a: 90u64, r: "XC".to_string()},
      Conversion {a: 50u64, r: "L".to_string()},
      Conversion {a: 40u64, r: "XL".to_string()},
      Conversion {a: 10u64, r: "X".to_string()},
      Conversion {a: 9u64, r: "IX".to_string()},
      Conversion {a: 5u64, r: "V".to_string()},
      Conversion {a: 4u64, r: "IV".to_string()},
      Conversion {a: 1u64, r: "I".to_string()},
    ]
}

fn convert(input: u64) -> String {
    let arabic_to_roman = create_arabic_to_roman_conversions();
    let mut num: u64 = input;
    let mut output: Vec<String> = Vec::new();

    while num > 0 {
        let val: Option<&Conversion> = arabic_to_roman.iter().find(|con| num >= con.a );
        if val.is_some() {
            let conversion: &Conversion = val.unwrap();
            output.push(val.unwrap().r.to_string());
            num -= conversion.a;
        } else {
            break;
        }
    }

    output.concat().to_string()
}

#[test]
fn test_convert() {
   assert_eq!("", convert(0u64));
   assert_eq!("I", convert(1u64));
   assert_eq!("V", convert(5u64));
   assert_eq!("II", convert(2u64));
   assert_eq!("IV", convert(4u64));
   assert_eq!("VII", convert(7u64));
   assert_eq!("IX", convert(9u64));
   assert_eq!("XIII", convert(13u64));
   assert_eq!("XXXIX", convert(39u64));

   assert_eq!("L", convert(50u64));
   assert_eq!("LXI", convert(61u64));
   assert_eq!("LXXIV", convert(74u64));
   assert_eq!("C", convert(100u64));
   assert_eq!("XL", convert(40u64));
   assert_eq!("XCI", convert(91u64));
   assert_eq!("D", convert(500u64));

   assert_eq!("CD", convert(400u64));
   assert_eq!("DXCIX", convert(599u64));
   assert_eq!("M", convert(1000u64));
   assert_eq!("MCDLI", convert(1451u64));
   assert_eq!("MMMMMDCCXI", convert(5711u64));
}

#[test]
fn test_get_output() {
    assert_eq!("invalid input", get_output(vec!["".to_string(), "my invalid input".to_string()]));
    assert_eq!("CVI", get_output(vec!["".to_string(), "106".to_string()]));
}
