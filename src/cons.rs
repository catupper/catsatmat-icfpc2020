#[derive(Debug, Clone)]
pub enum ConsNil<T, U> {
    Cons(T, U),
    Nil,
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

pub fn cons<T, U>(x: T, y: U) -> ConsNil<T, U> {
    ConsNil::Cons(x, y)
}
