use crate::ast::*;

pub fn generate(expr: &Expr) -> String {
    match expr {
        Expr::Atom(atom) => generate_atom(atom),
        Expr::Literal(s) => regex_escape(s),
        Expr::Sequence(left, right) => format!("{}{}", generate(left), generate(right)),
        Expr::Alternation(left, right) => {
            // Try to build a character class from the alternation
            match build_char_class(expr) {
                Some(cc) => cc,
                None => format!("{}|{}", generate(left), generate(right)),
            }
        }
        Expr::Quantified { quantifier, expr } => {
            let inner = generate(expr);
            let suffix = match &quantifier.kind {
                QuantifierKind::OneOrMore => "+",
                QuantifierKind::ZeroOrMore => "*",
                QuantifierKind::Optional => "?",
                QuantifierKind::Exactly(n) => &format!("{{{}}}", n),
                QuantifierKind::AtLeast(n) => &format!("{{{},}}", n),
                QuantifierKind::Between(min, max) => &format!("{{{},{}}}", min, max),
            };
            let lazy_suffix = if quantifier.lazy { "?" } else { "" };

            if inner.starts_with('[') {
                format!("{}{}{}", inner, suffix, lazy_suffix)
            } else if needs_wrapping(expr) {
                format!("(?:{}){}{}", inner, suffix, lazy_suffix)
            } else {
                format!("{}{}{}", inner, suffix, lazy_suffix)
            }
        }
        Expr::Group { kind, name, expr } => {
            let inner = generate(expr);
            match kind {
                GroupKind::Normal => format!("({})", inner),
                GroupKind::NonCapturing => format!("(?:{})", inner),
                GroupKind::Named => {
                    if let Some(n) = name {
                        format!("(?P<{}>{})", n, inner)
                    } else {
                        format!("({})", inner)
                    }
                }
            }
        }
        Expr::Anchor(anchor) => match anchor {
            Anchor::Start => "^".to_string(),
            Anchor::End => "$".to_string(),
            Anchor::WordBoundary => "\\b".to_string(),
        }
        Expr::Lookaround { kind, expr } => {
            let inner = generate(expr);
            match kind {
                LookaroundKind::FollowedBy => format!("(?={})", inner),
                LookaroundKind::NotFollowedBy => format!("(?!{})", inner),
                LookaroundKind::PrecededBy => format!("(?<={})", inner),
                LookaroundKind::NotPrecededBy => format!("(?<!{})", inner),
            }
        }
        Expr::Backref(backref) => match backref {
            Backref::Number(n) => format!("\\{}", n),
            Backref::Name(name) => format!("(?P={})", name),
        }
        Expr::NotCharClass(expr) => {
            match build_char_class(expr) {
                Some(cc) => {
                    format!("[^{}", &cc[1..])
                }
                None => {
                    let inner = generate(expr);
                    format!("[^{}]", inner)
                }
            }
        }
        Expr::Preset(preset) => match preset {
            Preset::Tld => "[a-z]{2,10}".to_string(),
            Preset::Email => r#"[\w\.-]+@[\w-]+\.[a-z]{2,10}"#.to_string(),
            Preset::Url => r#"https?://[\w\.-]+\.[a-z]{2,10}(?:/.*)?"#.to_string(),
            Preset::Ipv4 => r#"(?:\d{1,3}\.){3}\d{1,3}"#.to_string(),
        }
    }
}

fn generate_atom(atom: &Atom) -> String {
    match atom {
        Atom::Digit => "\\d".to_string(),
        Atom::NonDigit => "\\D".to_string(),
        Atom::WordChar => "\\w".to_string(),
        Atom::NonWordChar => "\\W".to_string(),
        Atom::Whitespace => "\\s".to_string(),
        Atom::NonWhitespace => "\\S".to_string(),
        Atom::Lowercase => "[a-z]".to_string(),
        Atom::Uppercase => "[A-Z]".to_string(),
        Atom::Letter => "[a-zA-Z]".to_string(),
        Atom::Anything => ".".to_string(),
        Atom::Dot => "\\.".to_string(),
        Atom::Dash => "\\-".to_string(),
        Atom::Tab => "\\t".to_string(),
        Atom::Newline => "\\n".to_string(),
        Atom::HexDigit => "[\\da-fA-F]".to_string(),
        Atom::CarriageReturn => "\\r".to_string(),
        Atom::Null => "\\0".to_string(),
        Atom::VerticalTab => "\\v".to_string(),
        Atom::FormFeed => "\\f".to_string(),
        Atom::Bell => "\\a".to_string(),
        Atom::Backslash => "\\\\".to_string(),
    }
}

