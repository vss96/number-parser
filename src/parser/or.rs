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

pub type Or2<U, V> = OrParser<U, V>;
pub type Or3<U, V, W> = Or2<U, Or2<V, W>>;
pub type Or4<U, V, W, X> = Or2<U, Or3<V, W, X>>;
pub type Or5<U, V, W, X, Y> = Or2<U, Or4<V, W, X, Y>>;
pub type Or6<U, V, W, X, Y, Z> = Or2<U, Or5<V, W, X, Y, Z>>;
pub type Or7<A, B, C, D, E, F, G> = Or2<A, Or6<B, C, D, E, F, G>>;
pub type Or8<A, B, C, D, E, F, G, H> = Or2<A, Or7<B, C, D, E, F, G, H>>;
pub type Or9<A, B, C, D, E, F, G, H, I> = Or2<A, Or8<B, C, D, E, F, G, H, I>>;
pub type Or10<A, B, C, D, E, F, G, H, I, J> = Or2<A, Or9<B, C, D, E, F, G, H, I, J>>;
// we don't have recursive types unless you use Box
