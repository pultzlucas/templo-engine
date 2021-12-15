use std::fmt::Debug;

#[cfg(test)]
mod tests;

mod functions;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    String,
    Integer,
    Nil
}

pub use functions::upper::Upper;
pub use functions::lower::Lower;
pub use functions::join::Join;
pub use functions::upper_first::UpperFirst;
pub use functions::get_char::GetChar;
pub use functions::get_index::GetIndex;
pub use functions::EngineFunction;




