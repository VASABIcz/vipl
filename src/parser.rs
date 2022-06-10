use std::env::args;
use crate::code_gen::FunctionDef;
use crate::data_types::*;
use crate::data_types::Token::*;
use crate::LiteralType::String;
use crate::Operation;
use crate::OperatorType::Plus;

fn parseeee() {
    /*
    required keyword function
    required identifier name
    required separator ORB

    optional identifier
    optional multiple {
        required separator colon
        required identifier
    }
    optional colon

    required separator CRB

    required separator OCB
    // BODY
    required separator CCB
     */
}

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) index: usize,
    pub(crate) ast: Vec<FunctionDef>
}

impl Parser {
    pub fn peek_token(self: &Self) -> Result<Token, std::string::String> {
        return match self.tokens.get(self.index) {
            None => {
                Err(format!("expected token got eofl"))
            }
            Some(v) => {
                Ok(v.clone())
            }
        }
    }

    pub fn get_token(self: &mut Self) -> Result<Token, std::string::String> {
        let res = match self.tokens.get(self.index) {
            None => {
                Err(format!("expected token got eofl"))
            }
            Some(v) => {
                Ok(v.clone())
            }
        };
        self.consume();
        res
    }

    pub fn get_token_assert(self: &mut Self, token: Token) {
        let r = self.get_token().unwrap();
        self.assert_token(r, token)
    }

