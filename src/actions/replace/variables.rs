use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use log::trace;

use crate::regex::CaptureGroup;

type Variables<'a> = HashMap<CaptureGroup, &'a str>;

/// In an input like `Hello $var World`, inject all variables.
///
/// Variables are treated as they occur in regular expressions: they can be [named or
/// numbered](https://docs.rs/regex/latest/regex/struct.Captures.html).
#[allow(clippy::too_many_lines)] // :(
pub(super) fn inject_variables(
    input: &str,
    variables: &Variables<'_>,
) -> Result<String, VariableExpressionError> {
    let mut state = State::default();
    let mut out = String::with_capacity(input.len());
    let mut to_remove = 0; // Remove this many pushed chars once a var is detected

    for c in input.chars() {
        trace!(
            "Injecting variables. Current output is: '{}', current state is {:?}",
            out.escape_debug(),
            state
        );
        out.push(c);

        state = match (state, c) {
            // Initial state
            (State::Noop, '$') => {
                to_remove = 1;
                State::Start
            }
            (State::Start, '$') => {
                // Ignore previous `$`, and only push one.
                assert_eq!(out.pop().expect("was pushed in earlier loop"), '$',);
                State::default()
            }
            (State::Noop, _) => State::default(),

            // Init
            (State::Start, '{') => {
                to_remove += 1;
                State::BracedStart
            }
            (State::Start, 'a'..='z' | 'A'..='Z' | '_') => State::BuildingNamedVar {
                name: String::from(c),
                braced: false,
            },
            (State::BracedStart, 'a'..='z' | 'A'..='Z' | '_') => State::BuildingNamedVar {
                name: String::from(c),
                braced: true,
            },
            (State::Start, '0'..='9') => State::BuildingNumberedVar {
                num: c.to_digit(10).expect("hard-coded digit is valid number") as usize,
                braced: false,
            },
            (State::BracedStart, '0'..='9') => State::BuildingNumberedVar {
                num: c.to_digit(10).expect("hard-coded digit is valid number") as usize,
                braced: true,
            },

            // Nothing useful matched, go back. This is order-dependent, see also
            // https://github.com/rust-lang/rust-clippy/issues/860
            #[allow(clippy::match_same_arms)]
            (State::Start | State::BracedStart, _) => State::Noop,

            // Building up
            (
                State::BuildingNamedVar { mut name, braced },
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9',
            ) => State::BuildingNamedVar {
                name: {
                    name.push(c);
                    name
                },
                braced,
            },
            (
                State::BuildingNumberedVar {
                    num: magnitude,
                    braced,
                },
                '0'..='9',
            ) => State::BuildingNumberedVar {
                num: {
                    magnitude * 10
                        + c.to_digit(10).expect("hard-coded digit is valid number") as usize
                },
                braced,
            },

            // Building stops
            (State::BuildingNamedVar { name, braced: true }, '}') => {
                to_remove += 1;

                State::FinishNamedVar(name)
            }
            (State::BuildingNumberedVar { num, braced: true }, '}') => {
                to_remove += 1;

                State::FinishNumberedVar(num)
            }
            (
                State::BuildingNamedVar {
                    name, braced: true, ..
                },
                _,
            ) => return Err(VariableExpressionError::MismatchedBraces(name)),
            (
                State::BuildingNumberedVar {
                    num, braced: true, ..
                },
                _,
            ) => return Err(VariableExpressionError::MismatchedBraces(num.to_string())),

            (State::FinishNamedVar(name) | State::BuildingNamedVar { name, .. }, _) => {
                trace!("Finishing up named variable '{name}'");
                match variables.get(&CaptureGroup::Named(name.clone())) {
                    Some(repl) => {
                        let tail = out
                            .pop()
                            .expect("chars are pushed unconditionally, one is present");
                        out.truncate(out.len() - (to_remove + name.len()));
                        out.push_str(repl);
                        out.push(tail);
                    }
                    None => return Err(VariableExpressionError::UndefinedVariable(name)),
                };

                match c {
                    '$' => {
                        to_remove = 1;
                        State::Start
                    }
                    _ => State::Noop,
                }
            }
            (State::FinishNumberedVar(num) | State::BuildingNumberedVar { num, .. }, _) => {
                trace!("Finishing up numbered variable '{num}'");
                match variables.get(&CaptureGroup::Numbered(num)) {
                    Some(repl) => {
                        let tail = out
                            .pop()
                            .expect("chars are pushed unconditionally, one is present");
                        out.truncate(out.len() - (to_remove + width(num)));
                        out.push_str(repl);
                        out.push(tail);
                    }
                    None => {
                        return Err(VariableExpressionError::UndefinedVariable(num.to_string()))
                    }
                };

                match c {
                    '$' => {
                        to_remove = 1;
                        State::Start
                    }
                    _ => State::Noop,
                }
            }
        }
    }

    trace!(
        "Finished character iteration, output is '{}', state is {:?}",
        out.escape_debug(),
        state
    );

    // Flush out any pending state
    let last = out.chars().last();
    state = match (&state, last) {
        (
            State::FinishNamedVar(name)
            | State::BuildingNamedVar {
                name,
                braced: false,
            },
            _,
        ) => {
            trace!("Finishing up named variable '{name}'");
            match variables.get(&CaptureGroup::Named(name.clone())) {
                Some(repl) => {
                    out.truncate(out.len() - (to_remove + name.len()));
                    out.push_str(repl);

                    state
                }
                None => return Err(VariableExpressionError::UndefinedVariable(name.clone())),
            }
        }
        (State::FinishNumberedVar(num) | State::BuildingNumberedVar { num, braced: false }, _) => {
            trace!("Finishing up numbered variable '{num}'");
            match variables.get(&CaptureGroup::Numbered(*num)) {
                Some(repl) => {
                    out.truncate(out.len() - (to_remove + width(*num)));
                    out.push_str(repl);

                    state
                }
                None => return Err(VariableExpressionError::UndefinedVariable(num.to_string())),
            }
        }
        (
            State::BuildingNamedVar {
                name, braced: true, ..
            },
            _,
        ) => return Err(VariableExpressionError::MismatchedBraces(name.clone())),
        (
            State::BuildingNumberedVar {
                num, braced: true, ..
            },
            _,
        ) => return Err(VariableExpressionError::MismatchedBraces(num.to_string())),
        (State::Noop | State::Start | State::BracedStart, _) => state,
    };

    trace!(
        "Done injecting variables, final output is '{}', final state is {:?}",
        out.escape_debug(),
        state
    );

    Ok(out)
}

