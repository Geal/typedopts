extern crate getopts;
extern crate typedopts;
extern crate serialize;

use std::os;
use getopts::{reqopt,optopt,optflag,getopts,OptGroup};
use typedopts::{DecodeResult,ErrorType};

#[derive(Decodable)]
enum Color {
  red,
  blue
}

#[derive(Decodable)]
struct TestStruct1  {
  data_int: u8,
  data_str: String,
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

  let opts = vec!(
    optopt("o", "", "set output file name", "NAME"),
    reqopt("d", "data_int", "number", "NB"),
    reqopt("s", "data_str", "str", "NB"),
    reqopt("c", "color", "scolored", ""),
    optopt("m", "maybe", "maybe int", ""),
    optflag("h", "help", "print this help menu")
  );
  let matches = match getopts(args.tail(), opts.as_slice()) {
    Ok(m) => { m }
    Err(f) => { println!("{}", f.to_err_msg()); return }
  };

  if matches.opt_present("h") {
    print_usage(program.as_slice(), opts.as_slice());
    return;
  }

  let result: typedopts::DecodeResult<TestStruct1> = typedopts::decode(matches);

  match result {
    Ok(decoded) => {
      println!("got data: s -> {} n -> {}", decoded.data_str, decoded.data_int);
      match decoded.color {
        Color::blue => { println!("blue");},
        Color::red  => {println!("red");}
      }

      match decoded.maybe {
        None    => println!("maybe is none"),
        Some(i) => println!("maybe is {}", i)
      }
    },
    Err(ErrorType::UnimplementedDecoder) => println!("this function is not implemented"),
    Err(ErrorType::MissingField(ref s))  => println!("the required field '{}' is not present", s),
    Err(ErrorType::ExpectedType(ref field, ref expected, ref value)) => {
      println!("Expected type '{}' for field '{}' but got value '{}'", expected, field, value)
    },
    Err(ErrorType::GenericError(_)) => println!("generic error")
  }
}
