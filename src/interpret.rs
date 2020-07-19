use crate::{
    Board,
    Expr::{self, *},
    Sender,
};

use std::collections::HashMap;

pub struct Interpreter {
    pub env: HashMap<i32, Expr>,
    pub sender: Sender,
}

pub fn multiple_draw(list: Expr) -> Vec<Board> {
    list.iter()
        .map(|points| {
            let points = points
                .iter()
                .map(|vec| {
                    if let Cons2(x, y) = vec {
                        if let (Int(x), Int(y)) = (*x, *y) {
                            return (x, y);
                        }
                    }
                    panic!("ParseFailed not vector");
                })
                .collect();
            Board { points }
        })
        .collect()
}

impl Interpreter {
    pub fn new(env: HashMap<i32, Expr>, sender: Sender) -> Self {
        Self { env, sender }
    }

    /// Apできなくなるまで評価する
    pub fn apply(&self, mut expr: Expr) -> Expr {
        loop {
            match expr {
                Def(ind) => {
                    log::debug!(":{}", ind);
                    expr = self.env.get(&ind).unwrap().clone();
                }
                Ap(func, arg) => {
                    let func = self.apply(*func);
                    expr = match func {
                        Add0 => Add1(arg),
                        Add1(x) => {
                            if let (Int(x), Int(y)) = (self.apply(*x), self.apply(*arg)) {
                                Int(x + y)
                            } else {
                                panic!("ParseError")
                            }
                        }
                        B0 => B1(arg),
                        B1(x) => B2(x, arg),
                        B2(x, y) => Ap(x, Box::new(Ap(y, arg))),
                        C0 => C1(arg),
                        C1(x) => C2(x, arg),
                        C2(x, y) => Ap(Box::new(Ap(x, arg)), y),
                        Car => {
                            if let Cons2(x, _) = self.apply(*arg) {
                                *x
                            } else {
                                panic!("ParseError")
                            }
                        }
                        Cdr => {
                            if let Cons2(_, y) = self.apply(*arg) {
                                *y
                            } else {
                                panic!("ParseError")
                            }
                        }
                        Cons0 => Cons1(arg),
                        Cons1(x) => Cons2(x, arg),
                        Cons2(x, y) => Ap(Box::new(Ap(arg, x)), y),
                        Div0 => Div1(arg),
                        Div1(x) => {
                            if let (Int(x), Int(y)) = (self.apply(*x), self.apply(*arg)) {
                                Int(x / y)
                            } else {
                                panic!("ParseError")
                            }
                        }
                        Eq0 => Eq1(arg),
                        Eq1(x) => {
                            if self.apply(*x) == self.apply(*arg) {
                                T0
                            } else {
                                F
                            }
                        }
                        F => I,
                        I => *arg,
                        IsNil => {
                            if let Nil = self.apply(*arg) {
                                T0
                            } else {
                                F
                            }
                        }
                        Lt0 => Lt1(arg),
                        Lt1(x) => {
                            if let (Int(x), Int(y)) = (self.apply(*x), self.apply(*arg)) {
                                if x < y {
                                    T0
                                } else {
                                    F
                                }
                            } else {
                                panic!("ParseError")
                            }
                        }
                        Mul0 => Mul1(arg),
                        Mul1(x) => {
                            if let (Int(x), Int(y)) = (self.apply(*x), self.apply(*arg)) {
                                Int(x * y)
                            } else {
                                panic!("ParseError")
                            }
                        }
                        Neg => {
                            if let Int(x) = self.apply(*arg) {
                                Int(-x)
                            } else {
                                panic!("ParseError")
                            }
                        }
                        Nil => T0,
                        S0 => S1(arg),
                        S1(x) => S2(x, arg),
                        S2(x, y) => Ap(Box::new(Ap(x, arg.clone())), Box::new(Ap(y, arg))),
                        T0 => T1(arg),
                        T1(x) => *x,
                        _ => {
                            panic!("Couldn't Apply\n{:?} \n{:?}", func, arg);
                        }
                    }
                }
                _ => break,
            }
        }
        expr
    }

    pub fn apply_cons(&self, expr: Expr) -> Expr {
        let expr = self.apply(expr);
        match expr {
            Cons2(x, y) => Cons2(Box::new(self.apply_cons(*x)), Box::new(self.apply_cons(*y))),
            x => x,
        }
    }

    /// return newState and draw_somthing
    pub fn f38(&self, protocol: Expr, expr: Expr) -> (Expr, Vec<Board>) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        println!("f38");
        if let Cons2(flag, expr) = self.apply(expr) {
            println!("Flag: {}", *flag);
            if let Cons2(new_state, expr) = self.apply(*expr) {
                let new_state = self.apply_cons(*new_state);
                println!("NewState: {}", new_state);
                if let Cons2(data, nil) = self.apply(*expr) {
                    assert_eq!(*nil, Nil);
                    if self.apply(*flag) == Int(0) {
                        return (new_state, multiple_draw(self.apply_cons(*data)));
                    } else {
                        println!("SENT: {:?}", data);
                        let recieve = Expr::demodulate(
                            &rt.block_on(self.sender.send(data.modulate())).unwrap(),
                        )
                        .0;
                        println!("RECIEVED: {:?}", recieve);
                        return self.interact(protocol, new_state, recieve);
                    }
                }
            }
        }
        panic!("Parse Failed: f38")
    }

    pub fn interact(&self, protocol: Expr, state: Expr, vector: Expr) -> (Expr, Vec<Board>) {
        self.f38(
            protocol.clone(),
            Expr::ap(Expr::ap(protocol, state), vector),
        )
    }
}
