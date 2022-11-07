use simple_si_units_macros::UnitStruct;
use simple_si_units_core::NumLike;


#[derive(UnitStruct)]
pub struct MyUnit<DT: NumLike>
{
	v: DT
}

