extern crate getopts;
extern crate serialize;
use decodeopts = lib;
use getopts::{reqopt,optopt,optflag,getopts,OptGroup};
use serialize::Decodable;
use std::task;
use std::any::AnyRefExt;
use decodeopts::{Error, UnimplementedDecoder,MissingField,ExpectedType};

mod lib;

#[deriving(Decodable)]
pub struct ParseInt {
  number: uint
}

#[test]
fn parse_int() {
  let opts = ~[reqopt("n", "number", "integer", "")];
  let matches = getopts([~"-n", ~"10"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseInt = Decodable::decode(&mut decoder);
  assert_eq!(decoded.number, 10);
}

#[test]
fn parse_not_int() {
  let opts = ~[reqopt("n", "number", "integer", "")];
  let matches = getopts([~"-n", ~"10.0"], opts).unwrap();
  let result = task::try(proc() {
    let mut decoder = decodeopts::Decoder::new(matches);
    let decoded: ParseInt = Decodable::decode(&mut decoder);
    1 as uint
  });
  println!("result: {}", result);
  assert!(result.is_err());
  match result {
    Ok(a) => assert!(false),
    Err(e)      => {
      let err = e.as_ref::<Error>().unwrap();
      assert_eq!(err.e, ExpectedType(~"number", ~"u64", ~"10.0"));
    }
  }
}

#[deriving(Decodable)]
pub struct ParseFloat {
  number: f32
}

#[test]
fn parse_float() {
  let opts = ~[reqopt("n", "number", "integer", "")];
  let matches = getopts([~"-n", ~"10"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseFloat = Decodable::decode(&mut decoder);
  assert_eq!(decoded.number, 10.0);
}
#[deriving(Decodable)]
pub struct ParseBoolean {
  boolean: bool
}

#[test]
fn parse_bool() {
  let opts = ~[reqopt("b", "boolean", "bool", "")];
  let matches = getopts([~"--boolean=true"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseBoolean = Decodable::decode(&mut decoder);
  assert!(decoded.boolean);
}


#[deriving(Decodable)]
pub struct ParseChar {
  character: char
}

#[test]
fn parse_char() {
  let opts = ~[reqopt("c", "character", "char", "")];
  let matches = getopts([~"-c", ~"a"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseChar = Decodable::decode(&mut decoder);
  assert_eq!(decoded.character, 'a');
}

#[deriving(Decodable)]
pub struct ParseString {
  string: ~str
}

#[test]
fn parse_string() {
  let opts = ~[reqopt("s", "string", "string", "")];
  let matches = getopts([~"-s", ~"abcd"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseString = Decodable::decode(&mut decoder);
  assert_eq!(decoded.string, ~"abcd");
}

#[deriving(Decodable, Eq, Show)]
enum Color {
  Red,
  Blue,
  Green
}

#[deriving(Decodable)]
pub struct ParseEnum {
  color: Color
}

#[test]
fn parse_enum() {
  let opts = ~[reqopt("c", "color", "enum", "")];
  let matches = getopts([~"--color", ~"Blue"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseEnum = Decodable::decode(&mut decoder);
  assert_eq!(decoded.color, Blue);
}


#[deriving(Decodable)]
pub struct ParseOption {
  option: Option<uint>
}

#[test]
fn parse_option() {
  let opts = ~[optopt("o", "option", "option", "")];
  let matches = getopts([~"-o", ~"1"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder);
  assert_eq!(decoded.option, Some(1));
}

#[test]
fn parse_none_option() {
  let opts = ~[optopt("o", "option", "option", ""), reqopt("a", "a", "number", "")];
  let matches = getopts([~"-a", ~"1"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder);
  assert_eq!(decoded.option, None);
}

#[deriving(Decodable)]
pub struct ParseStruct {
  string: ~str,
  optuint: Option<uint>,
  optenum: Option<Color>
}

#[test]
fn parse_struct_noopt() {
  let opts = ~[reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", "")];
  let matches = getopts([~"-s", ~"abcd"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder);
  assert_eq!(decoded.string, ~"abcd");
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn parse_struct_optenum() {
  let opts = ~[reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", "")];
  let matches = getopts([~"-s", ~"abcd", ~"-c", ~"Green"], opts).unwrap();
  let mut decoder = decodeopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder);
  assert_eq!(decoded.string, ~"abcd");
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, Some(Green));
}
