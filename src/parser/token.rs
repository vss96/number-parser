use super::Parser;

#[derive(Clone)]
pub struct TokenParser {
    token: String,
}

impl TokenParser {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Parser<String> for TokenParser {
    fn parse_from(&self, val: &String) -> super::ParseResult<String> {
        if val.starts_with(&self.token) {
            Ok((self.token.clone(), val[self.token.len()..].to_string()))
        } else {
            Err(format!("Could not find token: {}", self.token))
        }
    }
}
