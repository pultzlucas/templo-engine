//! # Templo Engine
//!
//! Template engine of Templo tool for insert variables inside of text files.

use std::io::Error;

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
/// to manipulate the argument value as well.
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
/// use templo_engine::*;
/// 
/// // Getting the input text
/// let input_text = std::fs::read_to_string("./input.py").unwrap();
///
/// // The arguments
/// let arguments = vec![
///     EngineArg {
///         key: String::from("class_name"),
///         value: String::from("dog"),
///         value_type: EngineArgType::String,
///     }
/// ];
///
/// // Inserting the arguments on text
/// let engine = Engine::new(arguments);
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
        let exp_catcher = Regex::new(r"\{>.*?<}").unwrap();
        let final_text = exp_catcher.replace_all(&text, |exp: &Captures| {
            Engine::process_expression(&self.args, exp)
        });
        Ok(final_text.to_string())
    }

    fn process_expression(args: &Vec<EngineArg>, exp: &Captures) -> String {
        // clean expression (remove the side brackets)
        let rmv_brackets_reg = Regex::new(r"[{}><\s]").unwrap();
        let exp = rmv_brackets_reg.replace_all(&exp[0], "");

        // tokenize expression
        let tokens = lexer::lex(exp.to_string());

        // build expression syntax tree
        let syntax_tree = parser::parse(tokens, args);

        // generate output text
        let res = generator::generate(syntax_tree).unwrap();

        res.node
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
    use super::*;

    const TEXT: &str = "{> get_index(word, get_char(0, word)) <}";

    #[test]
    fn engine_test() {
        let args = vec![EngineArg {
            key: "word".to_string(),
            value: "Hello".to_string(),
            value_type: EngineArgType::String,
        }];
        let engine = Engine::new(args);
        let out = engine.compile(TEXT.to_string()).unwrap();
        assert_eq!(out, "0")
    }
}