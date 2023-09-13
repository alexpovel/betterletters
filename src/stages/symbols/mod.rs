#[cfg(doc)]
use super::GermanStage;
use super::{tooling::StageResult, Stage};
use std::collections::VecDeque;

/// Replace ASCII symbols (`--`, `->`, `!=`, ...) with proper Unicode equivalents (`–`,
/// `→`, `≠`, ...).
///
/// This stage is greedy, i.e. it will try to replace as many symbols as possible,
/// replacing left-to-right as greedily as possible.
#[derive(Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub struct SymbolsStage;

macro_rules! fetch_next {
    ($it:expr, $stack:expr, $buf:expr $(, $label:tt)?) => {
        if let Some(c) = $it.pop_front() {
            $stack.push(c);
            c
        } else {
            $buf.push_str(&$stack.into_iter().collect::<String>());

            // Control flow, thus a macro is required. Optionally, allow a label for
            // more control, e.g. when looping while waiting.
            break $($label)?;
        }
    };
}

impl Stage for SymbolsStage {
    /// ## Implementation note
    ///
    /// Only relevant when looking at the source code.
    ///
    /// The implementation is in the style of coroutines as presented [in this
    /// article](https://www.chiark.greenend.org.uk/~sgtatham/quasiblog/coroutines-philosophy/).
    /// Instead of writing an explicit state machine (like in [`GermanStage`]), we use a
    /// generator coroutine to consume values from. The position in code itself is then
    /// our state. `undo_overfetching` is a bit like sending a value back into the
    /// coroutine so it can be yielded again.
    ///
    /// All in all, ugly and verbose, would not recommend, but a worthwhile experiment.
    fn substitute(&self, input: &str) -> StageResult {
        let mut deque = input.chars().collect::<VecDeque<_>>();
        let mut out = String::new();

        'outer: loop {
            let mut stack = Vec::new();

            match fetch_next!(deque, stack, out) {
                '-' => match fetch_next!(deque, stack, out) {
                    '-' => {
                        // Be greedy, could be last character
                        replace(&mut stack, Symbol::EnDash);

                        match fetch_next!(deque, stack, out) {
                            '-' => replace(&mut stack, Symbol::EmDash),
                            '>' => replace(&mut stack, Symbol::LongRightArrow),
                            _ => undo_overfetching(&mut deque, &mut stack),
                        }
                    }
                    '>' => replace(&mut stack, Symbol::ShortRightArrow),
                    _ => undo_overfetching(&mut deque, &mut stack),
                },
                '<' => match fetch_next!(deque, stack, out) {
                    '-' => {
                        // Be greedy, could be last character
                        replace(&mut stack, Symbol::ShortLeftArrow);

                        match fetch_next!(deque, stack, out) {
                            '-' => replace(&mut stack, Symbol::LongLeftArrow),
                            '>' => replace(&mut stack, Symbol::LeftRightArrow),
                            _ => undo_overfetching(&mut deque, &mut stack),
                        }
                    }
                    '=' => replace(&mut stack, Symbol::LessThanOrEqual),
                    _ => undo_overfetching(&mut deque, &mut stack),
                },
                '>' => match fetch_next!(deque, stack, out) {
                    '=' => replace(&mut stack, Symbol::GreaterThanOrEqual),
                    _ => undo_overfetching(&mut deque, &mut stack),
                },
                '!' => match fetch_next!(deque, stack, out) {
                    '=' => replace(&mut stack, Symbol::NotEqual),
                    _ => undo_overfetching(&mut deque, &mut stack),
                },
                '=' => match fetch_next!(deque, stack, out) {
                    '>' => replace(&mut stack, Symbol::RightDoubleArrow),
                    _ => undo_overfetching(&mut deque, &mut stack),
                },
                // "Your scientists were so preoccupied with whether or not they could,
                // they didn't stop to think if they should." ... this falls into the
                // "shouldn't" category:
                'h' => match fetch_next!(deque, stack, out) {
                    't' => match fetch_next!(deque, stack, out) {
                        't' => match fetch_next!(deque, stack, out) {
                            'p' => match fetch_next!(deque, stack, out) {
                                // Sorry, `http` not supported. Neither is `ftp`,
                                // `file`, ...
                                's' => match fetch_next!(deque, stack, out) {
                                    ':' => match fetch_next!(deque, stack, out) {
                                        '/' => match fetch_next!(deque, stack, out) {
                                            '/' => loop {
                                                match fetch_next!(deque, stack, out, 'outer) {
                                                    ' ' | '"' => break,
                                                    _ => {
                                                        // building up stack, ignoring
                                                        // all characters other than
                                                        // non-URI ones
                                                    }
                                                }
                                            },
                                            _ => undo_overfetching(&mut deque, &mut stack),
                                        },
                                        _ => undo_overfetching(&mut deque, &mut stack),
                                    },
                                    _ => undo_overfetching(&mut deque, &mut stack),
                                },
                                _ => undo_overfetching(&mut deque, &mut stack),
                            },
                            _ => undo_overfetching(&mut deque, &mut stack),
                        },
                        _ => undo_overfetching(&mut deque, &mut stack),
                    },
                    _ => undo_overfetching(&mut deque, &mut stack),
                },
                _ => {}
            }

            out.push_str(&stack.into_iter().collect::<String>());
        }

