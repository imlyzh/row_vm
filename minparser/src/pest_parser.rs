use pest::{error::Error, iterators::{Pair, Pairs}, Parser};
use pest_derive::Parser;

use crate::surface::{Args, Argument, Expression, Message, Symbol};

#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
pub struct RowVMParser {}

pub fn parse(input: &str) -> Result<Vec<Expression>, Error<Rule>> {
    let pairs = RowVMParser::parse(Rule::expression, input)?;
    let r = pairs.map(parse_to_expression).collect();
    Ok(r)
}

pub fn parse_to_expression(pair: Pair<Rule>) -> Expression {
    debug_assert_eq!(pair.as_rule(), Rule::expression);
    let mut pairs = pair.into_inner();
    let mut return_collect = vec![];
    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::message => {
                return_collect.push(parse_to_message(pair));
            }
            _ => {}
        }
    }
    return_collect
}

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

pub fn parse_to_arguments(pair: Pair<Rule>) -> Args {
    debug_assert_eq!(pair.as_rule(), Rule::arguments);
    pair.into_inner().map(parse_to_argument).collect()
}

pub fn parse_to_argument(pair: Pair<Rule>) -> Argument {
    debug_assert_eq!(pair.as_rule(), Rule::argument);
    let mut pairs = pair.into_inner();
    let expression = take_whitespace(&mut pairs).unwrap();
    let expr = parse_to_expression(expression);
    expr
}

pub fn parse_to_symbol(pair: Pair<Rule>) -> Symbol {
    debug_assert_eq!(pair.as_rule(), Rule::symbol);
    let pair = pair.into_inner().next().unwrap();
    let s = pair.as_str();
    match pair.as_rule() {
        Rule::number => Symbol::Number(s),
        Rule::Operator => Symbol::Operator(s),
        Rule::quote => Symbol::Quote(s),
        Rule::Identifier => Symbol::Identifier(s),
        _ => unreachable!(),
    }
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