    pub fn peek_token_equal(self: &Self, token: Token) -> bool {
        println!("token: {:?}",&token);
        let t = self.peek_token().unwrap();
        match token {
            Identifier(_) => {
                match t {
                    Identifier(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            _ => t == token
        }
        // return self.peek_token().unwrap() == token
    }

    pub fn consume(self: &mut Self) {
        self.index += 1;
    }

    pub fn assert_token(self: &Self, token: Token, token1: Token) {
        if token != token1 {
            panic!("expected {:?} got {:?}", token1, token)
        }
    }

    pub fn is_equal(self: &Self, token: Token, token1: Token) -> bool {
        return token == token1
    }

    pub fn get_string(self: &Self, token: Token) -> std::string::String {
        match token {
            Identifier(v) => {
                v
            }
            _ => panic!("expacted literal")
        }
    }

    pub fn get_token_string(self: &mut Self) -> std::string::String {
        match self.get_token().unwrap() {
            Identifier(v) => {
                v
            }
            _ => panic!("expacted literal")
        }
    }

    pub fn parse_return(self: &mut Self) -> Operation {
        unimplemented!()
    }

    pub fn parse_if(self: &mut Self) -> Operation {
        unimplemented!()
    }

    pub fn parse_variable(self: &mut Self) -> Operation {
        unimplemented!()
    }

    pub fn parse_expression(&mut self) -> Operation {
        let mut tokens = Vec::new();
        while !self.peek_token_equal(Token::Separator(SeparatorType::Semicolon)) {
            tokens.push(self.get_token().unwrap())
        }
        // self.get_token();
        Operation::Evaluation {
            exp: self.parse_test(&tokens)
        }
    }

    fn find_split_point(&self, tokens: &[Token]) -> usize {
        let mut index = 0;
        let mut bias = 0;

        for (i, t) in tokens.iter().enumerate() {
            match t {
                Identifier(v) => {

                }
                Operator(v) => {
                    match v {
                        OperatorType::Greater => {
                            unimplemented!()
                        }
                        OperatorType::Less => {
                            unimplemented!()
                        }
                        OperatorType::Unequal => {
                            unimplemented!()
                        }
                        OperatorType::Eql => {
                            unimplemented!()
                        }
                        OperatorType::Eqg => {
                            unimplemented!()
                        }
                        OperatorType::Equal => {
                            unimplemented!()
                        }

                        OperatorType::And => {
                            unimplemented!()
                        }
                        OperatorType::Or => {
                            unimplemented!()
                        }

                        OperatorType::Plus => {
                            // 3
                            if bias < 3 {
                                index = i;
                                break
                            }
                        }
                        OperatorType::Minus => {
                            // 3
                            if bias < 3 {
                                index = i;
                                break
                            }
                        }


                        OperatorType::Mul => {
                            // 1
                            if bias < 1 {
                                index = i
                            }
                        }
                        OperatorType::Div => {
                            // 1
                            if bias < 1 {
                                index = i
                            }
                        }
                    }
                }
                _ => {

                }
            }
        }

        return index
    }

    fn parse_test(&self, tokens: &[Token]) -> Expression {
        let i = self.find_split_point(&tokens);
        for t in tokens {
            println!("tok {:?}", t)
        }
        println!("split {} len {}", i, tokens.len());
        if tokens.len() == 1 {
            println!("yey");
            return match &tokens[i] {
                Identifier(v) => {
                    Expression::Variable(v.clone())
                }
                Literal(v) => {
                    match v {
                        LiteralType::Int(v) => {
                            Expression::Literal(*v)
                        }
                        String(_) => {
                            unimplemented!()
                        }
                    }
                }
                _ => panic!()
            }
        }
        return match &tokens[i] {
            Identifier(v) => {
                Expression::Variable(v.clone())
            }
            Literal(v) => {
                match v {
                    LiteralType::Int(v) => {
                        Expression::Literal(*v)
                    }
                    String(_) => {
                        unimplemented!()
                    }
                }
            }
            _ => {
                match &tokens[i] {
                    Identifier(v) => {
                        if tokens[i] == Token::Separator(SeparatorType::OpeningRoundBracket) {
                            // function
                            unimplemented!()
                        }
                        else {
                            // variable
                            panic!("expected function call got variable")
                        }
                    }
                    Operator(v) => {
                        match v {
                            OperatorType::Greater => {
                                unimplemented!()
                            }
                            OperatorType::Less => {
                                unimplemented!()
                            }
                            OperatorType::Plus => {
                                Expression::Operator(Op::Add(Box::new(self.parse_test(&tokens[0..i])), Box::new(self.parse_test(&tokens[i+1..tokens.len()]))))
                            }
                            OperatorType::Minus => {
                                Expression::Operator(Op::Sub(Box::new(self.parse_test(&tokens[0..i])), Box::new(self.parse_test(&tokens[i+1..tokens.len()]))))
                            }
                            OperatorType::Equal => {
                                unimplemented!()
                            }
                            OperatorType::Unequal => {
                                unimplemented!()
                            }
                            OperatorType::Eql => {
                                unimplemented!()
                            }
                            OperatorType::Eqg => {
                                unimplemented!()
                            }
                            OperatorType::And => {
                                unimplemented!()
                            }
                            OperatorType::Or => {
                                unimplemented!()
                            }
                            OperatorType::Mul => {
                                Expression::Operator(Op::Mul(Box::new(self.parse_test(&tokens[0..i])), Box::new(self.parse_test(&tokens[i+1..tokens.len()]))))
                            }
                            OperatorType::Div => {
                                Expression::Operator(Op::Div(Box::new(self.parse_test(&tokens[0..i])), Box::new(self.parse_test(&tokens[i+1..tokens.len()]))))
                            }
                        }
                    }
                    _ => panic!("e")
                }
            }
        }
    }

    pub fn parse_loop(self: &mut Self) -> Operation {
        unimplemented!()
    }

    pub fn parse_body(self: &mut Self) -> Vec<Operation> {
        let mut operations = Vec::new();

        while !self.peek_token_equal(Token::Separator(SeparatorType::Semicolon)) {
            operations.push(
                match self.peek_token().unwrap() {
                Keyword(v) => {
                    match v {
                        KeywordType::Return => {
                            self.parse_return()
                        }
                        KeywordType::Variable => {
                            self.parse_variable()
                        }
                        KeywordType::Loop => {
                            self.parse_loop()
                        }
                        KeywordType::If => {
                            self.parse_if()
                        }
                        _ => panic!("unexpected keyword while parsing body {:?}", self.peek_token().unwrap())
                    }
                }
                _ => self.parse_expression()
            }
            )
        }

        return operations
    }

    pub fn parse_function(self: &mut Self) -> FunctionDef {
        use std::string::String;
        let mut name = String::new();
        let mut args = Vec::new();
        let mut body: Vec<Operation> = Vec::new();


        self.get_token_assert(Token::Keyword(KeywordType::Function)); // fn
        name = self.get_token_string(); // test
        self.get_token_assert(Token::Separator(SeparatorType::OpeningRoundBracket));
        // FIXME
        if self.peek_token_equal(Token::Identifier("".to_string())) {
            args.push(self.get_token_string());
        }

        while self.peek_token_equal(Token::Separator(SeparatorType::Comma)) {
            self.get_token();
            // FIXME
            if self.peek_token_equal(Token::Identifier("".to_string())) {
                args.push(self.get_token_string());
            }
            else {
                break
            }

        }
        self.get_token_assert(Token::Separator(SeparatorType::ClosingRoundBracket));

        self.get_token_assert(Token::Separator(SeparatorType::OpeningCurlyBracket));

        body = self.parse_body();
        println!("{}", self.index);

        //self.get_token_assert(Token::Separator(SeparatorType::ClosingCurlyBracket));
        let f = FunctionDef {
            body,
            name,
            arguments: args
        };
        self.ast.push(f.clone());

        f
    }
}

// fn gen_ast_function(tokens: &[Token]) {
//     let mut name = String::new();
//     let mut arguments: Vec<String> = Vec::new();
//
//     let t = tokens[0] == Token::Keyword(_);
//
// }

fn gen_ast(tokens: Vec<Token>) {
    for (i, t) in tokens.iter().enumerate() {
        match t {
            Identifier(v) => {}
            Literal(_) => {}
            Token::Keyword(k) => {
                match k {
                    KeywordType::Return => {
                        panic!("expected function got return")
                    }
                    KeywordType::Variable => {
                        panic!("expected function got variable")
                    }
                    KeywordType::Else => {
                        panic!("expected function got else")
                    }
                    KeywordType::Loop => {
                        panic!("expected function got loop")
                    }
                    KeywordType::If => {
                        panic!("expected function got if")
                    }
                    KeywordType::Function => {
                        // gen_ast_function(&tokens)
                    }
                }
            }
            Separator(_) => {}
            Operator(_) => {}
        }
    }
}
