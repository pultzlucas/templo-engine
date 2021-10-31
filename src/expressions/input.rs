use crate::token::{Token, TokenType};

pub struct Input {
    value: String,
}

impl Input {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn is_input_exp(token: &Token) -> bool {
        if token.type_ == TokenType::Input {
            return true;
        }
        false
    }
}
