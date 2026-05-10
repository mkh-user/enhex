pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

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
        assert_eq!(regex, r#"^[\w\.-]+@[\w-]+\.[a-z]{2,10}$"#);
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
        assert_eq!(regex, r#"^https?://[\w\.-]+\.[a-z]{2,10}(?:/.*)?$"#);
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

    #[test]
    fn test_non_digit() {
        assert_eq!(
            compile("start + one_or_more(non_digit) + end").unwrap(),
            r"^\D+$"
        );
    }

    #[test]
    fn test_non_word_char() {
        assert_eq!(
            compile("start + one_or_more(non_word_char) + end").unwrap(),
            r"^\W+$"
        );
    }

    #[test]
    fn test_non_whitespace() {
        assert_eq!(
            compile("start + one_or_more(non_whitespace) + end").unwrap(),
            r"^\S+$"
        );
    }

    #[test]
    fn test_hex_digit_standalone() {
        assert_eq!(
            compile("start + exactly(6, hex_digit) + end").unwrap(),
            r"^[\da-fA-F]{6}$"
        );
    }

    #[test]
    fn test_hex_digit_in_alternation() {
        assert_eq!(
            compile("start + one_or_more(hex_digit | dash) + end").unwrap(),
            r"^[\da-fA-F-]+$"
        );
    }

    #[test]
    fn test_carriage_return() {
        assert_eq!(
            compile(r#"start + "text" + carriage_return + "more" + end"#).unwrap(),
            r"^text\rmore$"
        );
    }

    #[test]
    fn test_tab_and_newline() {
        assert_eq!(
            compile("start + tab + newline + end").unwrap(),
            r"^\t\n$"
        );
    }

    #[test]
    fn test_null() {
        assert_eq!(
            compile("start + null + end").unwrap(),
            r"^\0$"
        );
    }

    #[test]
    fn test_vertical_tab() {
        assert_eq!(
            compile("start + vertical_tab + end").unwrap(),
            r"^\v$"
        );
    }

    #[test]
    fn test_form_feed() {
        assert_eq!(
            compile("start + form_feed + end").unwrap(),
            r"^\f$"
        );
    }

    #[test]
    fn test_bell() {
        assert_eq!(
            compile("start + bell + end").unwrap(),
            r"^\a$"
        );
    }

    #[test]
    fn test_backslash() {
        assert_eq!(
            compile(r#"start + backslash + end"#).unwrap(),
            r"^\\$"
        );
    }

    #[test]
    fn test_windows_path_with_escapes() {
        let regex = compile(
            r#"start + uppercase + ":" + exactly(2, backslash) + one_or_more(word_char | backslash | dot | dash) + end"#
        ).unwrap();
        assert_eq!(regex, r"^[A-Z]:\\{2}[\w\\\.-]+$");
    }

    #[test]
    fn test_not_single_atom() {
        assert_eq!(
            compile("start + one_or_more(not(digit)) + end").unwrap(),
            r"^[^\d]+$"
        );
    }

    #[test]
    fn test_not_multiple_atoms() {
        assert_eq!(
            compile("start + one_or_more(not(digit | letter)) + end").unwrap(),
            r"^[^\da-zA-Z]+$"
        );
    }

    #[test]
    fn test_not_whitespace() {
        assert_eq!(
            compile("start + one_or_more(not(whitespace)) + end").unwrap(),
            r"^[^\s]+$"
        );
    }

    #[test]
    fn test_not_literals() {
        let regex = compile(
            r#"start + not("a" | "e" | "i" | "o" | "u") + end"#
        ).unwrap();
        assert_eq!(regex, r"^[^aeiou]$");
    }

    #[test]
    fn test_not_in_character_class_context() {
        assert_eq!(
            compile(r#"start + not("\\" | "\"") + end"#).unwrap(),
            r#"^[^\\"]$"#
        );
    }

    #[test]
    fn test_one_or_more_lazy() {
        assert_eq!(
            compile("start + one_or_more_lazy(digit) + end").unwrap(),
            r"^\d+?$"
        );
    }

    #[test]
    fn test_zero_or_more_lazy() {
        assert_eq!(
            compile("start + zero_or_more_lazy(anything) + end").unwrap(),
            r"^.*?$"
        );
    }

    #[test]
    fn test_optional_lazy() {
        assert_eq!(
            compile("start + optional_lazy(digit) + end").unwrap(),
            r"^\d??$"
        );
    }

    #[test]
    fn test_lazy_with_group() {
        assert_eq!(
            compile("start + group(zero_or_more_lazy(anything)) + end").unwrap(),
            r"^(.*?)$"
        );
    }

    #[test]
    fn test_lazy_with_character_class() {
        assert_eq!(
            compile("start + one_or_more_lazy(digit | letter) + end").unwrap(),
            r"^[\da-zA-Z]+?$"
        );
    }

    #[test]
    fn test_backref_number() {
        let regex = compile(
            r#"start + group(digit) + "-" + backref(1) + end"#
        ).unwrap();
        assert_eq!(regex, r"^(\d)-\1$");
    }

    #[test]
    fn test_backref_number_multiple() {
        let regex = compile(
            r#"start + group(digit) + group(letter) + "-" + backref(1) + backref(2) + end"#
        ).unwrap();
        assert_eq!(regex, r"^(\d)([a-zA-Z])-\1\2$");
    }

    #[test]
    fn test_backref_name() {
        let regex = compile(
            r#"start + named("tag", one_or_more(word_char)) + ">" + backref("tag") + end"#
        ).unwrap();
        assert_eq!(regex, r"^(?P<tag>\w+)>(?P=tag)$");
    }

    #[test]
    fn test_backref_html_tag() {
        let regex = compile(
            r#"start + "<" + group(one_or_more(word_char)) + ">" + zero_or_more_lazy(anything) + "</" + backref(1) + ">" + end"#
        ).unwrap();
        assert_eq!(regex, r"^<(\w+)>.*?</\1>$");
    }

    #[test]
    fn test_not_with_quantifier() {
        let regex = compile(
            "start + exactly(3, not(digit)) + end"
        ).unwrap();
        assert_eq!(regex, r"^[^\d]{3}$");
    }

    #[test]
    fn test_lazy_with_not() {
        let regex = compile(
            "start + zero_or_more_lazy(not(whitespace)) + end"
        ).unwrap();
        assert_eq!(regex, r"^[^\s]*?$");
    }

    #[test]
    fn test_negated_class_within_lookaround() {
        let regex = compile(
            "start + digit + not_followed_by(not(digit)) + end"
        ).unwrap();
        assert_eq!(regex, r"^\d(?![^\d])$");
    }

    #[test]
    fn test_hex_color() {
        let regex = compile(
            "start + \"#\" + exactly(6, hex_digit) + end"
        ).unwrap();
        assert_eq!(regex, r"^#[\da-fA-F]{6}$");
    }

    #[test]
    fn test_c_style_comment_lazy() {
        let regex = compile(
            r#"start + "/*" + zero_or_more_lazy(anything) + "*/" + end"#
        ).unwrap();
        assert_eq!(regex, r"^/\*.*?\*/$");
    }

    #[test]
    fn test_backref_with_quantifiers() {
        let regex = compile(
            r#"start + group(letter) + one_or_more(backref(1)) + end"#
        ).unwrap();
        assert_eq!(regex, r"^([a-zA-Z])\1+$");
    }

    #[test]
    fn test_regex_literal() {
        let regex = compile(
            r#"/[\w\.-]+@[\w-]+\.[a-z]{2,10}/"#
        ).unwrap();
        assert_eq!(regex, r"[\w\.-]+@[\w-]+\.[a-z]{2,10}")
    }

    #[test]
    fn test_regex_func() {
        let regex = compile(
            r#"regex("\\d{3}-\\d{4}")"#
        ).unwrap();
        assert_eq!(regex, r"\d{3}-\d{4}")
    }
}
