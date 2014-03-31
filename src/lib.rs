#![crate_id = "typedopts"]
#![crate_type = "lib"];

extern crate getopts;
extern crate serialize;
use getopts::Matches;
use std::from_str::FromStr;
use std::str::StrSlice;
use serialize::Decodable;
use std::task;
use std::any::AnyOwnExt;

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

pub type DecoderResult<T> =  Result<T, ErrorType>;

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

  fn unimplemented(&self) -> ! {
    fail!(Error { e: UnimplementedDecoder });
  }

  fn missing_field(&self, field: &str) -> ! {
    fail!(Error { e: MissingField(field.to_owned()) });
  }

  fn expected(&self, expected: &str, field: &str) -> ! {
    fail!(Error { e: ExpectedType(field.to_owned(),
                       expected.to_owned(),
                       self.matches.opt_str(self.cur).unwrap()) });
  }

}

pub fn decode<T:Send+Decodable<Decoder>>(matches: Matches) -> DecoderResult<T> {
  let result = task::try(proc() {
    let mut decoder = Decoder::new(matches);
    let a:T = Decodable::decode(&mut decoder);
    a
  });

  match result {
    Ok(data) => Ok(data),
    Err(e)   => {
      let err = e.move::<Error>().unwrap();
      Err(err.e)
    }
  }
}

impl<T:FromStr> Decoder {
  fn get_field<T:FromStr>(&self, field: &str) -> Option<T> {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => FromStr::from_str(s)
    }
  }
}

impl serialize::Decoder for Decoder {

  fn read_nil(&mut self) {

  }

  fn read_u64(&mut self)  -> u64  {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => match FromStr::from_str(s) {
        None     => self.expected("u64", self.cur),
        Some(nb) => nb
      }
    }
  }
  fn read_u32(&mut self)  -> u32  { self.read_u64() as u32 }
  fn read_u16(&mut self)  -> u16  { self.read_u64() as u16 }
  fn read_u8 (&mut self)  -> u8   { self.read_u64() as u8 }
  fn read_uint(&mut self) -> uint { self.read_u64() as uint }

  fn read_i64(&mut self) -> i64 {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => match FromStr::from_str(s) {
        None     => self.expected("i64", self.cur),
        Some(nb) => nb
      }
    }
  }
  fn read_i32(&mut self) -> i32 { self.read_i64() as i32 }
  fn read_i16(&mut self) -> i16 { self.read_i64() as i16 }
  fn read_i8 (&mut self) -> i8  { self.read_i64() as i8 }
  fn read_int(&mut self) -> int { self.read_i64() as int }

  fn read_f32(&mut self) -> f32 { self.read_f64() as f32 }
  fn read_f64(&mut self) -> f64 {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => match FromStr::from_str(s) {
        None     => self.expected("f64", self.cur),
        Some(nb) => nb
      }
    }
  }

  fn read_bool(&mut self) -> bool {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => match FromStr::from_str(s) {
        None     => self.expected("boolean", self.cur),
        Some(b) => b
      }
    }
  }

  fn read_char(&mut self) -> char {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => if s.char_len() == 1 { s.char_at(0) } else { self.expected("char", self.cur) }
    }
  }

  fn read_str(&mut self) -> ~str {
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => s
    }
  }

  fn read_enum<T>(&mut self, name: &str, f: |&mut Decoder| -> T) -> T {
    //println!("reading enum: {}", name);
    self.current_type = name.to_owned();
    f(self)
  }

  fn read_enum_variant<T>(&mut self, names: &[&str], f: |&mut Decoder, uint| -> T) -> T {
    //println!("reading enum variant({}): {}", self.cur, names);
    match self.matches.opt_str(self.cur) {
      None    => self.missing_field(self.cur),
      Some(s) => match names.iter().position(|&e| e == s) {
        None    => self.expected(self.current_type + " enum", self.cur),
        Some(i) => f(self, i)
      }
    }
    //f(self, 0)
  }

  fn read_enum_variant_arg<T>(&mut self, a_idx: uint, f: |&mut Decoder| -> T) -> T {
    //println!("reading enum variant_arg: {}", a_idx);
    f(self);
    self.unimplemented()
  }

  fn read_enum_struct_variant<T>(&mut self, names: &[&str], f: |&mut Decoder, uint| -> T) -> T {
    //println!("reading enum struct variant: {}", names);
    f(self, 0);
    self.unimplemented()
  }

  fn read_enum_struct_variant_field<T>(&mut self, f_name: &str, f_idx: uint, f: |&mut Decoder| -> T) -> T {
    //println!("reading enum struct variant field: {}, {}", f_name, f_idx);
    f(self);
    self.unimplemented()
  }

  fn read_struct<T>(&mut self, s_name: &str, len: uint, f: |&mut Decoder| -> T) -> T {
    //println!("reading struct: {} | len = {}", s_name, len);
    self.cur = s_name.to_owned();
    f(self)
  }

  fn read_struct_field<T>(&mut self, f_name: &str, f_idx: uint, f: |&mut Decoder| -> T) -> T {
    //println!("reading struct field: {} | idx = {}", f_name, f_idx);
    self.cur = f_name.to_owned();
    let data = f(self);
    //println!("got struct field data: {}", data);
    data
  }

  fn read_option<T>(&mut self, f: |&mut Decoder, bool| -> T) -> T {
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
    //f(self, true);
    //self.unimplemented()
  }

  fn read_tuple<T>(&mut self, f: |&mut Decoder, uint| -> T) -> T {
    f(self, 0);
    self.unimplemented()
  }

  fn read_tuple_arg<T>(&mut self, a_idx: uint, f: |&mut Decoder| -> T) -> T {
    f(self);
    self.unimplemented()
  }

  fn read_tuple_struct<T>(&mut self, s_name: &str, f: |&mut Decoder, uint| -> T) -> T {
    f(self, 0);
    self.unimplemented()
  }

  fn read_tuple_struct_arg<T>(&mut self, a_idx: uint, f: |&mut Decoder| -> T) -> T {
    f(self);
    self.unimplemented()
  }

  fn read_seq<T>(&mut self, f: |&mut Decoder, uint| -> T) -> T {
    f(self, 0);
    self.unimplemented()
  }

  fn read_seq_elt<T>(&mut self, idx: uint, f: |&mut Decoder| -> T) -> T {
    f(self);
    self.unimplemented()
  }

  fn read_map<T>(&mut self, f: |&mut Decoder, uint| -> T) -> T {
    f(self, 0);
    self.unimplemented()
  }

  fn read_map_elt_key<T>(&mut self, idx: uint, f: |&mut Decoder| -> T) -> T {
    f(self);
    self.unimplemented()
  }

  fn read_map_elt_val<T>(&mut self, idx: uint, f: |&mut Decoder| -> T) -> T {
    f(self);
    self.unimplemented()
  }

}
