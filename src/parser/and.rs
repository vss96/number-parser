use super::{ParseResult, Parser};

pub struct AndParser<P1, P2>(P1, P2);

impl<P1, P2, X, Y> Parser<(X, Y)> for And2<P1, P2>
where
    P1: Parser<X>,
    P2: Parser<Y>,
{
    fn parse_from(&self, val: &String) -> ParseResult<(X, Y)> {
        self.0.parse_from(val).and_then(|(x, follow)| {
            self.1
                .parse_from(&follow)
                .map(|(y, remaining)| ((x, y), remaining))
        })
    }
}

impl<P1, P2> AndParser<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self(p1, p2)
    }
}

pub type And2<U, V> = AndParser<U, V>;
pub type And3<U, V, W> = And2<U, And2<V, W>>;
