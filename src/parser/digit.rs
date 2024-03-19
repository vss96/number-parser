use crate::parser::or::Or10;

use super::or::Or2;
use super::or::Or3;
use super::or::Or4;
use super::or::Or5;
use super::or::Or6;
use super::or::Or7;
use super::or::Or8;
use super::or::Or9;

use super::token::TokenParser;
use super::Either;

pub type DigitParser = Or10<
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
    TokenParser,
>;
pub type DigitParserReturnType = Either<
    String,
    Either<
        String,
        Either<
            String,
            Either<
                String,
                Either<
                    String,
                    Either<String, Either<String, Either<String, Either<String, String>>>>,
                >,
            >,
        >,
    >,
>;

impl Default for DigitParser {
    fn default() -> Self {
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
        Or10::new(
            zero,
            Or9::new(
                one,
                Or8::new(
                    two,
                    Or7::new(
                        three,
                        Or6::new(
                            four,
                            Or5::new(five, Or4::new(six, Or3::new(seven, Or2::new(eight, nine)))),
                        ),
                    ),
                ),
            ),
        )
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
            Either::Left(_) => Digit::Zero,
            Either::Right(Either::Left(_)) => Digit::One,
            Either::Right(Either::Right(Either::Left(_))) => Digit::Two,
            Either::Right(Either::Right(Either::Right(Either::Left(_)))) => Digit::Three,
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Left(_))))) => {
                Digit::Four
            }
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Right(
                Either::Left(_),
            ))))) => Digit::Five,
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Right(
                Either::Right(Either::Left(_)),
            ))))) => Digit::Six,
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Right(
                Either::Right(Either::Right(Either::Left(_))),
            ))))) => Digit::Seven,
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Right(
                Either::Right(Either::Right(Either::Right(Either::Left(_)))),
            ))))) => Digit::Eight,
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Right(
                Either::Right(Either::Right(Either::Right(Either::Right(_)))),
            ))))) => Digit::Nine,
        }
    }
}
