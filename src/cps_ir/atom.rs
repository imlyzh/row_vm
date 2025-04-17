use super::IR;
use std::collections::HashMap;
#[derive(Debug)]
pub enum Atom {
  Var(String),
  I32(i32),
  I64(i64),
  U32(u32),
  U64(u64),
  Bool(bool),
  Char(char),
  StringLiteral(String),
  Lam(Vec<String>, Box<IR>),
}

#[derive(Debug,Clone)]
pub enum Value<'a>{
  Var(String),
  I32(i32),
  I64(i64),
  U32(u32),
  U64(u64),
  Bool(bool),
  Char(char),
  StringLiteral(String),
  Clo(&'a Vec<String>,&'a IR,HashMap<&'a str,usize>),
  Cont(&'a Vec<String>,&'a IR,HashMap<&'a str,usize>)
}
