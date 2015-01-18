extern crate getopts;
extern crate typedopts;

extern crate "rustc-serialize" as rustc_serialize;
use getopts::{reqopt,getopts};
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
  let opts = vec!(
    reqopt("n", "name", "insert a name here", ""),
    reqopt("q", "quantity", "insert a quantity here", "")
  );

  let matches = match getopts(args.tail(), opts.as_slice()) {
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
