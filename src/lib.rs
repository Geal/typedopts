#![crate_type = "lib"]

extern crate core;
extern crate getopts;
extern crate rustc_serialize;

use getopts::Matches;
use core::str::FromStr;
//use std::str::StrSlice;
use rustc_serialize::Decodable;
use std::num;
use std::result::Result;

#[derive(PartialEq, Eq, Show)]
pub enum ErrorType {
  UnimplementedDecoder,
  MissingField(String),
  ExpectedType(String, String, String),
  GenericError(String)
}

#[derive(PartialEq, Eq, Show)]
pub struct Error {
  e: ErrorType
}

pub type DecodeResult<T> =  Result<T, ErrorType>;

pub struct Decoder {
  matches: Matches,
  cur: String,
  current_type: String
}

impl Decoder {
  pub fn new(matches: Matches) -> Decoder {
    Decoder {
      matches: matches,
      cur: "".to_string(),
      current_type: "".to_string()
    }
  }

  fn expected(&self, expected_type: String) -> ErrorType {
    ErrorType::ExpectedType(self.cur.to_string(),
                       expected_type.to_string(),
                       self.matches.opt_str(self.cur.as_slice()).unwrap())
  }

}

pub fn decode<T:Send+Decodable>(matches: Matches) -> DecodeResult<T> {
  let mut decoder = Decoder::new(matches);
  Decodable::decode(&mut decoder)
}

impl ErrorType {
  pub fn to_err_msg(self) -> String {
    match self {
      ErrorType::UnimplementedDecoder => format!("this function is not implemented"),
      ErrorType::MissingField(ref s)  => format!("the required field '{}' is not present", s),
      ErrorType::ExpectedType(ref field, ref expected, ref value) => {
        format!("Expected type '{}' for field '{}' but got value '{}'", expected, field, value)
      },
      ErrorType::GenericError(ref s)  => format!("generic error: {}", s)
    }
  }
}
impl Decoder {
  fn get_field<T:FromStr>(&self, field: &str) -> Option<T> {
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => None,
      Some(s) => FromStr::from_str(s.as_slice()).ok()
    }
  }
}

macro_rules! read_primitive {
    ($name:ident, $ty:ty) => {
        fn $name(&mut self) -> DecodeResult<$ty> {
          match self.matches.opt_str(self.cur.as_slice()) {
            None    => Err(ErrorType::MissingField(self.cur.clone())),
            Some(s) => match FromStr::from_str(s.as_slice()).ok() {
              None     => Err(self.expected("u64".to_string())),
              Some(nb) => Ok(nb)
            }
          }
        }
    }
}

impl rustc_serialize::Decoder for Decoder {
  type Error = ErrorType;

  fn read_nil(&mut self) -> DecodeResult<()> {
    Err(ErrorType::UnimplementedDecoder)
  }

  read_primitive! { read_usize, usize }
  read_primitive! { read_u8, u8 }
  read_primitive! { read_u16, u16 }
  read_primitive! { read_u32, u32 }
  read_primitive! { read_u64, u64 }
  read_primitive! { read_isize, isize }
  read_primitive! { read_i8, i8 }
  read_primitive! { read_i16, i16 }
  read_primitive! { read_i32, i32 }
  read_primitive! { read_i64, i64 }

  fn read_f32(&mut self) -> DecodeResult<f32> { self.read_f64().map(|x| x as f32) }
  fn read_f64(&mut self) -> DecodeResult<f64> {
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => Err(ErrorType::MissingField(self.cur.clone())),
      Some(s) => match FromStr::from_str(s.as_slice()).ok() {
        None     => Err(self.expected("f64".to_string())),
        Some(nb) => Ok(nb)
      }
    }
  }

  fn read_bool(&mut self) -> DecodeResult<bool> {
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => Err(ErrorType::MissingField(self.cur.clone())),
      Some(s) => match FromStr::from_str(s.as_slice()).ok() {
        None     => Err(self.expected("boolean".to_string())),
        Some(b) => Ok(b)
      }
    }
  }

  fn read_char(&mut self) -> DecodeResult<char> {
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => Err(ErrorType::MissingField(self.cur.clone())),
      Some(s) => if s.as_slice().chars().count() == 1 { Ok(s.as_slice().char_at(0)) } else { Err(self.expected("char".to_string())) }
    }
  }

  fn read_str(&mut self) -> DecodeResult<String> {
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => Err(ErrorType::MissingField(self.cur.clone())),
      Some(s) => Ok(s)
    }
  }

  fn read_enum<T, F>(&mut self, name: &str, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    //println!("reading enum: {}", name);
    self.current_type = name.to_string();
    f(self)
  }

  fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder, usize) -> DecodeResult<T> {
    //println!("reading enum variant({}): {}", self.cur, names);
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => Err(ErrorType::MissingField(self.cur.clone())),
      Some(s) => match names.iter().position(|&e| e == s.as_slice()) {
        None    => {
          let mut s = self.current_type.clone();
          s.push_str(" enum");
          Err(self.expected(s))
        },
        Some(i) => f(self, i)
      }
    }
  }

  fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    //println!("reading enum variant_arg: {}", a_idx);
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder, usize) -> DecodeResult<T> {
    //println!("reading enum struct variant: {}", names);
    f(self, 0);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    //println!("reading enum struct variant field: {}, {}", f_name, f_idx);
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    //println!("reading struct: {} | len = {}", s_name, len);
    self.cur = s_name.to_string();
    f(self)
  }

  fn read_struct_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    //println!("reading struct field: {} | idx = {}", f_name, f_idx);
    self.cur = f_name.to_string();
    let data = f(self);
    //println!("got struct field data: {}", data);
    data
  }

  fn read_option<T, F>(&mut self, mut f: F) -> DecodeResult<T> where F: FnMut(&mut Decoder, bool) -> DecodeResult<T>
   {
    //println!("read_option");
    match self.matches.opt_str(self.cur.as_slice()) {
      None    => {
        //println!("option not there");
        f(self, false)
      },
      Some(_) => {
        //println!("option is there");
        f(self, true)
      }
    }
  }

  fn read_tuple<T, F>(&mut self,  tuple_len: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_tuple_struct<T, F>(&mut self, s_name: &str, len:usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder, usize) -> DecodeResult<T>
   {
    f(self, 0);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_map<T, F>(&mut self, f: F) -> DecodeResult<T> where F: FnOnce(&mut Decoder, usize) -> DecodeResult<T> {
    f(self, 0);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> DecodeResult<T> where F:FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> DecodeResult<T> where F:FnOnce(&mut Decoder) -> DecodeResult<T> {
    f(self);
    Err(ErrorType::UnimplementedDecoder)
  }

  fn error(&mut self, err: &str) -> ErrorType {
    ErrorType::GenericError(err.to_string())
  }
}
