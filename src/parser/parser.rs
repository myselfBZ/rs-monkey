use crate::{
    ast::ast::{self, Expression, Statement},
    lexer::lexer,
    token::token,
};
use std::fmt;

pub struct Parser {
    cur_tok: token::Token,
    peek_tok: token::Token,
    lexer: Box<lexer::Lexer>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
}

impl fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Precedence::Lowest => write!(f, "Lowest: 0"),
            Precedence::Equals => write!(f, "Equals: 1"),
            Precedence::LessGreater => write!(f, "LessGreater: 2"),
            Precedence::Sum => write!(f, "Sum: 3"),
            Precedence::Product => write!(f, "Product: 4"),
        }
    }
}

impl Parser {
    pub fn new(mut lexer: Box<lexer::Lexer>) -> Self {
        let cur_tok = lexer.next_token();
        let peek_tok = lexer.next_token();
        Parser {
            lexer: Box::new(*lexer),
            cur_tok,
            peek_tok,
        }
    }

    fn parse_ident(&mut self) -> ast::Expression {
        ast::Expression::Ident(self.cur_tok.to_string())
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let tok = self.cur_tok.clone();
        let exprs = self.parse_expression(Precedence::Lowest);
        if self.peek_tok == token::Token::Semicolon {
            self.next_token();
        }
        Some(ast::Statement::ExprsStatement { token: tok, exprs })
    }

    fn parse_grouped_expression(&mut self) -> ast::Expression {
        self.next_token();
        let node = self.parse_expression(Precedence::Lowest);
        if self.peek_tok == token::Token::Rparen {
            self.next_token();
        }
        node
    }

    fn parse_expression(&mut self, prec: Precedence) -> ast::Expression {
        let mut left = match &self.cur_tok {
            token::Token::Int(_) => self.parse_int(),
            token::Token::True => self.parse_bool(),
            token::Token::Func => self.parse_func(),
            token::Token::False => self.parse_bool(),
            token::Token::Minus => self.parse_prefix_ops(),
            token::Token::Bang => self.parse_prefix_ops(),
            token::Token::Lparen => self.parse_grouped_expression(),
            token::Token::If => self.parse_if(),
            token::Token::Ident(_) => self.parse_ident(),
            _ => ast::Expression::NoExprsn,
        };


        while self.cur_tok != token::Token::Semicolon
            && self.token_to_precedence(self.peek_tok.clone()) > prec
        {
            match &self.peek_tok {
                token::Token::Plus
                | token::Token::Minus
                | token::Token::Asterisk
                | token::Token::Slash
                | token::Token::Gt
                | token::Token::Lt
                | token::Token::Eq
                | token::Token::NotEq => {
                    self.next_token();
                    left = self.parse_infix(left)
                }

                _ => return Expression::NoExprsn,
            }
        }

        left
    }

    fn parse_params(&mut self) -> Vec<ast::Expression> {
        let mut params: Vec<Expression> = vec![];
        self.next_token();

        if self.peek_tok == token::Token::Rparen {
            return params;
        }

        self.next_token();

        let ident = ast::Expression::Ident(self.cur_tok.to_string());

        params.push(ident);

        while self.peek_tok == token::Token::Comma {
            self.next_token();
            self.next_token();
            let ident = self.parse_ident();
            params.push(ident);
        }

        params
    }

    fn parse_func(&mut self) -> ast::Expression {
        let params = self.parse_params();
        self.next_token();
        self.next_token();
        let body = self.parse_block();
        ast::Expression::FnExprsn { params, body }
    }

    fn parse_bool(&self) -> ast::Expression {
        ast::Expression::Boolean {
            token: self.cur_tok.clone(),
            value: self.cur_tok == token::Token::True,
        }
    }

    fn parse_int(&mut self) -> ast::Expression {
        let literal = match &self.cur_tok {
            token::Token::Int(s) => s,
            _ => return ast::Expression::NoExprsn,
        };
        let int: i32 = match literal.parse() {
            Ok(n) => n,
            Err(_s) => return ast::Expression::NoExprsn,
        };
        ast::Expression::Int(int)
    }

