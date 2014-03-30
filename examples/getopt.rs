extern crate getopts;
extern crate decodeopts;
extern crate serialize;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use serialize::Decodable;

#[deriving(Decodable)]
pub struct TestStruct1  {
    data_int: u8,
    data_str: ~str,
}

fn do_work(inp: &str, out: Option<~str>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
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
    optopt("d", "data_int", "number", "NB"),
    optopt("s", "data_str", "str", "NB"),
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

  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: TestStruct1 = Decodable::decode(&mut decoder);

  println!("got data: s -> {} n -> {}", decoded.data_str, decoded.data_int);

  /*if matches.opt_present("h") {
    print_usage(program, opts);
    return;
  }
  let output = matches.opt_str("o");
  let input: &str = if !matches.free.is_empty() {
    (*matches.free.get(0)).clone()
  } else {
    print_usage(program, opts);
    return;
  };
  do_work(input, output);*/
}
