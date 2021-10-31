mod functions;
mod lexer;
mod token;
mod expressions;
mod utils;
mod expression;
mod parser;
mod syntax_tree;

pub fn compile(input: String) -> Result<String, std::io::Error> {
    let _tokens = lexer::lex(input);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "JOIN(UPPER($var1, LOWER($var3, $var4), $var2)), $oi)";

    #[test]
    fn lexer_test() {
        let tokens = lexer::lex(INPUT.to_string());
        println!("{:?}", tokens);
    }

    
    #[test]
    fn parser_test() {
        let tokens = lexer::lex(INPUT.to_string());
        parser::parse(&tokens);
    }

    #[test]
    fn syntax_tree_test() {
        let tokens = lexer::lex(INPUT.to_string());
        syntax_tree::build_tree(tokens);
    }
}
