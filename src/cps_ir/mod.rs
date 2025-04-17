mod atom;
mod builtin_call;
mod interp;
mod ir;
#[cfg(test)]
mod test;

pub use atom::{Atom, Value};
pub use ir::IR;
pub use builtin_call::builtin_call;
pub use interp::{interp,Store};
