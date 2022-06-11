use crate::data_types::*;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CodeGen {
    offset: u32,
    pub(crate) generated: String,
    variables: HashMap<String, u32>,
}
#[derive(Debug, Clone)]
pub struct Body {
    state: CodeGen, // FIXME
    body: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub enum Operation {
    // special type return, break
    Variable {
        // FIXME variable declaration / assignment
        typ: VariableType,
        name: String,
        exp: Expression,
    },
    Loop {
        body: Body,
    },
    Evaluation {
        exp: Expression,
    },
    ControlFlow {
        exp: Expression,
        yes: Body,
        no: Body,
    },
    Return {
        exp: Expression,
    },
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub(crate) body: Vec<Operation>,
    pub(crate) name: String,
    pub(crate) arguments: Vec<String>,
}

impl CodeGen {
    pub fn inc(self: &mut Self) {
        self.offset += 1;
    }

    pub fn dec(self: &mut Self) {
        self.offset += 1;
    }

    pub fn new() -> Self {
        let mut a = Self {
            offset: 0,
            generated: "".to_string(),
            variables: HashMap::new(),
        };
        a.add_line("; V-I-P-L : v0.0000001".to_string());
        a.add_line("; Vasova".to_string());
        a.add_line("; Insane".to_string());
        a.add_line("; Programing".to_string());
        a.add_line("; Language".to_string());
        a.add_line("; TODO programing language :D\n".to_string());
        return a;
    }

    pub fn arithmetic<T: Display, R: Display>(self: &mut Self, op: T, v: R) {
        // self.add_line(format!(";//{} {} to [ebp-{}]", op, v, 4*(self.offset-1)));
        self.add_line(format!("mov eax, [ebp-{}]", 4 * (self.offset - 1)));
        self.add_line(format!("{op} eax, {}", v));
        self.add_line(format!("mov dword [ebp-{}], eax", 4 * (self.offset - 1)));
    }

    pub fn arithmetic_previous<T: Display>(self: &mut Self, op: T) {
        self.add_line(format!("mov eax, [ebp-{}]", 4 * (self.offset - 2)));
        self.add_line(format!("{op} eax, [ebp-{}]", 4 * (self.offset - 1)));
        self.add_line(format!("mov dword [ebp-{}], eax", 4 * (self.offset - 2)));
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

    pub fn get_variable_ptr(self: &mut Self, name: &str) -> Option<String> {
        return match self.variables.get(name) {
            None => None,
            Some(v) => Some(format!("ebp-{}", v * 4)),
        };
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
        };
    }

    pub fn gen_invoke_function(self: &mut Self, name: String) {
        self.add_line(format!("call {}", name));
    }

    pub fn gen_expression(self: &mut Self, a: Box<Expression>, free: bool) {
        match *a {
            Expression::Operator(op) => {
                match op {
                    Op::Function(_, _) => {
                        // unimplemented!();
                        self.handle_function_call(op)
                    }
                    _ => {
                        let (l, r, op) = match op {
                            Op::Sub(l, r) => (l, r, String::from("sub")),
                            Op::Mul(l, r) => (l, r, String::from("imul")),
                            Op::Add(l, r) => (l, r, String::from("add")),
                            Op::Div(l, r) => (l, r, String::from("div")),
                            _ => panic!("idk"),
                        };
                        match *l {
                            Expression::Literal(v1) => {
                                self.push(v1);
                                match *r {
                                    Expression::Literal(v) => {
                                        self.add_line(format!(
                                            "; {v1} {} {v}",
                                            CodeGen::asm2sym(&op)
                                        ));
                                        self.arithmetic(&op, v);
                                    }
                                    Expression::Operator(_) => self.gen_expression(r, false),
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
                                self.gen_expression(l, false);

                                match *r {
                                    Expression::Literal(v) => {
                                        self.arithmetic(op, v);
                                    }
                                    Expression::Operator(_) => {
                                        self.gen_expression(r, false);
                                        self.arithmetic_previous(op);
                                        self.pop0();
                                    }
                                    Expression::Variable(_) => {
                                        unimplemented!();
                                        // d.push_str(&format!("{op} [ebp-{}] {}", 4*(self.offset-1), 5))
                                    }
                                }

                                if free {
                                    self.pop0()
                                }
                            }
                            Expression::Variable(v) => {
                                self.push(v);
                                match *r {
                                    Expression::Literal(v) => {
                                        self.arithmetic(op, v);
                                    }
                                    Expression::Operator(_) => self.gen_expression(r, false),
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
            Expression::Literal(v) => {
                self.push(v);
                if free {
                    self.pop0();
                }
            }
            Expression::Variable(v) => {
                let b = self.get_variable_ptr(&v).unwrap();
                self.push(format!("[{}]", b)); // FIXME
                if free {
                    self.pop0();
                }
            }
        }
    }

    pub fn handle_argument(self: &mut Self, op: Box<Expression>) {
        self.gen_expression(op, false);
    }

    pub fn handle_function_call(self: &mut Self, function: Op) {
        match function {
            Op::Function(name, args) => {
                for (i, a) in args.iter().enumerate() {
                    self.add_line(format!("; argument {} for {}", i, name));
                    self.handle_argument(Box::new(a.clone()));
                }
                self.gen_invoke_function(name)
            }
            _ => panic!("not a function"),
        }
    }
    pub fn gen_variable_dec(self: &mut Self, name: &str, typ: VariableType, size: u32) {
        match typ {
            VariableType::Static => {
                unimplemented!()
            }
            VariableType::Var => {
                self.add_line(format!("; variable {}", name));
                for _ in 0..size {
                    self.push(0);
                }
                self.variables.insert(name.to_string(), self.offset - 1);
            }
        }
    }

    pub fn handle_body(self: &mut Self, body: Vec<Operation>) {
        for o in body {
            match o {
                Operation::Variable { typ: _, name, exp } => {
                    if self.get_variable_ptr(&name) == None {
                        self.gen_variable_dec(&name, VariableType::Var, 1);
                    }
                    self.gen_expression(Box::new(exp), false);
                    self.pop("eax");
                    let b = self.get_variable_ptr(&name).unwrap();
                    self.add_line(format!("mov dword [{}], eax", b)) // FIXME
                }
                Operation::Loop { .. } => {
                    unimplemented!()
                }
                Operation::Evaluation { exp } => self.gen_expression(Box::new(exp), true),
                Operation::ControlFlow { .. } => {
                    unimplemented!()
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }
}