/// Gets the width in characters of a number.
const fn width(num: usize) -> usize {
    if num == 0 {
        1
    } else {
        (num.ilog10() + 1) as usize
    }
}

/// State during injection of variables in an expression like `Hello $var World`.
#[derive(Debug, PartialEq, Eq, Default)]
enum State {
    #[default]
    /// Neutral state.
    Noop,
    /// The character denoting a variable declaration has been seen.
    Start,
    /// The detected, potential variable additionally starts with an opening brace.
    BracedStart,
    /// A named variable is detected and is being built up.
    BuildingNamedVar { name: String, braced: bool },
    /// A numbered variable is detected and is being built up.
    BuildingNumberedVar { num: usize, braced: bool },
    /// Processing of a named variable is done, finish it up.
    FinishNamedVar(String),
    /// Processing of a numbered variable is done, finish it up.
    FinishNumberedVar(usize),
}

/// An error in variable expressions.
#[derive(Debug, PartialEq, Eq)]
pub enum VariableExpressionError {
    /// A variable expression with mismatched number of braces.
    MismatchedBraces(String),
    /// A requested variable was not passed.
    UndefinedVariable(String),
}

impl fmt::Display for VariableExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedBraces(var) => {
                write!(f, "Mismatched braces for variable: '{var}'")
            }
            Self::UndefinedVariable(var) => {
                write!(f, "Undefined variable, unable to substitute: '{var}'")
            }
        }
    }
}

impl Error for VariableExpressionError {}

