use std::collections::HashMap;

/// 式
#[derive(Debug)]
pub enum Expr {
  Ap(Box<Expr>, Box<Expr>),
  Int(i32),
  Cst(String),
  Def(i32),
}
use Expr::*;

/// :12345 の形のものをパース
fn parse_def(word: &str) -> Option<i32> {
  if &word[0..0] == ":" {
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
      match word.parse::<i32>() {
        Ok(n) => {
          stack.push(Int(n));
        }
        Err(_) => {
          if let Some(n) = parse_def(word) {
            stack.push(Def(n));
          } else if word == "ap" {
            let e1 = stack.pop().unwrap();
            let e2 = stack.pop().unwrap();
            stack.push(Ap(Box::new(e1), Box::new(e2)));
          } else {
            stack.push(Cst(word.to_owned()));
          }
        }
      }
    }
    let e = stack.pop().unwrap();
    assert!(stack.is_empty());
    map.insert(def_n, e);
  }
  map
}
