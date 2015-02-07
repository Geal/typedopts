extern crate getopts;
extern crate typedopts;
extern crate "rustc-serialize" as rustc_serialize;

use std::os;
use getopts::Options;
use typedopts::{DecodeResult,ErrorType};
use rustc_serialize::Decodable;

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
  maybe: Option<int>
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
    print!("{}", opts.usage(brief.as_slice()));
}

fn main() {
  let args = os::args();

  let program = args[0].clone();

  let mut help_opts = Options::new();
  help_opts.optflag("h", "help", "print this help menu");

  help_opts.parse(args.tail()).map(|m| {
    if m.opt_present("h") {
      print_usage(program.as_slice(), generate_options());
      return;
    }
  });

  let mut opts = generate_options();
  let matches = match opts.parse(args.tail()) {
    Ok(m) => { m }
    Err(f) => { println!("{}", f.to_err_msg()); return }
  };

  if matches.opt_present("h") {
    print_usage(program.as_slice(), opts);
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
