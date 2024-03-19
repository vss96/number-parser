mod parser;

use crate::parser::{AndOperator, Digit, DigitParser, Many0, MapOperator, ParseResult, Parser};

fn main() {
    let val = String::from("1234");
    let digit_parser = DigitParser::default().map(|d| Digit::from(d));
    let many0_digit_parser = Many0::new(DigitParser::default().map(|d| Digit::from(d)));
    let many1_digit_parser = digit_parser.and(many0_digit_parser);
    let digits_parser =
        many1_digit_parser.map(|(p, v)| vec![p].into_iter().chain(v).collect::<Vec<Digit>>());

    let number_parser = digits_parser.map(|digits| {
        digits
            .into_iter()
            .fold(0, |acc, digit| acc * 10 + digit as u32)
    });

    let s: ParseResult<u32> = number_parser.parse_from(&val);
    println!("{:?}", s);
}
