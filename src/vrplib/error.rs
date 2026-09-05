use super::lexer::TokenError;
use super::parser::ParseError;

/// Represents errors that can occur during VRPLib loading.
///
/// Wraps either a tokenization failure or a structural parse failure,
/// both of which are specific to the VRPLib format.
#[derive(Debug, thiserror::Error)]
pub enum VrplibError {
    /// A tokenization error occurred during lexical analysis.
    #[error("token error: {0}")]
    Token(TokenError),

    /// A parse error occurred while interpreting the token stream.
    #[error("parse error: {0}")]
    Parse(ParseError),
}
