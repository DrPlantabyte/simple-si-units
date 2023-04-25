
//! This module provides nuclear SI units, such as absorbed radiation dose 
//! and radioactivity.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::mechanical::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num-bigfloat")]
use num_bigfloat;
#[cfg(feature="num-complex")]
use num_complex;



/// The absorbed radiation dose unit type, defined as grays in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AbsorbedDose<T: NumLike>{
	/// The value of this Absorbed dose in grays
	pub Gy: T
}

impl<T> AbsorbedDose<T> where T: NumLike {

	/// Returns the standard unit name of absorbed dose: "grays"
	pub fn unit_name() -> &'static str { "grays" }
	
	/// Returns the abbreviated name or symbol of absorbed dose: "Gy" for grays
	pub fn unit_symbol() -> &'static str { "Gy" }
	
	/// Returns a new absorbed dose value from the given number of grays
	///
	/// # Arguments
	/// * `Gy` - Any number-like type, representing a quantity of grays
	pub fn from_Gy(Gy: T) -> Self { AbsorbedDose{Gy: Gy} }
	
	/// Returns a copy of this absorbed dose value in grays
	pub fn to_Gy(&self) -> T { self.Gy.clone() }

	/// Returns a new absorbed dose value from the given number of grays
	///
	/// # Arguments
	/// * `grays` - Any number-like type, representing a quantity of grays
	pub fn from_grays(grays: T) -> Self { AbsorbedDose{Gy: grays} }
	
	/// Returns a copy of this absorbed dose value in grays
	pub fn to_grays(&self) -> T { self.Gy.clone() }

}

impl<T> fmt::Display for AbsorbedDose<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Gy, Self::unit_symbol())
	}
}

