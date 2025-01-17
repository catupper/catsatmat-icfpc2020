use crate::Expr::{self, *};
use std::collections::HashMap;

/// :12345 の形のものをパース
fn parse_def(word: &str) -> Option<i32> {
    if &word[0..1] == ":" {
        Some(word[1..].parse::<i32>().unwrap())
    } else {
        None
    }
}

/// パースする
/// galaxy は -1 扱い
pub fn parse(text: &str) -> HashMap<i32, Expr> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let head = words.next().unwrap();
        let def_n = if head == "galaxy" {
            -1
        } else {
            parse_def(head).unwrap()
        };
        assert_eq!(words.next().unwrap(), "=");
        let mut stack = Vec::new();
        let words = words.rev();
        for word in words {
            if let Ok(n) = word.parse::<i64>() {
                stack.push(Int(n));
            } else if let Some(n) = parse_def(word) {
                stack.push(Def(n));
            } else if word == "ap" {
                let e1 = stack.pop().unwrap();
                let e2 = stack.pop().unwrap();
                stack.push(Ap(Box::new(e1), Box::new(e2)));
            } else {
                let cst = match word {
                    "add" => Add0,
                    "b" => B0,
                    "c" => C0,
                    "car" => Car,
                    "cdr" => Cdr,
                    "cons" => Cons0,
                    "div" => Div0,
                    "eq" => Eq0,
                    "i" => I,
                    "isnil" => IsNil,
                    "lt" => Lt0,
                    "mul" => Mul0,
                    "neg" => Neg,
                    "nil" => Nil,
                    "s" => S0,
                    "t" => T0,
                    _ => panic!("Parse Failed"),
                };
                stack.push(cst);
            }
        }
        let e = stack.pop().unwrap();
        assert!(stack.is_empty());
        map.insert(def_n, e);
    }
    map
}
