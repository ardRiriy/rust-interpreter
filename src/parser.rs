use std::mem;
use std::borrow::Borrow;
use crate::lexer::*;

#[path="./lexer.rs"]
mod lexer;


#[derive(Debug)]
pub enum Expr {
    Number(i64),
    PrefixExpr {
        operator: Token,
        right: Box<Expr>
    },
    InfixExpr {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    }
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    SUM,
    PRODUCT,
    PREFIX,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    curr: Option<Token>,
    peek: Option<Token>
}

impl Parser {
    pub(crate) fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.tokenize();
        let peek = lexer.tokenize();
        Parser { lexer, curr, peek}
    }

    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.tokenize();
    }

    pub(crate) fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::LOWEST)
    }


    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            Token::Lparen => self.parse_grouped_expression(),
            _ => { None }
        }
    }


    fn parse_grouped_expression(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expression(Precedence::LOWEST);
        if self.is_peek(&Token::Rparen) {
            self.next();
            return expression;
        } else {
            return None;
        }
    }


    fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        let num = self.parse_expression(Precedence::PREFIX)?;
        return Some(Box::new(Expr::PrefixExpr {
            operator: Token::Minus,
            right: num
        }));
    }

    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.clone() {
            Some(Token::Number(x)) => Some(Box::new(Expr::Number(x))),
            _ => None
        }
    }


    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix().unwrap();
        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left).unwrap();
        }

        return Some(left);
    }

    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref().unwrap();
        match token {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }

    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?.clone();

        self.next();

        let precedence = Self::token_precedence(&token);
        let right = self.parse_expression(precedence).unwrap();
        return Some(Box::new(Expr::InfixExpr {
            left,
            operator: token.clone(),
            right
        }))
    }

    fn is_peek(&self, token: &Token) -> bool {
        if self.peek.is_none() {
            return false;
        }
        mem::discriminant(self.peek.as_ref().unwrap()) == mem::discriminant(token)
    }

    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::LOWEST;
        }
        return Self::token_precedence(token.as_ref().unwrap());
    }

    fn token_precedence(token: &Token) -> Precedence{
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Slash | Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

#[test]
fn test_parser() {
    do_parser(
        "1 + 2",
        r#"Some(InfixExpr { left: Number(1), operator: Plus, right: Number(2) })"#,
    );
    do_parser("- 1 + 2 * 3",
              r#"Some(InfixExpr { left: PrefixExpr { operator: Minus, right: Number(1) }, operator: Plus, right: InfixExpr { left: Number(2), operator: Asterisk, right: Number(3) } })"#);
}

#[cfg(test)]
fn do_parser(input: &str, expect: &str) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    assert_eq!(format!("{:?}", parser.parse()), expect);
}
