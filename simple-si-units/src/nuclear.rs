
//! This module provides nuclear SI units, such as radioactivity 
//! and radiation dose equivalent.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::mechanical::*;

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
		Radioactivity{Bq: Bq}
	}
	
	/// Returns a copy of this radioactivity value in becquerels
	pub fn to_Bq(self) -> T {
		return self.Bq.clone();
	}

	/// Returns a new radioactivity value from the given number of becquerels
	///
	/// # Arguments
	/// * `becquerels` - Any number-like type, representing a quantity of becquerels
	pub fn from_becquerels(becquerels: T) -> Self {
		Radioactivity{Bq: becquerels}
	}
	
	/// Returns a copy of this radioactivity value in becquerels
	pub fn to_becquerels(self) -> T {
		return self.Bq.clone();
	}

}

impl<T> fmt::Display for Radioactivity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Bq, Self::unit_symbol())
	}
}

impl<T> Radioactivity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this radioactivity value in becquerels
	pub fn to_Bq(self) -> T {
		return self.Bq.clone() * T::from(1.0_f64);
	}

	/// Returns a new radioactivity value from the given number of becquerels
	///
	/// # Arguments
	/// * `Bq` - Any number-like type, representing a quantity of becquerels
	pub fn from_Bq(Bq: T) -> Self {
		Radioactivity{Bq: Bq * T::from(1.0_f64)}
	}

	/// Returns a copy of this radioactivity value in becquerels
	pub fn to_becquerels(self) -> T {
		return self.Bq.clone() * T::from(1.0_f64);
	}

	/// Returns a new radioactivity value from the given number of becquerels
	///
	/// # Arguments
	/// * `becquerels` - Any number-like type, representing a quantity of becquerels
	pub fn from_becquerels(becquerels: T) -> Self {
		Radioactivity{Bq: becquerels * T::from(1.0_f64)}
	}

	/// Returns a copy of this radioactivity value in millibecquerels
	pub fn to_mBq(self) -> T {
		return self.Bq.clone() * T::from(1000.0_f64);
	}

	/// Returns a new radioactivity value from the given number of millibecquerels
	///
	/// # Arguments
	/// * `mBq` - Any number-like type, representing a quantity of millibecquerels
	pub fn from_mBq(mBq: T) -> Self {
		Radioactivity{Bq: mBq * T::from(0.001_f64)}
	}

	/// Returns a copy of this radioactivity value in microbecquerels
	pub fn to_uBq(self) -> T {
		return self.Bq.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new radioactivity value from the given number of microbecquerels
	///
	/// # Arguments
	/// * `uBq` - Any number-like type, representing a quantity of microbecquerels
	pub fn from_uBq(uBq: T) -> Self {
		Radioactivity{Bq: uBq * T::from(1e-06_f64)}
	}

	/// Returns a copy of this radioactivity value in nanobecquerels
	pub fn to_nBq(self) -> T {
		return self.Bq.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new radioactivity value from the given number of nanobecquerels
	///
	/// # Arguments
	/// * `nBq` - Any number-like type, representing a quantity of nanobecquerels
	pub fn from_nBq(nBq: T) -> Self {
		Radioactivity{Bq: nBq * T::from(1e-09_f64)}
	}

	/// Returns a copy of this radioactivity value in kilobecquerels
	pub fn to_kBq(self) -> T {
		return self.Bq.clone() * T::from(0.001_f64);
	}

	/// Returns a new radioactivity value from the given number of kilobecquerels
	///
	/// # Arguments
	/// * `kBq` - Any number-like type, representing a quantity of kilobecquerels
	pub fn from_kBq(kBq: T) -> Self {
		Radioactivity{Bq: kBq * T::from(1000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in megabecquerels
	pub fn to_MBq(self) -> T {
		return self.Bq.clone() * T::from(1e-06_f64);
	}

	/// Returns a new radioactivity value from the given number of megabecquerels
	///
	/// # Arguments
	/// * `MBq` - Any number-like type, representing a quantity of megabecquerels
	pub fn from_MBq(MBq: T) -> Self {
		Radioactivity{Bq: MBq * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in gigabecquerels
	pub fn to_GBq(self) -> T {
		return self.Bq.clone() * T::from(9.999999999999999e-10_f64);
	}

	/// Returns a new radioactivity value from the given number of gigabecquerels
	///
	/// # Arguments
	/// * `GBq` - Any number-like type, representing a quantity of gigabecquerels
	pub fn from_GBq(GBq: T) -> Self {
		Radioactivity{Bq: GBq * T::from(1000000000.0_f64)}
	}

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
		AbsorbedDose{Gy: Gy}
	}
	
	/// Returns a copy of this absorbed dose value in grays
	pub fn to_Gy(self) -> T {
		return self.Gy.clone();
	}

	/// Returns a new absorbed dose value from the given number of grays
	///
	/// # Arguments
	/// * `grays` - Any number-like type, representing a quantity of grays
	pub fn from_grays(grays: T) -> Self {
		AbsorbedDose{Gy: grays}
	}
	
	/// Returns a copy of this absorbed dose value in grays
	pub fn to_grays(self) -> T {
		return self.Gy.clone();
	}

}

impl<T> fmt::Display for AbsorbedDose<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Gy, Self::unit_symbol())
	}
}

impl<T> AbsorbedDose<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this absorbed dose value in grays
	pub fn to_Gy(self) -> T {
		return self.Gy.clone() * T::from(1.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of grays
	///
	/// # Arguments
	/// * `Gy` - Any number-like type, representing a quantity of grays
	pub fn from_Gy(Gy: T) -> Self {
		AbsorbedDose{Gy: Gy * T::from(1.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in grays
	pub fn to_grays(self) -> T {
		return self.Gy.clone() * T::from(1.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of grays
	///
	/// # Arguments
	/// * `grays` - Any number-like type, representing a quantity of grays
	pub fn from_grays(grays: T) -> Self {
		AbsorbedDose{Gy: grays * T::from(1.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in milligrays
	pub fn to_mGy(self) -> T {
		return self.Gy.clone() * T::from(1000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of milligrays
	///
	/// # Arguments
	/// * `mGy` - Any number-like type, representing a quantity of milligrays
	pub fn from_mGy(mGy: T) -> Self {
		AbsorbedDose{Gy: mGy * T::from(0.001_f64)}
	}

	/// Returns a copy of this absorbed dose value in micrograys
	pub fn to_uGy(self) -> T {
		return self.Gy.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of micrograys
	///
	/// # Arguments
	/// * `uGy` - Any number-like type, representing a quantity of micrograys
	pub fn from_uGy(uGy: T) -> Self {
		AbsorbedDose{Gy: uGy * T::from(1e-06_f64)}
	}

	/// Returns a copy of this absorbed dose value in nanograys
	pub fn to_nGy(self) -> T {
		return self.Gy.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of nanograys
	///
	/// # Arguments
	/// * `nGy` - Any number-like type, representing a quantity of nanograys
	pub fn from_nGy(nGy: T) -> Self {
		AbsorbedDose{Gy: nGy * T::from(1e-09_f64)}
	}

	/// Returns a copy of this absorbed dose value in kilograys
	pub fn to_kGy(self) -> T {
		return self.Gy.clone() * T::from(0.001_f64);
	}

	/// Returns a new absorbed dose value from the given number of kilograys
	///
	/// # Arguments
	/// * `kGy` - Any number-like type, representing a quantity of kilograys
	pub fn from_kGy(kGy: T) -> Self {
		AbsorbedDose{Gy: kGy * T::from(1000.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in megagrays
	pub fn to_MGy(self) -> T {
		return self.Gy.clone() * T::from(1e-06_f64);
	}

	/// Returns a new absorbed dose value from the given number of megagrays
	///
	/// # Arguments
	/// * `MGy` - Any number-like type, representing a quantity of megagrays
	pub fn from_MGy(MGy: T) -> Self {
		AbsorbedDose{Gy: MGy * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in gigagrays
	pub fn to_GGy(self) -> T {
		return self.Gy.clone() * T::from(9.999999999999999e-10_f64);
	}

	/// Returns a new absorbed dose value from the given number of gigagrays
	///
	/// # Arguments
	/// * `GGy` - Any number-like type, representing a quantity of gigagrays
	pub fn from_GGy(GGy: T) -> Self {
		AbsorbedDose{Gy: GGy * T::from(1000000000.0_f64)}
	}

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
		DoseEquivalent{Sv: Sv}
	}
	
	/// Returns a copy of this dose equivalent value in sieverts
	pub fn to_Sv(self) -> T {
		return self.Sv.clone();
	}

	/// Returns a new dose equivalent value from the given number of sieverts
	///
	/// # Arguments
	/// * `sieverts` - Any number-like type, representing a quantity of sieverts
	pub fn from_sieverts(sieverts: T) -> Self {
		DoseEquivalent{Sv: sieverts}
	}
	
	/// Returns a copy of this dose equivalent value in sieverts
	pub fn to_sieverts(self) -> T {
		return self.Sv.clone();
	}

}

impl<T> fmt::Display for DoseEquivalent<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Sv, Self::unit_symbol())
	}
}

impl<T> DoseEquivalent<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this dose equivalent value in sieverts
	pub fn to_Sv(self) -> T {
		return self.Sv.clone() * T::from(1.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of sieverts
	///
	/// # Arguments
	/// * `Sv` - Any number-like type, representing a quantity of sieverts
	pub fn from_Sv(Sv: T) -> Self {
		DoseEquivalent{Sv: Sv * T::from(1.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in sieverts
	pub fn to_sieverts(self) -> T {
		return self.Sv.clone() * T::from(1.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of sieverts
	///
	/// # Arguments
	/// * `sieverts` - Any number-like type, representing a quantity of sieverts
	pub fn from_sieverts(sieverts: T) -> Self {
		DoseEquivalent{Sv: sieverts * T::from(1.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in millisieverts
	pub fn to_mSv(self) -> T {
		return self.Sv.clone() * T::from(1000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of millisieverts
	///
	/// # Arguments
	/// * `mSv` - Any number-like type, representing a quantity of millisieverts
	pub fn from_mSv(mSv: T) -> Self {
		DoseEquivalent{Sv: mSv * T::from(0.001_f64)}
	}

	/// Returns a copy of this dose equivalent value in microsieverts
	pub fn to_uSv(self) -> T {
		return self.Sv.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of microsieverts
	///
	/// # Arguments
	/// * `uSv` - Any number-like type, representing a quantity of microsieverts
	pub fn from_uSv(uSv: T) -> Self {
		DoseEquivalent{Sv: uSv * T::from(1e-06_f64)}
	}

	/// Returns a copy of this dose equivalent value in nanosieverts
	pub fn to_nSv(self) -> T {
		return self.Sv.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of nanosieverts
	///
	/// # Arguments
	/// * `nSv` - Any number-like type, representing a quantity of nanosieverts
	pub fn from_nSv(nSv: T) -> Self {
		DoseEquivalent{Sv: nSv * T::from(1e-09_f64)}
	}

	/// Returns a copy of this dose equivalent value in kilosieverts
	pub fn to_kSv(self) -> T {
		return self.Sv.clone() * T::from(0.001_f64);
	}

	/// Returns a new dose equivalent value from the given number of kilosieverts
	///
	/// # Arguments
	/// * `kSv` - Any number-like type, representing a quantity of kilosieverts
	pub fn from_kSv(kSv: T) -> Self {
		DoseEquivalent{Sv: kSv * T::from(1000.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in megasieverts
	pub fn to_MSv(self) -> T {
		return self.Sv.clone() * T::from(1e-06_f64);
	}

	/// Returns a new dose equivalent value from the given number of megasieverts
	///
	/// # Arguments
	/// * `MSv` - Any number-like type, representing a quantity of megasieverts
	pub fn from_MSv(MSv: T) -> Self {
		DoseEquivalent{Sv: MSv * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in gigasieverts
	pub fn to_GSv(self) -> T {
		return self.Sv.clone() * T::from(9.999999999999999e-10_f64);
	}

	/// Returns a new dose equivalent value from the given number of gigasieverts
	///
	/// # Arguments
	/// * `GSv` - Any number-like type, representing a quantity of gigasieverts
	pub fn from_GSv(GSv: T) -> Self {
		DoseEquivalent{Sv: GSv * T::from(1000000000.0_f64)}
	}

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


