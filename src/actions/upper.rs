use super::Action;

/// Renders in uppercase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Upper {}

impl Action for Upper {
    fn act(&self, input: &str) -> String {
        input.replace('ß', "ẞ").to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    // German
    #[case("a", "A")]
    #[case("A", "A")]
    #[case("ä", "Ä")]
    #[case("Ä", "Ä")]
    #[case("ö", "Ö")]
    #[case("Ö", "Ö")]
    #[case("ü", "Ü")]
    #[case("Ü", "Ü")]
    #[case("ß", "ẞ")]
    #[case("ẞ", "ẞ")]
    #[case("aAäÄöÖüÜßẞ!", "AAÄÄÖÖÜÜẞẞ!")]
    #[case("ss", "SS")]
    //
    // Chinese
    #[case("你好!", "你好!")]
    //
    // Japanese
    #[case("こんにちは!", "こんにちは!")]
    //
    // Korean
    #[case("안녕하세요!", "안녕하세요!")]
    //
    // Russian
    #[case("привет!", "ПРИВЕТ!")]
    //
    // Emojis
    #[case("👋\0", "👋\0")]
    fn substitute(#[case] input: &str, #[case] expected: &str) {
        let result = Upper::default().act(input);
        assert_eq!(result, expected);
    }
}
