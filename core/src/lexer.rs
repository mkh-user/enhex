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

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
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
        self.advance(); // skip opening quote
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance(); // skip closing quote
                    return Ok(result);
                }
                '\\' => {
                    self.advance(); // skip backslash
                    match self.peek() {
                        Some('\\') => { result.push('\\'); self.advance(); }
                        Some('"') => { result.push('"'); self.advance(); }
                        Some('n') => { result.push('\n'); self.advance(); }
                        Some('t') => { result.push('\t'); self.advance(); }
                        Some(c) => {
                            return Err(ParseError::new(
                                format!("Unknown escape sequence: \\{}", c),
                                self.line,
                                self.col,
                            ));
                        }
                        None => {
                            return Err(ParseError::new(
                                "Unclosed string literal after escape",
                                self.line,
                                self.col,
                            ));
                        }
                    }
                }
                _ => {
                    result.push(self.advance().unwrap());
                }
            }
        }
        Err(ParseError::new("Unclosed string literal", self.line, self.col))
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

    fn read_regex_literal(&mut self) -> Result<String, ParseError> {
        self.advance(); // skip opening `/`
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                '/' => {
                    self.advance(); // skip closing `/`
                    // Read flags
                    let flags = self.read_identifier();
                    if !flags.is_empty() {
                        return Ok(format!("(?{}){}", flags, result));
                    }
                    return Ok(result);
                }
                '\\' => {
                    // Escaped slash or backslash inside regex
                    self.advance();
                    match self.peek() {
                        Some('/') => result.push('/'),
                        Some('\\') => result.push('\\'),
                        Some(c) => {
                            result.push('\\');
                            result.push(c);
                        }
                        None => break,
                    }
                    self.advance();
                }
                _ => {
                    result.push(ch);
                    self.advance();
                }
            }
        }
        Err(ParseError::new("Unclosed regex literal", self.line, self.col))
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
            '/' => {
                if let Some(next) = self.peek_next() {
                    if !next.is_whitespace() && next != '+' && next != '|' && next != ',' {
                        let s = self.read_regex_literal()?;
                        return Ok(Token::RegexLiteral(s));
                    }
                }
                Err(ParseError::new("Unexpected '/'", self.line, self.col))
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
                    "non_digit" => Token::NonDigit,
                    "word_char" => Token::WordChar,
                    "non_word_char" => Token::NonWordChar,
                    "whitespace" => Token::Whitespace,
                    "non_whitespace" => Token::NonWhitespace,
                    "lowercase" => Token::Lowercase,
                    "uppercase" => Token::Uppercase,
                    "letter" => Token::Letter,
                    "anything" => Token::Anything,
                    "dot" => Token::Dot,
                    "dash" => Token::Dash,
                    "tab" => Token::Tab,
                    "newline" => Token::Newline,
                    "hex_digit" => Token::HexDigit,
                    "carriage_return" => Token::CarriageReturn,
                    "null" => Token::Null,
                    "vertical_tab" => Token::VerticalTab,
                    "form_feed" => Token::FormFeed,
                    "bell" => Token::Bell,
                    "backslash" => Token::Backslash,

                    // Quantifiers
                    "one_or_more" => Token::OneOrMore,
                    "zero_or_more" => Token::ZeroOrMore,
                    "optional" => Token::Optional,
                    "exactly" => Token::Exactly,
                    "at_least" => Token::AtLeast,
                    "between" => Token::Between,
                    "one_or_more_lazy" => Token::OneOrMoreLazy,
                    "zero_or_more_lazy" => Token::ZeroOrMoreLazy,
                    "optional_lazy" => Token::OptionalLazy,

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

                    // Backreference
                    "backref" => Token::Backref,

                    // Negated char class
                    "not" => Token::Not,

                    // RegEx
                    "regex" => Token::RegexFunc,

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
