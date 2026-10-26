//! Recursive Descent Parser with Operator Precedence for POLER-SciLang

use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &TokenKind {
        self.tokens.get(self.pos).map_or(&TokenKind::Eof, |t| &t.kind)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            tok
        } else {
            Token {
                kind: TokenKind::Eof,
                line: 0,
                col: 0,
            }
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token, String> {
        let tok = self.advance();
        if std::mem::discriminant(&tok.kind) == std::mem::discriminant(&expected) {
            Ok(tok)
        } else {
            Err(format!(
                "Expected token {:?}, found {:?} at {}:{}",
                expected, tok.kind, tok.line, tok.col
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while *self.peek() != TokenKind::Eof {
            statements.push(self.parse_stmt()?);
        }

        Ok(Program { statements })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            TokenKind::Let => self.parse_let(),
            TokenKind::Fn => self.parse_fn(),
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::While => self.parse_while(),
            TokenKind::Emit => self.parse_emit(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Ident(_) => {
                // Check if assignment `ident = expr;`
                if let Some(next_tok) = self.tokens.get(self.pos + 1) {
                    if next_tok.kind == TokenKind::Assign {
                        let name_tok = self.advance();
                        let name = match name_tok.kind {
                            TokenKind::Ident(n) => n,
                            _ => unreachable!(),
                        };
                        self.advance(); // consume '='
                        let expr = self.parse_expr()?;
                        if *self.peek() == TokenKind::Semi {
                            self.advance();
                        }
                        return Ok(Stmt::Assign { name, expr });
                    }
                }
                // Otherwise it's an expression statement
                let expr = self.parse_expr()?;
                if *self.peek() == TokenKind::Semi {
                    self.advance();
                }
                Ok(Stmt::Expr(expr))
            }
            _ => {
                let expr = self.parse_expr()?;
                if *self.peek() == TokenKind::Semi {
                    self.advance();
                }
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'let'
        let name_tok = self.advance();
        let name = match name_tok.kind {
            TokenKind::Ident(n) => n,
            other => {
                return Err(format!(
                    "Expected identifier after 'let', found {:?} at {}:{}",
                    other, name_tok.line, name_tok.col
                ))
            }
        };

        self.expect(TokenKind::Assign)?;
        let init = self.parse_expr()?;
        if *self.peek() == TokenKind::Semi {
            self.advance();
        }
        Ok(Stmt::Let { name, init })
    }

    fn parse_emit(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'emit' / 'print'

        // Check if optional label: emit "Label": expr
        let mut label = None;
        if let TokenKind::Str(s) = self.peek() {
            if let Some(next) = self.tokens.get(self.pos + 1) {
                if next.kind == TokenKind::Colon {
                    label = Some(s.clone());
                    self.advance(); // consume string
                    self.advance(); // consume ':'
                }
            }
        }

        let expr = self.parse_expr()?;
        if *self.peek() == TokenKind::Semi {
            self.advance();
        }
        Ok(Stmt::Emit { label, expr })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'return'
        if *self.peek() == TokenKind::Semi {
            self.advance();
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expr()?;
            if *self.peek() == TokenKind::Semi {
                self.advance();
            }
            Ok(Stmt::Return(Some(expr)))
        }
    }

    fn parse_fn(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'fn'
        let name_tok = self.advance();
        let name = match name_tok.kind {
            TokenKind::Ident(n) => n,
            other => return Err(format!("Expected function name, found {:?}", other)),
        };

        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        if *self.peek() != TokenKind::RParen {
            loop {
                let p_tok = self.advance();
                match p_tok.kind {
                    TokenKind::Ident(p) => params.push(p),
                    other => return Err(format!("Expected parameter name, found {:?}", other)),
                }
                if *self.peek() == TokenKind::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let mut body = Vec::new();
        while *self.peek() != TokenKind::RBrace && *self.peek() != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;

        Ok(Stmt::FnDef { name, params, body })
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'if'
        let cond = self.parse_expr()?;
        self.expect(TokenKind::LBrace)?;

        let mut then_branch = Vec::new();
        while *self.peek() != TokenKind::RBrace && *self.peek() != TokenKind::Eof {
            then_branch.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;

        let mut else_branch = None;
        if *self.peek() == TokenKind::Else {
            self.advance(); // consume 'else'
            if *self.peek() == TokenKind::If {
                // else if
                let nested_if = self.parse_if()?;
                else_branch = Some(vec![nested_if]);
            } else {
                self.expect(TokenKind::LBrace)?;
                let mut eb = Vec::new();
                while *self.peek() != TokenKind::RBrace && *self.peek() != TokenKind::Eof {
                    eb.push(self.parse_stmt()?);
                }
                self.expect(TokenKind::RBrace)?;
                else_branch = Some(eb);
            }
        }

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'for'
        let var_tok = self.advance();
        let var = match var_tok.kind {
            TokenKind::Ident(v) => v,
            other => return Err(format!("Expected loop variable name, found {:?}", other)),
        };

        self.expect(TokenKind::In)?;
        let start = self.parse_expr()?;
        self.expect(TokenKind::DotDot)?;
        let end = self.parse_expr()?;

        self.expect(TokenKind::LBrace)?;
        let mut body = Vec::new();
        while *self.peek() != TokenKind::RBrace && *self.peek() != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;

        Ok(Stmt::For {
            var,
            start,
            end,
            body,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'while'
        let cond = self.parse_expr()?;
        self.expect(TokenKind::LBrace)?;
        let mut body = Vec::new();
        while *self.peek() != TokenKind::RBrace && *self.peek() != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;

        Ok(Stmt::While { cond, body })
    }

    // Expression parsing with Pratt/Precedence climbing
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_pipe()
    }

    fn parse_pipe(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_or()?;

        while *self.peek() == TokenKind::PipeRight {
            self.advance(); // consume |>
            let right = self.parse_call_or_ident()?;
            match right {
                Expr::Call { callee, mut args } => {
                    args.insert(0, left);
                    left = Expr::Call { callee, args };
                }
                Expr::Ident(name) => {
                    left = Expr::Call {
                        callee: name,
                        args: vec![left],
                    };
                }
                other => return Err(format!("Invalid pipeline right-hand expression: {:?}", other)),
            }
        }

        Ok(left)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;
        while *self.peek() == TokenKind::Or {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while *self.peek() == TokenKind::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_relational()?;
        while matches!(self.peek(), TokenKind::Eq | TokenKind::Neq) {
            let op = match self.advance().kind {
                TokenKind::Eq => BinaryOp::Eq,
                TokenKind::Neq => BinaryOp::Neq,
                _ => unreachable!(),
            };
            let right = self.parse_relational()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_convert_to()?;
        while matches!(self.peek(), TokenKind::Lt | TokenKind::Lte | TokenKind::Gt | TokenKind::Gte) {
            let op = match self.advance().kind {
                TokenKind::Lt => BinaryOp::Lt,
                TokenKind::Lte => BinaryOp::Lte,
                TokenKind::Gt => BinaryOp::Gt,
                TokenKind::Gte => BinaryOp::Gte,
                _ => unreachable!(),
            };
            let right = self.parse_convert_to()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_convert_to(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        if *self.peek() == TokenKind::To {
            self.advance(); // consume 'to'
            // Read target unit, which might be an identifier or string or slash path like km/h
            let mut unit_target = String::new();
            match self.advance().kind {
                TokenKind::Ident(id) => unit_target.push_str(&id),
                TokenKind::Str(s) => unit_target.push_str(&s),
                other => return Err(format!("Expected target unit after 'to', found {:?}", other)),
            }
            // If followed by / and another ident, e.g. "km / h"
            if *self.peek() == TokenKind::Slash {
                self.advance();
                if let TokenKind::Ident(denom) = self.advance().kind {
                    unit_target.push('/');
                    unit_target.push_str(&denom);
                }
            }
            left = Expr::Binary {
                op: BinaryOp::ConvertTo(unit_target),
                left: Box::new(left),
                right: Box::new(Expr::Bool(true)),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        while matches!(self.peek(), TokenKind::Plus | TokenKind::Minus) {
            let op = match self.advance().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        while matches!(self.peek(), TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let op = match self.advance().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_power()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary()?;
        if *self.peek() == TokenKind::Caret {
            self.advance(); // consume '^'
            let right = self.parse_power()?; // right-associative
            Ok(Expr::Binary {
                op: BinaryOp::Pow,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            TokenKind::Bang => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                TokenKind::LParen => {
                    // Function call if expr was an identifier
                    if let Expr::Ident(name) = expr {
                        self.advance(); // consume '('
                        let mut args = Vec::new();
                        if *self.peek() != TokenKind::RParen {
                            loop {
                                args.push(self.parse_expr()?);
                                if *self.peek() == TokenKind::Comma {
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                        }
                        self.expect(TokenKind::RParen)?;
                        expr = Expr::Call { callee: name, args };
                    } else {
                        break;
                    }
                }
                TokenKind::LBracket => {
                    self.advance(); // consume '['
                    let index = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_call_or_ident(&mut self) -> Result<Expr, String> {
        let ident_tok = self.advance();
        let name = match ident_tok.kind {
            TokenKind::Ident(n) => n,
            other => return Err(format!("Expected identifier in pipeline, found {:?}", other)),
        };

        if *self.peek() == TokenKind::LParen {
            self.advance();
            let mut args = Vec::new();
            if *self.peek() != TokenKind::RParen {
                loop {
                    args.push(self.parse_expr()?);
                    if *self.peek() == TokenKind::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect(TokenKind::RParen)?;
            Ok(Expr::Call { callee: name, args })
        } else {
            Ok(Expr::Ident(name))
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Number(val, unit) => {
                let mut final_unit = unit;
                // If number has no attached unit, check if next token is a unit identifier
                if final_unit.is_none() {
                    if let TokenKind::Ident(ref id) = self.peek() {
                        if crate::units::lookup_unit(id).is_some() {
                            let id_str = id.clone();
                            self.advance(); // consume unit identifier
                            final_unit = Some(id_str);
                        }
                    }
                }
                Ok(Expr::Number {
                    val,
                    unit: final_unit,
                })
            }
            TokenKind::Str(s) => Ok(Expr::Str(s)),
            TokenKind::Bool(b) => Ok(Expr::Bool(b)),
            TokenKind::Ident(id) => Ok(Expr::Ident(id)),
            TokenKind::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                // Vector or Matrix literal: [1, 2, 3] or [1, 2; 3, 4]
                self.parse_vector_or_matrix()
            }
            other => Err(format!(
                "Unexpected token in expression: {:?} at {}:{}",
                other, tok.line, tok.col
            )),
        }
    }

    fn parse_vector_or_matrix(&mut self) -> Result<Expr, String> {
        if *self.peek() == TokenKind::RBracket {
            self.advance();
            return Ok(Expr::Vector(Vec::new()));
        }

        let mut elements = Vec::new();
        let mut has_semicolon = false;
        let mut row_count = 1;
        let mut first_row_len = 0;
        let mut cur_row_len = 0;

        loop {
            let elem = self.parse_expr()?;
            elements.push(elem);
            cur_row_len += 1;

            if *self.peek() == TokenKind::Comma {
                self.advance();
            } else if *self.peek() == TokenKind::Semi {
                has_semicolon = true;
                self.advance();
                if first_row_len == 0 {
                    first_row_len = cur_row_len;
                } else if cur_row_len != first_row_len {
                    return Err(format!(
                        "Inconsistent row length in matrix literal: expected {}, got {}",
                        first_row_len, cur_row_len
                    ));
                }
                cur_row_len = 0;
                row_count += 1;
            } else if *self.peek() == TokenKind::RBracket {
                self.advance();
                break;
            } else {
                return Err(format!(
                    "Expected comma, semicolon or ']' in array literal, found {:?}",
                    self.peek()
                ));
            }
        }

        if has_semicolon {
            if first_row_len == 0 {
                first_row_len = cur_row_len;
            }
            let cols = first_row_len;
            Ok(Expr::Matrix {
                rows: row_count,
                cols,
                elements,
            })
        } else {
            Ok(Expr::Vector(elements))
        }
    }
}
