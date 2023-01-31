//! # Simple SI Units
//! Work in progress...

pub use simple_si_units_macros::UnitStruct;
pub use simple_si_units_core::NumLike;



// TODO: implement display for to-string representation (and have pretty version with size-aware
// unit suffixes)
/// Placeholder: Work in progress
#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Distance<T: NumLike>{
	pub m: T
}
impl<T> Distance<T> where T: NumLike {
	pub fn from_meters(m: T) -> Self{
		Distance{m}
	}
	pub fn to_meters(self) -> T{
		return self.m;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	pub fn from_au(au: T) -> Self{
		let m_per_au = T::from(1.495979e11f64);
		Distance{m: m_per_au * au}
	}
	pub fn to_au(self) -> T{
		let au_per_m = T::from(6.684585e-12f64);
		return au_per_m * self.m;
	}
}

/// Placeholder: Work in progress
#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Volume<T: NumLike>{
	pub m3: T
}
impl<T> Volume<T> where T: NumLike {
	pub fn from_cubic_meters(m3: T) -> Self{
		Volume{m3: m3}
	}
	pub fn to_cubic_meters(self) -> T{
		return self.m3;
	}
}

/// Unit tests
#[cfg(test)]
mod unit_tests {
	use super::*;

	/// Placeholder: Work in progress
	#[test]
	fn distance_add_subtract() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn distance_mul_div() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn distance_op_assign() {
		// +=, -=, *=, /=
		todo!();
	}
}
