#![no_std]

include!("../prelude.rs");

// JSON-like tokenizer and recursive-descent parser benchmark.
//
// The input strings are embedded, but the seed controls which document is
// parsed first and the threshold used by the path-collecting traversal.  All
// documents contain objects, arrays, strings with escapes, integers,
// booleans, and null; every run parses all of them.

use alloc::string::String;
use alloc::vec::Vec;

const INPUTS: [&str; 4] = [
    r#"{"name":"cogwasm","version":3,"active":true,"tags":["compiler","wasm","no_std"],"meta":{"stars":128,"forks":9,"ratio":null},"notes":"line\nbreak\ttab \"quoted\" \\ slash"}"#,
    r#"[1,-2,3,-4,5,[10,20,[30,40]],true,false,null,0,-99,42]"#,
    r#"{"empty":"","blank":{},"none":[],"cafe":"caf\u00e9","face":"\uD83D\uDE00","esc":"\b\f\r\n\t\/","nested":{"a":[1,2,3],"b":{"c":-7}}}"#,
    r#"{"items":[{"id":1,"value":10,"tags":["a","b"]},{"id":2,"value":20,"tags":[]},{"id":3,"value":-30,"tags":["c"]}],"total":0,"ok":true,"spare":null,"matrix":[[1,2],[3,4],[5,6]]}"#,
];

#[derive(Clone, PartialEq, Eq)]
enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Str(String),
    Num(i64),
    True,
    False,
    Null,
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
            b'{' => {
                self.pos += 1;
                Ok(Token::LBrace)
            }
            b'}' => {
                self.pos += 1;
                Ok(Token::RBrace)
            }
            b'[' => {
                self.pos += 1;
                Ok(Token::LBracket)
            }
            b']' => {
                self.pos += 1;
                Ok(Token::RBracket)
            }
            b':' => {
                self.pos += 1;
                Ok(Token::Colon)
            }
            b',' => {
                self.pos += 1;
                Ok(Token::Comma)
            }
            b'"' => Ok(Token::Str(self.string()?)),
            b'-' | b'0'..=b'9' => self.number(),
            b'a'..=b'z' => self.word(),
            _ => Err(()),
        }
    }

    fn word(&mut self) -> Result<Token, ()> {
        let start = self.pos;
        while let Some(&byte) = self.bytes.get(self.pos) {
            if byte.is_ascii_alphabetic() {
                self.pos += 1;
            } else {
                break;
            }
        }
        let text = &self.bytes[start..self.pos];
        if text == &b"true"[..] {
            Ok(Token::True)
        } else if text == &b"false"[..] {
            Ok(Token::False)
        } else if text == &b"null"[..] {
            Ok(Token::Null)
        } else {
            Err(())
        }
    }

    fn number(&mut self) -> Result<Token, ()> {
        let negative = self.bytes.get(self.pos) == Some(&b'-');
        if negative {
            self.pos += 1;
        }
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
            return Err(());
        }
        if negative {
            value = value.wrapping_neg();
        }
        Ok(Token::Num(value))
    }

    fn string(&mut self) -> Result<String, ()> {
        self.pos += 1;
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let Some(&byte) = self.bytes.get(self.pos) else {
                return Err(());
            };
            self.pos += 1;
            match byte {
                b'"' => return String::from_utf8(bytes).map_err(|_| ()),
                b'\\' => {
                    let Some(&escaped) = self.bytes.get(self.pos) else {
                        return Err(());
                    };
                    self.pos += 1;
                    match escaped {
                        b'"' => bytes.push(b'"'),
                        b'\\' => bytes.push(b'\\'),
                        b'/' => bytes.push(b'/'),
                        b'b' => bytes.push(0x08),
                        b'f' => bytes.push(0x0c),
                        b'n' => bytes.push(b'\n'),
                        b'r' => bytes.push(b'\r'),
                        b't' => bytes.push(b'\t'),
                        b'u' => self.unicode_escape(&mut bytes)?,
                        _ => return Err(()),
                    }
                }
                b if b < 0x20 => return Err(()),
                b => bytes.push(b),
            }
        }
    }

    fn hex4(&mut self) -> Result<u32, ()> {
        let mut value = 0u32;
        let mut count = 0;
        while count < 4 {
            let Some(&byte) = self.bytes.get(self.pos) else {
                return Err(());
            };
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as u32,
                b'a'..=b'f' => (byte - b'a' + 10) as u32,
                b'A'..=b'F' => (byte - b'A' + 10) as u32,
                _ => return Err(()),
            };
            value = value * 16 + digit;
            self.pos += 1;
            count += 1;
        }
        Ok(value)
    }

    fn unicode_escape(&mut self, out: &mut Vec<u8>) -> Result<(), ()> {
        let first = self.hex4()?;
        let code = if (0xD800..=0xDBFF).contains(&first) {
            if self.bytes.get(self.pos) != Some(&b'\\') || self.bytes.get(self.pos + 1) != Some(&b'u')
            {
                return Err(());
            }
            self.pos += 2;
            let second = self.hex4()?;
            if !(0xDC00..=0xDFFF).contains(&second) {
                return Err(());
            }
            0x1_0000 + ((first - 0xD800) << 10) + (second - 0xDC00)
        } else if (0xDC00..=0xDFFF).contains(&first) {
            return Err(());
        } else {
            first
        };
        let Some(ch) = core::char::from_u32(code) else {
            return Err(());
        };
        let mut buf = [0u8; 4];
        out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
        Ok(())
    }
}

