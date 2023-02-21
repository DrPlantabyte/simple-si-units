use macrotest;
use simple_si_units_core::NumLike;
// expand macros with this command:
// rm .\tests\expand\*.expanded.rs; cargo +nightly test --test macro_tests


#[test]
pub fn expand_macros() {
	macrotest::expand("tests/expand/*.rs")
}

#[test]
fn proc_macro_test() {
	println!("Macro test math output: {}", do_math(MyUnit{v: 1.5f64}, MyUnit{v: 0.3f64}));
}


///// expanded output pasted here for compiler test