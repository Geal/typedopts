extern crate getopts;
extern crate typedopts;
extern crate serialize;

use getopts::{reqopt,getopts};
use std::os;
use typedopts::{DecoderResult,UnimplementedDecoder,MissingField,ExpectedType};

#[deriving(Decodable)]
struct Args {
  name:     ~str,
  quantity: uint
}

fn main() {
  let args = os::args();
  let opts = ~[
    reqopt("n", "name", "insert a name here", ""),
    reqopt("q", "quantity", "insert a quantity here", "")
  ];

  let matches = match getopts(args.tail(), opts) {
    Ok(m) => { m },
    Err(f) => { fail!(f.to_err_msg()) }
  };

  let result: DecoderResult<Args> = typedopts::decode(matches);
  match result {
    Ok(decoded) => {
      println!("name is {}", decoded.name);
      println!("quantity is {}", decoded.quantity);
    },
    Err(UnimplementedDecoder) => println!("this function is not implemented"),
    Err(MissingField(ref s))  => println!("the required field '{}' is not present", s),
    Err(ExpectedType(ref field, ref expected, ref value)) => {
      println!("Expected type '{}' for field '{}' but got value '{}'", expected, field, value)
    }
  }
}
