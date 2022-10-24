//! # Simple SI Units
//! Work in progress...

use core::ops::Div;

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


impl<NT, TT> Div<TT> for GLength<NT>
where TT: Time<NumType=NT>, NT: Div<NT> + std::ops::Div<Output = NT>
{
	type Output = GVelocity<NT>;
	fn div(self, rhs: TT) -> Self::Output {
		let y: GVelocity<NT> = GVelocity{mps: self.meters() / rhs.seconds()};
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
