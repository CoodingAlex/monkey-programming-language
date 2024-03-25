pub enum TokenType {
    EOF,
}

pub struct Token {
    token_type: TokenType,
    literal: String,
}
