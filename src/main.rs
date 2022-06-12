mod code_gen;
mod data_types;
mod lexer;
mod lexer_prototype;
mod parser;
mod interpreter;

extern crate core;

use std::borrow::Borrow;
use std::collections::HashMap;
use crate::code_gen::{CodeGen, Operation};
use crate::data_types::*;
// use crate::lexer::*;
use crate::lexer_prototype::*;
use crate::parser::Parser;
use crate::interpreter::*;

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

fn print(args: Vec<LiteralType>) -> () {
    println!("builtin print");
    for a in args {
        println!("{:?}", a);
    }
}

// x = test(5-y*3, (7-3)*(5+8))
pub fn main() {
    let to_parse = r#"
    fn main() {
        let i = 0;
        loop {
            let i = i + 1;
            if (i-10) {
                break;
            }
        }
    }
    "#;
    let mut lexer = Lexer::new(to_parse.to_string());
    lexer.parse();
    let tokens = lexer.tokens();
    println!("tokens: ");
    print_list(&tokens);
    let mut parser = Parser {
        tokens,
        index: 0,
        ast: vec![],
    };
    parser.parse();
    let ast = parser.ast;
    println!("ast: {:?}", &ast);
    let mut builtin = HashMap::new();
    builtin.insert("print".to_string(), print as fn(Vec<LiteralType>));

    let mut i = Interpreter {
        functions: HashMap::new(),
        builtin: builtin.clone()
    };
    i.interpret(ast)


    // let mut g = CodeGen::new();
    // g.code_gen(ast);
    // let asm = g.generated;
    // println!("asm: {}", &asm)
}
