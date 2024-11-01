use proptest::prelude::*;
use srgn::view::ScopedViewBuilder;

use crate::properties::DEFAULT_NUMBER_OF_TEST_CASES;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(DEFAULT_NUMBER_OF_TEST_CASES))]
    #[test]
    fn test_uppercasing_uppercase_has_no_effect(
        // https://docs.rs/regex/latest/regex/#matching-one-character
        // https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values
        input in r"\p{Uppercase_Letter}*"
    ) {
        let mut view = ScopedViewBuilder::new(&input).build();

        view.upper();
        let res = view.to_string();

        assert_eq!(res, input);
    }
}
