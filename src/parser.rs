use crate::code_gen::{FunctionDef};
use crate::data_types::Token::*;
use crate::data_types::*;
use crate::LiteralType::String;
use crate::{CodeGen, Operation};

// TODO variable assignment
// TODO better errors?

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) index: usize,
    pub(crate) ast: Vec<FunctionDef>,
}

impl Parser {
    pub fn peek_token(self: &Self) -> Result<Token, std::string::String> {
        return match self.tokens.get(self.index) {
            None => Err(format!("expected token got eol")),
            Some(v) => Ok(v.clone()),
        };
    }

    pub fn get_token(self: &mut Self) -> Result<Token, std::string::String> {
        let res = match self.tokens.get(self.index) {
            None => Err(format!("expected token got eol")),
            Some(v) => Ok(v.clone()),
        };
        self.consume();
        res
    }

    pub fn get_token_assert(self: &mut Self, token: Token) {
        let r = self.get_token().unwrap();
        self.assert_token(r, token)
    }

    pub fn peek_token_equal(self: &Self, token: Token) -> bool {
        println!("peek token: {:?}", &token);
        let t = self.peek_token().unwrap();
        match token {
            Identifier(_) => match t {
                Identifier(_) => true,
                _ => false,
            },
            _ => t == token,
        }
        // return self.peek_token().unwrap() == token
    }

    pub fn consume(self: &mut Self) {
        self.index += 1;
    }

    pub fn has_token(&self) -> bool {
        self.tokens.len() > self.index
    }

    pub fn assert_token(self: &Self, token: Token, token1: Token) {
        if token != token1 {
            panic!("expected {:?} got {:?}", token1, token)
        }
    }

    pub fn is_equal(self: &Self, token: Token, token1: Token) -> bool {
        return token == token1;
    }

    pub fn get_string(self: &Self, token: Token) -> std::string::String {
        match token {
            Identifier(v) => v,
            _ => panic!("exacted Identifier got {:?}", token),
        }
    }

    pub fn get_token_string(self: &mut Self) -> std::string::String {
        let t = self.get_token().unwrap();
        self.get_string(t)
    }

    pub fn parse_return(self: &mut Self) -> Operation {
        self.get_token_assert(Token::Keyword(KeywordType::Return));
        let exp = self.parse_expression();
        self.get_token_assert(Token::Separator(SeparatorType::Semicolon));
        return match exp {
            Operation::Evaluation { exp } => Operation::Return { exp },
            _ => panic!("expected expression got {:?}", exp),
        };
    }

    pub fn parse_if(self: &mut Self) -> Operation {
        self.get_token_assert(Token::Keyword(KeywordType::If));
        self.get_token_assert(Token::Separator(SeparatorType::OpeningRoundBracket));
        let toks = self
            .reed_expression_until(|t| t == &Token::Separator(SeparatorType::ClosingRoundBracket));
        let exp = self.parse_test(&toks);
        self.get_token_assert(Token::Separator(SeparatorType::ClosingRoundBracket));
        self.get_token_assert(Token::Separator(SeparatorType::OpeningCurlyBracket));
        let if_body = self.parse_body();
        // self.get_token_assert(Token::Separator(SeparatorType::ClosingCurlyBracket));
        let mut else_body = vec![];
        if self.peek_token_equal(Token::Keyword(KeywordType::Else)) {
            self.get_token_assert(Token::Keyword(KeywordType::Else));
            self.get_token_assert(Token::Separator(SeparatorType::OpeningCurlyBracket));
            else_body = self.parse_body();
            //self.get_token_assert(Token::Separator(SeparatorType::ClosingCurlyBracket));
        }
        Operation::ControlFlow {
            exp,
            yes: if_body,
            no: else_body,
        }
    }

