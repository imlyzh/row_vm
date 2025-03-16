
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::surface::{Args, Message, Symbol};

#[derive(Parser)]
#[grammar="./src/grammar.pest"]
pub struct RowVMParser {}


pub fn parse_to_message(pair: Pair<Rule>) -> Message {
    debug_assert_eq!(pair.as_rule(), Rule::message);
    let mut pairs = pair.into_inner();
    let symbol = take_whitespace(&mut pairs).unwrap();
    let symbol = parse_to_symbol(symbol);
    if let Some(argument) = take_whitespace(&mut pairs) {
        Message {
            symbol: symbol,
            arguments: parse_to_arguments(argument),
        }
    } else {
        Message {
            symbol,
            arguments: vec![],
        }
    }
}

pub fn parse_to_arguments<'a>(pair: Pair<'a, Rule>) -> Args<'a> {
    debug_assert_eq!(pair.as_rule(), Rule::arguments);
    todo!()
}

pub fn parse_to_symbol(pair: Pair<Rule>) -> Symbol {
    debug_assert_eq!(pair.as_rule(), Rule::symbol);
    todo!()
}

pub fn take_whitespace<'a>(pairs: &mut Pairs<'a, Rule>) -> Option<Pair<'a, Rule>> {
    while let Some(pair) = pairs.next() {
        if matches!(pair.as_rule(), Rule::wcpad | Rule::scpad | Rule::sctpad) {
            continue;
        }
        return Some(pair);
    }
    None
}