    fn parse_statemnt(&mut self) -> Option<ast::Statement> {
        match self.cur_tok {
            token::Token::Let => self.parse_let(),
            token::Token::Return => {
                self.parse_return()
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn token_to_precedence(&self, tok: token::Token) -> Precedence {
        match tok {
            token::Token::Slash => Precedence::Product,
            token::Token::Gt => Precedence::LessGreater,
            token::Token::Lt => Precedence::LessGreater,
            token::Token::Asterisk => Precedence::Product,
            token::Token::Eq => Precedence::Equals,
            token::Token::NotEq => Precedence::Equals,
            token::Token::Plus => Precedence::Sum,
            token::Token::Minus => Precedence::Sum,
            _ => Precedence::Lowest,
        }
    }

    fn parse_return(&mut self) -> Option<ast::Statement> {
        let return_tok = self.cur_tok.clone();
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest);
        if self.peek_tok != token::Token::Semicolon {
            return None;
        }
        // move to the semi-colon
        self.next_token();
        Some(ast::Statement::Return {
            token: return_tok,
            exprs: return_value,
        })
    }

    fn parse_let(&mut self) -> Option<ast::Statement> {
        let let_tok = self.cur_tok.clone();
        match &self.peek_tok {
            token::Token::Ident(_) => self.next_token(),
            _ => return None,
        }
        let name = self.parse_ident().to_string();
        if self.peek_tok != token::Token::Assing {
            return None;
        }
        self.next_token();
        self.next_token();
        let val = self.parse_expression(Precedence::Lowest);
        // move to the Semicolon
        self.next_token();
        Some(ast::Statement::Let {
            token: let_tok,
            ident: ast::Expression::Ident(name),
            exprs: val,
        })
    }

    fn next_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token()
    }

    pub fn parse_program(&mut self) -> Vec<ast::Statement> {
        let mut statements = vec![];
        while self.cur_tok != token::Token::Eof {
            let stmnt = self.parse_statemnt();
            match stmnt {
                Some(n) => statements.push(n),
                None => return statements,
            }
            self.next_token();
        }

        statements
    }

    fn parse_prefix_ops(&mut self) -> ast::Expression {
        let tok = self.cur_tok.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Lowest);
        ast::Expression::PrefixExprsn {
            token: tok,
            exprsn: Box::new(right),
        }
    }

    fn parse_infix(&mut self, left: ast::Expression) -> ast::Expression {
        let opr = self.cur_tok.clone();
        self.next_token();
        let right = self.parse_expression(self.token_to_precedence(opr.clone()));
        ast::Expression::InfixExprsn {
            left: Box::new(left),
            right: Box::new(right),
            oprt: opr.to_string(),
        }
    }

    fn parse_block(&mut self) -> Vec<ast::Statement> {
        self.next_token();
        let mut stmnts: Vec<Statement> = vec![];
        while self.cur_tok != token::Token::Rbrace && self.cur_tok != token::Token::Eof {
            match self.parse_statemnt() {
                Some(s) => stmnts.push(s),
                None => return stmnts,
            };
            self.next_token();
        }
        stmnts
    }

    fn parse_if(&mut self) -> Expression {
        self.next_token();
        let condt = self.parse_expression(Precedence::Lowest);
        if self.peek_tok != token::Token::Lbrace {
            return ast::Expression::NoExprsn;
        }
        self.next_token();
        let consq = self.parse_block();
        self.next_token();
        if self.cur_tok == token::Token::Else {
            self.next_token();
            let altr = self.parse_block();
            let node = ast::Expression::IfExprsn {
                condt: Box::new(condt),
                conseq: consq,
                alter: altr,
            };
            return node;
        }
        ast::Expression::IfExprsn {
            condt: Box::new(condt),
            conseq: consq,
            alter: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::token;
    use crate::{ast, lexer, parser};
    #[test]
    fn test_let() {
        let src = "let x = 2;".to_string();
        let lex = Lexer::new(src);
        let mut parser = Parser::new(Box::new(lex));
        let statements = parser.parse_program();
        if statements.len() != 1 {
            panic!("expected only one statement got {}", statements.len())
        }
        let node = ast::ast::Statement::Let {
            token: crate::token::Token::Let,
            ident: ast::ast::Expression::Ident("x".to_string()),
            exprs: ast::ast::Expression::Int(2),
        };
        assert_eq!(statements[0], node)
    }
    #[test]
    fn test_return() {
        let src = "return 12;".to_string();
        let lex = Lexer::new(src);
        let mut parser = Parser::new(Box::new(lex));
        let statements = parser.parse_program();
        if statements.len() != 1 {
            panic!("expected only one statement got {}", statements.len())
        }
        let node = ast::ast::Statement::Return {
            token: crate::token::Token::Return,
            exprs: ast::ast::Expression::Int(12),
        };
        assert_eq!(statements[0], node)
    }

    #[test]
    fn test_int() {
        let src = "12;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        let expected = ast::ast::Statement::ExprsStatement {
            token: token::Token::Int("12".to_string()),
            exprs: ast::ast::Expression::Int(12),
        };
        if stmnts.len() != 1 {
            panic!("expected 1 statement got {}", stmnts.len())
        }
        assert_eq!(stmnts[0], expected)
    }

    #[test]
    fn test_bool() {
        let src = "false; true;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        let expected = [
            ast::ast::Statement::ExprsStatement {
                token: token::Token::False,
                exprs: ast::ast::Expression::Boolean {
                    token: token::Token::False,
                    value: false,
                },
            },
            ast::ast::Statement::ExprsStatement {
                token: token::Token::True,
                exprs: ast::ast::Expression::Boolean {
                    token: token::Token::True,
                    value: true,
                },
            },
        ];
        if stmnts.len() != 2 {
            panic!("expected 1 statement got {}", stmnts.len())
        }
        assert_eq!(stmnts, expected)
    }