enum Value {
    Null,
    Bool(bool),
    Num(i64),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(String, Value)>),
}

impl Value {
    fn kind(&self) -> u8 {
        match self {
            Value::Null => 0,
            Value::Bool(_) => 1,
            Value::Num(_) => 2,
            Value::Str(_) => 3,
            Value::Arr(_) => 4,
            Value::Obj(_) => 5,
        }
    }
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

    fn parse(&mut self) -> Result<Value, ()> {
        let value = self.parse_value()?;
        if !matches!(self.peek(), Token::Eof) {
            return Err(());
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<Value, ()> {
        match self.bump() {
            Token::Null => Ok(Value::Null),
            Token::True => Ok(Value::Bool(true)),
            Token::False => Ok(Value::Bool(false)),
            Token::Num(value) => Ok(Value::Num(value)),
            Token::Str(text) => Ok(Value::Str(text)),
            Token::LBracket => self.parse_array(),
            Token::LBrace => self.parse_object(),
            _ => Err(()),
        }
    }

    fn parse_array(&mut self) -> Result<Value, ()> {
        let mut items = Vec::new();
        if matches!(self.peek(), Token::RBracket) {
            self.bump();
            return Ok(Value::Arr(items));
        }
        loop {
            items.push(self.parse_value()?);
            match self.bump() {
                Token::Comma => continue,
                Token::RBracket => break,
                _ => return Err(()),
            }
        }
        Ok(Value::Arr(items))
    }

    fn parse_object(&mut self) -> Result<Value, ()> {
        let mut fields = Vec::new();
        if matches!(self.peek(), Token::RBrace) {
            self.bump();
            return Ok(Value::Obj(fields));
        }
        loop {
            let key = match self.bump() {
                Token::Str(text) => text,
                _ => return Err(()),
            };
            self.expect(&Token::Colon)?;
            let value = self.parse_value()?;
            fields.push((key, value));
            match self.bump() {
                Token::Comma => continue,
                Token::RBrace => break,
                _ => return Err(()),
            }
        }
        Ok(Value::Obj(fields))
    }
}

fn parse_document(src: &str) -> Result<Value, ()> {
    let tokens = Lexer::new(src).tokenize()?;
    Parser::new(tokens).parse()
}

struct Stats {
    nodes: u64,
    objects: u64,
    arrays: u64,
    strings: u64,
    numbers: u64,
    bools: u64,
    nulls: u64,
    sum: i64,
    min: i64,
    max: i64,
    string_bytes: u64,
    max_depth: u64,
    hash: u64,
}

impl Stats {
    fn new() -> Self {
        Stats {
            nodes: 0,
            objects: 0,
            arrays: 0,
            strings: 0,
            numbers: 0,
            bools: 0,
            nulls: 0,
            sum: 0,
            min: i64::MAX,
            max: i64::MIN,
            string_bytes: 0,
            max_depth: 0,
            hash: 0xcbf2_9ce4_8422_2325,
        }
    }

    fn mix(&mut self, value: u64) {
        self.hash = (self.hash ^ value)
            .wrapping_mul(0x100_0000_01b3)
            .rotate_left(7);
    }
}

fn walk(value: &Value, depth: u64, stats: &mut Stats) {
    stats.nodes += 1;
    if depth > stats.max_depth {
        stats.max_depth = depth;
    }
    stats.mix(value.kind() as u64);
    stats.mix(depth);
    match value {
        Value::Null => {
            stats.nulls += 1;
        }
        Value::Bool(flag) => {
            stats.bools += 1;
            stats.mix(*flag as u64);
        }
        Value::Num(number) => {
            stats.numbers += 1;
            stats.sum = stats.sum.wrapping_add(*number);
            if *number < stats.min {
                stats.min = *number;
            }
            if *number > stats.max {
                stats.max = *number;
            }
            stats.mix(*number as u64);
        }
        Value::Str(text) => {
            stats.strings += 1;
            stats.string_bytes += text.len() as u64;
            stats.mix(text.len() as u64);
            for &byte in text.as_bytes() {
                stats.mix(byte as u64);
            }
        }
        Value::Arr(items) => {
            stats.arrays += 1;
            stats.mix(items.len() as u64);
            for item in items {
                walk(item, depth + 1, stats);
            }
        }
        Value::Obj(fields) => {
            stats.objects += 1;
            stats.mix(fields.len() as u64);
            for (key, item) in fields {
                stats.mix(key.len() as u64);
                for &byte in key.as_bytes() {
                    stats.mix(byte as u64);
                }
                walk(item, depth + 1, stats);
            }
        }
    }
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

fn collect_paths(value: &Value, prefix: &mut String, out: &mut Vec<(String, i64)>, minimum: i64) {
    match value {
        Value::Num(number) => {
            if *number >= minimum {
                out.push((prefix.clone(), *number));
            }
        }
        Value::Arr(items) => {
            let mut index = 0;
            while index < items.len() {
                let mark = prefix.len();
                prefix.push('[');
                push_u64(prefix, index as u64);
                prefix.push(']');
                collect_paths(&items[index], prefix, out, minimum);
                prefix.truncate(mark);
                index += 1;
            }
        }
        Value::Obj(fields) => {
            for (key, item) in fields {
                let mark = prefix.len();
                prefix.push('.');
                prefix.push_str(key);
                collect_paths(item, prefix, out, minimum);
                prefix.truncate(mark);
            }
        }
        _ => {}
    }
}

fn body(seed: u64) -> u64 {
    let start = (seed as usize) % INPUTS.len();
    let minimum = (seed % 29) as i64 - 7;
    let mut summary = splitmix64(seed);
    let mut step = 0;
    while step < INPUTS.len() {
        let index = (start + step) % INPUTS.len();
        let input = INPUTS[index];
        let value = match parse_document(input) {
            Ok(value) => value,
            Err(()) => Value::Null,
        };

        let mut stats = Stats::new();
        walk(&value, 0, &mut stats);

        let mut paths: Vec<(String, i64)> = Vec::new();
        let mut prefix = String::new();
        collect_paths(&value, &mut prefix, &mut paths, minimum);
        paths.sort();

        emit_u64(input.len() as u64);
        emit_u64(stats.nodes);
        emit_u64(stats.objects);
        emit_u64(stats.arrays);
        emit_u64(stats.strings);
        emit_u64(stats.numbers);
        emit_u64(stats.bools);
        emit_u64(stats.nulls);
        emit_i64(stats.sum);
        emit_i64(stats.min);
        emit_i64(stats.max);
        emit_u64(stats.string_bytes);
        emit_u64(stats.max_depth);
        emit_u64(stats.hash);

        let shown = paths.len().min(12);
        emit_u64(shown as u64);
        let mut path_index = 0;
        while path_index < shown {
            emit_str(&paths[path_index].0);
            emit_i64(paths[path_index].1);
            path_index += 1;
        }

        summary = summary.rotate_left(11)
            ^ stats.hash
            ^ (paths.len() as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        step += 1;
    }
    emit_u64(summary);
    summary
}
