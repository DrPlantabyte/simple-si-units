
//! This module provides chemical SI units, such as catalytic activity 
//! and chemical concentration.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::geometry::*;
use super::mechanical::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};




/// The catalytic activity unit type, defined as moles per second in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct CatalyticActivity<T: NumLike>{
	/// The value of this Catalytic activity in moles per second
	pub molps: T
}

impl<T> CatalyticActivity<T> where T: NumLike {

	/// Returns the standard unit name of catalytic activity: "moles per second"
	pub fn unit_name() -> &'static str { "moles per second" }
	
	/// Returns the abbreviated name or symbol of catalytic activity: "mol/s" for moles per second
	pub fn unit_symbol() -> &'static str { "mol/s" }
	
	/// Returns a new catalytic activity value from the given number of moles per second
	///
	/// # Arguments
	/// * `molps` - Any number-like type, representing a quantity of moles per second
	pub fn from_molps(molps: T) -> Self { CatalyticActivity{molps: molps} }
	
	/// Returns a copy of this catalytic activity value in moles per second
	pub fn to_molps(&self) -> T { self.molps.clone() }

	/// Returns a new catalytic activity value from the given number of moles per second
	///
	/// # Arguments
	/// * `moles_per_second` - Any number-like type, representing a quantity of moles per second
	pub fn from_moles_per_second(moles_per_second: T) -> Self { CatalyticActivity{molps: moles_per_second} }
	
	/// Returns a copy of this catalytic activity value in moles per second
	pub fn to_moles_per_second(&self) -> T { self.molps.clone() }

}

impl<T> fmt::Display for CatalyticActivity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.molps, Self::unit_symbol())
	}
}