    #[test]
    fn test_prefix_ops() {
        let src = "!true;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        if stmnts.len() != 1 {
            panic!("expected 1 got {}", stmnts.len())
        }
        let expected = [ast::ast::Statement::ExprsStatement {
            token: token::Token::Bang,
            exprs: ast::ast::Expression::PrefixExprsn {
                token: token::Token::Bang,
                exprsn: Box::new(ast::ast::Expression::Boolean {
                    token: token::Token::True,
                    value: true,
                }),
            },
        }];
        assert_eq!(stmnts[0], expected[0])
    }
    #[test]
    fn test_infix() {
        let src = "1+1; 1+2*3;".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();
        if stmnts.len() != 2 {
            panic!("expected 2 got {}", stmnts.len())
        }
        let expected = [
            ast::ast::Statement::ExprsStatement {
                token: token::Token::Int("1".to_string()),
                exprs: ast::ast::Expression::InfixExprsn {
                    left: Box::new(ast::ast::Expression::Int(1)),
                    right: Box::new(ast::ast::Expression::Int(1)),
                    oprt: "+".to_string(),
                },
            },
            ast::ast::Statement::ExprsStatement {
                token: token::Token::Int("1".to_string()),
                exprs: ast::ast::Expression::InfixExprsn {
                    left: Box::new(ast::ast::Expression::Int(1)),
                    right: Box::new(ast::ast::Expression::InfixExprsn {
                        left: Box::new(ast::ast::Expression::Int(2)),
                        right: Box::new(ast::ast::Expression::Int(3)),
                        oprt: "*".to_string(),
                    }),
                    oprt: "+".to_string(),
                },
            },
        ];
        for (i, v) in stmnts.iter().enumerate() {
            assert_eq!(*v, expected[i])
        }
    }

    #[test]
    fn test_grouped() {
        let src = "(1 + 1) * 2".to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();

        if stmnts.len() != 1 {
            panic!("expected 2 got {}", stmnts.len())
        }

        let expected = ast::ast::Statement::ExprsStatement {
            token: token::Token::Lparen,
            exprs: ast::ast::Expression::InfixExprsn {
                right: Box::new(ast::ast::Expression::Int(2)),
                left: Box::new(ast::ast::Expression::InfixExprsn {
                    left: Box::new(ast::ast::Expression::Int(1)),
                    right: Box::new(ast::ast::Expression::Int(1)),
                    oprt: "+".to_string(),
                }),
                oprt: "*".to_string(),
            },
        };
        assert_eq!(stmnts[0], expected)
    }

    #[test]
    fn test_if() {
        let src = "if x > 1 {
            let x = 2;
            return 12;
        } else {
            let b = 2;
            return 3;
        }
        "
        .to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();

        if stmnts.len() != 1 {
            panic!("expected 1 got {}", stmnts.len())
        }
        let consq = vec![
            ast::ast::Statement::Let {
                token: token::Token::Let,
                ident: ast::ast::Expression::Ident("x".to_string()),
                exprs: ast::ast::Expression::Int(2),
            },
            ast::ast::Statement::Return {
                token: token::Token::Return,
                exprs: ast::ast::Expression::Int(12),
            },
        ];
        let smnts = vec![
            ast::ast::Statement::Let {
                token: token::Token::Let,
                ident: ast::ast::Expression::Ident("b".to_string()),
                exprs: ast::ast::Expression::Int(2),
            },
            ast::ast::Statement::Return {
                token: token::Token::Return,
                exprs: ast::ast::Expression::Int(3),
            },
        ];

        let expected = ast::ast::Statement::ExprsStatement {
            token: token::Token::If,
            exprs: ast::ast::Expression::IfExprsn {
                condt: Box::new(ast::ast::Expression::InfixExprsn {
                    left: Box::new(ast::ast::Expression::Ident("x".to_string())),
                    right: Box::new(ast::ast::Expression::Int(1)),
                    oprt: ">".to_string(),
                }),
                conseq: consq,
                alter: smnts,
            },
        };
        assert_eq!(stmnts[0], expected)
    }

    #[test]
    fn test_fn() {
        let src = "
            fn(param, paramsecond){
                let x = 12;
                return 12;
            }
        "
        .to_string();
        let lex = lexer::Lexer::new(src);
        let mut p = parser::Parser::new(Box::new(lex));
        let stmnts = p.parse_program();

        if stmnts.len() != 1 {
            panic!("expected 1 got {}", stmnts.len())
        }
        let expected = ast::ast::Statement::ExprsStatement {
            token: token::Token::Func,
            exprs: ast::ast::Expression::FnExprsn {
                params: vec![
                    ast::ast::Expression::Ident("param".to_string()),
                    ast::ast::Expression::Ident("paramsecond".to_string()),
                ],
                body: vec![
                    ast::ast::Statement::Let {
                        token: token::Token::Let,
                        exprs: ast::ast::Expression::Int(12),
                        ident: ast::ast::Expression::Ident("x".to_string()),
                    },
                    ast::ast::Statement::Return {
                        token: token::Token::Return,
                        exprs: ast::ast::Expression::Int(12),
                    },
                ],
            },
        };
        assert_eq!(stmnts[0], expected)
    }
}
