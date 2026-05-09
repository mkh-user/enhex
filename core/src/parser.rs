use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    peeked: Option<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            lexer: Lexer::new(input),
            peeked: None,
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        if let Some(tok) = self.peeked.take() {
            return Ok(tok);
        }
        self.lexer.next_token()
    }

    fn peek_token(&mut self) -> Result<Token, ParseError> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next_token()?);
        }
        Ok(self.peeked.clone().unwrap())
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expression()?;
        let tok = self.next_token()?;
        if tok != Token::Eof {
            return Err(ParseError::new(
                format!("Unexpected token after end of expression: {:?}", tok),
                0,
                0,
            ));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_sequence()
    }

    fn parse_sequence(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_alternation()?;

        while self.peek_token()? == Token::Plus {
            self.next_token()?; // consume '+'
            let right = self.parse_alternation()?;
            left = Expr::Sequence(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_alternation(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;

        while self.peek_token()? == Token::Pipe {
            self.next_token()?; // consume '|'
            let right = self.parse_term()?;
            left = Expr::Alternation(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let tok = self.next_token()?;

        match tok {
            // Atoms
            Token::Digit => Ok(Expr::Atom(Atom::Digit)),
            Token::NonDigit => Ok(Expr::Atom(Atom::NonDigit)),
            Token::WordChar => Ok(Expr::Atom(Atom::WordChar)),
            Token::NonWordChar => Ok(Expr::Atom(Atom::NonWordChar)),
            Token::Whitespace => Ok(Expr::Atom(Atom::Whitespace)),
            Token::NonWhitespace => Ok(Expr::Atom(Atom::NonWhitespace)),
            Token::Lowercase => Ok(Expr::Atom(Atom::Lowercase)),
            Token::Uppercase => Ok(Expr::Atom(Atom::Uppercase)),
            Token::Letter => Ok(Expr::Atom(Atom::Letter)),
            Token::Anything => Ok(Expr::Atom(Atom::Anything)),
            Token::Dot => Ok(Expr::Atom(Atom::Dot)),
            Token::Dash => Ok(Expr::Atom(Atom::Dash)),
            Token::Tab => Ok(Expr::Atom(Atom::Tab)),
            Token::Newline => Ok(Expr::Atom(Atom::Newline)),
            Token::HexDigit => Ok(Expr::Atom(Atom::HexDigit)),
            Token::CarriageReturn => Ok(Expr::Atom(Atom::CarriageReturn)),
            Token::Null => Ok(Expr::Atom(Atom::Null)),
            Token::VerticalTab => Ok(Expr::Atom(Atom::VerticalTab)),
            Token::FormFeed => Ok(Expr::Atom(Atom::FormFeed)),
            Token::Bell => Ok(Expr::Atom(Atom::Bell)),
            Token::Backslash => Ok(Expr::Atom(Atom::Backslash)),

            // Anchors
            Token::Start => Ok(Expr::Anchor(Anchor::Start)),
            Token::End => Ok(Expr::Anchor(Anchor::End)),
            Token::WordBoundary => Ok(Expr::Anchor(Anchor::WordBoundary)),

            // Literal string
            Token::LiteralString(s) => Ok(Expr::Literal(s)),

            // Backreference
            Token::Backref => {
                self.expect(Token::LParen)?;
                let tok = self.next_token()?;
                let backref = match tok {
                    Token::Integer(n) => Backref::Number(n),
                    Token::LiteralString(s) => Backref::Name(s),
                    _ => return Err(ParseError::new(
                        "Expected number or string literal for backreference",
                        0, 0,
                    )),
                };
                self.expect(Token::RParen)?;
                Ok(Expr::Backref(backref))
            }

            // Negated char class
            Token::Not => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::NotCharClass(Box::new(expr)))
            }

            // Quantifiers
            Token::OneOrMore
            | Token::ZeroOrMore
            | Token::Optional
            | Token::Exactly
            | Token::AtLeast
            | Token::Between
            | Token::OneOrMoreLazy
            | Token::ZeroOrMoreLazy
            | Token::OptionalLazy => self.parse_quantified(tok),

            // Groups
            Token::Group | Token::NonCapturing | Token::Named => self.parse_group(tok),

            // Lookaround
            Token::FollowedBy
            | Token::NotFollowedBy
            | Token::PrecededBy
            | Token::NotPrecededBy => self.parse_lookaround(tok),

            // Presets
            Token::Tld => {
                self.expect_parens()?;
                Ok(Expr::Preset(Preset::Tld))
            }
            Token::Email => {
                self.expect_parens()?;
                Ok(Expr::Preset(Preset::Email))
            }
            Token::Url => {
                self.expect_parens()?;
                Ok(Expr::Preset(Preset::Url))
            }
            Token::Ipv4 => {
                self.expect_parens()?;
                Ok(Expr::Preset(Preset::Ipv4))
            }

            // Parenthesized expression
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }

            tok => Err(ParseError::new(
                format!("Unexpected token: {:?}", tok),
                0,
                0,
            )),
        }
    }

    fn parse_quantified(&mut self, quant_token: Token) -> Result<Expr, ParseError> {
        match quant_token {
            Token::OneOrMore => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::OneOrMore, lazy: false },
                    expr: Box::new(expr),
                })
            }
            Token::OneOrMoreLazy => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::OneOrMore, lazy: true },
                    expr: Box::new(expr),
                })
            }
            Token::ZeroOrMore => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::ZeroOrMore, lazy: false },
                    expr: Box::new(expr),
                })
            }
            Token::ZeroOrMoreLazy => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::ZeroOrMore, lazy: true },
                    expr: Box::new(expr),
                })
            }
            Token::Optional => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::Optional, lazy: false },
                    expr: Box::new(expr),
                })
            }
            Token::OptionalLazy => {
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::Optional, lazy: true },
                    expr: Box::new(expr),
                })
            }
            Token::Exactly => {
                self.expect(Token::LParen)?;
                let tok = self.next_token()?;
                let n = match tok {
                    Token::Integer(n) => n,
                    _ => return Err(ParseError::new("Expected integer", 0, 0)),
                };
                self.expect(Token::Comma)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::Exactly(n), lazy: false },
                    expr: Box::new(expr),
                })
            }
            Token::AtLeast => {
                self.expect(Token::LParen)?;
                let tok = self.next_token()?;
                let n = match tok {
                    Token::Integer(n) => n,
                    _ => return Err(ParseError::new("Expected integer", 0, 0)),
                };
                self.expect(Token::Comma)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::AtLeast(n), lazy: false },
                    expr: Box::new(expr),
                })
            }
            Token::Between => {
                self.expect(Token::LParen)?;
                let tok1 = self.next_token()?;
                let min = match tok1 {
                    Token::Integer(n) => n,
                    _ => return Err(ParseError::new("Expected integer", 0, 0)),
                };
                self.expect(Token::Comma)?;
                let tok2 = self.next_token()?;
                let max = match tok2 {
                    Token::Integer(n) => n,
                    _ => return Err(ParseError::new("Expected integer", 0, 0)),
                };
                self.expect(Token::Comma)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Quantified {
                    quantifier: QuantifierData { kind: QuantifierKind::Between(min, max), lazy: false },
                    expr: Box::new(expr),
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_group(&mut self, group_token: Token) -> Result<Expr, ParseError> {
        let kind = match group_token {
            Token::Group => GroupKind::Normal,
            Token::NonCapturing => GroupKind::NonCapturing,
            Token::Named => GroupKind::Named,
            _ => unreachable!(),
        };

        self.expect(Token::LParen)?;

        let name = if kind == GroupKind::Named {
            let tok = self.next_token()?;
            match tok {
                Token::LiteralString(s) => {
                    self.expect(Token::Comma)?;
                    Some(s)
                }
                _ => return Err(ParseError::new("Expected string literal for group name", 0, 0)),
            }
        } else {
            None
        };

        let expr = self.parse_expression()?;
        self.expect(Token::RParen)?;

        Ok(Expr::Group {
            kind,
            name,
            expr: Box::new(expr),
        })
    }

    fn parse_lookaround(&mut self, look_token: Token) -> Result<Expr, ParseError> {
        let kind = match look_token {
            Token::FollowedBy => LookaroundKind::FollowedBy,
            Token::NotFollowedBy => LookaroundKind::NotFollowedBy,
            Token::PrecededBy => LookaroundKind::PrecededBy,
            Token::NotPrecededBy => LookaroundKind::NotPrecededBy,
            _ => unreachable!(),
        };

        self.expect(Token::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(Token::RParen)?;

        Ok(Expr::Lookaround {
            kind,
            expr: Box::new(expr),
        })
    }

    fn expect_parens(&mut self) -> Result<(), ParseError> {
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let tok = self.next_token()?;
        if tok == expected {
            Ok(())
        } else {
            Err(ParseError::new(
                format!("Expected {:?} but found {:?}", expected, tok),
                0,
                0,
            ))
        }
    }
}
