use super::Atom;

#[derive(Debug)]
pub enum IR {
    LetCont(String, Vec<String>, Box<IR>, Box<IR>),
    LetVal(String,String, Vec<Atom>, Box<IR>),
    If(Atom, String, String),
    App(Atom, Vec<Atom>, String),
    Fix(Vec<String>, Vec<Atom>, Box<IR>),
    Return(Atom)
}
