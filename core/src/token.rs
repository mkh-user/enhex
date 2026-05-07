#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Atoms
    Digit,
    WordChar,
    Whitespace,
    Lowercase,
    Uppercase,
    Letter,
    Anything,
    Dot,
    Dash,
    Tab,
    Newline,

    // Quantifiers
    OneOrMore,
    ZeroOrMore,
    Optional,
    Exactly,
    AtLeast,
    Between,

    // Groups
    Group,
    NonCapturing,
    Named,

    // Anchors
    Start,
    End,
    WordBoundary,

    // Lookaround
    FollowedBy,
    NotFollowedBy,
    PrecededBy,
    NotPrecededBy,

    // Presets
    Tld,
    Email,
    Url,
    Ipv4,

    // Primitives
    Integer(i64),
    LiteralString(String),

    // Operators
    Plus,
    Pipe,
    Comma,

    // Delimiters
    LParen,
    RParen,

    // Special
    Eof,
}