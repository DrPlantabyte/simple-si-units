use simple_si_units_macros::UnitStruct;
use simple_si_units_core::NumLike;


#[derive(UnitStruct)]
pub struct MyUnit<DT: NumLike>
{
	v: DT
}
fn do_math<DT: NumLike>(a: MyUnit<DT>, b: MyUnit<DT>) -> DT{
	let x = (a - b) / (a.clone() + b.clone());
	return x * x;
}

#[test]
fn proc_macro_test() {
	println!("Macro test math output: {}", do_math(MyUnit{v: 1.5f64}, MyUnit{v: 0.3f64}));
}
