
//! This module provides nuclear SI units, such as radioactivity 
//! and radiation dose equivalent.
use std::fmt;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;


/// The radioactivity unit type, defined as becquerel in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Radioactivity<T: NumLike>{
	pub Bq: T
}

impl<T> Radioactivity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "becquerel";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "Bq";
	}

	/// Returns a new radioactivity value from the given number of becquerel
	///
	/// # Arguments
	/// * `Bq` - Any number-like type, representing a quantity of becquerel
	pub fn from_Bq(Bq: T) -> Self {
		Radioactivity{Bq}
	}
	
	/// Returns a copy of this radioactivity value in becquerel
	pub fn to_Bq(self) -> T {
		return self.Bq.clone();
	}
}

impl<T> fmt::Display for Radioactivity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Bq, Self::unit_symbol())
	}
}

impl<T> Radioactivity<T> where T: NumLike+From<f64> {
	
}

/// The absorbed radiation dose unit type, defined as gray in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct AbsorbedDose<T: NumLike>{
	pub Gy: T
}

impl<T> AbsorbedDose<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "gray";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "Gy";
	}

	/// Returns a new absorbed dose value from the given number of gray
	///
	/// # Arguments
	/// * `Gy` - Any number-like type, representing a quantity of gray
	pub fn from_Gy(Gy: T) -> Self {
		AbsorbedDose{Gy}
	}
	
	/// Returns a copy of this absorbed dose value in gray
	pub fn to_Gy(self) -> T {
		return self.Gy.clone();
	}
}

impl<T> fmt::Display for AbsorbedDose<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Gy, Self::unit_symbol())
	}
}

impl<T> AbsorbedDose<T> where T: NumLike+From<f64> {
	
	// AbsorbedDose * Mass -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Mass<T>> for AbsorbedDose<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Mass<T>) -> Self::Output {
			Energy{J: self.Gy * rhs.kg}
		}
	}

}

/// The radiation dose equivalent unit type, defined as sievert in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct DoseEquivalent<T: NumLike>{
	pub Sv: T
}

impl<T> DoseEquivalent<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "sievert";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "Sv";
	}

	/// Returns a new dose equivalent value from the given number of sievert
	///
	/// # Arguments
	/// * `Sv` - Any number-like type, representing a quantity of sievert
	pub fn from_Sv(Sv: T) -> Self {
		DoseEquivalent{Sv}
	}
	
	/// Returns a copy of this dose equivalent value in sievert
	pub fn to_Sv(self) -> T {
		return self.Sv.clone();
	}
}

impl<T> fmt::Display for DoseEquivalent<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Sv, Self::unit_symbol())
	}
}

impl<T> DoseEquivalent<T> where T: NumLike+From<f64> {
	
	// DoseEquivalent * Mass -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Mass<T>> for DoseEquivalent<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Mass<T>) -> Self::Output {
			Energy{J: self.Sv * rhs.kg}
		}
	}

}


