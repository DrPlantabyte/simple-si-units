
//! This module provides chemical SI units, such as catalytic activity 
//! and chemical concentration.
use std::fmt;


/// The catalytic activity unit type, defined as moles per second in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct CatalyticActivity<T: NumLike>{
	pub molps: T
}

impl<T> CatalyticActivity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "moles per second";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "molps";
	}

	/// Returns a new catalytic activity value from the given number of moles per second
	///
	/// # Arguments
	/// * `molps` - Any number-like type, representing a quantity of moles per second
	pub fn from_molps(molps: T) -> Self {
		CatalyticActivity{molps}
	}
	
	/// Returns this catalytic activity value in moles per second
	pub fn to_molps(self) -> T {
		return self.molps;
	}
}

impl<T> fmt::Display for CatalyticActivity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.molps, Self::unit_symbol())
	}
}

impl<T> CatalyticActivity<T> where T: NumLike+From<f64> {
	
	// TODO: CatalyticActivity * Time -> Amount

	// TODO: CatalyticActivity / Amount -> Frequency

	// TODO: CatalyticActivity / Frequency -> Amount

}

/// The chemical concentration unit type, defined as moles per cubic meter in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Concentration<T: NumLike>{
	pub molpm3: T
}

impl<T> Concentration<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "moles per cubic meter";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "molpm3";
	}

	/// Returns a new chemical concentration value from the given number of moles per cubic meter
	///
	/// # Arguments
	/// * `molpm3` - Any number-like type, representing a quantity of moles per cubic meter
	pub fn from_molpm3(molpm3: T) -> Self {
		Concentration{molpm3}
	}
	
	/// Returns this chemical concentration value in moles per cubic meter
	pub fn to_molpm3(self) -> T {
		return self.molpm3;
	}
}

impl<T> fmt::Display for Concentration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.molpm3, Self::unit_symbol())
	}
}

impl<T> Concentration<T> where T: NumLike+From<f64> {
	
	// TODO: Concentration * Volume -> Amount

}


