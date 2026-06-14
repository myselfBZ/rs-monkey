use eval::eval::Evaluator;

mod eval;
mod lexer;
mod objects;
mod token;
mod ast;
mod parser;
fn main() {
    let src = String::from("
        let yes = 1 != 2;
        ");
    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(Box::new(lexer));
    let statmnts = parser.parse_program();
    let mut e = Evaluator::new(objects::objects::Env::new());
    let result = e.eval_program(statmnts);
    println!("{}", result)
}
