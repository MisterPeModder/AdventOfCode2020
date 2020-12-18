use core::panic;

struct Lexer<'a> {
    source: &'a str,
    next: Option<Token>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    OpeningParenthese,
    ClosingParenthese,
    Plus,
    Star,
    Number(u64),
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { source, next: None }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.next.is_some() {
            return self.next.take();
        } else if self.source.is_empty() {
            return None;
        }

        let mut start = 0;

        for (i, c) in self.source.char_indices() {
            start = i;
            if !c.is_whitespace() {
                break;
            }
        }
        self.source = &self.source[start..];
        if self.source.is_empty() {
            return None;
        }
        match self.source.as_bytes()[0] {
            b'(' => {
                self.source = &self.source[1..];
                Some(Token::OpeningParenthese)
            }
            b')' => {
                self.source = &self.source[1..];
                Some(Token::ClosingParenthese)
            }
            b'+' => {
                self.source = &self.source[1..];
                Some(Token::Plus)
            }
            b'*' => {
                self.source = &self.source[1..];
                Some(Token::Star)
            }
            b'0'..=b'9' => {
                let mut digits = 1;
                let bytes = self.source.as_bytes();

                while digits < bytes.len() && bytes[digits].is_ascii_digit() {
                    digits += 1;
                }
                let (num, other) = self.source.split_at(digits);
                self.source = other;
                Some(Token::Number(num.parse().ok()?))
            }
            _ => None,
        }
    }

    pub fn peek_token(&mut self) -> Option<Token> {
        if self.next.is_none() {
            if self.source.is_empty() {
                return None;
            }
            self.next = self.next_token();
        }
        self.next
    }
}

fn parse_rhs(lexer: &mut Lexer) -> u64 {
    match lexer.next_token() {
        Some(Token::OpeningParenthese) => eval_simple_expression(lexer),
        Some(Token::Number(n)) => n,
        Some(token) => panic!("expected opening parenthese or number, got {:?}", token),
        None => panic!("unexpected end of expression"),
    }
}

fn eval_simple_expression(lexer: &mut Lexer) -> u64 {
    let mut result: u64 = match lexer.next_token() {
        None => return 0,
        Some(Token::OpeningParenthese) => eval_simple_expression(lexer),
        Some(Token::Number(n)) => n,
        Some(token) => panic!("expected opening parenthese or number, got {:?}", token),
    };

    loop {
        match lexer.next_token() {
            None | Some(Token::ClosingParenthese) => break result,
            Some(Token::Plus) => result += parse_rhs(lexer),
            Some(Token::Star) => result *= parse_rhs(lexer),
            Some(token) => panic!("expected closing parenthese or operator, got {:?}", token),
        }
    }
}

fn eval_grouping(lexer: &mut Lexer) -> u64 {
    match lexer.next_token() {
        Some(Token::OpeningParenthese) => {
            let result = eval_product(lexer);
            match lexer.next_token() {
                Some(Token::ClosingParenthese) => result,
                Some(token) => panic!("expected closing parenthese, got {:?}", token),
                None => panic!("unclosed parenthese"),
            }
        }
        Some(Token::Number(n)) => n,
        Some(token) => panic!("expected opening parenthese or number, got {:?}", token),
        None => panic!("unexpected end of expression"),
    }
}

fn eval_addition(lexer: &mut Lexer) -> u64 {
    let mut result = eval_grouping(lexer);

    //println!("addition source: \"{}\"", lexer.source);
    while let Some(Token::Plus) = lexer.peek_token() {
        lexer.next_token();
        result += eval_grouping(lexer);
    }
    result
}

fn eval_product(lexer: &mut Lexer) -> u64 {
    let mut result = eval_addition(lexer);

    //println!("product source: \"{}\"", lexer.source);
    while let Some(Token::Star) = lexer.peek_token() {
        lexer.next_token();
        result *= eval_addition(lexer);
    }
    result
}

#[inline]
fn eval_expression(source: &str) -> u64 {
    let mut lexer = Lexer::new(source);
    let result = eval_product(&mut lexer);

    //println!("end source: \"{}\"", lexer.source);
    match lexer.next_token() {
        Some(token) => panic!("expected end of expression, got {:?}", token),
        None => result,
    }
}

#[aoc(day18, part1)]
pub fn day18_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| eval_simple_expression(&mut Lexer::new(l)))
        .sum()
}

#[aoc(day18, part2)]
pub fn day18_part2(input: &str) -> u64 {
    input.lines().map(|l| eval_expression(l)).sum()
}
