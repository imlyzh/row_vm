use super::Atom;

#[derive(Debug)]
pub enum IR {
    LetCont(String, Vec<String>, Box<IR>, Box<IR>),
    LetVal(String,String, Vec<Atom>, Box<IR>),
    If(Box<Atom>, String, String),
    App(Box<Atom>, Vec<Atom>, String),
    Fix(Vec<String>, Vec<Atom>, Box<IR>),
}
