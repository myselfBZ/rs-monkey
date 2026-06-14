use crate::ast::ast::{self, Expression};
use crate::objects::objects::{self};

pub struct Evaluator {
    env: objects::Env,
}

impl Evaluator {
    pub fn new(env: objects::Env) -> Self {
        Evaluator { env: env }
    }

    pub fn eval_program(&mut self, program: Vec<ast::Statement>) -> objects::Object {
        let mut result: objects::Object = objects::Object::Null;
        for s in program {
            result = self.eval_stmnt(s);
        }
        result
    }

    fn eval_stmnt(&mut self, stmnt: ast::Statement) -> objects::Object {
        match stmnt {
            ast::Statement::ExprsStatement { token: _, exprs } => self.eval_exprs(exprs),
            ast::Statement::Let {
                token: _,
                ident,
                exprs,
            } => {
                let val = self.eval_exprs(exprs);
                self.env.set(ident.to_string(), &val);
                return val;
            }
            _ => objects::Object::Null,
        }
    }

    fn eval_bang(&self, s: ast::Expression) -> objects::Object {
        match s {
            ast::Expression::Boolean { token, value } => return objects::Object::Bool(!value),
            _ => return objects::Object::Bool(false),
        }
    }

    fn eval_exprs(&self, s: ast::Expression) -> objects::Object {
        match s {
            ast::Expression::Int(s) => objects::Object::Int(s),
            ast::Expression::Boolean { token: _, value } => objects::Object::Bool(value),
            ast::Expression::InfixExprsn { left, right, oprt } => {
                self.eval_infix(*left, *right, oprt)
            }
            ast::Expression::PrefixExprsn { token, exprsn } => match token {
                crate::token::Token::Bang =>  return self.eval_bang(*exprsn),
                _ => objects::Object::Err(String::from("invalid prefix operator: {token}")),
            },
            ast::Expression::Ident(name) => {
                let val = self.env.get(name);
                match val {
                    Some(v) => return v,
                    None => return objects::Object::Err(String::from("identifier not found")),
                }
            }
            _ => objects::Object::Null,
        }
    }

    fn eval_infix(
        &self,
        left: ast::Expression,
        right: Expression,
        oprtr: String,
    ) -> objects::Object {
        let right = self.eval_exprs(right);
        let left = self.eval_exprs(left);
        match (right.clone(), left.clone()) {
            (objects::Object::Int(a), objects::Object::Int(b)) => match oprtr.as_str() {
                "+" => objects::Object::Int(a + b),
                "-" => objects::Object::Int(a - b),
                "*" => objects::Object::Int(a * b),
                "/" => objects::Object::Int(a / b),
                _ => objects::Object::Null,
            },
            (objects::Object::Err(_), _) => right,
            (_, objects::Object::Err(_)) => left,
            _ => objects::Object::Err(String::from(format!(
                "mismatched types: {right} and {left}"
            ))),
        }
    }
}
