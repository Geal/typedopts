extern crate getopts;
extern crate typedopts;
extern crate serialize;

use std::os;
use getopts::{reqopt,optopt,optflag,getopts,OptGroup};
use typedopts::{DecoderResult,UnimplementedDecoder,MissingField,ExpectedType};

#[deriving(Decodable)]
enum Color {
  red,
  blue
}

#[deriving(Decodable)]
struct TestStruct1  {
  data_int: u8,
  data_str: ~str,
  color: Color,
  maybe: Option<int>
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-o\t\tOutput");
    println!("-d --data_int\tNumber");
    println!("-h --help\tUsage");
}

fn main() {
  let args = os::args();

  let program = args[0].clone();

  let opts = ~[
    optopt("o", "", "set output file name", "NAME"),
    reqopt("d", "data_int", "number", "NB"),
    reqopt("s", "data_str", "str", "NB"),
    reqopt("c", "color", "scolored", ""),
    optopt("m", "maybe", "maybe int", ""),
    optflag("h", "help", "print this help menu")
  ];
  let matches = match getopts(args.tail(), opts) {
    Ok(m) => { m }
    Err(f) => { fail!(f.to_err_msg()) }
  };

  if matches.opt_present("h") {
    print_usage(program, opts);
    return;
  }

  let result: typedopts::DecoderResult<TestStruct1> = typedopts::decode(matches);

  match result {
    Ok(decoded) => {
      println!("got data: s -> {} n -> {}", decoded.data_str, decoded.data_int);
      match decoded.color {
        red  => println!("red"),
        blue => println!("blue")
      }

      match decoded.maybe {
        None    => println!("maybe is none"),
        Some(i) => println!("maybe is {}", i)
      }
    },
    Err(UnimplementedDecoder) => println!("this function is not implemented"),
    Err(MissingField(ref s))  => println!("the required field '{}' is not present", s),
    Err(ExpectedType(ref field, ref expected, ref value)) => {
      println!("Expected type '{}' for field '{}' but got value '{}'", expected, field, value)
    }
  }
}
