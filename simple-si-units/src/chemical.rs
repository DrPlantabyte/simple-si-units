
//! This module provides chemical SI units, such as catalytic activity 
//! and chemical concentration.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::geometry::*;
use super::mechanical::*;
use super::base::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;


/// The catalytic activity unit type, defined as moles per second in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct CatalyticActivity<T: NumLike>{
	/// The value of this Catalytic activity in moles per second
	pub molps: T
}

impl<T> CatalyticActivity<T> where T: NumLike {

	/// Returns the standard unit name of catalytic activity: "moles per second"
	pub fn unit_name() -> &'static str {
		return "moles per second";
	}
	
	/// Returns the abbreviated name or symbol of catalytic activity: "molps" for moles per second
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
	
	/// Returns a copy of this catalytic activity value in moles per second
	pub fn to_molps(self) -> T {
		return self.molps.clone();
	}
}

impl<T> fmt::Display for CatalyticActivity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.molps, Self::unit_symbol())
	}
}

impl<T> CatalyticActivity<T> where T: NumLike+From<f64> {
	
}

// CatalyticActivity * Time -> Amount
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> std::ops::Mul<Time<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Amount{mol: self.molps * rhs.s}
	}
}
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> std::ops::Mul<Time<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Amount{mol: self.molps.clone() * rhs.s}
	}
}
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> std::ops::Mul<&Time<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Amount{mol: self.molps * rhs.s.clone()}
	}
}
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> std::ops::Mul<&Time<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Amount{mol: self.molps.clone() * rhs.s.clone()}
	}
}

// CatalyticActivity / Amount -> Frequency
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> std::ops::Div<Amount<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps / rhs.mol}
	}
}
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> std::ops::Div<Amount<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps.clone() / rhs.mol}
	}
}
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> std::ops::Div<&Amount<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps / rhs.mol.clone()}
	}
}
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> std::ops::Div<&Amount<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps.clone() / rhs.mol.clone()}
	}
}

// CatalyticActivity / Frequency -> Amount
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> std::ops::Div<Frequency<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Amount{mol: self.molps / rhs.Hz}
	}
}
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> std::ops::Div<Frequency<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Amount{mol: self.molps.clone() / rhs.Hz}
	}
}
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> std::ops::Div<&Frequency<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Amount{mol: self.molps / rhs.Hz.clone()}
	}
}
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> std::ops::Div<&Frequency<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Amount{mol: self.molps.clone() / rhs.Hz.clone()}
	}
}

/// The chemical concentration unit type, defined as moles per cubic meter in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Concentration<T: NumLike>{
	/// The value of this Chemical concentration in moles per cubic meter
	pub molpm3: T
}

impl<T> Concentration<T> where T: NumLike {

	/// Returns the standard unit name of chemical concentration: "moles per cubic meter"
	pub fn unit_name() -> &'static str {
		return "moles per cubic meter";
	}
	
	/// Returns the abbreviated name or symbol of chemical concentration: "molpm3" for moles per cubic meter
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
	
	/// Returns a copy of this chemical concentration value in moles per cubic meter
	pub fn to_molpm3(self) -> T {
		return self.molpm3.clone();
	}
}

impl<T> fmt::Display for Concentration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.molpm3, Self::unit_symbol())
	}
}

impl<T> Concentration<T> where T: NumLike+From<f64> {
	
}

// Concentration * Volume -> Amount
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> std::ops::Mul<Volume<T>> for Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3 * rhs.m3}
	}
}
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> std::ops::Mul<Volume<T>> for &Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3.clone() * rhs.m3}
	}
}
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> std::ops::Mul<&Volume<T>> for Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3 * rhs.m3.clone()}
	}
}
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> std::ops::Mul<&Volume<T>> for &Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3.clone() * rhs.m3.clone()}
	}
}


