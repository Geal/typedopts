extern crate getopts;
extern crate serialize;
extern crate typedopts;

use getopts::{reqopt,optopt,optflag,getopts,OptGroup};
use serialize::Decodable;
use typedopts::{Error,ErrorType,DecodeResult};
use std::vec::Vec;

#[derive(Decodable,PartialEq,Eq,Show)]
struct ParseInt {
  number: uint
}

#[test]
fn parse_int() {
  let opts = vec!(reqopt("n", "number", "integer", ""));
  let matches = getopts(vec!("-n".to_string(), "10".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseInt = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.number, 10);
}

#[test]
fn parse_not_int() {
  let opts = vec!(reqopt("n", "number", "integer", ""));
  let matches = getopts(vec!("-n".to_string(), "10.0".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let result:DecodeResult<ParseInt> = Decodable::decode(&mut decoder);
  assert_eq!(result, Err(ErrorType::ExpectedType("number".to_string(), "u64".to_string(), "10.0".to_string())));
}

#[derive(Decodable)]
struct ParseFloat {
  number: f32
}

#[test]
fn parse_float() {
  let opts = vec!(reqopt("n", "number", "integer", ""));
  let matches = getopts(vec!("-n".to_string(), "10".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseFloat = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.number, 10.0);
}
#[derive(Decodable)]
struct ParseBoolean {
  boolean: bool
}

#[test]
fn parse_bool() {
  let opts = vec!(reqopt("b", "boolean", "bool", ""));
  let matches = getopts(vec!("--boolean=true".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseBoolean = Decodable::decode(&mut decoder).unwrap();
  assert!(decoded.boolean);
}


#[derive(Decodable)]
struct ParseChar {
  character: char
}

#[test]
fn parse_char() {
  let opts = vec!(reqopt("c", "character", "char", ""));
  let matches = getopts(vec!("-c".to_string(), "a".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseChar = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.character, 'a');
}

#[derive(Decodable)]
struct ParseString {
  string: String
}

#[test]
fn parse_string() {
  let opts = vec!(reqopt("s", "string", "string", ""));
  let matches = getopts(vec!("-s".to_string(), "abcd".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseString = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
}

#[derive(Decodable, PartialEq, Eq, Show)]
enum Color {
  Red,
  Blue,
  Green
}

#[derive(Decodable)]
struct ParseEnum {
  color: Color
}

#[test]
fn parse_enum() {
  let opts = vec!(reqopt("c", "color", "enum", ""));
  let matches = getopts(vec!("--color".to_string(), "Blue".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseEnum = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.color, Color::Blue);
}


#[derive(Decodable)]
struct ParseOption {
  option: Option<uint>
}

#[test]
fn parse_option() {
  let opts = vec!(optopt("o", "option", "option", ""));
  let matches = getopts(vec!("-o".to_string(), "1".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.option, Some(1));
}

#[test]
fn parse_none_option() {
  let opts = vec!(optopt("o", "option", "option", ""), reqopt("a", "a", "number", ""));
  let matches = getopts(vec!("-a".to_string(), "1".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.option, None);
}

#[derive(Decodable, Show)]
struct ParseStruct {
  string: String,
  optuint: Option<uint>,
  optenum: Option<Color>
}

#[test]
fn parse_struct_noopt() {
  let opts = vec!(reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", ""));
  let matches = getopts(vec!("-s".to_string(), "abcd".to_string()).as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn parse_struct_optenum() {
  let opts = vec!(reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", ""));
  let d = vec!("-s".to_string(), "abcd".to_string(),
     "-c".to_string(), "Green".to_string());
  let matches = getopts(d.as_slice(), opts.as_slice()).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, Some(Color::Green));
}

#[test]
fn decode_task_ok() {
  let opts = vec!(reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", ""));
  let matches = getopts(vec!("-s".to_string(), "abcd".to_string()).as_slice(), opts.as_slice()).unwrap();
  let result = typedopts::decode(matches);
  assert!(result.is_ok());
  let decoded: ParseStruct = result.unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn decode_task_err() {
  let opts = vec!(optopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", ""));
  let matches = getopts(vec!("-u".to_string(), "1".to_string()).as_slice(), opts.as_slice()).unwrap();
  let result: typedopts::DecodeResult<ParseStruct> = typedopts::decode(matches);
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert_eq!(err, ErrorType::MissingField("string".to_string()));
}
