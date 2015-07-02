extern crate getopts;
extern crate typedopts;
extern crate rustc_serialize;

use std::env;
use getopts::Options;
use typedopts::{DecodeResult,ErrorType};

#[allow(non_camel_case_types)]
#[derive(RustcDecodable)]
enum Color {
  red,
  blue
}

#[derive(RustcDecodable)]
struct TestStruct1  {
  data_int: u8,
  data_str: String,
  color: Color,
  maybe: Option<i8>
}

fn generate_options() -> Options {
  let mut opts = Options::new();
  opts.optopt("o", "", "set output file name", "NAME");
  opts.reqopt("d", "data-int", "number", "NB");
  opts.reqopt("s", "data-str", "str", "NB");
  opts.reqopt("c", "color", "scolored", "");
  opts.optopt("m", "maybe", "maybe int", "");
  opts.optflag("h", "help", "print this help menu");

  return opts;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = (&args[0]).clone();

  let mut help_opts = Options::new();
  help_opts.optflag("h", "help", "print this help menu");

  help_opts.parse(args).map(|m| {
    if m.opt_present("h") {
      print_usage(&program, generate_options());
      return;
    }
  });

  let opts = generate_options();
  let args2: Vec<String> = env::args().collect();
  let matches = match opts.parse(args2) {
    Ok(m) => { m }
    Err(f) => { println!("{}", f.to_string()); return }
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
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
