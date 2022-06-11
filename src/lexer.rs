
use std::collections::HashMap;




use std::string::String;

use crate::data_types::*;

// TODO fix this piece of shit
// FIXME numbers are identifiers?????
// FIXME ); is not crb + semicolon but identifier ");"
// fn _main -> keyword identifier identifier
// lexer isn't aware  of token types
// he cares about ascii type
pub struct Lexer {
    keywords: HashMap<String, Token>,
    separators: HashMap<String, Token>,
    operators: HashMap<String, Token>,
    state: State,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        let state = State::None;
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

        let operators = HashMap::new();

        keywords.insert("==".to_string(), Token::Operator(OperatorType::Equal));
        keywords.insert("&&".to_string(), Token::Operator(OperatorType::And));
        keywords.insert("||".to_string(), Token::Operator(OperatorType::Or));
        keywords.insert("+".to_string(), Token::Operator(OperatorType::Plus));
        keywords.insert("-".to_string(), Token::Operator(OperatorType::Minus));
        keywords.insert(">".to_string(), Token::Operator(OperatorType::Greater));
        keywords.insert("<".to_string(), Token::Operator(OperatorType::Less));
        keywords.insert("=>".to_string(), Token::Operator(OperatorType::Eqg));
        keywords.insert("<=".to_string(), Token::Operator(OperatorType::Eql));
        keywords.insert("-".to_string(), Token::Operator(OperatorType::Minus));
        keywords.insert("*".to_string(), Token::Operator(OperatorType::Mul));
        keywords.insert("/".to_string(), Token::Operator(OperatorType::Div));

        let mut separators = HashMap::new();

        separators.insert(
            "(".to_string(),
            Token::Separator(SeparatorType::OpeningRoundBracket),
        );
        separators.insert(
            ")".to_string(),
            Token::Separator(SeparatorType::ClosingRoundBracket),
        );
        separators.insert(
            "[".to_string(),
            Token::Separator(SeparatorType::OpeningSquareBracket),
        );
        separators.insert(
            "]".to_string(),
            Token::Separator(SeparatorType::ClosingSquareBracket),
        );
        separators.insert(
            "{".to_string(),
            Token::Separator(SeparatorType::OpeningCurlyBracket),
        );
        separators.insert(
            "}".to_string(),
            Token::Separator(SeparatorType::ClosingCurlyBracket),
        );
        separators.insert(";".to_string(), Token::Separator(SeparatorType::Semicolon));
        separators.insert(",".to_string(), Token::Separator(SeparatorType::Comma));
        separators.insert(":".to_string(), Token::Separator(SeparatorType::Colon));
        separators.insert("=".to_string(), Token::Separator(SeparatorType::Assign));

        Self {
            keywords,
            separators,
            operators,
            state,
            tokens: vec![],
        }
    }

    pub fn parse(self: &mut Self, text: String) -> Vec<Token> {
        let mut buf = String::new();
        let mut tokens: Vec<Token> = vec![];

        for c in text.chars() {
            if !c.is_ascii() {
                panic!("not ascii")
            } else if c.is_ascii_whitespace() {
                if !buf.is_empty() {
                    tokens.push(self.parse_slice(&buf));
                    buf.clear();
                    self.state = State::None
                }
            } else {
                match self.state {
                    State::Text => {
                        if c.is_ascii_alphabetic() || c == '_' {
                            buf.push(c);
                        } else if c.is_ascii_digit() {
                            tokens.push(self.parse_slice(&buf));
                            buf.clear();
                            buf.push(c);
                            self.state = State::Number
                        } else if c == '"' {
                            tokens.push(self.parse_slice(&buf));
                            buf.clear();
                            self.state = State::Str
                        } else {
                            // println!("{}", buf);
                            tokens.push(self.parse_slice(&buf));
                            buf.clear();
                            buf.push(c);
                            self.state = State::Symbol
                        }
                    }
                    State::Symbol => {
                        if c.is_ascii_alphabetic() {
                            tokens.push(self.parse_slice(&buf));
                            buf.clear();
                            buf.push(c);
                            self.state = State::Text
                        } else if c.is_ascii_digit() {
                            tokens.push(self.parse_slice(&buf));
                            buf.clear();
                            buf.push(c);
                            self.state = State::Number
                        } else if c == '"' {
                            tokens.push(self.parse_slice(&buf));
                            buf.clear();
                            self.state = State::Str
                        } else {
                            buf.push(c);
                        }
                    }
                    State::None => {
                        if c.is_ascii_alphabetic() || c == '_' {
                            buf.push(c);
                            self.state = State::Text
                        } else if c.is_ascii_digit() {
                            buf.push(c);
                            self.state = State::Number
                        } else if c == '"' {
                            buf.clear();
                            self.state = State::Str
                        } else {
                            buf.push(c);
                            self.state = State::Symbol
                        }
                    }
                    State::Number => {
                        if c.is_ascii_alphabetic() {
                            tokens.push(self.parse_number(&buf));
                            buf.clear();
                            buf.push(c);
                            self.state = State::Text
                        } else if c.is_ascii_digit() {
                            buf.push(c);
                        } else if c == '"' {
                            tokens.push(self.parse_number(&buf));
                            buf.clear();
                            self.state = State::Str
                        } else {
                            tokens.push(self.parse_number(&buf));
                            buf.clear();
                            buf.push(c);
                            self.state = State::Symbol
                        }
                    }
                    State::Str => {
                        if c == '"' && buf.chars().last() != Some('\\') {
                            tokens.push(self.parse_string(&buf));
                            buf.clear();
                            self.state = State::None;
                        } else {
                            buf.push(c);
                        }
                    }
                }
            }
        }
        let mut gudt = vec![];
        for t in tokens {
            match t {
                Token::Identifier(ref v) => {
                    if v.is_empty() {
                        continue;
                    }
                    gudt.push(t)
                }
                _ => gudt.push(t),
            }
        }
        return gudt;
    }

    fn parse_keyword(self: &Self, slice: &str) -> Option<&Token> {
        return self.keywords.get(slice);
    }

    fn parse_separator(self: &Self, slice: &str) -> Option<&Token> {
        return self.separators.get(slice);
    }

    fn parse_operator(self: &Self, slice: &str) -> Option<&Token> {
        return self.operators.get(slice);
    }

    fn parse_slice(self: &Self, slice: &str) -> Token {
        return match self.parse_keyword(slice) {
            Some(v) => v.clone(),
            None => {
                match self.parse_operator(slice) {
                    Some(v) => v.clone(),
                    None => {
                        match self.parse_separator(slice) {
                            Some(v) => v.clone(),
                            None => {
                                for _c in slice.chars() {
                                    // if !c.is_ascii_alphabetic() {
                                    //     panic!("{} is invalid token", slice)
                                    // }
                                }
                                Token::Identifier(slice.to_string())
                            }
                        }
                    }
                }
            }
        };
    }
    fn parse_number(self: &mut Self, slice: &str) -> Token {
        match slice.parse::<i32>() {
            Ok(v) => Token::Literal(LiteralType::Int(v)),
            Err(_) => {
                panic!("{} is not valid i32", slice)
            }
        }
    }
    fn parse_string(self: &mut Self, slice: &str) -> Token {
        Token::Literal(LiteralType::String(slice.to_string()))
    }
}
/*
fn main() {
    let mut parser = Parser::new();
    let to_parse = r#"
    fn main() {
        let a = x*y+1;
        let c = "angus";
    }
    "#;
    assert_token(Token::Literal(LiteralType::Int(5)), Token::Keyword(KeywordType::Function));
    parser.parse(to_parse.to_string())
}

 */
