#![crate_id = "typedopts"]
#![crate_type = "lib"]

extern crate getopts;
extern crate serialize;
use getopts::Matches;
use std::from_str::FromStr;
use std::str::StrSlice;
use serialize::Decodable;

#[deriving(Eq, Show)]
pub enum ErrorType {
  UnimplementedDecoder,
  MissingField(~str),
  ExpectedType(~str, ~str, ~str)
}

#[deriving(Eq, Show)]
pub struct Error {
  e: ErrorType
}

pub type DecodeResult<T> =  Result<T, ErrorType>;

pub struct Decoder {
  priv matches: Matches,
  priv cur: ~str,
  priv current_type: ~str
}

impl Decoder {
  pub fn new(matches: Matches) -> Decoder {
    Decoder {
      matches: matches,
      cur: ~"",
      current_type: ~""
    }
  }

  fn expected(&self, expected_type: &str) -> ErrorType {
    ExpectedType(self.cur.to_owned(),
                       expected_type.to_owned(),
                       self.matches.opt_str(self.cur).unwrap())
  }

}

pub fn decode<T:Send+Decodable<Decoder, ErrorType>>(matches: Matches) -> DecodeResult<T> {
  let mut decoder = Decoder::new(matches);
  Decodable::decode(&mut decoder)
}

impl ErrorType {
  pub fn to_err_msg(self) -> ~str {
    match self {
      UnimplementedDecoder => format!("this function is not implemented"),
      MissingField(ref s)  => format!("the required field '{}' is not present", s),
      ExpectedType(ref field, ref expected, ref value) => {
        format!("Expected type '{}' for field '{}' but got value '{}'", expected, field, value)
      }
    }
  }
}
impl<T:FromStr> Decoder {
  fn get_field<T:FromStr>(&self, field: &str) -> Option<T> {
    match self.matches.opt_str(self.cur) {
      None    => None,
      Some(s) => FromStr::from_str(s)
    }
  }
}

impl serialize::Decoder<ErrorType> for Decoder {

  fn read_nil(&mut self) -> DecodeResult<()> {
    Err(UnimplementedDecoder)
  }

  fn read_u64(&mut self)  -> DecodeResult<u64>  {
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => match FromStr::from_str(s) {
        None     => Err(self.expected(~"u64")),
        Some(nb) => Ok(nb)
      }
    }
  }
  fn read_u32(&mut self)  -> DecodeResult<u32>  { Ok(try!(self.read_u64()) as u32) }
  fn read_u16(&mut self)  -> DecodeResult<u16>  { Ok(try!(self.read_u64()) as u16) }
  fn read_u8 (&mut self)  -> DecodeResult<u8>   { Ok(try!(self.read_u64()) as u8) }
  fn read_uint(&mut self) -> DecodeResult<uint> { Ok(try!(self.read_u64()) as uint) }

  fn read_i64(&mut self) -> DecodeResult<i64> {
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => match FromStr::from_str(s) {
        None     => Err(self.expected(~"i64")),
        Some(nb) => Ok(nb)
      }
    }
  }
  fn read_i32(&mut self) -> DecodeResult<i32> { Ok(try!(self.read_i64()) as i32) }
  fn read_i16(&mut self) -> DecodeResult<i16> { Ok(try!(self.read_i64()) as i16) }
  fn read_i8 (&mut self) -> DecodeResult<i8>  { Ok(try!(self.read_i64()) as i8) }
  fn read_int(&mut self) -> DecodeResult<int> { Ok(try!(self.read_i64()) as int) }

  fn read_f32(&mut self) -> DecodeResult<f32> { Ok(try!(self.read_f64()) as f32) }
  fn read_f64(&mut self) -> DecodeResult<f64> {
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => match FromStr::from_str(s) {
        None     => Err(self.expected(~"f64")),
        Some(nb) => Ok(nb)
      }
    }
  }

  fn read_bool(&mut self) -> DecodeResult<bool> {
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => match FromStr::from_str(s) {
        None     => Err(self.expected("boolean")),
        Some(b) => Ok(b)
      }
    }
  }

  fn read_char(&mut self) -> DecodeResult<char> {
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => if s.char_len() == 1 { Ok(s.char_at(0)) } else { Err(self.expected("char")) }
    }
  }

  fn read_str(&mut self) -> DecodeResult<~str> {
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => Ok(s)
    }
  }

  fn read_enum<T>(&mut self, name: &str, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading enum: {}", name);
    self.current_type = name.to_owned();
    f(self)
  }

  fn read_enum_variant<T>(&mut self, names: &[&str], f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading enum variant({}): {}", self.cur, names);
    match self.matches.opt_str(self.cur) {
      None    => Err(MissingField(self.cur.clone())),
      Some(s) => match names.iter().position(|&e| e == s) {
        None    => Err(self.expected(self.current_type + " enum")),
        Some(i) => f(self, i)
      }
    }
  }

  fn read_enum_variant_arg<T>(&mut self, a_idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading enum variant_arg: {}", a_idx);
    f(self);
    Err(UnimplementedDecoder)
  }

  fn read_enum_struct_variant<T>(&mut self, names: &[&str], f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading enum struct variant: {}", names);
    f(self, 0);
    Err(UnimplementedDecoder)
  }

  fn read_enum_struct_variant_field<T>(&mut self, f_name: &str, f_idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading enum struct variant field: {}, {}", f_name, f_idx);
    f(self);
    Err(UnimplementedDecoder)
  }

  fn read_struct<T>(&mut self, s_name: &str, len: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading struct: {} | len = {}", s_name, len);
    self.cur = s_name.to_owned();
    f(self)
  }

  fn read_struct_field<T>(&mut self, f_name: &str, f_idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("reading struct field: {} | idx = {}", f_name, f_idx);
    self.cur = f_name.to_owned();
    let data = f(self);
    //println!("got struct field data: {}", data);
    data
  }

  fn read_option<T>(&mut self, f: |&mut Decoder, bool| -> DecodeResult<T>) -> DecodeResult<T> {
    //println!("read_option");
    match self.matches.opt_str(self.cur) {
      None    => {
        //println!("option not there");
        f(self, false)
      },
      Some(s) => {
        //println!("option is there");
        f(self, true)
      }
    }
  }

  fn read_tuple<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self, 0);
    Err(UnimplementedDecoder)
  }

  fn read_tuple_arg<T>(&mut self, a_idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self);
    Err(UnimplementedDecoder)
  }

  fn read_tuple_struct<T>(&mut self, s_name: &str, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self, 0);
    Err(UnimplementedDecoder)
  }

  fn read_tuple_struct_arg<T>(&mut self, a_idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self);
    Err(UnimplementedDecoder)
  }

  fn read_seq<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self, 0);
    Err(UnimplementedDecoder)
  }

  fn read_seq_elt<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self);
    Err(UnimplementedDecoder)
  }

  fn read_map<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self, 0);
    Err(UnimplementedDecoder)
  }

  fn read_map_elt_key<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self);
    Err(UnimplementedDecoder)
  }

  fn read_map_elt_val<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
    f(self);
    Err(UnimplementedDecoder)
  }

}
