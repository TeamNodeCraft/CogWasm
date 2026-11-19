#![no_std]

include!("../prelude.rs");

// Expression language benchmark: tokenizer, Pratt parser, and interpreter.
//
// The language is deliberately small but not trivial: integer literals,
// `+ - * / %`, parentheses, unary minus, and `let` statements over a small
// environment.  The seed selects the literals and operators of four extra
// programs, all evaluated after a fixed program.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

const FIXED: &str =
    "let a = 12 + 3 * (7 - 2); let b = a % 10; let c = -b + 42 / 6; c * 2 - a + 1";

#[derive(Clone, PartialEq, Eq)]
enum Token {
    Int(i64),
    Ident(String),
    Let,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LParen,
    RParen,
    Eq,
    Semi,
    Eof,
}

struct Lexer<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Lexer {
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, ()> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let done = matches!(token, Token::Eof);
            tokens.push(token);
            if done {
                break;
            }
        }
        Ok(tokens)
    }

    fn skip_ws(&mut self) {
        while let Some(&byte) = self.bytes.get(self.pos) {
            if matches!(byte, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, ()> {
        self.skip_ws();
        let Some(&byte) = self.bytes.get(self.pos) else {
            return Ok(Token::Eof);
        };
        match byte {
            b'+' => {
                self.pos += 1;
                Ok(Token::Plus)
            }
            b'-' => {
                self.pos += 1;
                Ok(Token::Minus)
            }
            b'*' => {
                self.pos += 1;
                Ok(Token::Star)
            }
            b'/' => {
                self.pos += 1;
                Ok(Token::Slash)
            }
            b'%' => {
                self.pos += 1;
                Ok(Token::Percent)
            }
            b'(' => {
                self.pos += 1;
                Ok(Token::LParen)
            }
            b')' => {
                self.pos += 1;
                Ok(Token::RParen)
            }
            b'=' => {
                self.pos += 1;
                Ok(Token::Eq)
            }
            b';' => {
                self.pos += 1;
                Ok(Token::Semi)
            }
            b'0'..=b'9' => self.number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.ident(),
            _ => Err(()),
        }
    }

    fn number(&mut self) -> Result<Token, ()> {
        let mut value = 0i64;
        let mut digits = 0;
        while let Some(&byte) = self.bytes.get(self.pos) {
            if byte.is_ascii_digit() {
                value = value
                    .wrapping_mul(10)
                    .wrapping_add((byte - b'0') as i64);
                self.pos += 1;
                digits += 1;
            } else {
                break;
            }
        }
        if digits == 0 {
            Err(())
        } else {
            Ok(Token::Int(value))
        }
    }

    fn ident(&mut self) -> Result<Token, ()> {
        let start = self.pos;
        while let Some(&byte) = self.bytes.get(self.pos) {
            if byte.is_ascii_alphanumeric() || byte == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let text = core::str::from_utf8(&self.bytes[start..self.pos]).map_err(|_| ())?;
        if text == "let" {
            Ok(Token::Let)
        } else {
            Ok(Token::Ident(String::from(text)))
        }
    }
}

#[derive(Clone, Copy)]
enum UnOp {
    Neg,
}

#[derive(Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

enum Expr {
    Int(i64),
    Var(String),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}

struct Program {
    lets: Vec<(String, Expr)>,
    result: Expr,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(mut tokens: Vec<Token>) -> Self {
        if tokens.is_empty() {
            tokens.push(Token::Eof);
        }
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        let last = self.tokens.len() - 1;
        &self.tokens[self.pos.min(last)]
    }

    fn bump(&mut self) -> Token {
        let token = self.peek().clone();
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, want: &Token) -> Result<(), ()> {
        let got = self.bump();
        if &got == want {
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_program(&mut self) -> Result<Program, ()> {
        let mut lets = Vec::new();
        while matches!(self.peek(), Token::Let) {
            self.bump();
            let name = match self.bump() {
                Token::Ident(name) => name,
                _ => return Err(()),
            };
            self.expect(&Token::Eq)?;
            let value = self.parse_expr(0)?;
            self.expect(&Token::Semi)?;
            lets.push((name, value));
        }
        let result = self.parse_expr(0)?;
        if !matches!(self.peek(), Token::Eof) {
            return Err(());
        }
        Ok(Program { lets, result })
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ()> {
        let mut lhs = self.parse_prefix()?;
        loop {
            let (op, left_bp, right_bp) = match self.peek() {
                Token::Plus => (BinOp::Add, 10u8, 11u8),
                Token::Minus => (BinOp::Sub, 10, 11),
                Token::Star => (BinOp::Mul, 20, 21),
                Token::Slash => (BinOp::Div, 20, 21),
                Token::Percent => (BinOp::Rem, 20, 21),
                _ => break,
            };
            if left_bp < min_bp {
                break;
            }
            self.bump();
            let rhs = self.parse_expr(right_bp)?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ()> {
        match self.bump() {
            Token::Int(value) => Ok(Expr::Int(value)),
            Token::Ident(name) => Ok(Expr::Var(name)),
            Token::Minus => {
                let operand = self.parse_expr(30)?;
                Ok(Expr::Unary(UnOp::Neg, Box::new(operand)))
            }
            Token::LParen => {
                let inner = self.parse_expr(0)?;
                self.expect(&Token::RParen)?;
                Ok(inner)
            }
            _ => Err(()),
        }
    }
}

fn parse_source(src: &str) -> Result<Program, ()> {
    let tokens = Lexer::new(src).tokenize()?;
    Parser::new(tokens).parse_program()
}

struct Env {
    vars: Vec<(String, i64)>,
}

impl Env {
    fn new() -> Self {
        Env { vars: Vec::new() }
    }

    fn get(&self, name: &str) -> i64 {
        let mut i = 0;
        while i < self.vars.len() {
            if self.vars[i].0.as_str() == name {
                return self.vars[i].1;
            }
            i += 1;
        }
        0
    }

    fn set(&mut self, name: String, value: i64) {
        let mut i = 0;
        while i < self.vars.len() {
            if self.vars[i].0 == name {
                self.vars[i].1 = value;
                return;
            }
            i += 1;
        }
        self.vars.push((name, value));
    }
}

fn eval(expr: &Expr, env: &mut Env) -> i64 {
    match expr {
        Expr::Int(value) => *value,
        Expr::Var(name) => env.get(name),
        Expr::Unary(UnOp::Neg, inner) => eval(inner, env).wrapping_neg(),
        Expr::Binary(op, left, right) => {
            let a = eval(left, env);
            let b = eval(right, env);
            match op {
                BinOp::Add => a.wrapping_add(b),
                BinOp::Sub => a.wrapping_sub(b),
                BinOp::Mul => a.wrapping_mul(b),
                // Division by zero is defined as zero so no input can trap.
                BinOp::Div => {
                    if b == 0 {
                        0
                    } else {
                        a.wrapping_div(b)
                    }
                }
                BinOp::Rem => {
                    if b == 0 {
                        0
                    } else {
                        a.wrapping_rem(b)
                    }
                }
            }
        }
    }
}

fn eval_program(program: &Program) -> i64 {
    let mut env = Env::new();
    for (name, expr) in &program.lets {
        let value = eval(expr, &mut env);
        env.set(name.clone(), value);
    }
    eval(&program.result, &mut env)
}

fn push_u64(out: &mut String, mut value: u64) {
    if value == 0 {
        out.push('0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut at = buf.len();
    while value > 0 {
        at -= 1;
        buf[at] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    if let Ok(text) = core::str::from_utf8(&buf[at..]) {
        out.push_str(text);
    }
}

fn push_i64(out: &mut String, value: i64) {
    if value < 0 {
        out.push('-');
        push_u64(out, value.unsigned_abs());
    } else {
        push_u64(out, value as u64);
    }
}

fn build_seeded_source(seed: u64, rng: &mut Rng) -> String {
    let count = 3 + (seed % 3) as usize;
    let mut src = String::new();
    let mut i = 0;
    while i < count {
        let literal = (rng.below(9) + 1) as i64;
        src.push_str("let v");
        push_u64(&mut src, i as u64);
        src.push_str(" = ");
        if i == 0 {
            push_i64(&mut src, literal);
            src.push_str(" * ");
            push_i64(&mut src, (rng.below(7) + 1) as i64);
            src.push_str(" + ");
            push_i64(&mut src, (rng.below(5) + 1) as i64);
        } else {
            src.push('v');
            push_u64(&mut src, (i - 1) as u64);
            match i % 3 {
                0 => src.push_str(" + "),
                1 => src.push_str(" - "),
                _ => src.push_str(" * "),
            }
            push_i64(&mut src, literal);
            if i % 4 == 0 {
                src.push_str(" % ");
                push_i64(&mut src, (rng.below(5) + 1) as i64);
            }
        }
        src.push_str(";\n");
        i += 1;
    }
    src.push_str("v0 + v");
    push_u64(&mut src, (count - 1) as u64);
    src.push_str(" * 2 - v1");
    src
}

fn check(source: &str, want: i64) {
    let got = match parse_source(source) {
        Ok(program) => eval_program(&program),
        Err(()) => i64::MIN,
    };
    assert!(got == want);
}

fn self_check() {
    check("1 + 2 * 3", 7);
    check("(1 + 2) * 3", 9);
    check("-4 + 10", 6);
    check("-(2 + 3) * 4", -20);
    check("20 / 3", 6);
    check("20 % 3", 2);
    check("7 - 2 - 3", 2);
    check("100 / 7 / 2", 7);
    check("1 + 2 * 3 % 4", 3);
    check("let x = 5; let y = x * x; y + x", 30);
    check("let q_1 = 4; q_1 + 1", 5);
    check("let a = 6; let a = a - 1; a * 3", 15);
    check("- - 5", 5);
    check("2 * (3 + 4) - -5", 19);
    check("let z = 8 / 3; let w = z % 2; w + z", 2);
    check("8 / 0", 0);
    check("8 % 0", 0);
}

fn body(seed: u64) -> u64 {
    self_check();

    let fixed = match parse_source(FIXED) {
        Ok(program) => eval_program(&program),
        Err(()) => 0,
    };
    emit_i64(fixed);

    let mut rng = Rng::new(seed);
    let mut summary = splitmix64(seed) ^ (fixed as u64);
    let mut round = 0u64;
    while round < 4 {
        let source = build_seeded_source(seed.wrapping_add(round), &mut rng);
        let value = match parse_source(&source) {
            Ok(program) => eval_program(&program),
            Err(()) => 0,
        };
        emit_u64(source.len() as u64);
        emit_i64(value);
        summary = summary.rotate_left(9) ^ (value as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        round += 1;
    }
    emit_u64(summary);
    summary
}
