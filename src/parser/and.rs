use super::{ParseResult, Parser};

impl<P1, P2, X, Y> Parser<(X, Y)> for (P1, P2)
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
