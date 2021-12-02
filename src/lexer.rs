use super::token::{Token, TokenType};
use regex::Regex;

pub fn lex(input: String) -> Vec<Token> {
    // REGEXES
    let alphanum_reg = Regex::new(r"^\s*\w+$").unwrap();
    let string_reg = Regex::new(r"^\s*'.*[^(\\')]*.*'$").unwrap();
    let integer_reg = Regex::new(r"^\d+$").unwrap();
    let input_reg = Regex::new(r"^\s*[^\d]\w+").unwrap();

    let input = format!("{}\0", input);
    let chars = input.chars().into_iter();

    let mut buffer = String::new();
    let mut tokens: Vec<Token> = vec![];

    for ch in chars {
        // FUNCTION
        if alphanum_reg.is_match(&buffer) && ch == '(' {
            tokens.push(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::Function,
            ));
            buffer = String::new();
        }

        // STRING
        if string_reg.is_match(&buffer) && !buffer.ends_with(r"\'") {
            tokens.push(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::String,
            ));
            buffer = String::new();
        }

        // INPUT
        if input_reg.is_match(&buffer) && !alphanum_reg.is_match(&ch.to_string()) && ch != '\'' {
            tokens.push(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::Input,
            ));
            buffer = String::new();
        }
        
        if integer_reg.is_match(&buffer) && !ch.is_numeric() {
            tokens.push(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::Integer,
            ));
            buffer = String::new();
        }

        // SYMBOLS
        let token = match buffer.as_str() {
            "(" => Some(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::BracketLeft,
            )),
            ")" => Some(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::BracketRight,
            )),
            "," => Some(Token::new(
                buffer.clone().trim().to_string(),
                TokenType::Separator,
            )),
            _ => None,
        };

        if let Some(token) = token {
            tokens.push(token);
            buffer = String::new();
        }

        buffer.push_str(&ch.to_string());
    }

    tokens
}