    pub fn parse_variable(self: &mut Self) -> Operation {
        println!("parsing var");
        let typ = if self.peek_token_equal(Token::Keyword(KeywordType::Variable)) {
            self.get_token();
            VariableType::Var
        }
        else {
            self.get_token_assert(Token::Keyword(KeywordType::ConstVar));
            VariableType::Static
        };
        let name = self.get_token_string();
        self.get_token_assert(Token::Separator(SeparatorType::Assign));
        let exp = self.parse_expression();
        self.get_token_assert(Token::Separator(SeparatorType::Semicolon));
        return match exp {
            Operation::Evaluation { exp } => Operation::Variable {
                typ,
                name,
                exp,
            },
            _ => panic!("expected expression got {:?}", exp),
        };
    }
    pub fn reed_expression_until(self: &mut Self, filter: fn(&Token) -> bool) -> Vec<Token> {
        println!("reed_expression_until {}", self.index);
        print_list(&self.tokens);
        let mut ignore_depth: u32 = 0; // negative depth isn't possible just crash lol
        let mut res = vec![];

        loop {
            let t = self.peek_token().unwrap();
            if ignore_depth == 0 && filter(&t) {
                break;
            }
            match t {
                Token::Separator(v) => match v {
                    SeparatorType::OpeningRoundBracket => {
                        ignore_depth += 1;
                    }
                    SeparatorType::ClosingRoundBracket => {
                        ignore_depth -= 1;
                    }
                    _ => {}
                },
                _ => {}
            }
            res.push(self.get_token().unwrap())
        }
        return res;
    }

    pub fn parse_function_call(&mut self, tokens: &[Token]) -> Expression {
        let mut p = Parser {
            tokens: tokens.to_vec(),
            index: 0,
            ast: vec![],
        };
        let mut args = vec![];

        let name = p.get_token_string();
        p.get_token_assert(Token::Separator(SeparatorType::OpeningRoundBracket));

        while !p.peek_token_equal(Token::Separator(SeparatorType::ClosingRoundBracket)) {
            let toks = p.reed_expression_until(|t| {
                if t == &Token::Separator(SeparatorType::ClosingRoundBracket)
                    || t == &Token::Separator(SeparatorType::Comma)
                {
                    // FIXME
                    true
                } else {
                    false
                }
            });
            println!("gona parse argz");
            print_list(&toks);
            args.push(p.parse_test(&toks));
            if p.peek_token_equal(Token::Separator(SeparatorType::Comma)) {
                p.get_token().unwrap();
            }
        }
        p.get_token().unwrap();
        Expression::Operator(Op::Function(name, args))
    }

    pub fn parse_expression(&mut self) -> Operation {
        println!("parse expr:");
        print_list(&self.tokens[self.index..self.tokens.len()]);
        let mut tokens = Vec::new();
        while !self.peek_token_equal(Token::Separator(SeparatorType::Semicolon)) {
            tokens.push(self.get_token().unwrap())
        }
        println!("exp to be parsed");
        print_list(&tokens);
        Operation::Evaluation {
            // FIXME not sure
            exp: self.parse_test(&tokens),
        }
    }

