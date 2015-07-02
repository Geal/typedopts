extern crate getopts;
extern crate typedopts;

extern crate rustc_serialize;
use getopts::Options;
use typedopts::{DecodeResult};
use std::env;

#[derive(RustcDecodable)]
struct Args {
  name:     String,
  quantity: u8
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut go = Options::new();
  go.reqopt("n", "name", "insert a name here", "");
  go.reqopt("q", "quantity", "insert a quantity here", "");

  let matches = match go.parse(args.last()) {
    Ok(m) => { m },
    Err(f) => { println!("{}", f.to_string()); return; }
  };

  let result: DecodeResult<Args> = typedopts::decode(matches);
  match result {
    Ok(decoded) => {
      println!("name is {}", decoded.name);
      println!("quantity is {}", decoded.quantity);
    },
    Err(f) => { println!("{:?}", f); return; }
  }
}