impl<T> AbsorbedDose<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this absorbed dose value in milligrays
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mGy(&self) -> T {
		return self.Gy.clone() * T::from(1000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of milligrays
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mGy` - Any number-like type, representing a quantity of milligrays
	pub fn from_mGy(mGy: T) -> Self {
		AbsorbedDose{Gy: mGy * T::from(0.001_f64)}
	}

	/// Returns a copy of this absorbed dose value in micrograys
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uGy(&self) -> T {
		return self.Gy.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of micrograys
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uGy` - Any number-like type, representing a quantity of micrograys
	pub fn from_uGy(uGy: T) -> Self {
		AbsorbedDose{Gy: uGy * T::from(1e-06_f64)}
	}

	/// Returns a copy of this absorbed dose value in nanograys
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nGy(&self) -> T {
		return self.Gy.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of nanograys
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nGy` - Any number-like type, representing a quantity of nanograys
	pub fn from_nGy(nGy: T) -> Self {
		AbsorbedDose{Gy: nGy * T::from(1e-09_f64)}
	}

	/// Returns a copy of this absorbed dose value in kilograys
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kGy(&self) -> T {
		return self.Gy.clone() * T::from(0.001_f64);
	}

	/// Returns a new absorbed dose value from the given number of kilograys
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kGy` - Any number-like type, representing a quantity of kilograys
	pub fn from_kGy(kGy: T) -> Self {
		AbsorbedDose{Gy: kGy * T::from(1000.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in megagrays
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MGy(&self) -> T {
		return self.Gy.clone() * T::from(1e-06_f64);
	}

	/// Returns a new absorbed dose value from the given number of megagrays
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MGy` - Any number-like type, representing a quantity of megagrays
	pub fn from_MGy(MGy: T) -> Self {
		AbsorbedDose{Gy: MGy * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in gigagrays
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GGy(&self) -> T {
		return self.Gy.clone() * T::from(1e-09_f64);
	}

	/// Returns a new absorbed dose value from the given number of gigagrays
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GGy` - Any number-like type, representing a quantity of gigagrays
	pub fn from_GGy(GGy: T) -> Self {
		AbsorbedDose{Gy: GGy * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in rads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_rad(&self) -> T {
		return self.Gy.clone() * T::from(100.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of rads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `rad` - Any number-like type, representing a quantity of rads
	pub fn from_rad(rad: T) -> Self {
		AbsorbedDose{Gy: rad * T::from(0.01_f64)}
	}

	/// Returns a copy of this absorbed dose value in kilorads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_krad(&self) -> T {
		return self.Gy.clone() * T::from(0.1_f64);
	}

	/// Returns a new absorbed dose value from the given number of kilorads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `krad` - Any number-like type, representing a quantity of kilorads
	pub fn from_krad(krad: T) -> Self {
		AbsorbedDose{Gy: krad * T::from(10.0_f64)}
	}

	/// Returns a copy of this absorbed dose value in millirads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mrad(&self) -> T {
		return self.Gy.clone() * T::from(100000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of millirads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mrad` - Any number-like type, representing a quantity of millirads
	pub fn from_mrad(mrad: T) -> Self {
		AbsorbedDose{Gy: mrad * T::from(1e-05_f64)}
	}

	/// Returns a copy of this absorbed dose value in microrads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_urad(&self) -> T {
		return self.Gy.clone() * T::from(100000000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of microrads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `urad` - Any number-like type, representing a quantity of microrads
	pub fn from_urad(urad: T) -> Self {
		AbsorbedDose{Gy: urad * T::from(1e-08_f64)}
	}

	/// Returns a copy of this absorbed dose value in ergs per gram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_erg(&self) -> T {
		return self.Gy.clone() * T::from(10000.0_f64);
	}

	/// Returns a new absorbed dose value from the given number of ergs per gram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `erg` - Any number-like type, representing a quantity of ergs per gram
	pub fn from_erg(erg: T) -> Self {
		AbsorbedDose{Gy: erg * T::from(0.0001_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<AbsorbedDose<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = AbsorbedDose<num_bigfloat::BigFloat>;
	fn mul(self, rhs: AbsorbedDose<num_bigfloat::BigFloat>) -> Self::Output {
		AbsorbedDose{Gy: self * rhs.Gy}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<AbsorbedDose<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = AbsorbedDose<num_bigfloat::BigFloat>;
	fn mul(self, rhs: AbsorbedDose<num_bigfloat::BigFloat>) -> Self::Output {
		AbsorbedDose{Gy: self.clone() * rhs.Gy}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<&AbsorbedDose<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = AbsorbedDose<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &AbsorbedDose<num_bigfloat::BigFloat>) -> Self::Output {
		AbsorbedDose{Gy: self * rhs.Gy.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<&AbsorbedDose<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = AbsorbedDose<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &AbsorbedDose<num_bigfloat::BigFloat>) -> Self::Output {
		AbsorbedDose{Gy: self.clone() * rhs.Gy.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<AbsorbedDose<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = AbsorbedDose<num_complex::Complex32>;
	fn mul(self, rhs: AbsorbedDose<num_complex::Complex32>) -> Self::Output {
		AbsorbedDose{Gy: self * rhs.Gy}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<AbsorbedDose<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = AbsorbedDose<num_complex::Complex32>;
	fn mul(self, rhs: AbsorbedDose<num_complex::Complex32>) -> Self::Output {
		AbsorbedDose{Gy: self.clone() * rhs.Gy}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&AbsorbedDose<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = AbsorbedDose<num_complex::Complex32>;
	fn mul(self, rhs: &AbsorbedDose<num_complex::Complex32>) -> Self::Output {
		AbsorbedDose{Gy: self * rhs.Gy.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&AbsorbedDose<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = AbsorbedDose<num_complex::Complex32>;
	fn mul(self, rhs: &AbsorbedDose<num_complex::Complex32>) -> Self::Output {
		AbsorbedDose{Gy: self.clone() * rhs.Gy.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<AbsorbedDose<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = AbsorbedDose<num_complex::Complex64>;
	fn mul(self, rhs: AbsorbedDose<num_complex::Complex64>) -> Self::Output {
		AbsorbedDose{Gy: self * rhs.Gy}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<AbsorbedDose<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = AbsorbedDose<num_complex::Complex64>;
	fn mul(self, rhs: AbsorbedDose<num_complex::Complex64>) -> Self::Output {
		AbsorbedDose{Gy: self.clone() * rhs.Gy}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&AbsorbedDose<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = AbsorbedDose<num_complex::Complex64>;
	fn mul(self, rhs: &AbsorbedDose<num_complex::Complex64>) -> Self::Output {
		AbsorbedDose{Gy: self * rhs.Gy.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&AbsorbedDose<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = AbsorbedDose<num_complex::Complex64>;
	fn mul(self, rhs: &AbsorbedDose<num_complex::Complex64>) -> Self::Output {
		AbsorbedDose{Gy: self.clone() * rhs.Gy.clone()}
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
	pub fn unit_name() -> &'static str { "sieverts" }
	
	/// Returns the abbreviated name or symbol of dose equivalent: "Sv" for sieverts
	pub fn unit_symbol() -> &'static str { "Sv" }
	
	/// Returns a new dose equivalent value from the given number of sieverts
	///
	/// # Arguments
	/// * `Sv` - Any number-like type, representing a quantity of sieverts
	pub fn from_Sv(Sv: T) -> Self { DoseEquivalent{Sv: Sv} }
	
	/// Returns a copy of this dose equivalent value in sieverts
	pub fn to_Sv(&self) -> T { self.Sv.clone() }

	/// Returns a new dose equivalent value from the given number of sieverts
	///
	/// # Arguments
	/// * `sieverts` - Any number-like type, representing a quantity of sieverts
	pub fn from_sieverts(sieverts: T) -> Self { DoseEquivalent{Sv: sieverts} }
	
	/// Returns a copy of this dose equivalent value in sieverts
	pub fn to_sieverts(&self) -> T { self.Sv.clone() }

}

impl<T> fmt::Display for DoseEquivalent<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Sv, Self::unit_symbol())
	}
}

impl<T> DoseEquivalent<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this dose equivalent value in millisieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mSv(&self) -> T {
		return self.Sv.clone() * T::from(1000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of millisieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mSv` - Any number-like type, representing a quantity of millisieverts
	pub fn from_mSv(mSv: T) -> Self {
		DoseEquivalent{Sv: mSv * T::from(0.001_f64)}
	}

	/// Returns a copy of this dose equivalent value in microsieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uSv(&self) -> T {
		return self.Sv.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of microsieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uSv` - Any number-like type, representing a quantity of microsieverts
	pub fn from_uSv(uSv: T) -> Self {
		DoseEquivalent{Sv: uSv * T::from(1e-06_f64)}
	}

	/// Returns a copy of this dose equivalent value in nanosieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nSv(&self) -> T {
		return self.Sv.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of nanosieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nSv` - Any number-like type, representing a quantity of nanosieverts
	pub fn from_nSv(nSv: T) -> Self {
		DoseEquivalent{Sv: nSv * T::from(1e-09_f64)}
	}

	/// Returns a copy of this dose equivalent value in kilosieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kSv(&self) -> T {
		return self.Sv.clone() * T::from(0.001_f64);
	}

	/// Returns a new dose equivalent value from the given number of kilosieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kSv` - Any number-like type, representing a quantity of kilosieverts
	pub fn from_kSv(kSv: T) -> Self {
		DoseEquivalent{Sv: kSv * T::from(1000.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in megasieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MSv(&self) -> T {
		return self.Sv.clone() * T::from(1e-06_f64);
	}

	/// Returns a new dose equivalent value from the given number of megasieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MSv` - Any number-like type, representing a quantity of megasieverts
	pub fn from_MSv(MSv: T) -> Self {
		DoseEquivalent{Sv: MSv * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in gigasieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GSv(&self) -> T {
		return self.Sv.clone() * T::from(1e-09_f64);
	}

	/// Returns a new dose equivalent value from the given number of gigasieverts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GSv` - Any number-like type, representing a quantity of gigasieverts
	pub fn from_GSv(GSv: T) -> Self {
		DoseEquivalent{Sv: GSv * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this dose equivalent value in roentgen equivalent man
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_rem(&self) -> T {
		return self.Sv.clone() * T::from(100.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of roentgen equivalent man
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `rem` - Any number-like type, representing a quantity of roentgen equivalent man
	pub fn from_rem(rem: T) -> Self {
		DoseEquivalent{Sv: rem * T::from(0.01_f64)}
	}

	/// Returns a copy of this dose equivalent value in milli-roentgen equivalents
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mrem(&self) -> T {
		return self.Sv.clone() * T::from(100000.0_f64);
	}

	/// Returns a new dose equivalent value from the given number of milli-roentgen equivalents
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mrem` - Any number-like type, representing a quantity of milli-roentgen equivalents
	pub fn from_mrem(mrem: T) -> Self {
		DoseEquivalent{Sv: mrem * T::from(1e-05_f64)}
	}

	/// Returns a copy of this dose equivalent value in kilo-roentgen equivalents
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_krem(&self) -> T {
		return self.Sv.clone() * T::from(0.1_f64);
	}

	/// Returns a new dose equivalent value from the given number of kilo-roentgen equivalents
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `krem` - Any number-like type, representing a quantity of kilo-roentgen equivalents
	pub fn from_krem(krem: T) -> Self {
		DoseEquivalent{Sv: krem * T::from(10.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<DoseEquivalent<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = DoseEquivalent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: DoseEquivalent<num_bigfloat::BigFloat>) -> Self::Output {
		DoseEquivalent{Sv: self * rhs.Sv}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<DoseEquivalent<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = DoseEquivalent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: DoseEquivalent<num_bigfloat::BigFloat>) -> Self::Output {
		DoseEquivalent{Sv: self.clone() * rhs.Sv}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<&DoseEquivalent<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = DoseEquivalent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &DoseEquivalent<num_bigfloat::BigFloat>) -> Self::Output {
		DoseEquivalent{Sv: self * rhs.Sv.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<&DoseEquivalent<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = DoseEquivalent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &DoseEquivalent<num_bigfloat::BigFloat>) -> Self::Output {
		DoseEquivalent{Sv: self.clone() * rhs.Sv.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<DoseEquivalent<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = DoseEquivalent<num_complex::Complex32>;
	fn mul(self, rhs: DoseEquivalent<num_complex::Complex32>) -> Self::Output {
		DoseEquivalent{Sv: self * rhs.Sv}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<DoseEquivalent<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = DoseEquivalent<num_complex::Complex32>;
	fn mul(self, rhs: DoseEquivalent<num_complex::Complex32>) -> Self::Output {
		DoseEquivalent{Sv: self.clone() * rhs.Sv}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&DoseEquivalent<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = DoseEquivalent<num_complex::Complex32>;
	fn mul(self, rhs: &DoseEquivalent<num_complex::Complex32>) -> Self::Output {
		DoseEquivalent{Sv: self * rhs.Sv.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&DoseEquivalent<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = DoseEquivalent<num_complex::Complex32>;
	fn mul(self, rhs: &DoseEquivalent<num_complex::Complex32>) -> Self::Output {
		DoseEquivalent{Sv: self.clone() * rhs.Sv.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<DoseEquivalent<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = DoseEquivalent<num_complex::Complex64>;
	fn mul(self, rhs: DoseEquivalent<num_complex::Complex64>) -> Self::Output {
		DoseEquivalent{Sv: self * rhs.Sv}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<DoseEquivalent<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = DoseEquivalent<num_complex::Complex64>;
	fn mul(self, rhs: DoseEquivalent<num_complex::Complex64>) -> Self::Output {
		DoseEquivalent{Sv: self.clone() * rhs.Sv}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&DoseEquivalent<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = DoseEquivalent<num_complex::Complex64>;
	fn mul(self, rhs: &DoseEquivalent<num_complex::Complex64>) -> Self::Output {
		DoseEquivalent{Sv: self * rhs.Sv.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&DoseEquivalent<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = DoseEquivalent<num_complex::Complex64>;
	fn mul(self, rhs: &DoseEquivalent<num_complex::Complex64>) -> Self::Output {
		DoseEquivalent{Sv: self.clone() * rhs.Sv.clone()}
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

/// The radioactivity unit type, defined as becquerels in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Radioactivity<T: NumLike>{
	/// The value of this Radioactivity in becquerels
	pub Bq: T
}

impl<T> Radioactivity<T> where T: NumLike {

	/// Returns the standard unit name of radioactivity: "becquerels"
	pub fn unit_name() -> &'static str { "becquerels" }
	
	/// Returns the abbreviated name or symbol of radioactivity: "Bq" for becquerels
	pub fn unit_symbol() -> &'static str { "Bq" }
	
	/// Returns a new radioactivity value from the given number of becquerels
	///
	/// # Arguments
	/// * `Bq` - Any number-like type, representing a quantity of becquerels
	pub fn from_Bq(Bq: T) -> Self { Radioactivity{Bq: Bq} }
	
	/// Returns a copy of this radioactivity value in becquerels
	pub fn to_Bq(&self) -> T { self.Bq.clone() }

	/// Returns a new radioactivity value from the given number of becquerels
	///
	/// # Arguments
	/// * `becquerels` - Any number-like type, representing a quantity of becquerels
	pub fn from_becquerels(becquerels: T) -> Self { Radioactivity{Bq: becquerels} }
	
	/// Returns a copy of this radioactivity value in becquerels
	pub fn to_becquerels(&self) -> T { self.Bq.clone() }

}

impl<T> fmt::Display for Radioactivity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Bq, Self::unit_symbol())
	}
}

impl<T> Radioactivity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this radioactivity value in millibecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mBq(&self) -> T {
		return self.Bq.clone() * T::from(1000.0_f64);
	}

	/// Returns a new radioactivity value from the given number of millibecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mBq` - Any number-like type, representing a quantity of millibecquerels
	pub fn from_mBq(mBq: T) -> Self {
		Radioactivity{Bq: mBq * T::from(0.001_f64)}
	}

	/// Returns a copy of this radioactivity value in microbecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uBq(&self) -> T {
		return self.Bq.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new radioactivity value from the given number of microbecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uBq` - Any number-like type, representing a quantity of microbecquerels
	pub fn from_uBq(uBq: T) -> Self {
		Radioactivity{Bq: uBq * T::from(1e-06_f64)}
	}

	/// Returns a copy of this radioactivity value in nanobecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nBq(&self) -> T {
		return self.Bq.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new radioactivity value from the given number of nanobecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nBq` - Any number-like type, representing a quantity of nanobecquerels
	pub fn from_nBq(nBq: T) -> Self {
		Radioactivity{Bq: nBq * T::from(1e-09_f64)}
	}

	/// Returns a copy of this radioactivity value in kilobecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kBq(&self) -> T {
		return self.Bq.clone() * T::from(0.001_f64);
	}

	/// Returns a new radioactivity value from the given number of kilobecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kBq` - Any number-like type, representing a quantity of kilobecquerels
	pub fn from_kBq(kBq: T) -> Self {
		Radioactivity{Bq: kBq * T::from(1000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in megabecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MBq(&self) -> T {
		return self.Bq.clone() * T::from(1e-06_f64);
	}

	/// Returns a new radioactivity value from the given number of megabecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MBq` - Any number-like type, representing a quantity of megabecquerels
	pub fn from_MBq(MBq: T) -> Self {
		Radioactivity{Bq: MBq * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in gigabecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GBq(&self) -> T {
		return self.Bq.clone() * T::from(1e-09_f64);
	}

	/// Returns a new radioactivity value from the given number of gigabecquerels
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GBq` - Any number-like type, representing a quantity of gigabecquerels
	pub fn from_GBq(GBq: T) -> Self {
		Radioactivity{Bq: GBq * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in curies
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Ci(&self) -> T {
		return self.Bq.clone() * T::from(2.7027027027027e-11_f64);
	}

	/// Returns a new radioactivity value from the given number of curies
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Ci` - Any number-like type, representing a quantity of curies
	pub fn from_Ci(Ci: T) -> Self {
		Radioactivity{Bq: Ci * T::from(37000000000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in millicuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mCi(&self) -> T {
		return self.Bq.clone() * T::from(2.7027027027027e-08_f64);
	}

	/// Returns a new radioactivity value from the given number of millicuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mCi` - Any number-like type, representing a quantity of millicuries
	pub fn from_mCi(mCi: T) -> Self {
		Radioactivity{Bq: mCi * T::from(37000000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in microcuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uCi(&self) -> T {
		return self.Bq.clone() * T::from(2.7027027027027e-05_f64);
	}

	/// Returns a new radioactivity value from the given number of microcuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uCi` - Any number-like type, representing a quantity of microcuries
	pub fn from_uCi(uCi: T) -> Self {
		Radioactivity{Bq: uCi * T::from(37000.0_f64)}
	}

	/// Returns a copy of this radioactivity value in nanocuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nCi(&self) -> T {
		return self.Bq.clone() * T::from(0.027027027027027_f64);
	}

	/// Returns a new radioactivity value from the given number of nanocuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nCi` - Any number-like type, representing a quantity of nanocuries
	pub fn from_nCi(nCi: T) -> Self {
		Radioactivity{Bq: nCi * T::from(37.0_f64)}
	}

	/// Returns a copy of this radioactivity value in picocuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_pCi(&self) -> T {
		return self.Bq.clone() * T::from(27.027027027027_f64);
	}

	/// Returns a new radioactivity value from the given number of picocuries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `pCi` - Any number-like type, representing a quantity of picocuries
	pub fn from_pCi(pCi: T) -> Self {
		Radioactivity{Bq: pCi * T::from(0.037_f64)}
	}

	/// Returns a copy of this radioactivity value in rutherfords
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Rd(&self) -> T {
		return self.Bq.clone() * T::from(1e-06_f64);
	}

	/// Returns a new radioactivity value from the given number of rutherfords
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Rd` - Any number-like type, representing a quantity of rutherfords
	pub fn from_Rd(Rd: T) -> Self {
		Radioactivity{Bq: Rd * T::from(1000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<Radioactivity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Radioactivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Radioactivity<num_bigfloat::BigFloat>) -> Self::Output {
		Radioactivity{Bq: self * rhs.Bq}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<Radioactivity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Radioactivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Radioactivity<num_bigfloat::BigFloat>) -> Self::Output {
		Radioactivity{Bq: self.clone() * rhs.Bq}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<&Radioactivity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Radioactivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Radioactivity<num_bigfloat::BigFloat>) -> Self::Output {
		Radioactivity{Bq: self * rhs.Bq.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl std::ops::Mul<&Radioactivity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Radioactivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Radioactivity<num_bigfloat::BigFloat>) -> Self::Output {
		Radioactivity{Bq: self.clone() * rhs.Bq.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<Radioactivity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Radioactivity<num_complex::Complex32>;
	fn mul(self, rhs: Radioactivity<num_complex::Complex32>) -> Self::Output {
		Radioactivity{Bq: self * rhs.Bq}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<Radioactivity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Radioactivity<num_complex::Complex32>;
	fn mul(self, rhs: Radioactivity<num_complex::Complex32>) -> Self::Output {
		Radioactivity{Bq: self.clone() * rhs.Bq}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&Radioactivity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Radioactivity<num_complex::Complex32>;
	fn mul(self, rhs: &Radioactivity<num_complex::Complex32>) -> Self::Output {
		Radioactivity{Bq: self * rhs.Bq.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&Radioactivity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Radioactivity<num_complex::Complex32>;
	fn mul(self, rhs: &Radioactivity<num_complex::Complex32>) -> Self::Output {
		Radioactivity{Bq: self.clone() * rhs.Bq.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<Radioactivity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Radioactivity<num_complex::Complex64>;
	fn mul(self, rhs: Radioactivity<num_complex::Complex64>) -> Self::Output {
		Radioactivity{Bq: self * rhs.Bq}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<Radioactivity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Radioactivity<num_complex::Complex64>;
	fn mul(self, rhs: Radioactivity<num_complex::Complex64>) -> Self::Output {
		Radioactivity{Bq: self.clone() * rhs.Bq}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&Radioactivity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Radioactivity<num_complex::Complex64>;
	fn mul(self, rhs: &Radioactivity<num_complex::Complex64>) -> Self::Output {
		Radioactivity{Bq: self * rhs.Bq.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl std::ops::Mul<&Radioactivity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Radioactivity<num_complex::Complex64>;
	fn mul(self, rhs: &Radioactivity<num_complex::Complex64>) -> Self::Output {
		Radioactivity{Bq: self.clone() * rhs.Bq.clone()}
	}
}



/// Converts a Radioactivity into the equivalent [uom](https://crates.io/crates/uom) type [Radioactivity](https://docs.rs/uom/0.34.0/uom/si/f32/type.Radioactivity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Radioactivity> for Radioactivity<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Radioactivity {
		uom::si::f32::Radioactivity::new::<uom::si::radioactivity::becquerel>(self.Bq.into())
	}
}

/// Creates a Radioactivity from the equivalent [uom](https://crates.io/crates/uom) type [Radioactivity](https://docs.rs/uom/0.34.0/uom/si/f32/type.Radioactivity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Radioactivity> for Radioactivity<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Radioactivity) -> Self {
		Radioactivity{Bq: T::from(src.value)}
	}
}

/// Converts a Radioactivity into the equivalent [uom](https://crates.io/crates/uom) type [Radioactivity](https://docs.rs/uom/0.34.0/uom/si/f64/type.Radioactivity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Radioactivity> for Radioactivity<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Radioactivity {
		uom::si::f64::Radioactivity::new::<uom::si::radioactivity::becquerel>(self.Bq.into())
	}
}

/// Creates a Radioactivity from the equivalent [uom](https://crates.io/crates/uom) type [Radioactivity](https://docs.rs/uom/0.34.0/uom/si/f64/type.Radioactivity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Radioactivity> for Radioactivity<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Radioactivity) -> Self {
		Radioactivity{Bq: T::from(src.value)}
	}
}


// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}

// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}

// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}

// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<Radioactivity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
impl<T> std::ops::Div<&Radioactivity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}

// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<Radioactivity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<Radioactivity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<&Radioactivity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<&Radioactivity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}

// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Radioactivity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Radioactivity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Radioactivity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Radioactivity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}

// 1/Radioactivity -> Time
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Radioactivity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Radioactivity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Radioactivity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Bq.clone()}
	}
}
/// Dividing a scalar value by a Radioactivity unit value returns a value of type Time
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Radioactivity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: &Radioactivity<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Bq.clone()}
	}
}



