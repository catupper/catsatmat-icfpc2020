#[derive(Debug, Clone)]
pub enum LambdaExp {
    Int(i64),
    Apply{
        func: Box<LambdaExp>,
        arg: Box<LambdaExp>
    },
    Lambda(var:i64, )
}

pub use ConsNil::*;

pub fn car<T, U>(cons: ConsNil<T, U>) -> Option<T> {
    match cons {
        ConsNil::Nil => None,
        ConsNil::Cons(x, _) => Some(x),
    }
}

pub fn cdr<T, U>(cons: ConsNil<T, U>) -> Option<U> {
    match cons {
        ConsNil::Nil => None,
        ConsNil::Cons(_, x) => Some(x),
    }
}
