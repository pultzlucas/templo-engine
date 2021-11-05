mod expression;
mod expressions;
mod functions;
mod lexer;
mod parser;
mod syntax_tree;
mod token;
mod utils;
mod generator;

pub fn compile(input: String) -> Result<String, std::io::Error> {
    let _tokens = lexer::lex(input);
    let _syntax_tree = parser::parse(_tokens);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    // UPPER_FIRST($oi1, $oi2, FN2($oi3, $oi4))
    // LOWER(FN($var2, $var3), $eae, JOIN($var4)))
    // UPPER(LOWER(FN($a)))
    // (join $var1 $var2)
    const INPUT: &'static str = "UPPER($oi, UPPER_FIRST($arg1, JOIN($arg3), $arg2), $a, $b)";

    #[test]
    fn lib() {
        let tokens = lexer::lex(INPUT.to_string());
        let tree = parser::parse(tokens);
        let _json = serde_json::to_string_pretty(&tree).unwrap();
        println!("{}", _json);
        // let _res = generator::generate(tree).unwrap();
        // println!("{}", res);

    }
}
