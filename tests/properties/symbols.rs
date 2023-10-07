use proptest::prelude::*;
use srgn::{
    actions::{Symbols, SymbolsInversion},
    scoping::ScopedViewBuilder,
    Action,
};

use crate::properties::DEFAULT_NUMBER_OF_TEST_CASES;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(DEFAULT_NUMBER_OF_TEST_CASES * 2))]
    /// Cannot be idempotent on non-ASCII input. Input might contain e.g. en-dash, which
    /// the symbols action will leave untouched, but will be decomposed into two hyphens
    /// by the symbols inversion action.
    #[test]
    fn test_inverting_symbols_is_idempotent_on_ascii_input(
        // https://docs.rs/regex/latest/regex/#matching-one-character
        // https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values
        input in r"[ -~]*(-|<|>|=|!){2,3}[ -~]*"
    ) {
        let applied = {
            let mut view = ScopedViewBuilder::new(&input).build();
            Symbols::default().map(&mut view);
            view.to_string()
        };

        let inverted = {
            let mut view = ScopedViewBuilder::new(&applied).build();
            SymbolsInversion::default().map(&mut view);
            view.to_string()
        };

        assert_eq!(input, inverted);
    }
}
