#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Plus,      // +
    Minus,     // -
    Asterisk,  // *
    Slash,     // /
    Lparen,    // (
    Rparen,    // )
}

#[derive(Debug)]
pub struct Lexer {
    str: Vec<char>,
    idx: usize,
}

impl Lexer {
    pub(crate) fn new(str: Vec<char>) -> Lexer {
        Lexer { str, idx: 0}
    }

    pub(crate) fn tokenize(&mut self) -> Option<Token> {
        return match self.curr(0) {
            Some(&c) => {
                if Self::is_number(c) {
                    let mut num = Self::to_number(c);
                    while let Some(nc) = self.curr(1) {
                        if Self::is_number(*nc) {
                            num = num * 10 + Self::to_number(*nc);
                            self.next();
                        } else {
                            break;
                        }
                    }
                    self.next();
                    Some(Token::Number(num))
                } else {
                    self.next();
                    match c {
                        '+' => Some(Token::Plus),
                        '-' => Some(Token::Minus),
                        '*' => Some(Token::Asterisk),
                        '/' => Some(Token::Slash),
                        '(' => Some(Token::Lparen),
                        ')' => Some(Token::Rparen),
                        _ => self.tokenize(),
                    }
                }
            },
            None => {
                self.next();
                None
            }
        }
    }

    fn curr(&self, prefix: usize) -> Option<&char> {
        self.str.get(self.idx + prefix)
    }

    fn next(&mut self){
        self.idx += 1;
    }

    fn is_number(c: char) -> bool {
        return c >= '0' && c <= '9'
    }

    fn to_number(c: char) -> i64 {
        return c as i64 - '0' as i64
    }
}

#[test]
fn lexer_test() {
    let mut lexer = Lexer::new("14 + 3".chars().collect());
    assert_eq!(lexer.tokenize(), Some(Token::Number(14)));
    assert_eq!(lexer.tokenize(), Some(Token::Plus));
    assert_eq!(lexer.tokenize(), Some(Token::Number(3)));
    assert_eq!(lexer.tokenize(), None);
}