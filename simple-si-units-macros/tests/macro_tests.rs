use macrotest;
use simple_si_units_core::NumLike;
// expand macros with this command:
// rm .\tests\expand\*.expanded.rs; cargo +nightly test --test macro_tests


#[test]
pub fn expand_macros() {
	macrotest::expand("tests/expand/*.rs")
}

///// expanded output pasted here for compiler test