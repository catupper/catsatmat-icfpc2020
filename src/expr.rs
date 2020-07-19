use std::collections::HashSet;

/// 式
/* Cstのリスト 増えるかもしれん！
add
ap
b
c
car
cdr
cons
div
eq
galaxy
i
isnil
lt
mul
neg
nil
s
t
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ap(Box<Expr>, Box<Expr>),
    Int(i64),
    //    Cst(String),
    Def(i32),
    Add0,
    Add1(Box<Expr>),
    B0,
    B1(Box<Expr>),
    B2(Box<Expr>, Box<Expr>),
    C0,
    C1(Box<Expr>),
    C2(Box<Expr>, Box<Expr>),
    Car,
    Cdr,
    Cons0,
    Cons1(Box<Expr>),
    Cons2(Box<Expr>, Box<Expr>),
    Div0,
    Div1(Box<Expr>),
    Eq0,
    Eq1(Box<Expr>),
    F,
    I,
    IsNil,
    Lt0,
    Lt1(Box<Expr>),
    Mul0,
    Mul1(Box<Expr>),
    Neg,
    Nil,
    S0,
    S1(Box<Expr>),
    S2(Box<Expr>, Box<Expr>),
    T0,
    T1(Box<Expr>),
}

use std::fmt;
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Cons2(_, _) | Expr::Nil => {
                write!(f, "[")?;
                let mut expr: &Expr = self;
                let mut first = true;
                loop {
                    match expr {
                        Expr::Cons2(expr1, expr2) => {
                            if !first {
                                write!(f, ",")?;
                            }
                            write!(f, " {}", expr1)?;
                            expr = expr2;
                            first = false;
                        }
                        Expr::Nil => {
                            write!(f, " ]")?;
                            break;
                        }
                        _ => {
                            write!(f, " | {} ]", expr)?;
                            break;
                        }
                    }
                }
                fmt::Result::Ok(())
            }
            Expr::Int(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Expr {
    pub fn iter(&self) -> ExprIter {
        ExprIter {
            expr: Some(Box::new(self.clone())),
        }
    }
    pub fn from_vector(vec: Vec<Expr>) -> Expr {
        vec.into_iter()
            .rev()
            .fold(Expr::Nil, |xs, x| Expr::Cons2(Box::new(x), Box::new(xs)))
    }
    pub fn travarse_defs(&self) -> HashSet<i32> {
        match self {
            Expr::Ap(expr1, expr2) => {
                let defs1 = expr1.travarse_defs();
                let defs2 = expr2.travarse_defs();
                defs1.union(&defs2).cloned().collect()
            }
            Expr::Def(id) => {
                let mut defs = HashSet::new();
                defs.insert(*id);
                defs
            }
            _ => HashSet::new(),
        }
    }

    pub fn ap(func: Expr, arg: Expr) -> Expr {
        Expr::Ap(Box::new(func), Box::new(arg))
    }

    pub fn vector(x: i64, y: i64) -> Expr {
        Expr::Cons2(Box::new(Expr::Int(x)), Box::new(Expr::Int(y)))
    }

    pub fn cons(x: Expr, y: Expr) -> Expr {
        Expr::Cons2(Box::new(x), Box::new(y))
    }

    pub fn modulate(&self) -> String {
        match self {
            Expr::Cons2(x, y) => "11".to_string() + &x.modulate() + &y.modulate(),
            Expr::Nil => "00".to_string(),
            Expr::Int(num) => Expr::modulate_integer(*num),
            _ => panic!(),
        }
    }

    pub fn demodulate_abs(src: &str) -> (i64, &str) {
        let n = src.chars().position(|c| c == '0').unwrap();
        let mut res = 0i64;
        for c in src[n + 1..n + 1 + 4 * n].chars() {
            res *= 2;
            if c == '1' {
                res += 1;
            }
        }
        (res, &src[n + 1 + 4 * n..])
    }

    pub fn demodulate(src: &str) -> (Self, &str) {
        match &src[0..2] {
            "00" => (Expr::Nil, &src[2..]),
            "01" => {
                let (abs, rest) = Expr::demodulate_abs(&src[2..]);
                (Expr::Int(abs), rest)
            }
            "10" => {
                let (abs, rest) = Expr::demodulate_abs(&src[2..]);
                (Expr::Int(-abs), rest)
            }
            "11" => {
                let (x, rest) = Expr::demodulate(&src[2..]);
                let (y, rest) = Expr::demodulate(rest);
                (Expr::cons(x, y), rest)
            }
            _ => panic!(),
        }
    }

    pub fn modulate_integer(num: i64) -> String {
        let mut res = String::new();
        let abs;
        if num < 0 {
            res += "10";
            abs = -num;
        } else {
            res += "01";
            abs = num;
        }
        let mut n = 0;
        while (1i64 << (n * 4)) <= abs {
            n += 1;
        }
        res += &"1".repeat(n);
        res += "0";
        for i in (0..n * 4).rev() {
            if (abs >> i) % 2 == 0 {
                res += "0";
            } else {
                res += "1";
            }
        }
        res
    }
}

pub struct ExprIter {
    expr: Option<Box<Expr>>,
}

//ConsをListのごとく舐める
//Nilはガン無視
impl Iterator for ExprIter {
    type Item = Expr;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(expr) = self.expr.clone() {
            let res = match *expr {
                Expr::Cons2(x, y) => {
                    self.expr = Some(y);
                    *x
                }
                expr => {
                    self.expr = None;
                    expr
                }
            };
            if let Expr::Nil = res {
                None
            } else {
                Some(res)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn modulate() {
        use crate::expr::Expr::*;
        println!(
            "{}",
            Expr::cons(Expr::cons(Expr::vector(0, 1), Nil), Nil).modulate()
        );
    }
    #[test]
    fn demodulate() {
        println!(
            "{:?}",
            Expr::demodulate("110110000111011111100001000010100011010000")
        );
    }
}
