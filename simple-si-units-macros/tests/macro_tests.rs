use simple_si_units_core::NumLike;
// expand macros with this command:
// rm .\tests\expand\*.expanded.rs; cargo +nightly test --test macro_tests

#[cfg(nightly)]
#[test]
pub fn expand_macros() {
	use macrotest;
	macrotest::expand("tests/expand/*.rs")
}

struct NumLikeTestStruct <DT: NumLike>
{
	x: DT
}
#[test]
fn numlike_test(){
	let a = NumLikeTestStruct{x: 1.5};
	let b = NumLikeTestStruct{x: 2.0};
	let _ = a.x * b.x;
}

/* // comment this line for testing, uncomment for publish
#[test]
fn proc_macro_test() {
	println!("Macro test math output: {}", do_math(MyUnit{v: 1.5f64}, MyUnit{v: 0.3f64}));
}
*/

///// expanded output pasted here for compiler test