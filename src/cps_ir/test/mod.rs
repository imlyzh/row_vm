use super::{interp, Atom, Store, Value, IR};
use std::collections::HashMap;

fn simple_interp<'a>(term: &'a IR) -> Value<'a> {
    let mut store = Store::new();
    interp(&term, HashMap::new(), &mut store)
}
//simple tests
#[test]
pub fn test1() {
    let term = IR::LetVal(
        "a".to_string(),
        "i32_add".to_string(),
        vec![Atom::I32(3), Atom::I32(4)],
        Box::new(IR::Return(Atom::Var("a".to_string()))),
    );
    let result = simple_interp(&term);
    assert_eq!(result, Value::I32(7))
}

#[test]
pub fn test2() {
    let term = IR::Fix(
        vec!["f".to_string()],
        vec![Atom::Lam(
            vec!["x".to_string()],
            Box::new(IR::Return(Atom::Var("x".to_string()))),
        )],
        Box::new(IR::App(
            Atom::Var("f".to_string()),
            vec![Atom::I32(100)],
            "k".to_string(),
        )),
    );
    let term_with_k = IR::LetCont(
        "k".to_string(),
        vec!["x".to_string()],
        Box::new(IR::Return(Atom::Var("x".to_string()))),
        Box::new(term),
    );
    let result = simple_interp(&term_with_k);
    assert_eq!(result,Value::I32(100));
}
