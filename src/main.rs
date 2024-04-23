use crate::lexer::{Lexer, Token};
use crate::parser::{Expr, Parser};
use std::borrow::Borrow;

mod parser;
mod lexer;

fn main() {
    println!("Hello, world!");
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(x) => *x,
        Expr::PrefixExpr { operator: _, right} => {
            -eval(right)
        },
        Expr::InfixExpr {
            left,
            operator,
            right
        } => {
            let l = eval(left);
            let r = eval(right);
            match *operator {
                Token::Plus => l + r,
                Token::Minus => l - r,
                Token::Asterisk => l * r,
                Token::Slash => l / r,
                _ => unreachable!()
            }
        }
    }
}

#[test]
fn test_eval() {
    do_eval("1 + 2", 3);
    do_eval("1 + 2 * 3", 7);
    do_eval("1 + (2 + 3) * -(3 / 3)", -4);
}


#[cfg(test)]
fn do_eval(input: &str, expect: i64) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let result = eval(parser.parse().unwrap().borrow());
    assert_eq!(result, expect);
}

