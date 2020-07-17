use crate::Expr::{self, *};

use std::collections::HashMap;

pub struct Interpreter {
    pub env: HashMap<i32, Expr>,
}

impl Interpreter {
    pub fn new(env: HashMap<i32, Expr>) -> Self {
        Self { env }
    }

    /// Ap(neg, Int(x))
    /// => Int(-x)
    /// neg が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_neg(&self, expr: Expr) -> Option<Expr> {
        if let Ap(neg, num) = expr {
            if let Cst(neg) = *neg {
                if &neg != "neg" {
                    return None;
                }
                if let Int(num) = self.apply(*num) {
                    return Some(Int(-num));
                } else {
                    panic!("ParseError");
                }
            }
        }
        None
    }

    /// Ap(Ap(Cst("add"), Int(x)), Int(y))
    /// => Int(x+y)
    /// add が出来るならやってSome(Expr::Int)を返す
    /// それ以外Noneを返す
    pub fn apply_add(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr1, num2) = expr {
            if let Ap(add, num1) = *expr1 {
                if let Cst(add) = *add {
                    if &add != "add" {
                        return None;
                    }
                    if let (Int(num1), Int(num2)) = (self.apply(*num1), self.apply(*num2)) {
                        return Some(Int(num1 + num2));
                    } else {
                        panic!("ParseError")
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(Cst("div"), Int(x)), Int(y))
    /// => Int(x+y)
    /// div が出来るならやってSome(Expr::Int)を返す
    /// それ以外Noneを返す
    pub fn apply_div(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr1, num2) = expr {
            if let Ap(div, num1) = *expr1 {
                if let Cst(div) = *div {
                    if &div != "div" {
                        return None;
                    }
                    if let (Int(num1), Int(num2)) = (self.apply(*num1), self.apply(*num2)) {
                        return Some(Int(num1 / num2));
                    } else {
                        panic!("ParseError")
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(Cst("mul"), Int(x)), Int(y))
    /// => Int(x+y)
    /// mul が出来るならやってSome(Expr::Int)を返す
    /// それ以外Noneを返す
    pub fn apply_mul(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr1, num2) = expr {
            if let Ap(mul, num1) = *expr1 {
                if let Cst(mul) = *mul {
                    if &mul != "mul" {
                        return None;
                    }
                    if let (Int(num1), Int(num2)) = (self.apply(*num1), self.apply(*num2)) {
                        return Some(Int(num1 * num2));
                    } else {
                        panic!("ParseError")
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(Cst("lt"), Int(x)), Int(y))
    /// => Cst("t") か Cst("f")
    /// mul が出来るならやってSome(Expr::Cst)を返す
    /// それ以外Noneを返す
    pub fn apply_lt(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr1, num2) = expr {
            if let Ap(mul, num1) = *expr1 {
                if let Cst(mul) = *mul {
                    if &mul != "mul" {
                        return None;
                    }
                    if let (Int(num1), Int(num2)) = (self.apply(*num1), self.apply(*num2)) {
                        return Some(if num1 < num2 {
                            Cst("t".to_string())
                        } else {
                            Cst("f".to_string())
                        });
                    } else {
                        panic!("ParseError")
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(Ap(b, x), y), z)
    /// => Ap(x, Ap(y, z))
    /// b が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_b(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr, z) = expr {
            if let Ap(expr, y) = *expr {
                if let Ap(expr, x) = *expr {
                    if let Cst(b) = *expr {
                        if &b == "b" {
                            return Some(Ap(x, Box::new(Ap(y, z))));
                        }
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(Ap(c, x), y), z)
    /// => Ap(Ap(x, z), y)
    /// c が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_c(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr, z) = expr {
            if let Ap(expr, y) = *expr {
                if let Ap(expr, x) = *expr {
                    if let Cst(c) = *expr {
                        if &c == "c" {
                            return Some(Ap(Box::new(Ap(x, y)), z));
                        }
                    }
                }
            }
        }
        None
    }

    /// Ap(i, x)
    /// => x
    /// i が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_i(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr, x) = expr {
            if let Cst(i) = *expr {
                if &i == "i" {
                    return Some(*x);
                }
            }
        }
        None
    }

    /// Ap(Ap(Ap(s, x), y), z)
    /// => Ap(Ap(x, z), Ap(y, z))
    /// s が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_s(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr, z) = expr {
            if let Ap(expr, y) = *expr {
                if let Ap(expr, x) = *expr {
                    if let Cst(s) = *expr {
                        if &s == "s" {
                            return Some(Ap(Box::new(Ap(x, z.clone())), Box::new(Ap(y, z))));
                        }
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(t, x), y))
    /// => x
    /// t が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_t(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr, _) = expr {
            if let Ap(expr, x) = *expr {
                if let Cst(s) = *expr {
                    if &s == "t" {
                        return Some(*x);
                    }
                }
            }
        }
        None
    }

    /// Ap(f, _)
    /// => i
    /// f が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_f(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr, _) = expr {
            if let Cst(s) = *expr {
                if &s == "f" {
                    return Some(Cst("i".to_string()));
                }
            }
        }
        None
    }

    /// Ap(car, Ap(Ap(cons, x), y))
    /// => x
    /// car が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_car(&self, expr: Expr) -> Option<Expr> {
        if let Ap(car, expr) = expr {
            if *car != Cst("car".to_string()) {
                return None;
            }
            if let Ap(expr, y) = self.apply(*expr) {
                if let Ap(cons, _) = *expr {
                    if let Cst(cons) = self.apply(*cons) {
                        if &cons == "cons" {
                            return Some(*y);
                        }
                    }
                }
            }
        }
        None
    }

    /// Ap(car, Ap(Ap(cons, x), y))
    /// => x
    /// cdr が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_cdr(&self, expr: Expr) -> Option<Expr> {
        if let Ap(cdr, expr) = expr {
            if *cdr != Cst("cdr".to_string()) {
                return None;
            }
            if let Ap(expr, y) = self.apply(*expr) {
                if let Ap(cons, _) = *expr {
                    if let Cst(cons) = self.apply(*cons) {
                        if &cons == "cons" {
                            return Some(*y);
                        }
                    }
                }
            }
        }
        None
    }

    /// Ap(Ap(Cst("eq"), Int(x)), Int(y))
    /// => Cst("t") か Cst("f")
    /// eq が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_eq(&self, expr: Expr) -> Option<Expr> {
        if let Ap(expr1, y) = expr {
            if let Ap(eq, x) = *expr1 {
                if let (Cst(eq), x, y) = (*eq, self.apply(*x), self.apply(*y)) {
                    if &eq == "eq" {
                        return Some(if x == y {
                            Cst("t".to_string())
                        } else {
                            Cst("f".to_string())
                        });
                    }
                }
            }
        }
        None
    }

    /// Ap(isnil, x)
    /// => Cst("t") か Cst("f")
    /// isnil が出来るならやってSome(Expr::Ap)を返す
    /// それ以外Noneを返す
    pub fn apply_isnil(&self, expr: Expr) -> Option<Expr> {
        if let Ap(isnil, nil) = expr {
            if let Cst(isnil) = *isnil {
                if &isnil == "isnil" {
                    if let Cst(nil) = self.apply(*nil) {
                        return Some(if &nil == "nil" {
                            Cst("t".to_string())
                        } else {
                            Cst("f".to_string())
                        });
                    } else {
                        return Some(Cst("f".to_string()));
                    }
                }
            }
        }
        None
    }

    /// Apできなくなるまで評価する
    pub fn apply(&self, mut expr: Expr) -> Expr {
        loop {
            if let Some(new_expr) = self.apply_neg(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_add(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_div(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_mul(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_lt(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_b(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_c(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_i(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_s(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_t(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_f(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_car(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_cdr(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_eq(expr.clone()) {
                expr = new_expr;
            } else if let Some(new_expr) = self.apply_isnil(expr.clone()) {
                expr = new_expr;
            } else if let Def(id) = expr {
                println!("{}", id);
                expr = self.env.get(&id).unwrap().clone();
            } else if let Ap(expr1, expr2) = &expr {
                println!("{:?}\n\n", expr);
                let new_expr1 = self.apply(*expr1.clone());
                if new_expr1 == **expr1 {
                    break;
                }
                expr = Ap(Box::new(new_expr1), expr2.clone());
            } else {
                break;
            }
        }
        expr
    }
}
