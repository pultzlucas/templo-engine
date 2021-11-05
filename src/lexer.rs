use super::token::{Token, TokenType};
use regex::Regex;

pub fn lex(input: String) -> Vec<Token> {
    // REGEXES
    let alphanum_reg = Regex::new(r"\w+").unwrap();

    let input = format!("{}\0", input);
    let chars = input.chars().into_iter();

    let mut buffer = String::new();
    let mut tokens: Vec<Token> = vec![];

    // loop to get the tokens
    for ch in chars {
        
        let valid_symbols = ["(", ")", ",", "$", " ", "\0"].join("");
        let valid_chars_reg = Regex::new(&format!(r"[\w{}]", valid_symbols)).unwrap();
        
        if !valid_chars_reg.is_match(&ch.to_string()) {
            panic!("Invalid char '{}'", ch);
        }

        // Skip whitespace
        if ch == ' ' {
            continue;
        }

        // FUNCTION
        if alphanum_reg.is_match(&buffer) && ch == '(' {
            tokens.push(Token::new(buffer.clone(), TokenType::Function));
            buffer = String::new();
        }

        // INPUT
        if alphanum_reg.is_match(&buffer) && !alphanum_reg.is_match(&ch.to_string()) {
            tokens.push(Token::new(buffer.clone(), TokenType::Input));
            buffer = String::new();
        }

        // SYMBOLS
        let token = match buffer.as_str() {
            "(" => Some(Token::new(buffer.clone(), TokenType::BracketLeft)),
            ")" => Some(Token::new(buffer.clone(), TokenType::BracketRight)),
            "," => Some(Token::new(buffer.clone(), TokenType::Comma)),
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
