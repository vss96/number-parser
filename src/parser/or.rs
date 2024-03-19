use super::Either;
use super::{ParseResult, Parser};

#[derive(Clone)]
pub struct OrParser<P1, P2>(P1, P2);

impl<P1, P2, U, V> Parser<Either<U, V>> for OrParser<P1, P2>
where
    P1: Parser<U>,
    P2: Parser<V>,
{
    fn parse_from(&self, val: &String) -> ParseResult<Either<U, V>> {
        self.0
            .parse_from(val)
            .map(|(val, follow)| (Either::Left(val), follow))
            .or_else(|_| {
                self.1
                    .parse_from(val)
                    .map(|(val, follow)| (Either::Right(val), follow))
            })
    }
}

impl<P1, P2> OrParser<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self(p1, p2)
    }
}
