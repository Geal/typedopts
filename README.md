# Type aware command line parsing in Rust

[![Build Status](https://travis-ci.org/Geal/typedopts.png?branch=master)](https://travis-ci.org/Geal/typedopts)

# Rationale

While getopts is available for the Rust platform, it is not very practical,
since the generated map of properties must be checked for existence,
then cast to the required types, leading to a lot of options and pattern
matching.

# Usage

With this library, you can define a structure representing the possible
parameters, deriving Decodable, and parse it directly:


```Rust
extern crate getopts;
extern crate typedopts;
extern crate serialize;

use getopts::{reqopt,getopts};
use std::os;
use typedopts::{DecoderResult,UnimplementedDecoder,MissingField,ExpectedType};

#[deriving(Decodable)]
pub struct Args {
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
    Err(f) => { println!("{}", f.to_err_msg()); return; }
  };

  let result: DecoderResult<Args> = typedopts::decode(matches);
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
#[deriving(Decodable)]
enum Color {
  red,
  green,
  blue
}

#[deriving(Decodable)]
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
#[deriving(Decodable)]
pub struct ParseOption {
  option: Option<uint>
}

/* ... */
match decoded.option {
  Some(i) => println!(optional number is {}", i),
  None    => println!("no number was provided")
}
```
