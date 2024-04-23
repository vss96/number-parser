use super::or::OrParser;
use super::token::TokenParser;
use super::Either;
use super::OrOperator;
use super::Parser;

pub type DigitParser = OrParser<
    OrParser<
        OrParser<
            OrParser<
                OrParser<
                    OrParser<
                        OrParser<
                            OrParser<OrParser<TokenParser, TokenParser>, TokenParser>,
                            TokenParser,
                        >,
                        TokenParser,
                    >,
                    TokenParser,
                >,
                TokenParser,
            >,
            TokenParser,
        >,
        TokenParser,
    >,
    TokenParser,
>;
pub type DigitParserReturnType = Either<
    Either<
        Either<
            Either<
                Either<
                    Either<Either<Either<Either<String, String>, String>, String>, String>,
                    String,
                >,
                String,
            >,
            String,
        >,
        String,
    >,
    String,
>;

impl DigitParser {
    pub fn default() -> impl Parser<DigitParserReturnType> {
        let zero = TokenParser::new("0".to_string());
        let one = TokenParser::new("1".to_string());
        let two = TokenParser::new("2".to_string());
        let three = TokenParser::new("3".to_string());
        let four = TokenParser::new("4".to_string());
        let five = TokenParser::new("5".to_string());
        let six = TokenParser::new("6".to_string());
        let seven = TokenParser::new("7".to_string());
        let eight = TokenParser::new("8".to_string());
        let nine = TokenParser::new("9".to_string());
        zero.or(one)
            .or(two)
            .or(three)
            .or(four)
            .or(five)
            .or(six)
            .or(seven)
            .or(eight)
            .or(nine)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl From<DigitParserReturnType> for Digit {
    fn from(value: DigitParserReturnType) -> Self {
        match value {
            Either::Right(_) => Digit::Nine,
            Either::Left(Either::Right(_)) => Digit::Eight,
            Either::Left(Either::Left(Either::Right(_))) => Digit::Seven,
            Either::Left(Either::Left(Either::Left(Either::Right(_)))) => Digit::Six,
            Either::Left(Either::Left(Either::Left(Either::Left(Either::Right(_))))) => Digit::Five,
            Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(
                Either::Right(_),
            ))))) => Digit::Four,
            Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(
                Either::Right(_),
            )))))) => Digit::Three,
            Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(
                Either::Left(Either::Right(_)),
            )))))) => Digit::Two,
            Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(
                Either::Left(Either::Left(Either::Right(_))),
            )))))) => Digit::One,
            Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(Either::Left(
                Either::Left(Either::Left(Either::Left(_))),
            )))))) => Digit::Zero,
        }
    }
}
