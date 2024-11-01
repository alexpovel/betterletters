use proptest::prelude::*;
use srgn::regex::Regex;
use srgn::view::ScopedViewBuilder;
use srgn::RegexPattern;

use crate::properties::DEFAULT_NUMBER_OF_TEST_CASES;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(DEFAULT_NUMBER_OF_TEST_CASES))]
    #[test]
    fn test_squeezing_anything_at_all_makes_the_input_shorter(
        // https://docs.rs/regex/latest/regex/#matching-one-character
        // https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values
        input in r"\p{Any}*AA\p{Any}*"
    ) {
        let mut builder = ScopedViewBuilder::new(&input);
        builder.explode(&Regex::new(RegexPattern::new("A").unwrap()));
        let mut view = builder.build();

        view.squeeze();
        let res = view.to_string();

        assert!(res.len() < input.len());
    }
}
