extern crate core;

use std::any::Any;
use std::fmt::{Display, format};
use std::process::id;
use crate::Tree::Operator;

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

#[derive(Debug, Clone)]
enum Op {
    Sub(Box<Tree>, Box<Tree>),
    Mul(Box<Tree>, Box<Tree>),
    Add(Box<Tree>, Box<Tree>),
    Div(Box<Tree>, Box<Tree>),
    Function(String, Box<Option<Tree>>, Box<Option<Tree>>, Box<Option<Tree>>, Box<Option<Tree>>, Box<Option<Tree>>, Box<Option<Tree>>)
}

#[derive(Debug, Clone)]
enum OpType {
    Sub,
    Mul,
    Add,
    Div
}

#[derive(Debug, Clone)]
enum Tree {
    Literal(u32),
    Operator(Op),
    Variable(String)
}

#[derive(Debug, Clone)]
struct State {
    offset: u32,
    generated: String
}

impl State {
    pub fn inc(self: &mut Self) {
        self.offset+=1;
    }

    pub fn dec(self: &mut Self) {
        self.offset+=1;
    }

    pub fn new() -> Self {
        let mut a = Self {
            offset: 0,
            generated: "".to_string()
        };
        a.add_line("; V-I-P-L : v0.0000001".to_string());
        a.add_line("; Vasova".to_string());
        a.add_line("; Insane".to_string());
        a.add_line("; Programing".to_string());
        a.add_line("; Language".to_string());
        a.add_line("; TODO programing language :D\n".to_string());
        return a
    }

    pub fn arithmetic<T: Display, R: Display>(self: &mut Self, op: T, v: R) {
        self.add_line(format!(";//{} {} to [ebp-{}]", op, v, 4*(self.offset-1)));
        self.add_line(format!("mov rax, [ebp-{}]", 4*(self.offset-1)));
        self.add_line(format!("{op} rax, {}", v));
        self.add_line(format!("mov [ebp-{}], rax", 4*(self.offset-1)));
    }

    pub fn arithmetic_previous<T: Display>(self: &mut Self, op: T) {
        self.add_line(format!("mov rax, [ebp-{}]", 4*(self.offset-2)));
        self.add_line(format!("{op} rax, [ebp-{}]", 4*(self.offset-1)));
        self.add_line(format!("mov [ebp-{}], rax", 4*(self.offset-2)));
    }

    fn push<T: Display>(self: &mut Self, v: T) {
        self.add_line(format!("push {}", v));
        self.inc()
    }

    fn pop<T: Display>(self: &mut Self, v: T) {
        self.add_line(format!("pop {}", v));
        self.dec()
    }

    fn pop0(self: &mut Self) {
        self.add_line(String::from("pop"));
        self.dec()
    }

    pub fn add_line(self: &mut Self, s: String) {
        self.generated.push_str(&s);
        self.generated.push_str("\n");
    }

    pub fn code_gen(self: &mut Self, ast: Tree) {
        match ast {
            Tree::Literal(v) => {}
            Tree::Operator(v) => {
                match v {
                    Op::Sub(l, r) => {}
                    Op::Mul(l, r) => {}
                    Op::Add(r, l) => {}
                    Op::Div(l, r) => {}
                    Op::Function(name, a1, a2, a3, a4, a5, a6) => {}
                }
            }
            Tree::Variable(v) => {}
        }
    }

    pub fn gen_invoke_function(self: &mut Self, name: String) {
        self.add_line(format!("call {}", name));
    }

