use regex::{Captures, Regex};

mod function_call;
mod functions;
mod generator;
mod lexer;
mod parser;
mod syntax_tree;
mod token;
mod utils;

#[derive(Debug, Clone)]
pub struct Input {
    key: String,
    value: String,
    value_type: InputValueType,
}

#[derive(Debug, Clone)]
pub enum InputValueType {
    String,
    Integer,
}

pub fn compile(text: String, inputs: &Vec<Input>) -> Result<String, std::io::Error> {
    let exp_reg = Regex::new(r"\{>.*?<}").unwrap();
    let final_text = exp_reg
        .replace_all(&text, |caps: &Captures| {
            let rmv_brackets_reg = Regex::new(r"[{}><\s]").unwrap();
            let exp = rmv_brackets_reg.replace_all(&caps[0], "");

            let tokens = lexer::lex(exp.to_string());
            let syntax_tree = parser::parse(tokens, inputs);
            let res = generator::generate(syntax_tree).unwrap();

            res
        })
        .to_string();

    Ok(final_text)
}

#[cfg(test)]
mod tests {
    use regex::Captures;

    use super::*;
    // const EXP_FN: &'static str =
    //     r#"str(upper_first(name), ' ', upper_first('engine'), ' ', upper_first('owyeah!'))"#;
    const EXP_INPUT: &'static str = r#"name"#;
    #[test]
    fn parser() {
        let inputs = vec![Input {
            key: "name".to_string(),
            value: "templo".to_string(),
            value_type: InputValueType::String,
        }];
        let tokens = lexer::lex(EXP_INPUT.to_string());
        // tokens.iter().for_each(|token| println!("{:?}", token));
        let tree = parser::parse(tokens, &inputs);
        // let _json = serde_json::to_string_pretty(&tree).unwrap();
        // println!("{}", _json);
        let _res = generator::generate(tree).unwrap();
        println!("{}", _res);
    }

    #[test]
    fn miner() {
        let text = std::fs::read_to_string("./test-file.py").unwrap();
        let exp_reg = Regex::new(r"\{>.*?<}").unwrap();

        let final_text = exp_reg
            .replace_all(&text, |caps: &Captures| {
                let rmv_brackets_reg = Regex::new(r"[{}><\s]").unwrap();
                let exp = rmv_brackets_reg.replace_all(&caps[0], "");

                let inputs = vec![Input {
                    key: "class_name".to_string(),
                    value: "cat".to_string(),
                    value_type: InputValueType::String,
                }];

                let tokens = lexer::lex(exp.to_string());
                let syntax_tree = parser::parse(tokens, &inputs);
                let res = generator::generate(syntax_tree).unwrap();

                res
            })
            .to_string();

        println!("{}", final_text);
    }
}
