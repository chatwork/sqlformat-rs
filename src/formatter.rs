use crate::tokenizer::{Token, TokenKind};

pub(crate) fn format(tokens: &[Token<'_>]) -> String {
    let mut formatted_query = String::new();
    for token in tokens {
        if token.kind == TokenKind::Whitespace {
            formatted_query += " ";
        } else {
            formatted_query += token.value;
        }
    }
    formatted_query.trim().to_string()
}
