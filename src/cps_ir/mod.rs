mod atom;
mod builtin_call;
mod interp;
mod ir;

pub use atom::{Atom, Value};
pub use ir::IR;
pub use builtin_call::builtin_call;
pub use interp::interp;
