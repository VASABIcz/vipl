
use crate::Token::{Separator};
use crate::{print_list, KeywordType, LiteralType, OperatorType, SeparatorType, Token};
use std::collections::HashMap;
use std::string::String;

pub struct Lexer {
    text: String,
    index: usize,
    tokens: Vec<Token>,
    buf: String,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn get_char(&mut self) -> char {
        let res = self.text.as_bytes()[self.index] as char;
        self.index += 1;
        res
    }

    pub fn peek_char(&self) -> char {
        self.text.as_bytes()[self.index] as char
    }

    pub fn has_char(&self) -> bool {
        self.index < self.text.len()
    }

    pub fn new(text: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("fn".to_string(), Token::Keyword(KeywordType::Function));
        keywords.insert("if".to_string(), Token::Keyword(KeywordType::If));
        keywords.insert("else".to_string(), Token::Keyword(KeywordType::Else));
        keywords.insert("True".to_string(), Token::Literal(LiteralType::Int(1)));
        keywords.insert("False".to_string(), Token::Literal(LiteralType::Int(0)));
        keywords.insert("return".to_string(), Token::Keyword(KeywordType::Return));
        keywords.insert("var".to_string(), Token::Keyword(KeywordType::Variable));
        keywords.insert("let".to_string(), Token::Keyword(KeywordType::Variable));
        keywords.insert("loop".to_string(), Token::Keyword(KeywordType::Loop));
        keywords.insert("break".to_string(), Token::Keyword(KeywordType::Break));
        Self {
            text,
            index: 0,
            tokens: vec![],
            buf: String::new(),
            keywords,
        }
    }

    pub fn flush_buf(&mut self) {
        if self.buf.len() > 0 {
            let v = self.parse_buf();
            self.tokens.push(v);
        }
        self.buf.clear();
    }

    pub fn parse_buf(&mut self) -> Token {
        return match self.parse_keyword() {
            Some(v) => v.clone(),
            None => match self.parse_number() {
                Some(v) => v.clone(),
                None => Token::Identifier(self.buf.to_string()),
            },
        };
    }

    fn parse_keyword(self: &Self) -> Option<&Token> {
        return self.keywords.get(&self.buf);
    }

    fn parse_number(self: &mut Self) -> Option<Token> {
        match self.buf.parse::<i32>() {
            Ok(v) => Some(Token::Literal(LiteralType::Int(v))),
            Err(_) => None,
        }
    }

    pub fn buf_get(&mut self) {
        let v = self.get_char();
        self.buf.push(v)
    }

    pub fn add(&mut self, token: Token) {
        self.tokens.push(token)
    }

    pub fn parse(&mut self) {
        while self.has_char() {
            match self.get_char() {
                '}' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::ClosingCurlyBracket));
                }
                '{' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::OpeningCurlyBracket));
                }
                ']' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::ClosingSquareBracket));
                }
                '[' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::OpeningSquareBracket));
                }
                '(' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::OpeningRoundBracket));
                }
                ')' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::ClosingRoundBracket));
                }
                ';' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::Semicolon));
                }
                ',' => {
                    self.flush_buf();
                    self.add(Separator(SeparatorType::Colon));
                }
                '+' => {
                    self.flush_buf();
                    self.add(Token::Operator(OperatorType::Plus));
                }
                '-' => {
                    self.flush_buf();
                    self.add(Token::Operator(OperatorType::Minus));
                }
                '*' => {
                    self.flush_buf();
                    self.add(Token::Operator(OperatorType::Mul));
                }
                '/' => {
                    self.flush_buf();
                    self.add(Token::Operator(OperatorType::Div));
                }
                '=' => {
                    self.flush_buf();
                    self.add(Token::Separator(SeparatorType::Assign));
                }
                '\'' => {
                    while self.peek_char() != '\'' {
                        self.buf_get();
                    }
                    self.get_char();
                    self.flush_buf()
                }
                '\"' => {
                    while self.peek_char() != '\"' {
                        self.buf_get();
                    }
                    self.get_char();
                    self.flush_buf()
                }
                ref i if { i.is_whitespace() } => self.flush_buf(),
                v => self.buf.push(v),
            }
        }
        print_list(&self.tokens);
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

pub fn test(to_lex: String) {
    let mut lexer = Lexer::new(to_lex);
    lexer.parse();
}