#[cfg(test)]
mod test {
    use rstest::*;

    use super::*;

    #[fixture]
    fn variables() -> Variables<'static> {
        Variables::from([
            (CaptureGroup::Named("var1".to_owned()), "val1"),
            (CaptureGroup::Named("VAR_2".to_owned()), "val2"),
            (CaptureGroup::Numbered(2), "nval"),
        ])
    }

    #[rstest]
    // Base cases without variables
    #[case("", Ok(""))]
    #[case("Regular content", Ok("Regular content"))]
    // Escaping works
    #[case("I have $$5", Ok("I have $5"))]
    //
    // Basic named variable
    #[case("$var1", Ok("val1"))]
    #[case("$var1 ", Ok("val1 "))]
    #[case(" $var1", Ok(" val1"))]
    #[case(" $var1 ", Ok(" val1 "))]
    //
    // Basic named variables
    #[case("$var1 $VAR_2", Ok("val1 val2"))]
    #[case("$var1$VAR_2", Ok("val1val2"))]
    #[case(" $var1 $VAR_2", Ok(" val1 val2"))]
    #[case("$var1 $VAR_2 ", Ok("val1 val2 "))]
    #[case(" $var1 $VAR_2 ", Ok(" val1 val2 "))]
    //
    // Basic numbered variables
    #[case("$2", Ok("nval"))]
    #[case("$2 ", Ok("nval "))]
    #[case(" $2", Ok(" nval"))]
    #[case(" $2 ", Ok(" nval "))]
    //
    // Mixed content
    #[case("Hello $2 World $var1", Ok("Hello nval World val1"))]
    //
    // Braces for separation
    #[case("${var1}", Ok("val1"))]
    #[case("X${var1}X", Ok("Xval1X"))]
    #[case("${2}", Ok("nval"))]
    #[case("3${2}3", Ok("3nval3"))]
    #[case("Hello${2}2U Sir${var1}Mister", Ok("Hellonval2U Sirval1Mister"))]
    //
    // Variable multiple times
    #[case("$var1$var1", Ok("val1val1"))]
    #[case("${var1}${var1}", Ok("val1val1"))]
    #[case("${var1}$var1", Ok("val1val1"))]
    #[case("${2}$2", Ok("nvalnval"))]
    #[case("${var1}$var1 ${2}$2", Ok("val1val1 nvalnval"))]
    //
    // Undefined variables
    #[case("$NO", Err(VariableExpressionError::UndefinedVariable("NO".to_owned())))]
    #[case("$NO such thing", Err(VariableExpressionError::UndefinedVariable("NO".to_owned())))]
    #[case("$NO$ON", Err(VariableExpressionError::UndefinedVariable("NO".to_owned())))]
    // Numbers will be stringified
    #[case("$1337", Err(VariableExpressionError::UndefinedVariable("1337".to_owned())))]
    #[case("$1337 is missing", Err(VariableExpressionError::UndefinedVariable("1337".to_owned())))]
    #[case("$1337$7331", Err(VariableExpressionError::UndefinedVariable("1337".to_owned())))]
    //
    // Improperly closed braces
    #[case("${var1", Err(VariableExpressionError::MismatchedBraces("var1".to_owned())))]
    #[case("${var1 woops", Err(VariableExpressionError::MismatchedBraces("var1".to_owned())))]
    // Excess trailing ones are fine tho
    #[case("${var1}}", Ok("val1}"))]
    //
    // Remaining edge cases
    // Aborting a (brace) start
    #[case("$?", Ok("$?"))]
    #[case("${?", Ok("${?"))]
    fn test_inject_variables(
        #[case] expression: &str,
        #[case] expected: Result<&str, VariableExpressionError>,
        variables: Variables<'_>,
    ) {
        let result = inject_variables(expression, &variables);
        let expected = expected.map(str::to_owned);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(9, 1)]
    #[case(10, 2)]
    #[case(99, 2)]
    #[case(100, 3)]
    fn test_width(#[case] num: usize, #[case] expected: usize) {
        assert_eq!(width(num), expected);
    }
}
