//!
//! The lexical analyzer.
//!

#[cfg(test)]
mod tests;

pub(crate) mod error;
pub(crate) mod stream;
pub(crate) mod token;

pub use self::error::Error;
pub use self::stream::TokenStream;
pub use self::token::lexeme::identifier::Identifier;
pub use self::token::lexeme::keyword::Keyword;
pub use self::token::lexeme::literal::boolean::Boolean as BooleanLiteral;
pub use self::token::lexeme::literal::integer::Integer as IntegerLiteral;
pub use self::token::lexeme::literal::string::String as StringLiteral;
pub use self::token::lexeme::literal::Literal;
pub use self::token::lexeme::symbol::Symbol;
pub use self::token::lexeme::Lexeme;
pub use self::token::location::Location;
pub use self::token::Token;