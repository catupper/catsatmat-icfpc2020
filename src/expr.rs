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
    Cst(String),
    Def(i32),
    B0,
    B1(Box<Expr>),
    B2(Box<Expr>, Box<Expr>),
    C0,
    C1(Box<Expr>),
    C2(Box<Expr>, Box<Expr>),
    S0,
    S1(Box<Expr>),
    S2(Box<Expr>, Box<Expr>),
    T0,
    T1(Box<Expr>),
    Nil,
    Cons0,
    Cons1(Box<Expr>),
    Cons2(Box<Expr>, Box<Expr>),
}

impl Expr {
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
}
