extern crate getopts;
extern crate serialize;
extern crate typedopts;

use getopts::{reqopt,optopt,optflag,getopts,OptGroup};
use serialize::Decodable;
use typedopts::{Error,DecodeResult,UnimplementedDecoder,MissingField,ExpectedType};
use std::vec::Vec;

#[deriving(Decodable,PartialEq,Eq,Show)]
struct ParseInt {
  number: uint
}

#[test]
fn parse_int() {
  let opts = [reqopt("n", "number", "integer", "")];
  let matches = getopts(["-n".to_string(), "10".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseInt = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.number, 10);
}

#[test]
fn parse_not_int() {
  let opts = [reqopt("n", "number", "integer", "")];
  let matches = getopts(["-n".to_string(), "10.0".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let result:DecodeResult<ParseInt> = Decodable::decode(&mut decoder);
  assert_eq!(result, Err(ExpectedType("number".to_string(), "u64".to_string(), "10.0".to_string())));
}

#[deriving(Decodable)]
struct ParseFloat {
  number: f32
}

#[test]
fn parse_float() {
  let opts = [reqopt("n", "number", "integer", "")];
  let matches = getopts(["-n".to_string(), "10".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseFloat = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.number, 10.0);
}
#[deriving(Decodable)]
struct ParseBoolean {
  boolean: bool
}

#[test]
fn parse_bool() {
  let opts = [reqopt("b", "boolean", "bool", "")];
  let matches = getopts(["--boolean=true".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseBoolean = Decodable::decode(&mut decoder).unwrap();
  assert!(decoded.boolean);
}


#[deriving(Decodable)]
struct ParseChar {
  character: char
}

#[test]
fn parse_char() {
  let opts = [reqopt("c", "character", "char", "")];
  let matches = getopts(["-c".to_string(), "a".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseChar = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.character, 'a');
}

#[deriving(Decodable)]
struct ParseString {
  string: String
}

#[test]
fn parse_string() {
  let opts = [reqopt("s", "string", "string", "")];
  let matches = getopts(["-s".to_string(), "abcd".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseString = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
}

#[deriving(Decodable, PartialEq, Eq, Show)]
enum Color {
  Red,
  Blue,
  Green
}

#[deriving(Decodable)]
struct ParseEnum {
  color: Color
}

#[test]
fn parse_enum() {
  let opts = [reqopt("c", "color", "enum", "")];
  let matches = getopts(["--color".to_string(), "Blue".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseEnum = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.color, Blue);
}


#[deriving(Decodable)]
struct ParseOption {
  option: Option<uint>
}

#[test]
fn parse_option() {
  let opts = [optopt("o", "option", "option", "")];
  let matches = getopts(["-o".to_string(), "1".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.option, Some(1));
}

#[test]
fn parse_none_option() {
  let opts = [optopt("o", "option", "option", ""), reqopt("a", "a", "number", "")];
  let matches = getopts(["-a".to_string(), "1".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.option, None);
}

#[deriving(Decodable, Show)]
struct ParseStruct {
  string: String,
  optuint: Option<uint>,
  optenum: Option<Color>
}

#[test]
fn parse_struct_noopt() {
  let opts = [reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", "")];
  let matches = getopts(["-s".to_string(), "abcd".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn parse_struct_optenum() {
  let opts = [reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", "")];
  let matches = getopts(["-s".to_string(), "abcd".to_string(),
    "-c".to_string(), "Green".to_string()], opts).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder).unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, Some(Green));
}

#[test]
fn decode_task_ok() {
  let opts = [reqopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", "")];
  let matches = getopts(["-s".to_string(), "abcd".to_string()], opts).unwrap();
  let result = typedopts::decode(matches);
  assert!(result.is_ok());
  let decoded: ParseStruct = result.unwrap();
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optuint, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn decode_task_err() {
  let opts = [optopt("s", "string", "string", ""),
               optopt("u", "optuint", "Option<uint>", ""),
               optopt("c", "optenum", "Option<Color>", "")];
  let matches = getopts(["-u".to_string(), "1".to_string()], opts).unwrap();
  let result: typedopts::DecodeResult<ParseStruct> = typedopts::decode(matches);
  assert!(result.is_err());
  let err = result.unwrap_err();
  assert_eq!(err, MissingField("string".to_string()));
}
