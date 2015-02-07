extern crate getopts;
extern crate typedopts;

extern crate "rustc-serialize" as rustc_serialize;
use getopts::Options;
use std::os;
use typedopts::{DecodeResult,ErrorType};
use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
struct Args {
  name:     String,
  quantity: uint
}

fn main() {
  let args = os::args();
  let mut go = Options::new();
  go.reqopt("n", "name", "insert a name here", "");
  go.reqopt("q", "quantity", "insert a quantity here", "");

  let matches = match go.parse(args.tail()) {
    Ok(m) => { m },
    Err(f) => { println!("{}", f.to_err_msg()); return; }
  };

  let result: DecodeResult<Args> = typedopts::decode(matches);
  match result {
    Ok(decoded) => {
      println!("name is {}", decoded.name);
      println!("quantity is {}", decoded.quantity);
    },
    Err(f) => { println!("{}", f.to_err_msg()); return; }
  }
}
