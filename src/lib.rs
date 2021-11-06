//! # Templo Engine
//!
//! Template engine of Templo tool for insert variables inside of text files.

use std::fmt::Error;

use regex::{Captures, Regex};

mod function_call;
mod functions;
mod generator;
mod lexer;
mod parser;
mod syntax_tree;
mod token;
mod utils;

/// Inserts variables into input text and returns it as result.
///
/// # example
///
/// The input text can have some placeholders represented by "{> arg <}". These placeholders will be 
/// used to insert the arguments passed to the engine. The engine provides some native functions
/// to manipulate the argument value.
///
/// input.py
/// ```py
/// class {> upper_first(class_name) <}:
///     def __init__(self):
///     self.name = '{> class_name <}'
///
/// obj = {> upper_first(class_name) <}()
///
/// print(f'The class name is {obj.name}')
///
/// ```
///
/// ## execution
///
/// ```
/// // Getting the input text
/// let input_text = std::fs::read_to_string("./input.py").unwrap();
///
/// // The arguments
/// let arguments: Vec<templo_engine::EngineArg> = vec![
///     templo_engine::EngineArg {
///         key: "class_name".to_string(),
///         value: "dog".to_string(),
///         value_type: templo_engine::EngineArgType::String,
///     }
/// ];
///
/// // Inserting the arguments on text
/// let engine = templo_engine::Engine::new(arguments);
/// let text = engine.compile(input_text);
///
/// // writing the output file
/// std::fs::write("./output.py", text.unwrap()).unwrap();
/// ```
///
/// output.py
/// ```py
/// class Dog:
///     def __init__(self):
///     self.name = 'dog'
///
/// obj = Dog()
///
/// print(f'The class name is {obj.name}')
/// ```
pub struct Engine {
    args: Vec<EngineArg>,
}

impl Engine {
    pub fn new(args: Vec<EngineArg>) -> Self {
        Self { args }
    }

    pub fn compile(&self, text: String) -> Result<String, Error> {
        let exp_reg = Regex::new(r"\{>.*?<}").unwrap();
        let final_text = exp_reg
            .replace_all(&text, |caps: &Captures| {
                let rmv_brackets_reg = Regex::new(r"[{}><\s]").unwrap();
                let exp = rmv_brackets_reg.replace_all(&caps[0], "");

                let tokens = lexer::lex(exp.to_string());
                let syntax_tree = parser::parse(tokens, &self.args);
                let res = generator::generate(syntax_tree).unwrap();

                res
            })
            .to_string();

        Ok(final_text)
    }
}

/// Engine arguments struct
#[derive(Debug, Clone)]
pub struct EngineArg {
    pub key: String,
    pub value: String,
    pub value_type: EngineArgType,
}

#[derive(Debug, Clone)]
pub enum EngineArgType {
    String,
    Integer,
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
        let inputs = vec![EngineArg {
            key: "name".to_string(),
            value: "templo".to_string(),
            value_type: EngineArgType::String,
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
        let text = std::fs::read_to_string("./input.py").unwrap();
        let exp_reg = Regex::new(r"\{>.*?<}").unwrap();

        let final_text = exp_reg
            .replace_all(&text, |caps: &Captures| {
                let rmv_brackets_reg = Regex::new(r"[{}><\s]").unwrap();
                let exp = rmv_brackets_reg.replace_all(&caps[0], "");

                let inputs = vec![EngineArg {
                    key: "class_name".to_string(),
                    value: "cat".to_string(),
                    value_type: EngineArgType::String,
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
