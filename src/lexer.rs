//! Lexer for POLER-SciLang

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64, Option<String>),
    Str(String),
    Bool(bool),
    Ident(String),

    // Keywords
    Let,
    Fn,
    Return,
    If,
    Else,
    For,
    In,
    While,
    Emit,
    To,

    // Operators & Symbols
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Percent,
    PipeRight, // |>

    Assign, // =
    Eq,     // ==
    Neq,    // !=
    Lt,     // <
    Lte,    // <=
    Gt,     // >
    Gte,    // >=
    And,    // &&
    Or,     // ||
    Bang,   // !

    DotDot, // ..
    Dot,    // .
    Comma,  // ,
    Semi,   // ;
    Colon,  // :

    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    LBrace,   // {
    RBrace,   // }

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            _phantom: std::marker::PhantomData,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_next() == Some('/') {
                // Line comment
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if ch == '#' {
                // Hash comment
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if ch == '/' && self.peek_next() == Some('*') {
                // Block comment
                self.advance();
                self.advance();
                while self.pos < self.chars.len() {
                    if self.peek() == Some('*') && self.peek_next() == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            let start_line = self.line;
            let start_col = self.col;

            let ch = match self.peek() {
                Some(c) => c,
                None => {
                    tokens.push(Token {
                        kind: TokenKind::Eof,
                        line: start_line,
                        col: start_col,
                    });
                    break;
                }
            };

            // Numbers
            if ch.is_ascii_digit() || (ch == '.' && self.peek_next().map_or(false, |c| c.is_ascii_digit())) {
                tokens.push(self.read_number(start_line, start_col)?);
                continue;
            }

            // String literals
            if ch == '"' || ch == '\'' {
                tokens.push(self.read_string(ch, start_line, start_col)?);
                continue;
            }

            // Identifiers / Keywords
            if ch.is_alphabetic() || ch == '_' {
                tokens.push(self.read_ident(start_line, start_col));
                continue;
            }

            // Two-character operators
            if ch == '|' && self.peek_next() == Some('>') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::PipeRight, line: start_line, col: start_col });
                continue;
            }
            if ch == '=' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::Eq, line: start_line, col: start_col });
                continue;
            }
            if ch == '!' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::Neq, line: start_line, col: start_col });
                continue;
            }
            if ch == '<' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::Lte, line: start_line, col: start_col });
                continue;
            }
            if ch == '>' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::Gte, line: start_line, col: start_col });
                continue;
            }
            if ch == '&' && self.peek_next() == Some('&') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::And, line: start_line, col: start_col });
                continue;
            }
            if ch == '|' && self.peek_next() == Some('|') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::Or, line: start_line, col: start_col });
                continue;
            }
            if ch == '.' && self.peek_next() == Some('.') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::DotDot, line: start_line, col: start_col });
                continue;
            }

            // Single characters
            self.advance();
            let kind = match ch {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '^' => TokenKind::Caret,
                '%' => TokenKind::Percent,
                '=' => TokenKind::Assign,
                '<' => TokenKind::Lt,
                '>' => TokenKind::Gt,
                '!' => TokenKind::Bang,
                '.' => TokenKind::Dot,
                ',' => TokenKind::Comma,
                ';' => TokenKind::Semi,
                ':' => TokenKind::Colon,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                other => return Err(format!("Unexpected character '{}' at line {}:{}", other, start_line, start_col)),
            };

            tokens.push(Token {
                kind,
                line: start_line,
                col: start_col,
            });
        }

        Ok(tokens)
    }

    fn read_number(&mut self, line: usize, col: usize) -> Result<Token, String> {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Fractional part
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            s.push(self.advance().unwrap());
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Scientific notation: 1e9, 2.5e-11
        if let Some(c) = self.peek() {
            if c == 'e' || c == 'E' {
                s.push(self.advance().unwrap());
                if let Some(sign) = self.peek() {
                    if sign == '+' || sign == '-' {
                        s.push(self.advance().unwrap());
                    }
                }
                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        s.push(d);
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        let val: f64 = s
            .parse()
            .map_err(|e| format!("Failed to parse number '{}' at {}:{}: {}", s, line, col, e))?;

        // Attached unit (e.g. 10km, 300K, 15m/s)
        let mut unit = None;
        if let Some(c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                let mut u = String::new();
                while let Some(uc) = self.peek() {
                    if uc.is_alphanumeric() || uc == '_' || uc == '^' || uc == '/' {
                        u.push(uc);
                        self.advance();
                    } else {
                        break;
                    }
                }
                if !u.is_empty() {
                    unit = Some(u);
                }
            }
        }

        Ok(Token {
            kind: TokenKind::Number(val, unit),
            line,
            col,
        })
    }

    fn read_string(&mut self, quote: char, line: usize, col: usize) -> Result<Token, String> {
        self.advance(); // consume open quote
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c == quote {
                self.advance();
                return Ok(Token {
                    kind: TokenKind::Str(s),
                    line,
                    col,
                });
            } else if c == '\\' {
                self.advance();
                if let Some(esc) = self.advance() {
                    match esc {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        'r' => s.push('\r'),
                        '\\' => s.push('\\'),
                        '\'' => s.push('\''),
                        '"' => s.push('"'),
                        other => s.push(other),
                    }
                }
            } else {
                s.push(c);
                self.advance();
            }
        }

        Err(format!("Unterminated string literal at {}:{}", line, col))
    }

    fn read_ident(&mut self, line: usize, col: usize) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let kind = match s.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "while" => TokenKind::While,
            "emit" | "print" => TokenKind::Emit,
            "to" => TokenKind::To,
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            _ => TokenKind::Ident(s),
        };

        Token { kind, line, col }
    }
}
