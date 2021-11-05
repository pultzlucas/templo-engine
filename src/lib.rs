mod function_call;
mod functions;
mod lexer;
mod parser;
mod syntax_tree;
mod token;
mod utils;
mod generator;

#[derive(Debug, Clone)]
pub struct Input {
    key: String,
    value: String,
    value_type: InputValueType
}


#[derive(Debug, Clone)]
pub enum InputValueType {
    String,
    Integer
}

pub fn compile(buffer: String, inputs: Vec<Input>) -> Result<String, std::io::Error> {
    let _tokens = lexer::lex(buffer);
    let _syntax_tree = parser::parse(_tokens, inputs);
    let _res = generator::generate(_syntax_tree)?;
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXP: &'static str = r#"str(upper_first(name), ' ', upper_first('engine'), ' ', upper_first('owyeah!'))"#;
    #[test]
    fn lib() {
        let inputs = vec![
            Input {
                key: "name".to_string(),
                value: "templo".to_string(),
                value_type: InputValueType::String
            }
        ];
        let tokens = lexer::lex(EXP.to_string());
        // tokens.iter().for_each(|token| println!("{:?}", token));
        let tree = parser::parse(tokens, inputs);
        // let _json = serde_json::to_string_pretty(&tree).unwrap();
        // println!("{}", _json);
        let _res = generator::generate(tree).unwrap();
        println!("{}", _res);

    }
}
