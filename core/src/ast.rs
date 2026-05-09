#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Literal(String),
    Sequence(Box<Expr>, Box<Expr>),
    Alternation(Box<Expr>, Box<Expr>),
    Quantified {
        quantifier: QuantifierData,
        expr: Box<Expr>,
    },
    Group {
        kind: GroupKind,
        name: Option<String>,
        expr: Box<Expr>,
    },
    Anchor(Anchor),
    Lookaround {
        kind: LookaroundKind,
        expr: Box<Expr>,
    },
    Preset(Preset),
    Backref(Backref),
    NotCharClass(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
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
    CarriageReturn,
    HexDigit,
    Null,
    VerticalTab,
    FormFeed,
    Bell,
    Backslash,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantifierData {
    pub kind: QuantifierKind,
    pub lazy: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuantifierKind {
    OneOrMore,
    ZeroOrMore,
    Optional,
    Exactly(i64),
    AtLeast(i64),
    Between(i64, i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupKind {
    Normal,
    NonCapturing,
    Named,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    Start,
    End,
    WordBoundary,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LookaroundKind {
    FollowedBy,
    NotFollowedBy,
    PrecededBy,
    NotPrecededBy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Preset {
    Tld,
    Email,
    Url,
    Ipv4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Backref {
    Number(i64),
    Name(String),
}
