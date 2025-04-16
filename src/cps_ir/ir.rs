#[derive(Debug)]
use super::Atom;
pub enum IR{
  LetCont(String, Vec<String>, IR),
  BuiltinCall(String, Vec<Atom>, String),
  If(Atom, String, String),
}