        Ok(out.into())
    }
}

enum Symbol {
    // Typographic symbols
    EmDash,
    EnDash,
    // Arrows
    ShortRightArrow,
    ShortLeftArrow,
    LongRightArrow,
    LongLeftArrow,
    LeftRightArrow,
    RightDoubleArrow,
    // Math
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl From<Symbol> for char {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::EnDash => '–',
            Symbol::EmDash => '—',
            //
            Symbol::ShortRightArrow => '→',
            Symbol::ShortLeftArrow => '←',
            Symbol::LongRightArrow => '⟶',
            Symbol::LongLeftArrow => '⟵',
            Symbol::LeftRightArrow => '↔',
            Symbol::RightDoubleArrow => '⇒',
            //
            Symbol::NotEqual => '≠',
            Symbol::LessThanOrEqual => '≤',
            Symbol::GreaterThanOrEqual => '≥',
        }
    }
}

/// We might greedily overfetch and then end up with a [`char`] on the `stack` we do not
/// know how to handle. However, *subsequent, other states might*. Hence, be a good
/// citizen and put it back where it came from.
///
/// This allows matching sequences like `--!=` to be `–≠`, which might otherwise end up
/// as `–!=` (because the next iteration only sees `=`, `!` was already consumed).
fn undo_overfetching<T>(deque: &mut VecDeque<T>, stack: &mut Vec<T>) {
    deque.push_front(
        stack
            .pop()
            .expect("Pop should only happen after having just pushed, so stack shouldn't be empty"),
    );
}

