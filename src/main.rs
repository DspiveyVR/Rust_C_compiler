use std::fs;

pub mod lexer;
pub use crate::lexer::lexer::lex;
pub use crate::lexer::lexer::{Token, KeywordType};

pub mod parser;
pub use crate::parser::parser::parse;
pub use crate::parser::parser::{Program, StatementType, ExpressionType};

//pub mod code_generation;
//pub use crate::code_generation::code_generation::generate_assembly;


fn main() {
    let path = "/home/Dspivey/Programming/rust_projects/c_compiler/return_2.c";
    let infile = fs::read_to_string(path).expect("Unable to read file");

    //generate_assembly(parse(lex(&infile)));
    parse(lex(&infile));
}
