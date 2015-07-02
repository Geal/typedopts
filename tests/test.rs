extern crate getopts;
extern crate typedopts;

extern crate rustc_serialize;
use getopts::Options;
use rustc_serialize::Decodable;
use typedopts::{Error,ErrorType,DecodeResult};

#[derive(RustcDecodable,PartialEq,Eq,Debug)]
struct ParseInt {
  number: u8
}

#[test]
fn parse_int() {
  let mut go = Options::new();
  go.reqopt("n", "number", "integer", "");
  let matches = go.parse(vec!["-n".to_string(), "10".to_string()]).ok().expect("getopts match");
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseInt = Decodable::decode(&mut decoder).ok().expect("ParseInt");
  assert_eq!(decoded.number, 10);
}

#[test]
fn parse_not_int() {
  let mut go = Options::new();
  go.reqopt("n", "number", "integer", "");
  let matches = go.parse(vec!["-n".to_string(), "10.0".to_string()]).ok().expect("getopts match");
  let mut decoder = typedopts::Decoder::new(matches);
  let result:DecodeResult<ParseInt> = Decodable::decode(&mut decoder);
  assert_eq!(result, Err(ErrorType::ExpectedType("number".to_string(), "u64".to_string(), "10.0".to_string())));
}

#[derive(RustcDecodable)]
struct ParseFloat {
  number: f32
}

#[test]
fn parse_float() {
  let mut go = Options::new();
  go.reqopt("n", "number", "integer", "");
  let matches = go.parse(vec!["-n".to_string(), "10".to_string()]).ok().expect("getopts match");
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseFloat = Decodable::decode(&mut decoder).ok().expect("ParseFloat");
  assert_eq!(decoded.number, 10.0);
}
#[derive(RustcDecodable)]
struct ParseBoolean {
  boolean: bool
}

#[test]
fn parse_bool() {
  let mut go = Options::new();
  go.reqopt("b", "boolean", "bool", "");
  let matches = go.parse(vec!["--boolean=true".to_string()]).ok().expect("getopts match");
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseBoolean = Decodable::decode(&mut decoder).ok().expect("ParseBoolean");
  assert!(decoded.boolean);
}


#[derive(RustcDecodable)]
struct ParseChar {
  character: char
}

#[test]
fn parse_char() {
  let mut go = Options::new();
  go.reqopt("c", "character", "char", "");
  let matches = go.parse(vec!["-c".to_string(), "a".to_string()]).ok().expect("getopts match");
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseChar = Decodable::decode(&mut decoder).ok().expect("ParseChar");
  assert_eq!(decoded.character, 'a');
}

#[derive(RustcDecodable)]
struct ParseString {
  string: String
}

#[test]
fn parse_string() {
  let mut go = Options::new();
  go.reqopt("s", "string", "string", "");
  let matches = go.parse(vec!["-s".to_string(), "abcd".to_string()]).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseString = Decodable::decode(&mut decoder).ok().expect("ParseString");
  assert_eq!(decoded.string, "abcd".to_string());
}

#[derive(RustcDecodable, PartialEq, Eq, Debug)]
enum Color {
  Red,
  Blue,
  Green
}

#[derive(RustcDecodable)]
struct ParseEnum {
  color: Color
}

#[test]
fn parse_enum() {
  let mut go = Options::new();
  go.reqopt("c", "color", "enum", "");
  let matches = go.parse(vec!["--color".to_string(), "Blue".to_string()]).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseEnum = Decodable::decode(&mut decoder).ok().expect("ParseEnum");
  assert_eq!(decoded.color, Color::Blue);
}


#[derive(RustcDecodable)]
struct ParseOption {
  option: Option<u8>
}

#[test]
fn parse_option() {
  let mut go = Options::new();
  go.optopt("o", "option", "option", "");
  let matches = go.parse(vec!["-o".to_string(), "1".to_string()]).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder).ok().expect("ParseOption");
  assert_eq!(decoded.option, Some(1));
}

#[test]
fn parse_none_option() {
  let mut go = Options::new();
  go.optopt("o", "option", "option", "");
  go.reqopt("a", "a", "number", "");
  let matches = go.parse(vec!["-a".to_string(), "1".to_string()]).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseOption = Decodable::decode(&mut decoder).ok().expect("ParseOption");
  assert_eq!(decoded.option, None);
}

#[derive(RustcDecodable, Debug)]
struct ParseStruct {
  string: String,
  optu8: Option<u8>,
  optenum: Option<Color>
}

#[test]
fn parse_struct_noopt() {
  let mut go = Options::new();
  go.reqopt("s", "string", "string", "");
  go.optopt("u", "optu8", "Option<u8>", "");
  go.optopt("c", "optenum", "Option<Color>", "");
  let matches = go.parse(vec!["-s".to_string(), "abcd".to_string()]).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder).ok().expect("ParseStruct");
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optu8, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn parse_struct_optenum() {
  let mut go = Options::new();
  go.reqopt("s", "string", "string", "");
  go.optopt("u", "optu8", "Option<u8>", "");
  go.optopt("c", "optenum", "Option<Color>", "");
  let d = vec!("-s".to_string(), "abcd".to_string(),
     "-c".to_string(), "Green".to_string());
  let matches = go.parse(d).unwrap();
  let mut decoder = typedopts::Decoder::new(matches);
  let decoded: ParseStruct = Decodable::decode(&mut decoder).ok().expect("ParseStruct");
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optu8, None);
  assert_eq!(decoded.optenum, Some(Color::Green));
}

#[test]
fn decode_task_ok() {
  let mut go = Options::new();
  go.reqopt("s", "string", "string", "");
  go.optopt("u", "optu8", "Option<u8>", "");
  go.optopt("c", "optenum", "Option<Color>", "");
  let matches = go.parse(vec!["-s".to_string(), "abcd".to_string()]).unwrap();
  let result = typedopts::decode(matches);
  assert!(result.is_ok());
  let decoded: ParseStruct = result.ok().expect("ParseStruct");
  assert_eq!(decoded.string, "abcd".to_string());
  assert_eq!(decoded.optu8, None);
  assert_eq!(decoded.optenum, None);
}

#[test]
fn decode_task_err() {
  let mut go = Options::new();
  go.optopt("s", "string", "string", "");
  go.optopt("u", "optu8", "Option<u8>", "");
  go.optopt("c", "optenum", "Option<Color>", "");
  let matches = go.parse(vec!["-u".to_string(), "1".to_string()]).unwrap();
  let result: typedopts::DecodeResult<ParseStruct> = typedopts::decode(matches);
  assert!(result.is_err());
  let err = result.err().expect("ErrorType");
  assert_eq!(err, ErrorType::MissingField("string".to_string()));
}
