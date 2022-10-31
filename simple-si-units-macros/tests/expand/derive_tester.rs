use simple_si_units_macros::Unit;
use simple_si_units_core::UnitData;


#[derive(Unit)]
pub struct MyUnit<DT: UnitData>
{
	v: DT
}

