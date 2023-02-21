
//! This module provides nuclear SI units, such as radioactivity 
//! and radiation dose equivalent.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::mechanical::*;
use super::base::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;


/// The radioactivity unit type, defined as becquerels in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Radioactivity<T: NumLike>{
	/// The value of this Radioactivity in becquerels
	pub Bq: T
}

impl<T> Radioactivity<T> where T: NumLike {

	/// Returns the standard unit name of radioactivity: "becquerels"
	pub fn unit_name() -> &'static str {
		return "becquerels";
	}
	
	/// Returns the abbreviated name or symbol of radioactivity: "Bq" for becquerels
	pub fn unit_symbol() -> &'static str {
		return "Bq";
	}

	/// Returns a new radioactivity value from the given number of becquerels
	///
	/// # Arguments
	/// * `Bq` - Any number-like type, representing a quantity of becquerels
	pub fn from_Bq(Bq: T) -> Self {
		Radioactivity{Bq}
	}
	
	/// Returns a copy of this radioactivity value in becquerels
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

/// The absorbed radiation dose unit type, defined as grays in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AbsorbedDose<T: NumLike>{
	/// The value of this Absorbed dose in grays
	pub Gy: T
}

impl<T> AbsorbedDose<T> where T: NumLike {

	/// Returns the standard unit name of absorbed dose: "grays"
	pub fn unit_name() -> &'static str {
		return "grays";
	}
	
	/// Returns the abbreviated name or symbol of absorbed dose: "Gy" for grays
	pub fn unit_symbol() -> &'static str {
		return "Gy";
	}

	/// Returns a new absorbed dose value from the given number of grays
	///
	/// # Arguments
	/// * `Gy` - Any number-like type, representing a quantity of grays
	pub fn from_Gy(Gy: T) -> Self {
		AbsorbedDose{Gy}
	}
	
	/// Returns a copy of this absorbed dose value in grays
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
	
}

// AbsorbedDose * Mass -> Energy
/// Multiplying a AbsorbedDose by a Mass returns a value of type Energy
impl<T> std::ops::Mul<Mass<T>> for AbsorbedDose<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Energy{J: self.Gy * rhs.kg}
	}
}
/// Multiplying a AbsorbedDose by a Mass returns a value of type Energy
impl<T> std::ops::Mul<Mass<T>> for &AbsorbedDose<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Energy{J: self.Gy.clone() * rhs.kg}
	}
}
/// Multiplying a AbsorbedDose by a Mass returns a value of type Energy
impl<T> std::ops::Mul<&Mass<T>> for AbsorbedDose<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Energy{J: self.Gy * rhs.kg.clone()}
	}
}
/// Multiplying a AbsorbedDose by a Mass returns a value of type Energy
impl<T> std::ops::Mul<&Mass<T>> for &AbsorbedDose<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Energy{J: self.Gy.clone() * rhs.kg.clone()}
	}
}

/// The radiation dose equivalent unit type, defined as sieverts in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct DoseEquivalent<T: NumLike>{
	/// The value of this Dose equivalent in sieverts
	pub Sv: T
}

impl<T> DoseEquivalent<T> where T: NumLike {

	/// Returns the standard unit name of dose equivalent: "sieverts"
	pub fn unit_name() -> &'static str {
		return "sieverts";
	}
	
	/// Returns the abbreviated name or symbol of dose equivalent: "Sv" for sieverts
	pub fn unit_symbol() -> &'static str {
		return "Sv";
	}

	/// Returns a new dose equivalent value from the given number of sieverts
	///
	/// # Arguments
	/// * `Sv` - Any number-like type, representing a quantity of sieverts
	pub fn from_Sv(Sv: T) -> Self {
		DoseEquivalent{Sv}
	}
	
	/// Returns a copy of this dose equivalent value in sieverts
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
	
}

// DoseEquivalent * Mass -> Energy
/// Multiplying a DoseEquivalent by a Mass returns a value of type Energy
impl<T> std::ops::Mul<Mass<T>> for DoseEquivalent<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Energy{J: self.Sv * rhs.kg}
	}
}
/// Multiplying a DoseEquivalent by a Mass returns a value of type Energy
impl<T> std::ops::Mul<Mass<T>> for &DoseEquivalent<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Energy{J: self.Sv.clone() * rhs.kg}
	}
}
/// Multiplying a DoseEquivalent by a Mass returns a value of type Energy
impl<T> std::ops::Mul<&Mass<T>> for DoseEquivalent<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Energy{J: self.Sv * rhs.kg.clone()}
	}
}
/// Multiplying a DoseEquivalent by a Mass returns a value of type Energy
impl<T> std::ops::Mul<&Mass<T>> for &DoseEquivalent<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Energy{J: self.Sv.clone() * rhs.kg.clone()}
	}
}


