use super::token::Token;
use crate::{expressions::function_call::FunctionCall, token::TokenType};
pub struct Expression {
    pub tokens: Vec<Token>,
    pub exp_type: ExpressionType,
}

pub enum ExpressionType {
    FunctionCall,
    Input,
}

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        if !Self::parse(&tokens) {
            panic!("Tokens vector especified was not an expression.");
        }

        let exp_type = if let Some(exp_type) = Self::get_exp_type(&tokens) {
            exp_type
        } else {
            panic!("Cannot find an expression compatible with tokens.");
        };

        Self { exp_type, tokens }
    }

    pub fn parse(tokens: &Vec<Token>) -> bool {
        if tokens.is_empty() {
            return true;
        }

        if tokens[0].type_ == TokenType::Function || tokens[0].type_ == TokenType::Input {
            return true;
        }

        false
    }

    fn get_exp_type(tokens: &Vec<Token>) -> Option<ExpressionType> {
        if FunctionCall::is_fn_call_exp(tokens) {
            return Some(ExpressionType::FunctionCall);
        }

        if tokens[0].type_ == TokenType::Input {
            return Some(ExpressionType::Input);
        }
        None
    }
}
