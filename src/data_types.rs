

use std::fmt::{Debug};

#[derive(Debug, Clone)]
pub enum Op {
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Function(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum OpType {
    Sub,
    Mul,
    Add,
    Div,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(i32),
    Operator(Op),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum VariableType {
    Static,
    Var,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeywordType {
    Return,
    Variable,
    Else,
    Loop,
    If,
    Function,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SeparatorType {
    OpeningRoundBracket,
    ClosingRoundBracket,
    OpeningSquareBracket,
    ClosingSquareBracket,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    Semicolon,
    Comma,
    Colon,
    Assign,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperatorType {
    Greater,
    Less,
    Plus,
    Minus,
    Equal,
    Unequal,
    Eql,
    Eqg,
    And,
    Or,
    Mul,
    Div,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiteralType {
    // Float(f64),
    Int(i32),
    String(String),
    // Bool(bool)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    Literal(LiteralType),
    Keyword(KeywordType),
    Separator(SeparatorType),
    Operator(OperatorType),
}

pub enum State {
    Text,
    Symbol,
    Number,
    Str,
    None,
}

pub fn print_list<V: Debug>(list: &[V]) {
    for i in list {
        println!("{:?}", i);
    }
    println!()
}
