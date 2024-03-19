use std::iter::Cloned;

use super::{
    and::{And2, AndParser},
    ParseResult, Parser,
};

pub struct Many0<T>(T);

impl<T, U> Parser<Vec<U>> for Many0<T>
where
    T: Parser<U>,
{
    fn parse_from(&self, val: &String) -> ParseResult<Vec<U>> {
        let mut init = val.clone();
        let mut p_list = vec![];
        loop {
            let res = self.0.parse_from(&init);
            match res {
                Ok((p, follow)) => {
                    p_list.push(p);
                    init = follow;
                }
                Err(_) => {
                    return Ok((p_list, init));
                }
            }
        }
    }
}

impl<T> Many0<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

pub type Many1<T> = And2<T, Many0<T>>;

pub type Many1ReturnType<T> = ParseResult<(T, Vec<T>)>;

#[cfg(test)]
mod test_many0 {
    use crate::parser::{
        digit::{Digit, DigitParser},
        many::Many0,
        Parser,
    };

    #[test]
    fn should_parse_many_when_available() {
        let val = String::from("12345");
        let digit_parser = DigitParser::default();
        let res = Many0(digit_parser).parse_from(&val);
        let digits = res.map(|f| {
            (
                f.0.into_iter()
                    .map(|x| Digit::from(x))
                    .collect::<Vec<Digit>>(),
                f.1,
            )
        });
        assert_eq!(
            Ok((
                vec![
                    Digit::One,
                    Digit::Two,
                    Digit::Three,
                    Digit::Four,
                    Digit::Five
                ],
                String::from("")
            )),
            digits
        );
    }
}

#[cfg(test)]
mod test_many1 {
    use crate::parser::{
        digit::{Digit, DigitParser},
        many::{Many0, Many1},
        Parser,
    };

    #[test]
    fn should_have_atleast_one_digit() {
        let val = String::from("1abc");
        let digit_parser = DigitParser::default();
        let res = Many1::new(digit_parser.clone(), Many0::new(digit_parser)).parse_from(&val);
        let digits = res.map(|((p, v), follow)| {
            (
                vec![p]
                    .into_iter()
                    .chain(v)
                    .map(|x| Digit::from(x))
                    .collect::<Vec<Digit>>(),
                follow,
            )
        });

        assert_eq!(Ok((vec![Digit::One,], String::from("abc"))), digits);
    }

    #[test]
    fn should_err_if_no_digits_found() {
        let val = String::from("abc");
        let digit_parser = DigitParser::default();
        let res = Many1::new(digit_parser.clone(), Many0::new(digit_parser)).parse_from(&val);

        assert_eq!(
            Err(String::from("Could not find token: 9")), //last token is 9
            res
        );
    }
}