impl<T> CatalyticActivity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this catalytic activity value in count per second
	pub fn to_Nps(&self) -> T {
		return self.molps.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new catalytic activity value from the given number of count per second
	///
	/// # Arguments
	/// * `Nps` - Any number-like type, representing a quantity of count per second
	pub fn from_Nps(Nps: T) -> Self {
		CatalyticActivity{molps: Nps * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this catalytic activity value in millimoles per second
	pub fn to_mmolps(&self) -> T {
		return self.molps.clone() * T::from(1000.0_f64);
	}

	/// Returns a new catalytic activity value from the given number of millimoles per second
	///
	/// # Arguments
	/// * `mmolps` - Any number-like type, representing a quantity of millimoles per second
	pub fn from_mmolps(mmolps: T) -> Self {
		CatalyticActivity{molps: mmolps * T::from(0.001_f64)}
	}

	/// Returns a copy of this catalytic activity value in micromoles per second
	pub fn to_umolps(&self) -> T {
		return self.molps.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new catalytic activity value from the given number of micromoles per second
	///
	/// # Arguments
	/// * `umolps` - Any number-like type, representing a quantity of micromoles per second
	pub fn from_umolps(umolps: T) -> Self {
		CatalyticActivity{molps: umolps * T::from(1e-06_f64)}
	}

	/// Returns a copy of this catalytic activity value in nanomoles per second
	pub fn to_nmolps(&self) -> T {
		return self.molps.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new catalytic activity value from the given number of nanomoles per second
	///
	/// # Arguments
	/// * `nmolps` - Any number-like type, representing a quantity of nanomoles per second
	pub fn from_nmolps(nmolps: T) -> Self {
		CatalyticActivity{molps: nmolps * T::from(1e-09_f64)}
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
	pub fn unit_name() -> &'static str { "moles per cubic meter" }
	
	/// Returns the abbreviated name or symbol of chemical concentration: "mol/m³" for moles per cubic meter
	pub fn unit_symbol() -> &'static str { "mol/m³" }
	
	/// Returns a new chemical concentration value from the given number of moles per cubic meter
	///
	/// # Arguments
	/// * `molpm3` - Any number-like type, representing a quantity of moles per cubic meter
	pub fn from_molpm3(molpm3: T) -> Self { Concentration{molpm3: molpm3} }
	
	/// Returns a copy of this chemical concentration value in moles per cubic meter
	pub fn to_molpm3(&self) -> T { self.molpm3.clone() }

	/// Returns a new chemical concentration value from the given number of moles per cubic meter
	///
	/// # Arguments
	/// * `moles_per_cubic_meter` - Any number-like type, representing a quantity of moles per cubic meter
	pub fn from_moles_per_cubic_meter(moles_per_cubic_meter: T) -> Self { Concentration{molpm3: moles_per_cubic_meter} }
	
	/// Returns a copy of this chemical concentration value in moles per cubic meter
	pub fn to_moles_per_cubic_meter(&self) -> T { self.molpm3.clone() }

	/// Returns a new chemical concentration value from the given number of millimolar
	///
	/// # Arguments
	/// * `mM` - Any number-like type, representing a quantity of moles per cubic meter
	pub fn from_mM(mM: T) -> Self { Concentration{molpm3: mM} }
	
	/// Returns a copy of this chemical concentration value in millimolar
	pub fn to_mM(&self) -> T { self.molpm3.clone() }

}

impl<T> fmt::Display for Concentration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.molpm3, Self::unit_symbol())
	}
}

impl<T> Concentration<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this chemical concentration value in count per cubic meter
	pub fn to_Npm3(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic meter
	///
	/// # Arguments
	/// * `Npm3` - Any number-like type, representing a quantity of count per cubic meter
	pub fn from_Npm3(Npm3: T) -> Self {
		Concentration{molpm3: Npm3 * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per cubic meter
	pub fn to_count_per_cubic_meter(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic meter
	///
	/// # Arguments
	/// * `count_per_cubic_meter` - Any number-like type, representing a quantity of count per cubic meter
	pub fn from_count_per_cubic_meter(count_per_cubic_meter: T) -> Self {
		Concentration{molpm3: count_per_cubic_meter * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per liter
	pub fn to_NpL(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+26_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per liter
	///
	/// # Arguments
	/// * `NpL` - Any number-like type, representing a quantity of count per liter
	pub fn from_NpL(NpL: T) -> Self {
		Concentration{molpm3: NpL * T::from(1.6605390671738499e-21_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per liter
	pub fn to_count_per_L(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+26_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per liter
	///
	/// # Arguments
	/// * `count_per_L` - Any number-like type, representing a quantity of count per liter
	pub fn from_count_per_L(count_per_L: T) -> Self {
		Concentration{molpm3: count_per_L * T::from(1.6605390671738499e-21_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per cubic centimeter
	pub fn to_Npcc(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+29_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic centimeter
	///
	/// # Arguments
	/// * `Npcc` - Any number-like type, representing a quantity of count per cubic centimeter
	pub fn from_Npcc(Npcc: T) -> Self {
		Concentration{molpm3: Npcc * T::from(1.6605390671738501e-18_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per cubic centimeter
	pub fn to_count_per_cc(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+29_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic centimeter
	///
	/// # Arguments
	/// * `count_per_cc` - Any number-like type, representing a quantity of count per cubic centimeter
	pub fn from_count_per_cc(count_per_cc: T) -> Self {
		Concentration{molpm3: count_per_cc * T::from(1.6605390671738501e-18_f64)}
	}

	/// Returns a copy of this chemical concentration value in moles per L
	pub fn to_M(&self) -> T {
		return self.molpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new chemical concentration value from the given number of moles per L
	///
	/// # Arguments
	/// * `M` - Any number-like type, representing a quantity of moles per L
	pub fn from_M(M: T) -> Self {
		Concentration{molpm3: M * T::from(1000.0_f64)}
	}

	/// Returns a copy of this chemical concentration value in moles per liter
	pub fn to_molarity(&self) -> T {
		return self.molpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new chemical concentration value from the given number of moles per liter
	///
	/// # Arguments
	/// * `molarity` - Any number-like type, representing a quantity of moles per liter
	pub fn from_molarity(molarity: T) -> Self {
		Concentration{molpm3: molarity * T::from(1000.0_f64)}
	}

	/// Returns a copy of this chemical concentration value in micromolar
	pub fn to_uM(&self) -> T {
		return self.molpm3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new chemical concentration value from the given number of micromolar
	///
	/// # Arguments
	/// * `uM` - Any number-like type, representing a quantity of micromolar
	pub fn from_uM(uM: T) -> Self {
		Concentration{molpm3: uM * T::from(0.001_f64)}
	}

	/// Returns a copy of this chemical concentration value in nanomolar
	pub fn to_nM(&self) -> T {
		return self.molpm3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new chemical concentration value from the given number of nanomolar
	///
	/// # Arguments
	/// * `nM` - Any number-like type, representing a quantity of nanomolar
	pub fn from_nM(nM: T) -> Self {
		Concentration{molpm3: nM * T::from(1e-06_f64)}
	}

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



