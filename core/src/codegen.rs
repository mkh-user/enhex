use crate::ast::*;

pub fn generate(expr: &Expr) -> String {
    match expr {
        Expr::Atom(atom) => generate_atom(atom),
        Expr::Literal(s) => regex_escape(s),
        Expr::Sequence(left, right) => format!("{}{}", generate(left), generate(right)),
        Expr::Alternation(left, right) => format!("{}|{}", generate(left), generate(right)),
        Expr::Quantified { quantifier, expr } => {
            let inner = generate(expr);
            match quantifier {
                Quantifier::OneOrMore => format!("{}+", wrap_if_needed(&inner, expr)),
                Quantifier::ZeroOrMore => format!("{}*", wrap_if_needed(&inner, expr)),
                Quantifier::Optional => format!("{}?", wrap_if_needed(&inner, expr)),
                Quantifier::Exactly(n) => format!("{}{{{}}}", wrap_if_needed(&inner, expr), n),
                Quantifier::AtLeast(n) => format!("{}{{{},}}", wrap_if_needed(&inner, expr), n),
                Quantifier::Between(min, max) => {
                    format!("{}{{{},{}}}", wrap_if_needed(&inner, expr), min, max)
                }
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
        },
        Expr::Lookaround { kind, expr } => {
            let inner = generate(expr);
            match kind {
                LookaroundKind::FollowedBy => format!("(?={})", inner),
                LookaroundKind::NotFollowedBy => format!("(?!{})", inner),
                LookaroundKind::PrecededBy => format!("(?<={})", inner),
                LookaroundKind::NotPrecededBy => format!("(?<!{})", inner),
            }
        }
        Expr::Preset(preset) => match preset {
            Preset::Tld => "[a-z]{2,10}".to_string(),
            Preset::Email => {
                r#"[\w\.-]+@[\w-]+\.[a-z]{2,10}"#.to_string()
            }
            Preset::Url => {
                r#"https?://[\w\.-]+\.[a-z]{2,10}(?:/.*)?"#.to_string()
            }
            Preset::Ipv4 => {
                r#"(?:\d{1,3}\.){3}\d{1,3}"#.to_string()
            }
        },
    }
}

fn generate_atom(atom: &Atom) -> String {
    match atom {
        Atom::Digit => "\\d".to_string(),
        Atom::WordChar => "\\w".to_string(),
        Atom::Whitespace => "\\s".to_string(),
        Atom::Lowercase => "[a-z]".to_string(),
        Atom::Uppercase => "[A-Z]".to_string(),
        Atom::Letter => "[a-zA-Z]".to_string(),
        Atom::Anything => ".".to_string(),
        Atom::Dot => "\\.".to_string(),
        Atom::Dash => "\\-".to_string(),
        Atom::Tab => "\\t".to_string(),
        Atom::Newline => "\\n".to_string(),
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

fn wrap_if_needed(inner: &str, expr: &Expr) -> String {
    match expr {
        Expr::Alternation(_, _) => {
            if let Some(chars) = try_into_char_class(expr) {
                return format!("[{}]", chars);
            }
            format!("(?:{})", inner)
        }
        Expr::Sequence(_, _) => {
            format!("(?:{})", inner)
        }
        _ => inner.to_string(),
    }
}

fn try_into_char_class(expr: &Expr) -> Option<String> {
    let mut chars = String::new();
    let mut exprs = vec![expr];
    
    while let Some(current) = exprs.pop() {
        match current {
            Expr::Alternation(left, right) => {
                exprs.push(right);
                exprs.push(left);
            }
            Expr::Atom(atom) => {
                chars.push_str(&generate_atom(atom));
            }
            Expr::Literal(s) if s.len() == 1 => {
                chars.push_str(&regex_escape(s));
            }
            _ => return None,
        }
    }
    
    Some(chars)
}