mod lexer;
mod parser;
mod code_gen;
mod data_types;

extern crate core;

use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::{Display, format};
use std::ops::Bound::Excluded;
use std::process::id;
use crate::code_gen::{CodeGen, FunctionDef, Operation};
use crate::data_types::*;
use crate::lexer::*;
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
 */



// x = test(5-y*3, (7-3)*(5+8))
pub fn main() {
    // test(5-y*3, (7-3)*(5+8))
    let idk = Op::Function(
        String::from("test"),
        Box::new(
            Some(
            Expression::Operator(
                Op::Sub(
                    Box::new(
                        Expression::Operator(
                            Op::Mul(
                                Box::new(Expression::Literal(69)),
                                Box::new(Expression::Literal(5))
                            )
                        )
                    ),
                    Box::new(Expression::Literal(3))
                )
            )
        )),
        Box::new(Some(
            Expression::Operator(
                Op::Mul(
                    Box::new(Expression::Operator(
                        Op::Sub(
                            Box::new(Expression::Literal(7)),
                            Box::new(Expression::Literal(3))
                        ),
                    )),
                    Box::new(Expression::Operator(
                        Op::Add(
                            Box::new(Expression::Literal(5)),
                            Box::new(Expression::Literal(8))
                        ),
                    ))
                )
            )
        )),
        Box::new(None),
        Box::new(None),
        Box::new(None),
        Box::new(None),
    );

    let test2 = Op::Function(
        String::from("test"),
        Box::new(
            Some(Expression::Operator(Op::Add(
                Box::new(Expression::Operator(Op::Add(
                    Box::new(Expression::Literal(53)),
                    Box::new(Expression::Literal(69))
                ))
                ),
                Box::new(Expression::Operator(Op::Add(
                    Box::new(Expression::Literal(53)),
                    Box::new(Expression::Literal(69))
                ))
                )
            ))
        )),
        Box::new(None),
        Box::new(None),
        Box::new(None),
        Box::new(None),
        Box::new(None)
    );
    let test3 = Operation::Variable {
        typ: VariableType::Static,
        name: "angus".to_string(),
        exp: Expression::Operator(
            Op::Add(
                Box::new(Expression::Literal(5)),
                Box::new(Expression::Literal(7))
            )
        )
    };
    let mut state = CodeGen::new();
    //let res = state.handle_function_call(idk);
    state.handle_body(vec![test3]);
    let mut lexer = Lexer::new();
    let to_parse = r#"
    fn main(a) {
     6*8-3;
    }
    "#;
    let tokens = lexer.parse(to_parse.to_string());
    println!("tokens: \n");
    for x in &tokens {
        println!("{:?}", x);
    }
    let mut parser = Parser {
        tokens,
        index: 0,
        ast: vec![]
    };
    parser.parse_function();
    println!("ast: \n");
    for x in &parser.ast {
        println!("{:?}", x);
    }
    match &parser.ast[0] {
        FunctionDef { body, .. } => {
            let mut cd = CodeGen::new();
            cd.handle_body(body.clone());
            println!("{}", cd.generated);
        }
    }
}