    pub fn handle_idk(self: &mut Self, a: Box<Tree>, free: bool) {
        match *a {
            Tree::Operator(op) => {
                match op {
                    Op::Function(_, _, _, _, _, _, _) => {
                        // unimplemented!();
                        self.handle_function(op)
                    }
                    _ => {
                        let (l, r, op) = match op {
                            Op::Sub(l, r) => {
                                (l, r, String::from("sub"))
                            }
                            Op::Mul(l, r) => {
                                (l, r, String::from("mul"))
                            }
                            Op::Add(l, r) => {
                                (l, r, String::from("add"))
                            }
                            Op::Div(l, r) => {
                                (l, r, String::from("div"))
                            }
                            _ => panic!("idk")
                        };
                        match *l {
                            Tree::Literal(v) => {
                                self.push(v);
                                match *r {
                                    Tree::Literal(v) => {
                                        self.arithmetic(op, v)
                                    }
                                    Tree::Operator(_) => {
                                        self.handle_idk(r, false)
                                    }
                                    Tree::Variable(_) => {
                                        unimplemented!();
                                        // d.push_str(&format!("{op} ebp-{} {}", 4*(self.offset-1), 8))
                                    }
                                }
                                if free {
                                    self.pop0();
                                }
                            }
                            Tree::Operator(_) => {
                                self.handle_idk(l, false);

                                match *r {
                                    Tree::Literal(v) => {
                                        self.arithmetic(op, v);
                                    }
                                    Tree::Operator(_) => {
                                        self.handle_idk(r, false);
                                        self.arithmetic_previous(op);
                                        self.dec();
                                        self.add_line(String::from("pop"))
                                    }
                                    Tree::Variable(_) => {
                                        unimplemented!();
                                        // d.push_str(&format!("{op} [ebp-{}] {}", 4*(self.offset-1), 5))
                                    }
                                }
                            }
                            Tree::Variable(v) => {
                                self.push(v);
                                match *r {
                                    Tree::Literal(v) => {
                                        self.arithmetic(op, v);
                                    }
                                    Tree::Operator(_) => {
                                        self.handle_idk(r, false)
                                    }
                                    Tree::Variable(_) => {
                                        unimplemented!();
                                        // d.push_str(&format!("{op} [ebp-{}] {}", 4*(self.offset-1), 8))
                                    }
                                }
                                if free {
                                    self.pop0();
                                }
                            }
                        }
                    }
                }
            }
            _ => panic!("expected operator")
        }
    }

    pub fn handle_argument(self: &mut Self, a: Box<Tree>) {
        match *a {
            Tree::Literal(v) => {
                self.push(v);
            }
            Tree::Operator(_) => {
                let mut xd = 1;
                self.handle_idk(a.clone(), false);
            }
            Tree::Variable(v) => {
                unimplemented!();
                format!("push {}", 5);
            }
        }
    }

    pub fn handle_function(self: &mut Self, function: Op) {
        match function {
            Op::Function(name, a1, a2, a3, a4, a5, a6) => {
                if !(*a1).is_none() {
                    self.add_line(format!("; argument 1 for {}", name));
                    self.handle_argument((Box::new((*a1).unwrap())))
                }
                if !(*a2).is_none() {
                    self.add_line(format!("; argument 2 for {}", name));
                    self.handle_argument((Box::new((*a2).unwrap())))
                }
                self.gen_invoke_function(name)
            }
            _ => panic!("not a function")
        }
    }
}

// x = test(5-y*3, (7-3)*(5+8))
pub fn main() {
    // test(5-y*3, (7-3)*(5+8))
    let idk = Op::Function(
        String::from("test"),
        Box::new(
            Some(
            Tree::Operator(
                Op::Sub(
                    Box::new(
                        Tree::Operator(
                            Op::Mul(
                                Box::new(Tree::Literal(69)),
                                Box::new(Tree::Literal(5))
                            )
                        )
                    ),
                    Box::new(Tree::Literal(3))
                )
            )
        )),
        Box::new(Some(
            Tree::Operator(
                Op::Mul(
                    Box::new(Tree::Operator(
                        Op::Sub(
                            Box::new(Tree::Literal(7)),
                            Box::new(Tree::Literal(3))
                        ),
                    )),
                    Box::new(Tree::Operator(
                        Op::Add(
                            Box::new(Tree::Literal(5)),
                            Box::new(Tree::Literal(8))
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
            Some(Tree::Operator(Op::Add(
                Box::new(Tree::Operator(Op::Add(
                    Box::new(Tree::Literal(53)),
                    Box::new(Tree::Literal(69))
                ))
                ),
                Box::new(Tree::Operator(Op::Add(
                    Box::new(Tree::Literal(53)),
                    Box::new(Tree::Literal(69))
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

    let mut state = State::new();
    let res = state.handle_function(test2);
    println!("\n");
    println!("{}", state.generated);
}
