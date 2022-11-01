//! # Simple SI Units
//! Work in progress...

pub use simple_si_units_macros::Unit;
pub use simple_si_units_core::UnitData;




/// Placeholder: Work in progress
trait Length {
	type NumType;

	/// Placeholder: Work in progress
	fn meters(self) -> Self::NumType; 
}

/// Placeholder: Work in progress
trait Time {
	type NumType;

	/// Placeholder: Work in progress	
	fn seconds(self) -> Self::NumType; 
}

/// Placeholder: Work in progress
trait Velocity {
	type NumType;
	
	/// Placeholder: Work in progress
	fn meters_per_second(self) -> Self::NumType; 
}


/// Placeholder: Work in progress
#[derive(Debug, Copy, Clone, PartialEq)]
struct GLength<MT> {
	m: MT
}

impl<MT> Length for GLength<MT> {
	type NumType = MT;
	fn meters(self) -> Self::NumType{
		self.m
	}
}

/// Placeholder: Work in progress
#[derive(Debug, Copy, Clone, PartialEq)]
struct GTime<MT> {
	s: MT
}

impl<MT> Time for GTime<MT> {
	type NumType = MT;
	fn seconds(self) -> Self::NumType{
		self.s
	}
}

/// Placeholder: Work in progress
#[derive(Debug, Copy, Clone, PartialEq)]
struct GVelocity<MT> {
	mps: MT
}

impl<MT> Velocity for GVelocity<MT> {
	type NumType = MT;
	fn meters_per_second(self) -> Self::NumType{
		self.mps
	}
}


impl<DT, TT> std::ops::Div<TT> for GLength<DT>
where TT: Time<NumType=DT>, DT: std::ops::Div<DT> + std::ops::Div<Output = DT>
{
	type Output = GVelocity<DT>;
	fn div(self, rhs: TT) -> Self::Output {
		let y: GVelocity<DT> = GVelocity{mps: self.meters() / rhs.seconds()};
		return y;
	}
}


/// Unit tests
#[cfg(test)]
mod tests {
	use super::*;

	/// Placeholder: Work in progress
	#[test]
	fn velocity_api() {
		let d = GLength{m: 5.0f64};
		let t = GTime{s: 2.0f64};
		let v = d / t;
		assert_eq!(v, GVelocity{mps: 2.5f64});
	}
}
