pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

use parser::Parser;
use codegen::generate;

/// Compile an EnhEx pattern string to a Regex string.
///
/// # Example
///
/// ```
/// let regex = enhex_core::compile("start + one_or_more(digit) + end").unwrap();
/// assert_eq!(regex, r"^\d+$");
/// ```
pub fn compile(input: &str) -> Result<String, error::ParseError> {
    let mut parser = Parser::new(input);
    let ast = parser.parse()?;
    Ok(generate(&ast))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(compile("start + one_or_more(digit) + end").unwrap(), r"^\d+$");
    }

    #[test]
    fn test_email() {
        let regex = compile(
            r#"start + one_or_more(word_char | dot | dash) + "@" + one_or_more(word_char | dash) + "." + tld() + end"#
        ).unwrap();
        assert_eq!(regex, r#"^[\w\.\-]+@[\w\-]+\.[a-z]{2,10}$"#);
    }

    #[test]
    fn test_phone() {
        assert_eq!(
            compile(r#"start + "09" + exactly(9, digit) + end"#).unwrap(),
            r#"^09\d{9}$"#
        );
    }

    #[test]
    fn test_url() {
        let regex = compile(
            r#"start + "http" + optional("s") + "://" + one_or_more(word_char | dot | dash) + "." + tld() + optional("/" + zero_or_more(anything)) + end"#
        ).unwrap();
        assert_eq!(regex, r#"^https?://[\w\.\-]+\.[a-z]{2,10}(?:/.*)?$"#);
    }

    #[test]
    fn test_date() {
        let regex = compile(
            r#"start + exactly(4, digit) + "-" + between(1, 2, digit) + "-" + between(1, 2, digit) + end"#
        ).unwrap();
        assert_eq!(regex, r"^\d{4}-\d{1,2}-\d{1,2}$");
    }

    #[test]
    fn test_preset_email() {
        let regex = compile("start + email() + end").unwrap();
        assert_eq!(regex, r#"^[\w\.-]+@[\w-]+\.[a-z]{2,10}$"#);
    }

    #[test]
    fn test_preset_ipv4() {
        let regex = compile("start + ipv4() + end").unwrap();
        assert_eq!(regex, r"^(?:\d{1,3}\.){3}\d{1,3}$");
    }

    #[test]
    fn test_named_group() {
        let regex = compile(
            r#"start + named("area", exactly(3, digit)) + "-" + exactly(4, digit) + end"#
        ).unwrap();
        assert_eq!(regex, r"^(?P<area>\d{3})-\d{4}$");
    }

    #[test]
    fn test_lookahead() {
        let regex = compile(r#"digit + followed_by("$") + end"#).unwrap();
        assert_eq!(regex, r"\d(?=\$)$");
    }

    #[test]
    fn test_comment() {
        let regex = compile(
            "# this is a comment\nstart + one_or_more(digit) + end # another comment"
        ).unwrap();
        assert_eq!(regex, r"^\d+$");
    }
}