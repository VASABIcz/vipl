mod code_gen;
mod data_types;
mod lexer;
mod parser;
mod lexer_prototype;

extern crate core;

use crate::code_gen::{CodeGen, FunctionDef, Operation};
use crate::data_types::*;
// use crate::lexer::*;
use crate::lexer_prototype::*;
use crate::parser::Parser;







/*
DONE
absolute minimal assembly code gen from ast?
TODO
add support for variables
could be saved to hashmap with stack offset
variable declaration
variable assignment
TODO
parser
ast gen
TODO
read about standards of functions in asm
track function calls for dynamic stack resizing
currently now way to call function and get stack size pre call
function oriented stack
TODO
heep/static allocation
function that would not be saved as stack offset rather heep pointer
TODO
strings somehow
TODO
learn assembly i still dont know how to read that shit or how it works
TODO
function declaration


fn main() {
    let test = 5*8*(5-2);
    // valid expr lit, operator, separator[(),], identifier
    if ((3/3)*(4-2)) {
        angus(5*(3-test(5*8))+5, (2+6)/5);
    }
    else {
        return test;
    }
    return 0;
}


 */

// x = test(5-y*3, (7-3)*(5+8))
pub fn main() {
    let to_parse = r#"
    fn __main(){
        let a=79+38*test(91);
    }
    "#;
    let mut lexer = Lexer::new(to_parse.to_string());
    lexer.parse();
    let tokens = lexer.tokens();
    let mut parser = Parser {
        tokens,
        index: 0,
        ast: vec![]
    };
    let ast = parser.parse_function();
    println!("ast: {:?}", &ast)
}
