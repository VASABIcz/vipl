extern crate core;

use std::any::Any;
use std::fmt::{Display, format};
use std::process::id;
use crate::Expression::Operator;

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
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Function(String, Box<Option<Expression>>, Box<Option<Expression>>, Box<Option<Expression>>, Box<Option<Expression>>, Box<Option<Expression>>, Box<Option<Expression>>)
}

#[derive(Debug, Clone)]
enum OpType {
    Sub,
    Mul,
    Add,
    Div
}

#[derive(Debug, Clone)]
enum Expression {
    Literal(u32),
    Operator(Op),
    Variable(String)
}

#[derive(Debug, Clone)]

struct State {
    offset: u32,
    generated: String
}

enum VariableType {
    Static,
    Var
}

enum Operation{
    // special type return, break
    Variable {  // FIXME variable declaration / assignment
        typ: VariableType,
        name: String,
        exp: Expression
    },
    Loop {
        body: Body
    },
    Evaluation {
        exp: Expression
    },
    ControlFlow {
        exp: Expression,
        yes: Body,
        no: Body
    }
}

struct Body {
    state: State, // FIXME
    body: Vec<Operation>
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
        // self.add_line(format!(";//{} {} to [ebp-{}]", op, v, 4*(self.offset-1)));
        self.add_line(format!("mov eax, [ebp-{}]", 4*(self.offset-1)));
        self.add_line(format!("{op} eax, {}", v));
        self.add_line(format!("mov dword [ebp-{}], eax", 4*(self.offset-1)));
    }

    pub fn arithmetic_previous<T: Display>(self: &mut Self, op: T) {
        self.add_line(format!("mov eax, [ebp-{}]", 4*(self.offset-2)));
        self.add_line(format!("{op} eax, [ebp-{}]", 4*(self.offset-1)));
        self.add_line(format!("mov dword [ebp-{}], eax", 4*(self.offset-2)));
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
        self.add_line(String::from("pop edx"));
        self.dec()
    }

    pub fn add_line(self: &mut Self, s: String) {
        self.generated.push_str(&s);
        self.generated.push_str("\n");
    }

    pub fn asm2sym(asm: &str) -> &str {
        return match asm {
            "add" => "+",
            "imul" => "*",
            "sub" => "-",
            "div" => "/",
            _ => {
                eprintln!("unknown asm {} in asm2sym", asm);
                "UNKNOWN"
            }
        }
    }

    pub fn code_gen(self: &mut Self, ast: Expression) {
        match ast {
            Expression::Literal(v) => {}
            Expression::Operator(v) => {
                match v {
                    Op::Sub(l, r) => {}
                    Op::Mul(l, r) => {}
                    Op::Add(r, l) => {}
                    Op::Div(l, r) => {}
                    Op::Function(name, a1, a2, a3, a4, a5, a6) => {}
                }
            }
            Expression::Variable(v) => {}
        }
    }

    pub fn gen_invoke_function(self: &mut Self, name: String) {
        self.add_line(format!("call {}", name));
    }

    pub fn handle_arithmetics(self: &mut Self, a: Box<Expression>, free: bool) {
        match *a {
            Expression::Operator(op) => {
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
                                (l, r, String::from("imul"))
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
                            Expression::Literal(v1) => {
                                self.push(v1);
                                match *r {
                                    Expression::Literal(v) => {
                                        self.add_line(format!("; {v1} {} {v}", State::asm2sym(&op)));
                                        self.arithmetic(&op, v);
                                    }
                                    Expression::Operator(_) => {
                                        self.handle_arithmetics(r, false)
                                    }
                                    Expression::Variable(_) => {
                                        unimplemented!();
                                        // d.push_str(&format!("{op} ebp-{} {}", 4*(self.offset-1), 8))
                                    }
                                }
                                if free {
                                    self.pop0();
                                }
                            }
                            Expression::Operator(_) => {
                                self.handle_arithmetics(l, false);

                                match *r {
                                    Expression::Literal(v) => {
                                        self.arithmetic(op, v);
                                    }
                                    Expression::Operator(_) => {
                                        self.handle_arithmetics(r, false);
                                        self.arithmetic_previous(op);
                                        self.pop0();
                                    }
                                    Expression::Variable(_) => {
                                        unimplemented!();
                                        // d.push_str(&format!("{op} [ebp-{}] {}", 4*(self.offset-1), 5))
                                    }
                                }
                            }
                            Expression::Variable(v) => {
                                self.push(v);
                                match *r {
                                    Expression::Literal(v) => {
                                        self.arithmetic(op, v);
                                    }
                                    Expression::Operator(_) => {
                                        self.handle_arithmetics(r, false)
                                    }
                                    Expression::Variable(_) => {
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

    pub fn handle_argument(self: &mut Self, a: Box<Expression>) {
        match *a {
            Expression::Literal(v) => {
                self.push(v);
            }
            Expression::Operator(_) => {
                self.handle_arithmetics(a.clone(), false);
            }
            Expression::Variable(v) => {
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

    pub fn handle_all(self: &mut Self, op: Box<Expression>) {
        match *op {
            Expression::Literal(v) => {
                self.handle_arithmetics(op, true);
            }
            Operator(v) => {
                self.handle_arithmetics(op.clone(), true);
            }
            Expression::Variable(v) => {
                self.handle_arithmetics(op.clone(), true);
            }
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

    let mut state = State::new();
    let res = state.handle_function(idk);
    println!("\n");
    println!("{}", state.generated);
}
