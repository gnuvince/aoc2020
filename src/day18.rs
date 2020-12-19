use std::io;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Num(i64),
    LParen,
    RParen,
    Plus,
    Star,
}

#[derive(Debug)]
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Num(x) => *x,
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

fn lex(s: &[u8]) -> anyhow::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut num_lexeme = String::new();
    for b in s {
        if *b == b'(' {
            tokens.push(Token::LParen);
        } else if *b == b')' {
            if !num_lexeme.is_empty() {
                tokens.push(Token::Num(num_lexeme.parse::<i64>()?));
                num_lexeme.clear();
            }
            tokens.push(Token::RParen);
        } else if *b == b'+' {
            tokens.push(Token::Plus);
        } else if *b == b'*' {
            tokens.push(Token::Star);
        } else if *b == b' ' {
            if !num_lexeme.is_empty() {
                tokens.push(Token::Num(num_lexeme.parse::<i64>()?));
                num_lexeme.clear();
            }
        } else {
            num_lexeme.push(*b as char);
        }
    }

    if !num_lexeme.is_empty() {
        tokens.push(Token::Num(num_lexeme.parse::<i64>()?));
    }
    return Ok(tokens);
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}


/*
expr ::= term (('+'|'*') term)*
term ::= x
       | '(' expr ')'
*/

impl Parser {
    fn expr(&mut self) -> anyhow::Result<Expr> {
        let mut t = self.term()?;
        while self.pos < self.tokens.len() &&
            (self.tokens[self.pos] == Token::Star || self.tokens[self.pos] == Token::Plus)
        {
            let op = self.tokens[self.pos];
            self.pos += 1;
            let right = self.term()?;
            match op {
                Token::Star => t = Expr::Mul(Box::new(t), Box::new(right)),
                Token::Plus => t = Expr::Add(Box::new(t), Box::new(right)),
                _ => anyhow::bail!("unexpected operator: {:?}", op),
            }
        }
        return Ok(t);
    }

    fn term(&mut self) -> anyhow::Result<Expr> {
        match self.tokens[self.pos] {
            Token::Num(x) => {
                self.pos += 1;
                return Ok(Expr::Num(x));
            }
            Token::LParen => {
                self.pos += 1;
                let inner = self.expr()?;
                anyhow::ensure!(self.tokens[self.pos] == Token::RParen, "expected ')'");
                self.pos += 1;
                return Ok(inner);
            }
            _ => {
                anyhow::bail!("unexpected token: {:?}", self.tokens[self.pos]);
            }
        }
    }

    fn expr2(&mut self) -> anyhow::Result<Expr> {
        let mut t = self.add()?;
        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Star {
            self.pos += 1;
            let right = self.add()?;
            t = Expr::Mul(Box::new(t), Box::new(right));
        }
        return Ok(t);
    }

    fn add(&mut self) -> anyhow::Result<Expr> {
        let mut t = self.term2()?;
        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Plus {
            self.pos += 1;
            let right = self.term2()?;
            t = Expr::Add(Box::new(t), Box::new(right));
        }
        return Ok(t);
    }

    fn term2(&mut self) -> anyhow::Result<Expr> {
        match self.tokens[self.pos] {
            Token::Num(x) => {
                self.pos += 1;
                return Ok(Expr::Num(x));
            }
            Token::LParen => {
                self.pos += 1;
                let inner = self.expr2()?;
                anyhow::ensure!(self.tokens[self.pos] == Token::RParen, "expected ')'");
                self.pos += 1;
                return Ok(inner);
            }
            _ => {
                anyhow::bail!("unexpected token: {:?}", self.tokens[self.pos]);
            }
        }
    }
}

fn part1(lines: &[Vec<u8>]) -> anyhow::Result<i64> {
    let mut sum: i64 = 0;
    for line in lines {
        let tokens = lex(line)?;
        let mut parser = Parser { tokens, pos: 0 };
        let expr = parser.expr()?;
        sum += expr.eval();
    }
    return Ok(sum);
}

fn part2(lines: &[Vec<u8>]) -> anyhow::Result<i64> {
    let mut sum: i64 = 0;
    for line in lines {
        let tokens = lex(line)?;
        let mut parser = Parser { tokens, pos: 0 };
        let expr = parser.expr2()?;
        sum += expr.eval();
    }
    return Ok(sum);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines(stdin)?;

    println!("part 1: {}", part1(&lines)?);
    println!("part 2: {}", part2(&lines)?);

    return Ok(());
}