/// Replace the entire `stack` with the given `symbol`.
fn replace(stack: &mut Vec<char>, symbol: Symbol) {
    stack.clear();
    stack.push(symbol.into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case(" ", " ")]
    // Typographic symbols
    #[case("--", "–")]
    #[case("---", "—")]
    // Arrows
    #[case("->", "→")]
    #[case("-->", "⟶")]
    #[case("<-", "←")]
    #[case("<--", "⟵")]
    #[case("<->", "↔")]
    #[case("=>", "⇒")]
    // Math
    #[case("<=", "≤")]
    #[case(">=", "≥")]
    #[case("!=", "≠")]
    fn test_symbol_substitution_base_cases(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("A-", "A-")]
    #[case("A--", "A–")]
    #[case("A---", "A—")]
    //
    #[case("-A", "-A")]
    #[case("--A", "–A")]
    #[case("---A", "—A")]
    //
    #[case("A->", "A→")]
    #[case("A-->", "A⟶")]
    #[case("A<->", "A↔")]
    #[case("A=>", "A⇒")]
    //
    #[case("<-A", "←A")]
    #[case("<--A", "⟵A")]
    #[case("<->A", "↔A")]
    #[case("=>A", "⇒A")]
    //
    #[case("A<=", "A≤")]
    #[case("A>=", "A≥")]
    #[case("A!=", "A≠")]
    fn test_symbol_substitution_neighboring_single_letter(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("A-B", "A-B")]
    #[case("A--B", "A–B")]
    #[case("A---B", "A—B")]
    //
    #[case("A->B", "A→B")]
    #[case("A-->B", "A⟶B")]
    #[case("A<->B", "A↔B")]
    #[case("A=>B", "A⇒B")]
    #[case("A<-B", "A←B")]
    #[case("A<--B", "A⟵B")]
    #[case("A<->B", "A↔B")]
    #[case("A=>B", "A⇒B")]
    //
    #[case("A<=B", "A≤B")]
    #[case("A>=B", "A≥B")]
    #[case("A!=B", "A≠B")]
    fn test_symbol_substitution_neighboring_letters(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("A - B", "A - B")]
    #[case("A -- B", "A – B")]
    #[case("A --- B", "A — B")]
    //
    #[case("A -> B", "A → B")]
    #[case("A --> B", "A ⟶ B")]
    #[case("A <-> B", "A ↔ B")]
    #[case("A => B", "A ⇒ B")]
    #[case("A <- B", "A ← B")]
    #[case("A <-- B", "A ⟵ B")]
    #[case("A <-> B", "A ↔ B")]
    #[case("A => B", "A ⇒ B")]
    //
    #[case("A <= B", "A ≤ B")]
    #[case("A >= B", "A ≥ B")]
    #[case("A != B", "A ≠ B")]
    fn test_symbol_substitution_neighboring_letters_with_spaces(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("-X-", "-X-")]
    #[case("--X--", "–X–")]
    #[case("---X---", "—X—")]
    //
    #[case("-X>", "-X>")]
    #[case("->X->", "→X→")]
    #[case("--X-->", "–X⟶")]
    #[case("---X-->", "—X⟶")]
    //
    #[case("<-X-", "←X-")]
    #[case("<--X--", "⟵X–")]
    //
    #[case("<--X-->", "⟵X⟶")]
    fn test_symbol_substitution_disrupting_symbols(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("I breathe -- I live.", "I breathe – I live.")]
    #[case("To think---to breathe.", "To think—to breathe.")]
    #[case("A joke --> A laugh.", "A joke ⟶ A laugh.")]
    #[case("A <= B => C", "A ≤ B ⇒ C")]
    #[case("->In->Out->", "→In→Out→")]
    fn test_symbol_substitution_sentences(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("----", "—-")]
    #[case("-----", "—–")]
    #[case("------", "——")]
    //
    #[case(">->", ">→")]
    #[case("->->", "→→")]
    #[case("->-->", "→⟶")]
    #[case("->--->", "→—>")]
    #[case("->--->->", "→—>→")]
    //
    #[case("<-<-", "←←")]
    #[case("<-<--", "←⟵")]
    #[case("<-<---", "←⟵-")]
    #[case("<-<---<", "←⟵-<")]
    //
    #[case("<->->", "↔→")]
    #[case("<-<->->", "←↔→")]
    //
    #[case("<=<=", "≤≤")]
    #[case("<=<=<=", "≤≤≤")]
    #[case(">=>=", "≥≥")]
    #[case(">=>=>=", "≥≥≥")]
    //
    #[case(">=<=", "≥≤")]
    #[case(">=<=<=", "≥≤≤")]
    //
    #[case("!=!=", "≠≠")]
    #[case("!=!=!=", "≠≠≠")]
    fn test_symbol_substitution_ambiguous_sequences(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("–", "–")]
    #[case("—", "—")]
    #[case("→", "→")]
    #[case("←", "←")]
    #[case("⟶", "⟶")]
    #[case("⟵", "⟵")]
    #[case("↔", "↔")]
    #[case("⇒", "⇒")]
    #[case("≠", "≠")]
    #[case("≤", "≤")]
    #[case("≥", "≥")]
    fn test_symbol_substitution_existing_symbol(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://www.example.com", "https://www.example.com")]
    #[case("https://www.example.com/", "https://www.example.com/")]
    #[case("https://www.example.com/->", "https://www.example.com/->")]
    //
    #[case("\"https://www.example.com/\"->", "\"https://www.example.com/\"→")]
    #[case("https://www.example.com/ ->", "https://www.example.com/ →")]
    //
    #[case("h->", "h→")]
    #[case("ht->", "ht→")]
    #[case("htt->", "htt→")]
    #[case("http->", "http→")]
    #[case("https->", "https→")]
    #[case("https:->", "https:→")]
    #[case("https:/->", "https:/→")]
    #[case("https://->", "https://->")] // Pivot point
    fn test_symbol_substitution_uri(#[case] input: &str, #[case] expected: &str) {
        let stage = SymbolsStage;
        let result: String = stage.substitute(input).unwrap().into();

        assert_eq!(result, expected);
    }
}
