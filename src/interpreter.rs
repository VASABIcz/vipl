use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use crate::code_gen::FunctionDef;
use crate::{Expression, LiteralType, Op, Operation};
use crate::Expression::Literal;

// TODO outer scope variable lookup

#[derive(Debug, Clone)]
struct Scope {
    variables: HashMap<std::string::String, LiteralType>,
    scope: Option<Box<Scope>>
}

pub struct Interpreter {
    pub(crate) functions: HashMap<std::string::String, FunctionDef>,
    pub builtin: HashMap<std::string::String, fn(Vec<LiteralType>)>,
    pub heep: HashMap<std::string::String, LiteralType>
}

impl Interpreter {
    pub fn interpret(&mut self, ast: Vec<FunctionDef>) {
        for f in ast {
            self.functions.insert(f.name.clone(), f.clone());
        }

        let f = self.functions.get("main").unwrap().clone();

        self.execute_main(&f)
    }

    fn execute_main(&mut self, f: &FunctionDef) {
        let mut s = Scope { variables: HashMap::new(), scope: None };
        for o in f.body.clone() {
            self.execute_operation(&o, &mut s);
        }
        println!("vars: {:?}", s.variables);
    }

    fn execute_operation(&self, o: &Operation, s: &mut Scope) -> bool {
        match o {
            Operation::Variable { name, exp, typ } => {
                let x = s.scope.clone();
                match x {
                    None => {
                        match s.variables.get(name) {
                            None => {
                                let e = self.evaluate_exp(exp, s);
                                s.variables.insert(name.clone(), e);
                            }
                            Some(v) => {
                                let e = self.evaluate_exp(exp, s);
                                s.variables.insert(name.clone(), e);
                            }
                        }
                    }
                    Some(_) => {
                        let e = self.evaluate_exp(exp, s);
                        s.scope.as_mut().unwrap().variables.insert(name.clone(), e.clone());
                    }
                }
                false
            }
            Operation::Loop { body } => {
                let mut s = Scope { variables: HashMap::new(), scope: Some(Box::new(s.clone())) }; // AHHHHHHHHHHHHHHHHHHHHHHHH
                loop {
                    for o in body.clone() {
                        println!("{:?}", &o);
                        if self.execute_operation(&o, &mut s) {
                            break
                        }
                    }
                    thread::sleep(Duration::from_millis(10))
                }
                false
            }
            Operation::Evaluation { exp } => {
                println!("exp res: {:?}", self.evaluate_exp(exp, s));
                false
            }
            Operation::ControlFlow { exp, yes, no } => {


                if self.evaluate_exp(exp, s) == LiteralType::Int(1) {
                    let mut s = Scope { variables: HashMap::new(), scope: Some(Box::new(s.clone())) };
                    for o in yes.clone() {
                        if self.execute_operation(&o, &mut s) {
                            return true;
                        }
                    }
                }
                else {
                    let mut s = Scope { variables: HashMap::new(), scope: Some(Box::new(s.clone())) };
                    for o in no.clone() {
                        if self.execute_operation(&o, &mut s) {
                            return true;
                        }
                    }
                }
                false
            }
            Operation::Return { .. } => {
                true
            }
            Operation::Break => {
                true
            }
        }
    }

    fn evaluate_exp(&self, e: &Expression, s: &Scope) -> LiteralType {
        return match e {
            Expression::Literal(v) => {
                LiteralType::Int(*v)
            }
            Expression::Operator(op) => {
                match op {
                    Op::Sub(l, r) => {
                        match self.evaluate_exp(l, s) {
                            LiteralType::Int(l) => {
                                match self.evaluate_exp(r, s) {
                                    LiteralType::Int(r) => {
                                        LiteralType::Int(l-r)
                                    }
                                    LiteralType::String(_) => {
                                        unimplemented!()
                                    }
                                }
                            }
                            LiteralType::String(_) => {
                                unimplemented!()
                            }
                        }
                    }
                    Op::Mul(l, r) => {
                        match self.evaluate_exp(l, s) {
                            LiteralType::Int(l) => {
                                match self.evaluate_exp(r, s) {
                                    LiteralType::Int(r) => {
                                        LiteralType::Int(l*r)
                                    }
                                    LiteralType::String(_) => {
                                        unimplemented!()
                                    }
                                }
                            }
                            LiteralType::String(_) => {
                                unimplemented!()
                            }
                        }
                    }
                    Op::Add(l, r) => {
                        match self.evaluate_exp(l, s) {
                            LiteralType::Int(l) => {
                                match self.evaluate_exp(r, s) {
                                    LiteralType::Int(r) => {
                                        println!("wtf {} {}", &l, &r);
                                        LiteralType::Int(l+r)
                                    }
                                    LiteralType::String(_) => {
                                        unimplemented!()
                                    }
                                }
                            }
                            LiteralType::String(_) => {
                                unimplemented!()
                            }
                        }
                    }
                    Op::Div(l, r) => {
                        match self.evaluate_exp(l, s) {
                            LiteralType::Int(l) => {
                                match self.evaluate_exp(r, s) {
                                    LiteralType::Int(r) => {
                                        LiteralType::Int(l/r)
                                    }
                                    LiteralType::String(_) => {
                                        unimplemented!()
                                    }
                                }
                            }
                            LiteralType::String(_) => {
                                unimplemented!()
                            }
                        }
                    }
                    Op::Function(n, args) => {
                        match self.builtin.get(n).clone() {
                            None => {
                                let f = self.functions.get(n).clone().unwrap().clone();
                                let mut s = Scope { variables: HashMap::new(), scope: Some(Box::new(s.clone())) };
                                let mut arguments = vec![];
                                for arg in args {
                                    let e = self.evaluate_exp(arg, &mut s).clone();
                                    arguments.push(e);
                                }
                                for (i, name) in f.arguments.iter().enumerate() {
                                    s.variables.insert(name.clone(), arguments[i].clone());
                                }
                                for o in f.body.clone() {
                                    self.execute_operation(&o, &mut s);
                                }
                                println!("vars: {:?}", s.variables);
                            }
                            Some(f) => {
                                let mut arguments = vec![];
                                for arg in args {
                                    arguments.push(self.evaluate_exp(arg, s).clone());
                                }
                                f(arguments)
                            }
                        }
                        LiteralType::Int(69)
                    }
                }
            }
            Expression::Variable(n) => {
                match &s.scope {
                    None => {
                        let idk = s.variables.get(n).unwrap().clone();
                        println!("sadge {} {:?} {:?}",&n, &idk, &s);
                        idk
                    }
                    Some(s) => {
                        let idk = (*s).variables.get(n).unwrap().clone();
                        println!("Uwu {:?}", &idk);
                        idk
                    }
                }
            }
            Expression::Str(v) => {
                LiteralType::String(v.clone())
            }
        };
    }
}