fn regex_escape(s: &str) -> String {
    let special = ['.', '+', '*', '?', '[', ']', '(', ')', '{', '}', '^', '$', '|', '\\'];
    let mut result = String::new();
    for ch in s.chars() {
        if special.contains(&ch) {
            result.push('\\');
        }
        result.push(ch);
    }
    result
}

/// Check if an expression needs to be wrapped in (?:...) when quantified
fn needs_wrapping(expr: &Expr) -> bool {
    match expr {
        Expr::Alternation(_, _) => true,
        Expr::Sequence(_, _) => true,
        Expr::Literal(s) => s.len() > 1,
        _ => false,
    }
}

/// Try to build a regex character class [abc] from an alternation tree.
/// Returns Some(class_string) if all leaves are single characters or atom shorthands.
fn build_char_class(expr: &Expr) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if !collect_char_class_parts(expr, &mut parts) {
        return None;
    }
    if parts.is_empty() {
        return None;
    }

    // Move '-' to the end to avoid range ambiguity
    let mut sorted_parts = Vec::new();
    let mut dash_part = None;
    for part in parts {
        if part == "-" {
            dash_part = Some(part);
        } else {
            sorted_parts.push(part);
        }
    }
    if let Some(d) = dash_part {
        sorted_parts.push(d);
    }

    Some(format!("[{}]", sorted_parts.join("")))
}

fn collect_char_class_parts(expr: &Expr, parts: &mut Vec<String>) -> bool {
    match expr {
        Expr::Alternation(left, right) => {
            collect_char_class_parts(left, parts) && collect_char_class_parts(right, parts)
        }
        Expr::Atom(atom) => {
            match atom {
                Atom::Digit => parts.push("\\d".to_string()),
                Atom::NonDigit => parts.push("\\D".to_string()),
                Atom::WordChar => parts.push("\\w".to_string()),
                Atom::NonWordChar => parts.push("\\W".to_string()),
                Atom::Whitespace => parts.push("\\s".to_string()),
                Atom::NonWhitespace => parts.push("\\S".to_string()),
                Atom::Lowercase => parts.push("a-z".to_string()),
                Atom::Uppercase => parts.push("A-Z".to_string()),
                Atom::Letter => parts.push("a-zA-Z".to_string()),
                Atom::Newline => parts.push("\\n".to_string()),
                Atom::Dot => parts.push("\\.".to_string()),
                Atom::Dash => parts.push("-".to_string()),
                Atom::HexDigit => parts.push("\\da-fA-F".to_string()),
                Atom::CarriageReturn => parts.push("\\r".to_string()),
                Atom::Null => parts.push("\\0".to_string()),
                Atom::VerticalTab => parts.push("\\v".to_string()),
                Atom::FormFeed => parts.push("\\f".to_string()),
                Atom::Bell => parts.push("\\a".to_string()),
                Atom::Backslash => parts.push("\\\\".to_string()),
                _ => return false,
            }
            true
        }
        Expr::Literal(s) => {
            if s.len() != 1 {
                return false;
            }
            let ch = s.chars().next().unwrap();
            match ch {
                ']' => parts.push("\\]".to_string()),
                '^' => parts.push("\\^".to_string()),
                '\\' => parts.push("\\\\".to_string()),
                _ => {
                    // Escape special regex chars inside character class
                    match ch {
                        '.' | '+' | '*' | '?' | '|' | '(' | ')' | '{' | '}' | '$' => {
                            parts.push(format!("\\{}", ch));
                        }
                        _ => parts.push(ch.to_string()),
                    }
                }
            }
            true
        }
        _ => false,
    }
}
