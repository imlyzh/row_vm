#[derive(Debug)]
pub enum Value{
  I32(i32),
  I64(i64),
  U32(u32),
  U64(u64),
  Bool(bool),
  Char(char),
  StringLiteral(String),
}

#[derive(Debug)]
pub enum Atom {
  Var(String),
  Val(Value)
}