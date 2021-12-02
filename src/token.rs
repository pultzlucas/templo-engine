#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
}

impl Token {
    pub fn new(value: String, type_: TokenType) -> Self {
        Self { value, type_ }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Function,
    BracketLeft,
    BracketRight,
    Separator,
    Input,
    String,
    Integer
}
