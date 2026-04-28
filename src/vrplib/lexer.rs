use super::{Token, TokenParseError};

pub fn tokenize(line: &str) -> Result<Token, TokenParseError> {
    Token::try_from(line)
}
