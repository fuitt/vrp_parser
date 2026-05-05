use super::{Token, TokenError};

pub(crate) fn tokenize(line: &str) -> Result<Token, TokenError> {
    Token::try_from(line)
}
