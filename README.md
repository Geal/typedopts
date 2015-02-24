# Type aware command line parsing in Rust

[![Build Status](https://travis-ci.org/Geal/typedopts.png?branch=master)](https://travis-ci.org/Geal/typedopts)

# Rationale

While getopts is available for the Rust platform, it is not very practical,
since the generated map of properties must be checked for existence,
then cast to the required types, leading to a lot of options and pattern
matching.

Typedopts is available on [crates.io](https://crates.io/crates/typedopts)

# Usage

With this library, you can define a structure representing the possible
parameters, deriving Decodable, and parse it directly:


```Rust
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
  let mut opts = Options::new();
  opts.reqopt("n", "name", "insert a name here", "");
  opts.reqopt("q", "quantity", "insert a quantity here", "");

  let matches = match opts.parse(args.tail()) {
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
```

If the decode function finds the correct arguments and is able to parse them,
you will get an instance of the structure you need in the Result. Otherwise,
you receive an enum of errors:

- UnimplementedDecoder: required but should not happen (if you find a use case where you encounter it, please open an issue)
- MissingField: the option was not found. If you use the reqopt function from getopts, it would warn you before typedopts
- ExpectedType: the option's value is not the one needed

Unless you want to handle the errors yourself, you can use directly the
to__err__msg function.

# Examples

Most types can be handled by the library, as long as they can derive from
Decodable. Integers, Unsigned integers, floating point numbers, booleans,
characters, strings are easy to do.

Enumerations will require that you define them as Decodable too:

```Rust
#[deriving(RustcDecodable)]
enum Color {
  red,
  green,
  blue
}

#[deriving(RustcDecodable)]
pub struct ParseEnum {
  color: Color
}
/* ... */
if(decoded.color == blue) {
  /* ... */
}
```

Options are also supported:

```Rust
#[deriving(RustcDecodable)]
pub struct ParseOption {
  option: Option<uint>
}

/* ... */
match decoded.option {
  Some(i) => println!("optional number is {}", i),
  None    => println!("no number was provided")
}
```
## Handling --help with required options

By default, if you define any options as required, getopts will need them everytime, even when
you just want to display the help and usage instructions.

A way to fix it is available in the examples/getopt.rs file. Basically, you define a function
to generate the getopts instance (trust me on this, it will avoid move problems):

```Rust
extern crate getopts;
extern crate typedopts;
extern crate "rustc-serialize" as rustc_serialize;

use std::os;
use getopts::Options;
use typedopts::{DecodeResult,ErrorType};
use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
struct TestStruct1  {
  data_int: u8,
  maybe: Option<u8>
}

fn generate_options() -> Options {
  let mut opts = Options::new();
  opts.reqopt("d", "data-int", "number", "NB");
  opts.optopt("m", "maybe", "maybe number", "");
  opts.optflag("h", "help", "print this help menu");

  return opts;
}
```

Then you define the usage function (from the getopts documentation):

```Rust
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(brief.as_slice()));
}
```

Then, in your argument parsing function, you will parse the options
two times, one to detect the help argument, another one for the main
arguments.

```Rust
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
```

If the help argument is found, show the help then return immediately,
otherwise, parse arguments normally.