    fn find_split_point(&self, tokens: &[Token]) -> usize {
        // FIXME
        // 90% of operators arent implemented
        let mut index = 0;
        let mut bias = 0;
        let mut ignore_depth = 0; // used for handling brackets, function calls

        for (i, t) in tokens.iter().enumerate() {
            match &t {
                Identifier(_v) => {
                    if tokens.len() >= i + 2 {
                        if tokens[i + 1] == Token::Separator(SeparatorType::OpeningRoundBracket) {
                            continue;
                        }
                    }
                }
                Separator(v) => match v {
                    SeparatorType::OpeningRoundBracket => {
                        ignore_depth += 1;
                        continue;
                    }
                    SeparatorType::ClosingRoundBracket => {
                        ignore_depth -= 1;
                        if ignore_depth < 0 {
                            panic!("idk what to type here")
                        }
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
            if ignore_depth == 0 {
                match t {
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
                                    bias = 3;
                                    index = i;
                                    break;
                                }
                            }
                            OperatorType::Minus => {
                                // 3
                                if bias < 3 {
                                    bias = 3;
                                    index = i;
                                    break;
                                }
                            }

                            OperatorType::Mul => {
                                // 1
                                if bias < 1 {
                                    bias = 1;
                                    index = i
                                }
                            }
                            OperatorType::Div => {
                                // 1
                                if bias < 1 {
                                    bias = 1;
                                    index = i
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        return index;
    }

    fn parse_test(&mut self, tokens: &[Token]) -> Expression {
        println!("parse test:");
        print_list(&tokens);
        let i = self.find_split_point(&tokens);

        if i == 0 && tokens.len() > 1 {
            match &tokens[i] {
                Identifier(v) => {
                    if tokens[i + 1] == Token::Separator(SeparatorType::OpeningRoundBracket) {
                        println!("parsing function");
                        print_list(&tokens);
                        return self.parse_function_call(&tokens);
                    } else {
                        // println!("hah {}", v);
                        return Expression::Variable(v.clone());
                        // panic!("expected expression got {:?}", &tokens[i])
                    }
                }
                Separator(v) => {
                    if v == &SeparatorType::OpeningRoundBracket {
                        return self.parse_test(&tokens[1..tokens.len() - 1]);
                    } else {
                        panic!("unexpected {:?}", v);
                    }
                }
                Token::Literal(LiteralType::String(v)) => {
                    return Expression::Str(v.clone());
                }
                _ => panic!()
            }
        }

        // println!("split {} len {}", i, tokens.len());
        // if tokens.len() == 1 {
        //     println!("yey");
        //     return match &tokens[i] {
        //         Identifier(v) => {
        //             Expression::Variable(v.clone())
        //         }
        //         Literal(v) => {
        //             match v {
        //                 LiteralType::Int(v) => {
        //                     Expression::Literal(*v)
        //                 }
        //                 String(_) => {
        //                     unimplemented!()
        //                 }
        //             }
        //         }
        //         _ => panic!()
        //     }
        // }
        return match &tokens[i] {
            Identifier(v) => Expression::Variable(v.clone()),
            Literal(v) => match v {
                LiteralType::Int(v) => Expression::Literal(*v),
                String(v) => {
                    Expression::Str(v.clone())
                    //unimplemented!()
                }
            },
            _ => {
                match &tokens[i] {
                    Identifier(_v) => {
                        if tokens[i] == Token::Separator(SeparatorType::OpeningRoundBracket) {
                            if self.peek_token_equal(Token::Separator(
                                SeparatorType::ClosingRoundBracket,
                            )) {
                                // (5*8+(3-7), a+5)
                                match self.get_token().unwrap() {
                                    Identifier(_v) => {}
                                    Separator(_v) => {}
                                    _ => panic!(""),
                                }
                            }
                            // function
                            unimplemented!()
                        } else {
                            // variable
                            panic!("expected function call got variable")
                        }
                    }
                    Operator(v) => match v {
                        OperatorType::Greater => {
                            unimplemented!()
                        }
                        OperatorType::Less => {
                            unimplemented!()
                        }
                        OperatorType::Plus => Expression::Operator(Op::Add(
                            Box::new(self.parse_test(&tokens[0..i])),
                            Box::new(self.parse_test(&tokens[i + 1..tokens.len()])),
                        )),
                        OperatorType::Minus => Expression::Operator(Op::Sub(
                            Box::new(self.parse_test(&tokens[0..i])),
                            Box::new(self.parse_test(&tokens[i + 1..tokens.len()])),
                        )),
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
                        OperatorType::Mul => Expression::Operator(Op::Mul(
                            Box::new(self.parse_test(&tokens[0..i])),
                            Box::new(self.parse_test(&tokens[i + 1..tokens.len()])),
                        )),
                        OperatorType::Div => Expression::Operator(Op::Div(
                            Box::new(self.parse_test(&tokens[0..i])),
                            Box::new(self.parse_test(&tokens[i + 1..tokens.len()])),
                        )),
                    },
                    _ => panic!("invalid token in expression {:?}", &tokens[i]),
                }
            }
        };
    }

    pub fn parse_loop(self: &mut Self) -> Operation {
        self.get_token_assert(Token::Keyword(KeywordType::Loop));
        self.get_token_assert(Token::Separator(SeparatorType::OpeningCurlyBracket));
        let body = self.parse_body();
        // self.get_token_assert(Token::Separator(SeparatorType::ClosingCurlyBracket));

        Operation::Loop {
            body,
        }
    }

    pub fn parse_break(self: &mut Self) -> Operation {
        self.get_token_assert(Token::Keyword(KeywordType::Break));
        self.get_token_assert(Token::Separator(SeparatorType::Semicolon));

        Operation::Break
    }

    pub fn parse_body_expr(&mut self) -> Operation {
        let o = self.parse_expression();
        self.get_token_assert(Token::Separator(SeparatorType::Semicolon));
        println!("after body parse");
        print_list(&self.tokens[self.index..self.tokens.len()]);
        o
    }

    pub fn parse_body(self: &mut Self) -> Vec<Operation> {
        let mut operations = Vec::new();

        while !self.peek_token_equal(Token::Separator(SeparatorType::ClosingCurlyBracket)) {
            // FIXME
            println!("body {:?}", self.peek_token().unwrap());
            operations.push(match self.peek_token().unwrap() {
                Keyword(v) => match v {
                    KeywordType::Return => self.parse_return(),
                    KeywordType::Variable => self.parse_variable(),
                    KeywordType::Loop => self.parse_loop(),
                    KeywordType::If => self.parse_if(),
                    KeywordType::Break => self.parse_break(),
                    KeywordType::ConstVar => self.parse_variable(),
                    _ => panic!(
                        "unexpected keyword while parsing body {:?}",
                        self.peek_token().unwrap()
                    ),
                },
                _ => {
                    println!("OOF");
                    self.parse_body_expr()
                }
            });
        }
        self.get_token_assert(Token::Separator(SeparatorType::ClosingCurlyBracket));
        println!("angus");
        return operations;
    }

    pub fn parse_function(self: &mut Self) -> FunctionDef {
        let mut args = Vec::new();

        self.get_token_assert(Token::Keyword(KeywordType::Function)); // fn
        let name = self.get_token_string(); // test
        self.get_token_assert(Token::Separator(SeparatorType::OpeningRoundBracket));
        // FIXME
        if self.peek_token_equal(Token::Identifier("".to_string())) {
            args.push(self.get_token_string());
        }

        while self.peek_token_equal(Token::Separator(SeparatorType::Comma)) {
            self.get_token().unwrap();
            // FIXME
            if self.peek_token_equal(Token::Identifier("".to_string())) {
                args.push(self.get_token_string());
            } else {
                break;
            }
        }
        self.get_token_assert(Token::Separator(SeparatorType::ClosingRoundBracket));

        self.get_token_assert(Token::Separator(SeparatorType::OpeningCurlyBracket));

        let body = self.parse_body();
        // println!("{}", self.index);

        // self.get_token_assert(Token::Separator(SeparatorType::ClosingCurlyBracket));
        let f = FunctionDef {
            body,
            name,
            arguments: args,
        };
        self.ast.push(f.clone());

        f
    }

    pub fn parse(&mut self) {
        while self.has_token() {
            self.parse_function();
        }
    }
}

// fn gen_ast_function(tokens: &[Token]) {
//     let mut name = String::new();
//     let mut arguments: Vec<String> = Vec::new();
//
//     let t = tokens[0] == Token::Keyword(_);
//
// }
