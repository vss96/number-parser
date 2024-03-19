use super::{or::OrParser, MapParser, Parser};

pub trait MapCombinator<U>: Parser<U> {
    fn map<F, V>(self, f: F) -> MapParser<Self, F, U, V>
    where
        Self: Sized,
        F: Fn(U) -> V,
    {
        MapParser::new(self, f)
    }
}

impl<T, P: Parser<T>> MapCombinator<T> for P {}

pub trait AndCombinator<U>: Parser<U> {
    fn and<V, T>(self, v: T) -> (Self, T)
    where
        Self: Sized,
        T: Parser<V>,
    {
        (self, v)
    }
}

impl<T, P: Parser<T>> AndCombinator<T> for P {}

pub trait OrCombinator<U>: Parser<U> {
    fn or<V, T>(self, t: T) -> OrParser<Self, T>
    where
        Self: Sized,
        T: Parser<V>,
    {
        OrParser::new(self, t)
    }
}

impl<T, P: Parser<T>> OrCombinator<T> for P {}