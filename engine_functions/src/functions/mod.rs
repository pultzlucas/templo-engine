use crate::ValueType;
use std::fmt::Debug;

pub mod upper;
pub mod lower;
pub mod join;
pub mod upper_first;
pub mod get_char;
pub mod get_index;

pub trait EngineFunction: Debug {
    fn call(&self, args: &Vec<String>) -> String;
    // fn validate_args(&self, args: &Vec<String>) -> Result<(), Error>;
    fn return_type(&self) -> ValueType {
        ValueType::Nil
    }

    fn params_type(&self) -> Option<Vec<ValueType>>;
    fn params_quant(&self) -> usize {
        if let Some(params) = self.params_type() {
            params.len()
        } else {
            0
        }
    }
}
