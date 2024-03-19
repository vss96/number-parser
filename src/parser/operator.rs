use super::{or::OrParser, Either, MapParser, Parser};

pub trait MapOperator<U>: Parser<U> {
    fn map<F, V>(self, f: F) -> impl Parser<V>
    where
        Self: Sized,
        F: Fn(U) -> V,
    {
        MapParser::new(self, f)
    }
}

impl<T, P: Parser<T>> MapOperator<T> for P {}

pub trait AndOperator<U>: Parser<U> {
    fn and<V, T>(self, t: T) -> impl Parser<(U, V)>
    where
        Self: Sized,
        T: Parser<V>,
    {
        (self, t)
    }
}

impl<T, P: Parser<T>> AndOperator<T> for P {}

pub trait OrOperator<U>: Parser<U> {
    fn or<V, T>(self, t: T) -> impl Parser<Either<U, V>>
    where
        Self: Sized,
        T: Parser<V>,
    {
        OrParser::new(self, t)
    }
}

impl<T, P: Parser<T>> OrOperator<T> for P {}
