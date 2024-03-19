pub type ParseResult<T> = Result<(T, String), String>;

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
mod or;
mod token;
mod operator;

pub use digit::{Digit, DigitParser};
pub use many::Many0;
pub use map::MapParser;
pub use operator::{AndOperator, MapOperator, OrOperator};
