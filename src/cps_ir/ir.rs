use super::{Atom};

#[derive(Debug)]
pub enum IR {
    LetCont(String, Vec<String>, Box<IR>, Box<IR>),
    LetVal(String,String, Vec<Atom>, Box<IR>),
    If(Atom, Cont, Cont),
    App(Atom, Vec<Atom>, Cont),
    Fix(Vec<String>, Vec<Atom>, Box<IR>),
    Return(Atom)
}

#[derive(Debug)]
pub enum Cont {
    Named(String),
    Return
}