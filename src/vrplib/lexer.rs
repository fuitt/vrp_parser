use super::{Token, TokenParseError};

pub(crate) fn tokenize(line: &str) -> Result<Token, TokenParseError> {
    Token::try_from(line)
}
