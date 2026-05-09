#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Atoms
    Digit,
    NonDigit,
    WordChar,
    NonWordChar,
    Whitespace,
    NonWhitespace,
    Lowercase,
    Uppercase,
    Letter,
    Anything,
    Dot,
    Dash,
    Tab,
    Newline,
    HexDigit,
    CarriageReturn,
    Null,
    VerticalTab,
    FormFeed,
    Bell,
    Backslash,

    // Quantifiers
    OneOrMore,
    ZeroOrMore,
    Optional,
    Exactly,
    AtLeast,
    Between,
    OneOrMoreLazy,
    ZeroOrMoreLazy,
    OptionalLazy,

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

    // Backreference
    Backref,

    // Negated char class
    Not,

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
