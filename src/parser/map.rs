use crate::parser::{ParseResult, Parser};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct MapParser<T: Parser<U>, F: Fn(U) -> V, U, V> {
    u: PhantomData<U>,
    v: PhantomData<V>,
    t: T,
    f: F,
}

impl<T: Parser<U>, F: Fn(U) -> V, U, V> Parser<V> for MapParser<T, F, U, V>
where
    T: Parser<U>,
    F: Fn(U) -> V,
{
    fn parse_from(&self, val: &String) -> ParseResult<V> {
        self.t
            .parse_from(val)
            .map(|(v, follow)| ((self.f)(v), follow))
    }
}

impl<T: Parser<U>, F: Fn(U) -> V, U, V> MapParser<T, F, U, V> {
    pub fn new(t: T, f: F) -> Self {
        Self {
            t,
            f,
            u: PhantomData,
            v: PhantomData,
        }
    }
}
