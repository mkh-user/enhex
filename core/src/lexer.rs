use crate::error::ParseError;
use crate::token::Token;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_string(&mut self) -> Result<String, ParseError> {
        // Skip opening quote
        self.advance();
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // skip closing quote
                return Ok(result);
            }
            result.push(self.advance().unwrap());
        }
        Err(ParseError::new(
            "Unclosed string literal",
            self.line,
            self.col,
        ))
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        result
    }

    fn read_integer(&mut self) -> i64 {
        let s = self.read_identifier();
        s.parse().unwrap_or(0)
    }

    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        while self.peek() == Some('#') {
            self.skip_comment();
            self.skip_whitespace();
        }

        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        let line = self.line;
        let col = self.col;

        match ch {
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            '|' => {
                self.advance();
                Ok(Token::Pipe)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '(' => {
                self.advance();
                Ok(Token::LParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RParen)
            }
            '"' => {
                let s = self.read_string()?;
                Ok(Token::LiteralString(s))
            }
            c if c.is_ascii_digit() => {
                let n = self.read_integer();
                Ok(Token::Integer(n))
            }
            c if c.is_alphanumeric() || c == '_' => {
                let ident = self.read_identifier();
                let tok = match ident.as_str() {
                    // Atoms
                    "digit" => Token::Digit,
                    "word_char" => Token::WordChar,
                    "whitespace" => Token::Whitespace,
                    "lowercase" => Token::Lowercase,
                    "uppercase" => Token::Uppercase,
                    "letter" => Token::Letter,
                    "anything" => Token::Anything,
                    "dot" => Token::Dot,
                    "dash" => Token::Dash,
                    "tab" => Token::Tab,
                    "newline" => Token::Newline,

                    // Quantifiers
                    "one_or_more" => Token::OneOrMore,
                    "zero_or_more" => Token::ZeroOrMore,
                    "optional" => Token::Optional,
                    "exactly" => Token::Exactly,
                    "at_least" => Token::AtLeast,
                    "between" => Token::Between,

                    // Groups
                    "group" => Token::Group,
                    "non_capturing" => Token::NonCapturing,
                    "named" => Token::Named,

                    // Anchors
                    "start" => Token::Start,
                    "end" => Token::End,
                    "word_boundary" => Token::WordBoundary,

                    // Lookaround
                    "followed_by" => Token::FollowedBy,
                    "not_followed_by" => Token::NotFollowedBy,
                    "preceded_by" => Token::PrecededBy,
                    "not_preceded_by" => Token::NotPrecededBy,

                    // Presets
                    "tld" => Token::Tld,
                    "email" => Token::Email,
                    "url" => Token::Url,
                    "ipv4" => Token::Ipv4,

                    _ => {
                        return Err(ParseError::new(
                            format!("Unknown token '{}'. Did you mean something else?", ident),
                            line,
                            col,
                        ))
                    }
                };
                Ok(tok)
            }
            _ => Err(ParseError::new(
                format!("Unexpected character '{}'", ch),
                line,
                col,
            )),
        }
    }
}