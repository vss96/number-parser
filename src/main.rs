mod parser;

use parser::{And2, MapParser};

use crate::parser::{Digit, DigitParser, Many0, ParseResult, Parser};

fn main() {
    let val = String::from("1234");
    let digit_parser = MapParser::new(DigitParser::default(), |d| Digit::from(d));
    let many1_digit_parser = And2::new(digit_parser.clone(), Many0::new(digit_parser));
    let digits_parser = MapParser::new(many1_digit_parser, |(p, v)| {
        vec![p].into_iter().chain(v).collect::<Vec<Digit>>()
    });

    let number_parser = MapParser::new(digits_parser, |digits| {
        digits
            .into_iter()
            .fold(0, |acc, digit| acc * 10 + digit as u32)
    });

    let s: ParseResult<u32> = number_parser.parse_from(&val);
    println!("{:?}", s);
}
