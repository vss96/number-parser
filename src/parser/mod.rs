pub type ParseResult<T> = Result<(T, String), String>;

pub type _Parser<T> = dyn Fn(&String) -> ParseResult<T>;

pub trait Parser<T>: Sized {
    fn parse_from(&self, val: &String) -> ParseResult<T>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

mod and;
mod digit;
mod many;
mod map;
mod one_or_many;
mod or;
mod token;

pub use and::And2;
pub use digit::{Digit, DigitParser};
pub use many::{Many0, Many1};
pub use map::MapParser;
