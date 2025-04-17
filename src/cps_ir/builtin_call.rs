use super::Value;
use std::ops::{BitAnd,BitOr,BitXor};
pub fn builtin_call<'a>(op: &'a str, args_value: Vec<Value<'a>>) -> Value<'a> {
    match op {
        "i32_add" => {
            if args_value.len() != 2 {
                panic!("i32_add: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 + v2),
                _ => {
                    panic!("i32_add: wrong type of arguments");
                }
            }
        },
        "i32_sub" => {
            if args_value.len() != 2 {
                panic!("i32_sub: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 - v2),
                _ => {
                    panic!("i32_sub: wrong type of arguments");
                }
            }
        },
        "i32_mul" => {
            if args_value.len() != 2 {
                panic!("i32_mul: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 * v2),
                _ => {
                    panic!("i32_mul: wrong type of arguments");
                }
            }
        },
        "i32_div" => {
            if args_value.len() != 2 {
                panic!("i32_div: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => {
                    if *v2 == 0 {
                        panic!("i32_div: number divided by zero");
                    };
                    Value::I32(v1/v2)
                },
                _ => {
                    panic!("i32_div: wrong type of arguments");
                }
            }
        },
        "i32_eq" => {
            if args_value.len() != 2 {
                panic!("i32_eq: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 == v2),
                _ => {
                    panic!("i32_eq: wrong type of arguments");
                }
            }
        },
        "i32_gt" => {
            if args_value.len() != 2 {
                panic!("i32_gt: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 > v2),
                _ => {
                    panic!("i32_gt: wrong type of arguments");
                }
            }
        },
        "i32_geq" => {
            if args_value.len() != 2 {
                panic!("i32_geq: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 >= v2),
                _ => {
                    panic!("i32_geq: wrong type of arguments");
                }
            }
        },
        "i32_lt" => {
            if args_value.len() != 2 {
                panic!("i32_lt: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 < v2),
                _ => {
                    panic!("i32_lt: wrong type of arguments");
                }
            }
        },
        "i32_leq" => {
            if args_value.len() != 2 {
                panic!("i32_leq: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 <= v2),
                _ => {
                    panic!("i32_leq: wrong type of arguments");
                }
            }
        },
        "i32_and" => {
            if args_value.len() != 2 {
                panic!("i32_and: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1.bitand(v2)),
                _ => {
                    panic!("i32_and: wrong type of arguments");
                }
            }
        },
        "i32_or" => {
            if args_value.len() != 2 {
                panic!("i32_or: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1.bitor(v2)),
                _ => {
                    panic!("i32_or: wrong type of arguments");
                }
            }
        },
        "i32_xor" => {
            if args_value.len() != 2 {
                panic!("i32_xor: wrong number of arguments");
            }
            match (&args_value[0], &args_value[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1.bitxor(v2)),
                _ => {
                    panic!("i32_xor: wrong type of arguments");
                }
            }
        },
        "i32_not" => {
            if args_value.len() != 1 {
                panic!("i32_not: wrong number of arguments");
            }
            match &args_value[0] {
                Value::I32(v1) => Value::I32(v1.reverse_bits()),
                _ => {
                    panic!("i32_not: wrong type of arguments");
                }
            }
        },
        _ => {
            panic!("unrecognized builtin call: {}", op)
        },
    }
}
