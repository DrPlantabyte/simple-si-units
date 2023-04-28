
//! This module provides base SI units, such as amount 
//! and mass.
use core::fmt;
use super::UnitStruct;
use super::NumLike;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num-bigfloat")]
use num_bigfloat;
#[cfg(feature="num-complex")]
use num_complex;



/// The amount unit type, defined as moles in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Amount<T: NumLike>{
	/// The value of this Amount in moles
	pub mol: T
}

impl<T> Amount<T> where T: NumLike {

	/// Returns the standard unit name of amount: "moles"
	pub fn unit_name() -> &'static str { "moles" }
	
	/// Returns the abbreviated name or symbol of amount: "mol" for moles
	pub fn unit_symbol() -> &'static str { "mol" }
	
	/// Returns a new amount value from the given number of moles
	///
	/// # Arguments
	/// * `moles` - Any number-like type, representing a quantity of moles
	pub fn from_moles(moles: T) -> Self { Amount{mol: moles} }
	
	/// Returns a copy of this amount value in moles
	pub fn to_moles(&self) -> T { self.mol.clone() }

	/// Returns a new amount value from the given number of moles
	///
	/// # Arguments
	/// * `mol` - Any number-like type, representing a quantity of moles
	pub fn from_mol(mol: T) -> Self { Amount{mol: mol} }
	
	/// Returns a copy of this amount value in moles
	pub fn to_mol(&self) -> T { self.mol.clone() }

}

impl<T> fmt::Display for Amount<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.mol, Self::unit_symbol())
	}
}

impl<T> Amount<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this amount value in count
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_count(&self) -> T {
		return self.mol.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new amount value from the given number of count
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `count` - Any number-like type, representing a quantity of count
	pub fn from_count(count: T) -> Self {
		Amount{mol: count * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this amount value in millimoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mmol(&self) -> T {
		return self.mol.clone() * T::from(1000.0_f64);
	}

	/// Returns a new amount value from the given number of millimoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mmol` - Any number-like type, representing a quantity of millimoles
	pub fn from_mmol(mmol: T) -> Self {
		Amount{mol: mmol * T::from(0.001_f64)}
	}

	/// Returns a copy of this amount value in micromoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_umol(&self) -> T {
		return self.mol.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new amount value from the given number of micromoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `umol` - Any number-like type, representing a quantity of micromoles
	pub fn from_umol(umol: T) -> Self {
		Amount{mol: umol * T::from(1e-06_f64)}
	}

	/// Returns a copy of this amount value in nanomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nmol(&self) -> T {
		return self.mol.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new amount value from the given number of nanomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nmol` - Any number-like type, representing a quantity of nanomoles
	pub fn from_nmol(nmol: T) -> Self {
		Amount{mol: nmol * T::from(1e-09_f64)}
	}

	/// Returns a copy of this amount value in picomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_pmol(&self) -> T {
		return self.mol.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new amount value from the given number of picomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `pmol` - Any number-like type, representing a quantity of picomoles
	pub fn from_pmol(pmol: T) -> Self {
		Amount{mol: pmol * T::from(1e-12_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Amount<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Amount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Amount<num_bigfloat::BigFloat>) -> Self::Output {
		Amount{mol: self * rhs.mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Amount<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Amount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Amount<num_bigfloat::BigFloat>) -> Self::Output {
		Amount{mol: self.clone() * rhs.mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Amount<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Amount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Amount<num_bigfloat::BigFloat>) -> Self::Output {
		Amount{mol: self * rhs.mol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Amount<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Amount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Amount<num_bigfloat::BigFloat>) -> Self::Output {
		Amount{mol: self.clone() * rhs.mol.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Amount<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Amount<num_complex::Complex32>;
	fn mul(self, rhs: Amount<num_complex::Complex32>) -> Self::Output {
		Amount{mol: self * rhs.mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Amount<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Amount<num_complex::Complex32>;
	fn mul(self, rhs: Amount<num_complex::Complex32>) -> Self::Output {
		Amount{mol: self.clone() * rhs.mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Amount<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Amount<num_complex::Complex32>;
	fn mul(self, rhs: &Amount<num_complex::Complex32>) -> Self::Output {
		Amount{mol: self * rhs.mol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Amount<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Amount<num_complex::Complex32>;
	fn mul(self, rhs: &Amount<num_complex::Complex32>) -> Self::Output {
		Amount{mol: self.clone() * rhs.mol.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Amount<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Amount<num_complex::Complex64>;
	fn mul(self, rhs: Amount<num_complex::Complex64>) -> Self::Output {
		Amount{mol: self * rhs.mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Amount<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Amount<num_complex::Complex64>;
	fn mul(self, rhs: Amount<num_complex::Complex64>) -> Self::Output {
		Amount{mol: self.clone() * rhs.mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Amount<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Amount<num_complex::Complex64>;
	fn mul(self, rhs: &Amount<num_complex::Complex64>) -> Self::Output {
		Amount{mol: self * rhs.mol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Amount<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Amount<num_complex::Complex64>;
	fn mul(self, rhs: &Amount<num_complex::Complex64>) -> Self::Output {
		Amount{mol: self.clone() * rhs.mol.clone()}
	}
}



/// Converts a Amount into the equivalent [uom](https://crates.io/crates/uom) type [AmountOfSubstance](https://docs.rs/uom/0.34.0/uom/si/f32/type.AmountOfSubstance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::AmountOfSubstance> for Amount<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::AmountOfSubstance {
		uom::si::f32::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(self.mol.into())
	}
}

/// Creates a Amount from the equivalent [uom](https://crates.io/crates/uom) type [AmountOfSubstance](https://docs.rs/uom/0.34.0/uom/si/f32/type.AmountOfSubstance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::AmountOfSubstance> for Amount<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::AmountOfSubstance) -> Self {
		Amount{mol: T::from(src.value)}
	}
}

/// Converts a Amount into the equivalent [uom](https://crates.io/crates/uom) type [AmountOfSubstance](https://docs.rs/uom/0.34.0/uom/si/f64/type.AmountOfSubstance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::AmountOfSubstance> for Amount<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::AmountOfSubstance {
		uom::si::f64::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(self.mol.into())
	}
}

/// Creates a Amount from the equivalent [uom](https://crates.io/crates/uom) type [AmountOfSubstance](https://docs.rs/uom/0.34.0/uom/si/f64/type.AmountOfSubstance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::AmountOfSubstance> for Amount<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::AmountOfSubstance) -> Self {
		Amount{mol: T::from(src.value)}
	}
}


// Amount / Time -> CatalyticActivity
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> core::ops::Div<Time<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol / rhs.s}
	}
}
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> core::ops::Div<Time<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() / rhs.s}
	}
}
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> core::ops::Div<&Time<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol / rhs.s.clone()}
	}
}
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> core::ops::Div<&Time<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() / rhs.s.clone()}
	}
}

// Amount / CatalyticActivity -> Time
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> core::ops::Div<CatalyticActivity<T>> for Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol / rhs.molps}
	}
}
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> core::ops::Div<CatalyticActivity<T>> for &Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol.clone() / rhs.molps}
	}
}
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> core::ops::Div<&CatalyticActivity<T>> for Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol / rhs.molps.clone()}
	}
}
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> core::ops::Div<&CatalyticActivity<T>> for &Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol.clone() / rhs.molps.clone()}
	}
}

// Amount / Concentration -> Volume
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> core::ops::Div<Concentration<T>> for Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Concentration<T>) -> Self::Output {
		Volume{m3: self.mol / rhs.molpm3}
	}
}
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> core::ops::Div<Concentration<T>> for &Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Concentration<T>) -> Self::Output {
		Volume{m3: self.mol.clone() / rhs.molpm3}
	}
}
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> core::ops::Div<&Concentration<T>> for Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Concentration<T>) -> Self::Output {
		Volume{m3: self.mol / rhs.molpm3.clone()}
	}
}
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> core::ops::Div<&Concentration<T>> for &Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Concentration<T>) -> Self::Output {
		Volume{m3: self.mol.clone() / rhs.molpm3.clone()}
	}
}

// Amount / Volume -> Concentration
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> core::ops::Div<Volume<T>> for Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol / rhs.m3}
	}
}
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> core::ops::Div<Volume<T>> for &Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol.clone() / rhs.m3}
	}
}
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> core::ops::Div<&Volume<T>> for Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol / rhs.m3.clone()}
	}
}
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> core::ops::Div<&Volume<T>> for &Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol.clone() / rhs.m3.clone()}
	}
}

// Amount * Frequency -> CatalyticActivity
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> core::ops::Mul<Frequency<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol * rhs.Hz}
	}
}
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> core::ops::Mul<Frequency<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() * rhs.Hz}
	}
}
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> core::ops::Mul<&Frequency<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol * rhs.Hz.clone()}
	}
}
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> core::ops::Mul<&Frequency<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() * rhs.Hz.clone()}
	}
}

/// The electrical current unit type, defined as amperes in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Current<T: NumLike>{
	/// The value of this Electrical current in amperes
	pub A: T
}

impl<T> Current<T> where T: NumLike {

	/// Returns the standard unit name of electrical current: "amperes"
	pub fn unit_name() -> &'static str { "amperes" }
	
	/// Returns the abbreviated name or symbol of electrical current: "A" for amperes
	pub fn unit_symbol() -> &'static str { "A" }
	
	/// Returns a new electrical current value from the given number of amperes
	///
	/// # Arguments
	/// * `A` - Any number-like type, representing a quantity of amperes
	pub fn from_A(A: T) -> Self { Current{A: A} }
	
	/// Returns a copy of this electrical current value in amperes
	pub fn to_A(&self) -> T { self.A.clone() }

	/// Returns a new electrical current value from the given number of amperes
	///
	/// # Arguments
	/// * `amps` - Any number-like type, representing a quantity of amperes
	pub fn from_amps(amps: T) -> Self { Current{A: amps} }
	
	/// Returns a copy of this electrical current value in amperes
	pub fn to_amps(&self) -> T { self.A.clone() }

}

impl<T> fmt::Display for Current<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.A, Self::unit_symbol())
	}
}

impl<T> Current<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical current value in milliamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mA(&self) -> T {
		return self.A.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical current value from the given number of milliamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mA` - Any number-like type, representing a quantity of milliamperes
	pub fn from_mA(mA: T) -> Self {
		Current{A: mA * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical current value in microamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uA(&self) -> T {
		return self.A.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical current value from the given number of microamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uA` - Any number-like type, representing a quantity of microamperes
	pub fn from_uA(uA: T) -> Self {
		Current{A: uA * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical current value in nanoamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nA(&self) -> T {
		return self.A.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical current value from the given number of nanoamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nA` - Any number-like type, representing a quantity of nanoamperes
	pub fn from_nA(nA: T) -> Self {
		Current{A: nA * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical current value in kiloamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kA(&self) -> T {
		return self.A.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical current value from the given number of kiloamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kA` - Any number-like type, representing a quantity of kiloamperes
	pub fn from_kA(kA: T) -> Self {
		Current{A: kA * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical current value in megaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MA(&self) -> T {
		return self.A.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical current value from the given number of megaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MA` - Any number-like type, representing a quantity of megaamperes
	pub fn from_MA(MA: T) -> Self {
		Current{A: MA * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical current value in gigaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GA(&self) -> T {
		return self.A.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical current value from the given number of gigaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GA` - Any number-like type, representing a quantity of gigaamperes
	pub fn from_GA(GA: T) -> Self {
		Current{A: GA * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Current<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Current<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Current<num_bigfloat::BigFloat>) -> Self::Output {
		Current{A: self * rhs.A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Current<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Current<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Current<num_bigfloat::BigFloat>) -> Self::Output {
		Current{A: self.clone() * rhs.A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Current<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Current<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Current<num_bigfloat::BigFloat>) -> Self::Output {
		Current{A: self * rhs.A.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Current<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Current<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Current<num_bigfloat::BigFloat>) -> Self::Output {
		Current{A: self.clone() * rhs.A.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Current<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Current<num_complex::Complex32>;
	fn mul(self, rhs: Current<num_complex::Complex32>) -> Self::Output {
		Current{A: self * rhs.A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Current<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Current<num_complex::Complex32>;
	fn mul(self, rhs: Current<num_complex::Complex32>) -> Self::Output {
		Current{A: self.clone() * rhs.A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Current<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Current<num_complex::Complex32>;
	fn mul(self, rhs: &Current<num_complex::Complex32>) -> Self::Output {
		Current{A: self * rhs.A.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Current<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Current<num_complex::Complex32>;
	fn mul(self, rhs: &Current<num_complex::Complex32>) -> Self::Output {
		Current{A: self.clone() * rhs.A.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Current<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Current<num_complex::Complex64>;
	fn mul(self, rhs: Current<num_complex::Complex64>) -> Self::Output {
		Current{A: self * rhs.A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Current<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Current<num_complex::Complex64>;
	fn mul(self, rhs: Current<num_complex::Complex64>) -> Self::Output {
		Current{A: self.clone() * rhs.A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Current<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Current<num_complex::Complex64>;
	fn mul(self, rhs: &Current<num_complex::Complex64>) -> Self::Output {
		Current{A: self * rhs.A.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Current<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Current<num_complex::Complex64>;
	fn mul(self, rhs: &Current<num_complex::Complex64>) -> Self::Output {
		Current{A: self.clone() * rhs.A.clone()}
	}
}



/// Converts a Current into the equivalent [uom](https://crates.io/crates/uom) type [ElectricCurrent](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricCurrent.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricCurrent> for Current<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricCurrent {
		uom::si::f32::ElectricCurrent::new::<uom::si::electric_current::ampere>(self.A.into())
	}
}

/// Creates a Current from the equivalent [uom](https://crates.io/crates/uom) type [ElectricCurrent](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricCurrent.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricCurrent> for Current<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricCurrent) -> Self {
		Current{A: T::from(src.value)}
	}
}

/// Converts a Current into the equivalent [uom](https://crates.io/crates/uom) type [ElectricCurrent](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricCurrent.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricCurrent> for Current<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricCurrent {
		uom::si::f64::ElectricCurrent::new::<uom::si::electric_current::ampere>(self.A.into())
	}
}

/// Creates a Current from the equivalent [uom](https://crates.io/crates/uom) type [ElectricCurrent](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricCurrent.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricCurrent> for Current<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricCurrent) -> Self {
		Current{A: T::from(src.value)}
	}
}


// Current * Time -> Charge
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> core::ops::Mul<Time<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Charge{C: self.A * rhs.s}
	}
}
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> core::ops::Mul<Time<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Charge{C: self.A.clone() * rhs.s}
	}
}
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> core::ops::Mul<&Time<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Charge{C: self.A * rhs.s.clone()}
	}
}
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> core::ops::Mul<&Time<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Charge{C: self.A.clone() * rhs.s.clone()}
	}
}

// Current / Charge -> Frequency
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> core::ops::Div<Charge<T>> for Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Frequency{Hz: self.A / rhs.C}
	}
}
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> core::ops::Div<Charge<T>> for &Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Frequency{Hz: self.A.clone() / rhs.C}
	}
}
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> core::ops::Div<&Charge<T>> for Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Frequency{Hz: self.A / rhs.C.clone()}
	}
}
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> core::ops::Div<&Charge<T>> for &Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Frequency{Hz: self.A.clone() / rhs.C.clone()}
	}
}

// Current / Conductance -> Voltage
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> core::ops::Div<Conductance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Voltage{V: self.A / rhs.S}
	}
}
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> core::ops::Div<Conductance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Voltage{V: self.A.clone() / rhs.S}
	}
}
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> core::ops::Div<&Conductance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Voltage{V: self.A / rhs.S.clone()}
	}
}
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> core::ops::Div<&Conductance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Voltage{V: self.A.clone() / rhs.S.clone()}
	}
}

// Current * Inductance -> MagneticFlux
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> core::ops::Mul<Inductance<T>> for Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A * rhs.H}
	}
}
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> core::ops::Mul<Inductance<T>> for &Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A.clone() * rhs.H}
	}
}
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Inductance<T>> for Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A * rhs.H.clone()}
	}
}
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Inductance<T>> for &Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A.clone() * rhs.H.clone()}
	}
}

// Current * MagneticFlux -> Energy
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> core::ops::Mul<MagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A * rhs.Wb}
	}
}
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> core::ops::Mul<MagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A.clone() * rhs.Wb}
	}
}
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> core::ops::Mul<&MagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A * rhs.Wb.clone()}
	}
}
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> core::ops::Mul<&MagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A.clone() * rhs.Wb.clone()}
	}
}

// Current * Resistance -> Voltage
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> core::ops::Mul<Resistance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Voltage{V: self.A * rhs.Ohm}
	}
}
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> core::ops::Mul<Resistance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Voltage{V: self.A.clone() * rhs.Ohm}
	}
}
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> core::ops::Mul<&Resistance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Voltage{V: self.A * rhs.Ohm.clone()}
	}
}
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> core::ops::Mul<&Resistance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Voltage{V: self.A.clone() * rhs.Ohm.clone()}
	}
}

// Current * Voltage -> Power
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> core::ops::Mul<Voltage<T>> for Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Power{W: self.A * rhs.V}
	}
}
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> core::ops::Mul<Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Power{W: self.A.clone() * rhs.V}
	}
}
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> core::ops::Mul<&Voltage<T>> for Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Power{W: self.A * rhs.V.clone()}
	}
}
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> core::ops::Mul<&Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Power{W: self.A.clone() * rhs.V.clone()}
	}
}

// Current / Voltage -> Conductance
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> core::ops::Div<Voltage<T>> for Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Conductance{S: self.A / rhs.V}
	}
}
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> core::ops::Div<Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Conductance{S: self.A.clone() / rhs.V}
	}
}
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> core::ops::Div<&Voltage<T>> for Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Conductance{S: self.A / rhs.V.clone()}
	}
}
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> core::ops::Div<&Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Conductance{S: self.A.clone() / rhs.V.clone()}
	}
}

// Current / Frequency -> Charge
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> core::ops::Div<Frequency<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Charge{C: self.A / rhs.Hz}
	}
}
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> core::ops::Div<Frequency<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Charge{C: self.A.clone() / rhs.Hz}
	}
}
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> core::ops::Div<&Frequency<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Charge{C: self.A / rhs.Hz.clone()}
	}
}
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> core::ops::Div<&Frequency<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Charge{C: self.A.clone() / rhs.Hz.clone()}
	}
}

/// The distance (aka length) unit type, defined as meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Distance<T: NumLike>{
	/// The value of this Distance in meters
	pub m: T
}

impl<T> Distance<T> where T: NumLike {

	/// Returns the standard unit name of distance: "meters"
	pub fn unit_name() -> &'static str { "meters" }
	
	/// Returns the abbreviated name or symbol of distance: "m" for meters
	pub fn unit_symbol() -> &'static str { "m" }
	
	/// Returns a new distance value from the given number of meters
	///
	/// # Arguments
	/// * `m` - Any number-like type, representing a quantity of meters
	pub fn from_m(m: T) -> Self { Distance{m: m} }
	
	/// Returns a copy of this distance value in meters
	pub fn to_m(&self) -> T { self.m.clone() }

	/// Returns a new distance value from the given number of meters
	///
	/// # Arguments
	/// * `meters` - Any number-like type, representing a quantity of meters
	pub fn from_meters(meters: T) -> Self { Distance{m: meters} }
	
	/// Returns a copy of this distance value in meters
	pub fn to_meters(&self) -> T { self.m.clone() }

}

impl<T> fmt::Display for Distance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m, Self::unit_symbol())
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this distance value in millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_cm(&self) -> T {
		return self.m.clone() * T::from(100.0_f64);
	}

	/// Returns a new distance value from the given number of millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `cm` - Any number-like type, representing a quantity of millimeters
	pub fn from_cm(cm: T) -> Self {
		Distance{m: cm * T::from(0.01_f64)}
	}

	/// Returns a copy of this distance value in millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mm(&self) -> T {
		return self.m.clone() * T::from(1000.0_f64);
	}

	/// Returns a new distance value from the given number of millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mm` - Any number-like type, representing a quantity of millimeters
	pub fn from_mm(mm: T) -> Self {
		Distance{m: mm * T::from(0.001_f64)}
	}

	/// Returns a copy of this distance value in micrometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_um(&self) -> T {
		return self.m.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new distance value from the given number of micrometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `um` - Any number-like type, representing a quantity of micrometers
	pub fn from_um(um: T) -> Self {
		Distance{m: um * T::from(1e-06_f64)}
	}

	/// Returns a copy of this distance value in nanometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nm(&self) -> T {
		return self.m.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new distance value from the given number of nanometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nm` - Any number-like type, representing a quantity of nanometers
	pub fn from_nm(nm: T) -> Self {
		Distance{m: nm * T::from(1e-09_f64)}
	}

	/// Returns a copy of this distance value in picometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_pm(&self) -> T {
		return self.m.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new distance value from the given number of picometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `pm` - Any number-like type, representing a quantity of picometers
	pub fn from_pm(pm: T) -> Self {
		Distance{m: pm * T::from(1e-12_f64)}
	}

	/// Returns a copy of this distance value in kilometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_km(&self) -> T {
		return self.m.clone() * T::from(0.001_f64);
	}

	/// Returns a new distance value from the given number of kilometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `km` - Any number-like type, representing a quantity of kilometers
	pub fn from_km(km: T) -> Self {
		Distance{m: km * T::from(1000.0_f64)}
	}

	/// Returns a copy of this distance value in astronomical units
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_au(&self) -> T {
		return self.m.clone() * T::from(6.68458712226845e-12_f64);
	}

	/// Returns a new distance value from the given number of astronomical units
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `au` - Any number-like type, representing a quantity of astronomical units
	pub fn from_au(au: T) -> Self {
		Distance{m: au * T::from(149597870700.0_f64)}
	}

	/// Returns a copy of this distance value in parsecs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_parsec(&self) -> T {
		return self.m.clone() * T::from(3.24077624525171e-17_f64);
	}

	/// Returns a new distance value from the given number of parsecs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `parsec` - Any number-like type, representing a quantity of parsecs
	pub fn from_parsec(parsec: T) -> Self {
		Distance{m: parsec * T::from(3.08568047999355e+16_f64)}
	}

	/// Returns a copy of this distance value in light-years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_lyr(&self) -> T {
		return self.m.clone() * T::from(1.05702343681763e-16_f64);
	}

	/// Returns a new distance value from the given number of light-years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `lyr` - Any number-like type, representing a quantity of light-years
	pub fn from_lyr(lyr: T) -> Self {
		Distance{m: lyr * T::from(9460528169656200.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Distance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Distance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Distance<num_bigfloat::BigFloat>) -> Self::Output {
		Distance{m: self * rhs.m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Distance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Distance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Distance<num_bigfloat::BigFloat>) -> Self::Output {
		Distance{m: self.clone() * rhs.m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Distance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Distance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Distance<num_bigfloat::BigFloat>) -> Self::Output {
		Distance{m: self * rhs.m.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Distance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Distance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Distance<num_bigfloat::BigFloat>) -> Self::Output {
		Distance{m: self.clone() * rhs.m.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Distance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Distance<num_complex::Complex32>;
	fn mul(self, rhs: Distance<num_complex::Complex32>) -> Self::Output {
		Distance{m: self * rhs.m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Distance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Distance<num_complex::Complex32>;
	fn mul(self, rhs: Distance<num_complex::Complex32>) -> Self::Output {
		Distance{m: self.clone() * rhs.m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Distance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Distance<num_complex::Complex32>;
	fn mul(self, rhs: &Distance<num_complex::Complex32>) -> Self::Output {
		Distance{m: self * rhs.m.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Distance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Distance<num_complex::Complex32>;
	fn mul(self, rhs: &Distance<num_complex::Complex32>) -> Self::Output {
		Distance{m: self.clone() * rhs.m.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Distance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Distance<num_complex::Complex64>;
	fn mul(self, rhs: Distance<num_complex::Complex64>) -> Self::Output {
		Distance{m: self * rhs.m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Distance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Distance<num_complex::Complex64>;
	fn mul(self, rhs: Distance<num_complex::Complex64>) -> Self::Output {
		Distance{m: self.clone() * rhs.m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Distance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Distance<num_complex::Complex64>;
	fn mul(self, rhs: &Distance<num_complex::Complex64>) -> Self::Output {
		Distance{m: self * rhs.m.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Distance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Distance<num_complex::Complex64>;
	fn mul(self, rhs: &Distance<num_complex::Complex64>) -> Self::Output {
		Distance{m: self.clone() * rhs.m.clone()}
	}
}



/// Converts a Distance into the equivalent [uom](https://crates.io/crates/uom) type [Length](https://docs.rs/uom/0.34.0/uom/si/f32/type.Length.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Length> for Distance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Length {
		uom::si::f32::Length::new::<uom::si::length::meter>(self.m.into())
	}
}

/// Creates a Distance from the equivalent [uom](https://crates.io/crates/uom) type [Length](https://docs.rs/uom/0.34.0/uom/si/f32/type.Length.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Length> for Distance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Length) -> Self {
		Distance{m: T::from(src.value)}
	}
}

/// Converts a Distance into the equivalent [uom](https://crates.io/crates/uom) type [Length](https://docs.rs/uom/0.34.0/uom/si/f64/type.Length.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Length> for Distance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Length {
		uom::si::f64::Length::new::<uom::si::length::meter>(self.m.into())
	}
}

/// Creates a Distance from the equivalent [uom](https://crates.io/crates/uom) type [Length](https://docs.rs/uom/0.34.0/uom/si/f64/type.Length.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Length> for Distance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Length) -> Self {
		Distance{m: T::from(src.value)}
	}
}


// Distance * Distance -> Area
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> core::ops::Mul<Distance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m * rhs.m}
	}
}
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> core::ops::Mul<Distance<T>> for &Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m.clone() * rhs.m}
	}
}
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> core::ops::Mul<&Distance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m * rhs.m.clone()}
	}
}
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> core::ops::Mul<&Distance<T>> for &Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m.clone() * rhs.m.clone()}
	}
}

// Distance / Time -> Velocity
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> core::ops::Div<Time<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.m / rhs.s}
	}
}
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> core::ops::Div<Time<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.m.clone() / rhs.s}
	}
}
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> core::ops::Div<&Time<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Velocity{mps: self.m / rhs.s.clone()}
	}
}
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> core::ops::Div<&Time<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Velocity{mps: self.m.clone() / rhs.s.clone()}
	}
}

// Distance * Area -> Volume
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> core::ops::Mul<Area<T>> for Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Volume{m3: self.m * rhs.m2}
	}
}
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> core::ops::Mul<Area<T>> for &Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Volume{m3: self.m.clone() * rhs.m2}
	}
}
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> core::ops::Mul<&Area<T>> for Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Volume{m3: self.m * rhs.m2.clone()}
	}
}
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> core::ops::Mul<&Area<T>> for &Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Volume{m3: self.m.clone() * rhs.m2.clone()}
	}
}

// Distance * Density -> AreaDensity
/// Multiplying a Distance by a Density returns a value of type AreaDensity
impl<T> core::ops::Mul<Density<T>> for Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m * rhs.kgpm3}
	}
}
/// Multiplying a Distance by a Density returns a value of type AreaDensity
impl<T> core::ops::Mul<Density<T>> for &Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m.clone() * rhs.kgpm3}
	}
}
/// Multiplying a Distance by a Density returns a value of type AreaDensity
impl<T> core::ops::Mul<&Density<T>> for Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m * rhs.kgpm3.clone()}
	}
}
/// Multiplying a Distance by a Density returns a value of type AreaDensity
impl<T> core::ops::Mul<&Density<T>> for &Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m.clone() * rhs.kgpm3.clone()}
	}
}

// Distance * Force -> Energy
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> core::ops::Mul<Force<T>> for Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Energy{J: self.m * rhs.N}
	}
}
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> core::ops::Mul<Force<T>> for &Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Energy{J: self.m.clone() * rhs.N}
	}
}
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> core::ops::Mul<&Force<T>> for Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Energy{J: self.m * rhs.N.clone()}
	}
}
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> core::ops::Mul<&Force<T>> for &Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Energy{J: self.m.clone() * rhs.N.clone()}
	}
}

// Distance * Frequency -> Velocity
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> core::ops::Mul<Frequency<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Velocity{mps: self.m * rhs.Hz}
	}
}
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> core::ops::Mul<Frequency<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Velocity{mps: self.m.clone() * rhs.Hz}
	}
}
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> core::ops::Mul<&Frequency<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Velocity{mps: self.m * rhs.Hz.clone()}
	}
}
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> core::ops::Mul<&Frequency<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Velocity{mps: self.m.clone() * rhs.Hz.clone()}
	}
}

// Distance / Velocity -> Time
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> core::ops::Div<Velocity<T>> for Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Time{s: self.m / rhs.mps}
	}
}
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> core::ops::Div<Velocity<T>> for &Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Time{s: self.m.clone() / rhs.mps}
	}
}
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> core::ops::Div<&Velocity<T>> for Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Time{s: self.m / rhs.mps.clone()}
	}
}
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> core::ops::Div<&Velocity<T>> for &Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Time{s: self.m.clone() / rhs.mps.clone()}
	}
}

/// The luminosity unit type, defined as candela in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Luminosity<T: NumLike>{
	/// The value of this Luminosity in candela
	pub cd: T
}

impl<T> Luminosity<T> where T: NumLike {

	/// Returns the standard unit name of luminosity: "candela"
	pub fn unit_name() -> &'static str { "candela" }
	
	/// Returns the abbreviated name or symbol of luminosity: "cd" for candela
	pub fn unit_symbol() -> &'static str { "cd" }
	
	/// Returns a new luminosity value from the given number of candela
	///
	/// # Arguments
	/// * `cd` - Any number-like type, representing a quantity of candela
	pub fn from_cd(cd: T) -> Self { Luminosity{cd: cd} }
	
	/// Returns a copy of this luminosity value in candela
	pub fn to_cd(&self) -> T { self.cd.clone() }

	/// Returns a new luminosity value from the given number of candela
	///
	/// # Arguments
	/// * `candela` - Any number-like type, representing a quantity of candela
	pub fn from_candela(candela: T) -> Self { Luminosity{cd: candela} }
	
	/// Returns a copy of this luminosity value in candela
	pub fn to_candela(&self) -> T { self.cd.clone() }

}

impl<T> fmt::Display for Luminosity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.cd, Self::unit_symbol())
	}
}

impl<T> Luminosity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this luminosity value in millicandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mcd(&self) -> T {
		return self.cd.clone() * T::from(1000.0_f64);
	}

	/// Returns a new luminosity value from the given number of millicandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mcd` - Any number-like type, representing a quantity of millicandela
	pub fn from_mcd(mcd: T) -> Self {
		Luminosity{cd: mcd * T::from(0.001_f64)}
	}

	/// Returns a copy of this luminosity value in microcandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ucd(&self) -> T {
		return self.cd.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new luminosity value from the given number of microcandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ucd` - Any number-like type, representing a quantity of microcandela
	pub fn from_ucd(ucd: T) -> Self {
		Luminosity{cd: ucd * T::from(1e-06_f64)}
	}

	/// Returns a copy of this luminosity value in nanocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ncd(&self) -> T {
		return self.cd.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new luminosity value from the given number of nanocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ncd` - Any number-like type, representing a quantity of nanocandela
	pub fn from_ncd(ncd: T) -> Self {
		Luminosity{cd: ncd * T::from(1e-09_f64)}
	}

	/// Returns a copy of this luminosity value in kilocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kcd(&self) -> T {
		return self.cd.clone() * T::from(0.001_f64);
	}

	/// Returns a new luminosity value from the given number of kilocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kcd` - Any number-like type, representing a quantity of kilocandela
	pub fn from_kcd(kcd: T) -> Self {
		Luminosity{cd: kcd * T::from(1000.0_f64)}
	}

	/// Returns a copy of this luminosity value in megacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Mcd(&self) -> T {
		return self.cd.clone() * T::from(1e-06_f64);
	}

	/// Returns a new luminosity value from the given number of megacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Mcd` - Any number-like type, representing a quantity of megacandela
	pub fn from_Mcd(Mcd: T) -> Self {
		Luminosity{cd: Mcd * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this luminosity value in gigacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Gcd(&self) -> T {
		return self.cd.clone() * T::from(1e-09_f64);
	}

	/// Returns a new luminosity value from the given number of gigacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Gcd` - Any number-like type, representing a quantity of gigacandela
	pub fn from_Gcd(Gcd: T) -> Self {
		Luminosity{cd: Gcd * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Luminosity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Luminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Luminosity<num_bigfloat::BigFloat>) -> Self::Output {
		Luminosity{cd: self * rhs.cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Luminosity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Luminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Luminosity<num_bigfloat::BigFloat>) -> Self::Output {
		Luminosity{cd: self.clone() * rhs.cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Luminosity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Luminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Luminosity<num_bigfloat::BigFloat>) -> Self::Output {
		Luminosity{cd: self * rhs.cd.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Luminosity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Luminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Luminosity<num_bigfloat::BigFloat>) -> Self::Output {
		Luminosity{cd: self.clone() * rhs.cd.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Luminosity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Luminosity<num_complex::Complex32>;
	fn mul(self, rhs: Luminosity<num_complex::Complex32>) -> Self::Output {
		Luminosity{cd: self * rhs.cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Luminosity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Luminosity<num_complex::Complex32>;
	fn mul(self, rhs: Luminosity<num_complex::Complex32>) -> Self::Output {
		Luminosity{cd: self.clone() * rhs.cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Luminosity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Luminosity<num_complex::Complex32>;
	fn mul(self, rhs: &Luminosity<num_complex::Complex32>) -> Self::Output {
		Luminosity{cd: self * rhs.cd.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Luminosity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Luminosity<num_complex::Complex32>;
	fn mul(self, rhs: &Luminosity<num_complex::Complex32>) -> Self::Output {
		Luminosity{cd: self.clone() * rhs.cd.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Luminosity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Luminosity<num_complex::Complex64>;
	fn mul(self, rhs: Luminosity<num_complex::Complex64>) -> Self::Output {
		Luminosity{cd: self * rhs.cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Luminosity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Luminosity<num_complex::Complex64>;
	fn mul(self, rhs: Luminosity<num_complex::Complex64>) -> Self::Output {
		Luminosity{cd: self.clone() * rhs.cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Luminosity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Luminosity<num_complex::Complex64>;
	fn mul(self, rhs: &Luminosity<num_complex::Complex64>) -> Self::Output {
		Luminosity{cd: self * rhs.cd.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Luminosity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Luminosity<num_complex::Complex64>;
	fn mul(self, rhs: &Luminosity<num_complex::Complex64>) -> Self::Output {
		Luminosity{cd: self.clone() * rhs.cd.clone()}
	}
}



/// Converts a Luminosity into the equivalent [uom](https://crates.io/crates/uom) type [LuminousIntensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.LuminousIntensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::LuminousIntensity> for Luminosity<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::LuminousIntensity {
		uom::si::f32::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(self.cd.into())
	}
}

/// Creates a Luminosity from the equivalent [uom](https://crates.io/crates/uom) type [LuminousIntensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.LuminousIntensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::LuminousIntensity> for Luminosity<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::LuminousIntensity) -> Self {
		Luminosity{cd: T::from(src.value)}
	}
}

/// Converts a Luminosity into the equivalent [uom](https://crates.io/crates/uom) type [LuminousIntensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.LuminousIntensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::LuminousIntensity> for Luminosity<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::LuminousIntensity {
		uom::si::f64::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(self.cd.into())
	}
}

/// Creates a Luminosity from the equivalent [uom](https://crates.io/crates/uom) type [LuminousIntensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.LuminousIntensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::LuminousIntensity> for Luminosity<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::LuminousIntensity) -> Self {
		Luminosity{cd: T::from(src.value)}
	}
}


// Luminosity * SolidAngle -> LuminousFlux
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Mul<SolidAngle<T>> for Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd * rhs.sr}
	}
}
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Mul<SolidAngle<T>> for &Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd.clone() * rhs.sr}
	}
}
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Mul<&SolidAngle<T>> for Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd * rhs.sr.clone()}
	}
}
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Mul<&SolidAngle<T>> for &Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd.clone() * rhs.sr.clone()}
	}
}

/// The mass unit type, defined as kilograms in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Mass<T: NumLike>{
	/// The value of this Mass in kilograms
	pub kg: T
}

impl<T> Mass<T> where T: NumLike {

	/// Returns the standard unit name of mass: "kilograms"
	pub fn unit_name() -> &'static str { "kilograms" }
	
	/// Returns the abbreviated name or symbol of mass: "kg" for kilograms
	pub fn unit_symbol() -> &'static str { "kg" }
	
	/// Returns a new mass value from the given number of kilograms
	///
	/// # Arguments
	/// * `kg` - Any number-like type, representing a quantity of kilograms
	pub fn from_kg(kg: T) -> Self { Mass{kg: kg} }
	
	/// Returns a copy of this mass value in kilograms
	pub fn to_kg(&self) -> T { self.kg.clone() }

	/// Returns a new mass value from the given number of kilograms
	///
	/// # Arguments
	/// * `kilograms` - Any number-like type, representing a quantity of kilograms
	pub fn from_kilograms(kilograms: T) -> Self { Mass{kg: kilograms} }
	
	/// Returns a copy of this mass value in kilograms
	pub fn to_kilograms(&self) -> T { self.kg.clone() }

}

impl<T> fmt::Display for Mass<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kg, Self::unit_symbol())
	}
}

impl<T> Mass<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this mass value in grams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_g(&self) -> T {
		return self.kg.clone() * T::from(1000.0_f64);
	}

	/// Returns a new mass value from the given number of grams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `g` - Any number-like type, representing a quantity of grams
	pub fn from_g(g: T) -> Self {
		Mass{kg: g * T::from(0.001_f64)}
	}

	/// Returns a copy of this mass value in milligrams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mg(&self) -> T {
		return self.kg.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new mass value from the given number of milligrams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mg` - Any number-like type, representing a quantity of milligrams
	pub fn from_mg(mg: T) -> Self {
		Mass{kg: mg * T::from(1e-06_f64)}
	}

	/// Returns a copy of this mass value in micrograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ug(&self) -> T {
		return self.kg.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new mass value from the given number of micrograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ug` - Any number-like type, representing a quantity of micrograms
	pub fn from_ug(ug: T) -> Self {
		Mass{kg: ug * T::from(1e-09_f64)}
	}

	/// Returns a copy of this mass value in nanograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ng(&self) -> T {
		return self.kg.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new mass value from the given number of nanograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ng` - Any number-like type, representing a quantity of nanograms
	pub fn from_ng(ng: T) -> Self {
		Mass{kg: ng * T::from(1e-12_f64)}
	}

	/// Returns a copy of this mass value in picograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_pg(&self) -> T {
		return self.kg.clone() * T::from(1000000000000000.0_f64);
	}

	/// Returns a new mass value from the given number of picograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `pg` - Any number-like type, representing a quantity of picograms
	pub fn from_pg(pg: T) -> Self {
		Mass{kg: pg * T::from(1e-15_f64)}
	}

	/// Returns a copy of this mass value in tons
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_tons(&self) -> T {
		return self.kg.clone() * T::from(0.001_f64);
	}

	/// Returns a new mass value from the given number of tons
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `tons` - Any number-like type, representing a quantity of tons
	pub fn from_tons(tons: T) -> Self {
		Mass{kg: tons * T::from(1000.0_f64)}
	}

	/// Returns a copy of this mass value in earth masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_earth_mass(&self) -> T {
		return self.kg.clone() * T::from(1.6744248350691502e-25_f64);
	}

	/// Returns a new mass value from the given number of earth masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `earth_mass` - Any number-like type, representing a quantity of earth masses
	pub fn from_earth_mass(earth_mass: T) -> Self {
		Mass{kg: earth_mass * T::from(5.9722e+24_f64)}
	}

	/// Returns a copy of this mass value in jupiter masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_jupiter_mass(&self) -> T {
		return self.kg.clone() * T::from(5.26703887074687e-28_f64);
	}

	/// Returns a new mass value from the given number of jupiter masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `jupiter_mass` - Any number-like type, representing a quantity of jupiter masses
	pub fn from_jupiter_mass(jupiter_mass: T) -> Self {
		Mass{kg: jupiter_mass * T::from(1.8986e+27_f64)}
	}

	/// Returns a copy of this mass value in solar masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_solar_mass(&self) -> T {
		return self.kg.clone() * T::from(5.0287898217294e-31_f64);
	}

	/// Returns a new mass value from the given number of solar masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `solar_mass` - Any number-like type, representing a quantity of solar masses
	pub fn from_solar_mass(solar_mass: T) -> Self {
		Mass{kg: solar_mass * T::from(1.9885500000000002e+30_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Mass<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Mass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Mass<num_bigfloat::BigFloat>) -> Self::Output {
		Mass{kg: self * rhs.kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Mass<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Mass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Mass<num_bigfloat::BigFloat>) -> Self::Output {
		Mass{kg: self.clone() * rhs.kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Mass<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Mass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Mass<num_bigfloat::BigFloat>) -> Self::Output {
		Mass{kg: self * rhs.kg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Mass<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Mass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Mass<num_bigfloat::BigFloat>) -> Self::Output {
		Mass{kg: self.clone() * rhs.kg.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Mass<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Mass<num_complex::Complex32>;
	fn mul(self, rhs: Mass<num_complex::Complex32>) -> Self::Output {
		Mass{kg: self * rhs.kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Mass<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Mass<num_complex::Complex32>;
	fn mul(self, rhs: Mass<num_complex::Complex32>) -> Self::Output {
		Mass{kg: self.clone() * rhs.kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Mass<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Mass<num_complex::Complex32>;
	fn mul(self, rhs: &Mass<num_complex::Complex32>) -> Self::Output {
		Mass{kg: self * rhs.kg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Mass<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Mass<num_complex::Complex32>;
	fn mul(self, rhs: &Mass<num_complex::Complex32>) -> Self::Output {
		Mass{kg: self.clone() * rhs.kg.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Mass<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Mass<num_complex::Complex64>;
	fn mul(self, rhs: Mass<num_complex::Complex64>) -> Self::Output {
		Mass{kg: self * rhs.kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Mass<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Mass<num_complex::Complex64>;
	fn mul(self, rhs: Mass<num_complex::Complex64>) -> Self::Output {
		Mass{kg: self.clone() * rhs.kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Mass<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Mass<num_complex::Complex64>;
	fn mul(self, rhs: &Mass<num_complex::Complex64>) -> Self::Output {
		Mass{kg: self * rhs.kg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Mass<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Mass<num_complex::Complex64>;
	fn mul(self, rhs: &Mass<num_complex::Complex64>) -> Self::Output {
		Mass{kg: self.clone() * rhs.kg.clone()}
	}
}



/// Converts a Mass into the equivalent [uom](https://crates.io/crates/uom) type [Mass](https://docs.rs/uom/0.34.0/uom/si/f32/type.Mass.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Mass> for Mass<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Mass {
		uom::si::f32::Mass::new::<uom::si::mass::kilogram>(self.kg.into())
	}
}

/// Creates a Mass from the equivalent [uom](https://crates.io/crates/uom) type [Mass](https://docs.rs/uom/0.34.0/uom/si/f32/type.Mass.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Mass> for Mass<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Mass) -> Self {
		Mass{kg: T::from(src.value)}
	}
}

/// Converts a Mass into the equivalent [uom](https://crates.io/crates/uom) type [Mass](https://docs.rs/uom/0.34.0/uom/si/f64/type.Mass.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Mass> for Mass<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Mass {
		uom::si::f64::Mass::new::<uom::si::mass::kilogram>(self.kg.into())
	}
}

/// Creates a Mass from the equivalent [uom](https://crates.io/crates/uom) type [Mass](https://docs.rs/uom/0.34.0/uom/si/f64/type.Mass.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Mass> for Mass<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Mass) -> Self {
		Mass{kg: T::from(src.value)}
	}
}


// Mass / Area -> AreaDensity
/// Dividing a Mass by a Area returns a value of type AreaDensity
impl<T> core::ops::Div<Area<T>> for Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg / rhs.m2}
	}
}
/// Dividing a Mass by a Area returns a value of type AreaDensity
impl<T> core::ops::Div<Area<T>> for &Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg.clone() / rhs.m2}
	}
}
/// Dividing a Mass by a Area returns a value of type AreaDensity
impl<T> core::ops::Div<&Area<T>> for Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg / rhs.m2.clone()}
	}
}
/// Dividing a Mass by a Area returns a value of type AreaDensity
impl<T> core::ops::Div<&Area<T>> for &Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg.clone() / rhs.m2.clone()}
	}
}

// Mass / Volume -> Density
/// Dividing a Mass by a Volume returns a value of type Density
impl<T> core::ops::Div<Volume<T>> for Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Density{kgpm3: self.kg / rhs.m3}
	}
}
/// Dividing a Mass by a Volume returns a value of type Density
impl<T> core::ops::Div<Volume<T>> for &Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Density{kgpm3: self.kg.clone() / rhs.m3}
	}
}
/// Dividing a Mass by a Volume returns a value of type Density
impl<T> core::ops::Div<&Volume<T>> for Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Density{kgpm3: self.kg / rhs.m3.clone()}
	}
}
/// Dividing a Mass by a Volume returns a value of type Density
impl<T> core::ops::Div<&Volume<T>> for &Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Density{kgpm3: self.kg.clone() / rhs.m3.clone()}
	}
}

// Mass * Acceleration -> Force
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> core::ops::Mul<Acceleration<T>> for Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Force{N: self.kg * rhs.mps2}
	}
}
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> core::ops::Mul<Acceleration<T>> for &Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Force{N: self.kg.clone() * rhs.mps2}
	}
}
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> core::ops::Mul<&Acceleration<T>> for Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Force{N: self.kg * rhs.mps2.clone()}
	}
}
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> core::ops::Mul<&Acceleration<T>> for &Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Force{N: self.kg.clone() * rhs.mps2.clone()}
	}
}

// Mass / AreaDensity -> Area
/// Dividing a Mass by a AreaDensity returns a value of type Area
impl<T> core::ops::Div<AreaDensity<T>> for Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		Area{m2: self.kg / rhs.kgpm2}
	}
}
/// Dividing a Mass by a AreaDensity returns a value of type Area
impl<T> core::ops::Div<AreaDensity<T>> for &Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		Area{m2: self.kg.clone() / rhs.kgpm2}
	}
}
/// Dividing a Mass by a AreaDensity returns a value of type Area
impl<T> core::ops::Div<&AreaDensity<T>> for Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		Area{m2: self.kg / rhs.kgpm2.clone()}
	}
}
/// Dividing a Mass by a AreaDensity returns a value of type Area
impl<T> core::ops::Div<&AreaDensity<T>> for &Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		Area{m2: self.kg.clone() / rhs.kgpm2.clone()}
	}
}

// Mass / Density -> Volume
/// Dividing a Mass by a Density returns a value of type Volume
impl<T> core::ops::Div<Density<T>> for Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		Volume{m3: self.kg / rhs.kgpm3}
	}
}
/// Dividing a Mass by a Density returns a value of type Volume
impl<T> core::ops::Div<Density<T>> for &Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		Volume{m3: self.kg.clone() / rhs.kgpm3}
	}
}
/// Dividing a Mass by a Density returns a value of type Volume
impl<T> core::ops::Div<&Density<T>> for Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		Volume{m3: self.kg / rhs.kgpm3.clone()}
	}
}
/// Dividing a Mass by a Density returns a value of type Volume
impl<T> core::ops::Div<&Density<T>> for &Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		Volume{m3: self.kg.clone() / rhs.kgpm3.clone()}
	}
}

// Mass * Velocity -> Momentum
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> core::ops::Mul<Velocity<T>> for Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg * rhs.mps}
	}
}
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> core::ops::Mul<Velocity<T>> for &Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg.clone() * rhs.mps}
	}
}
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> core::ops::Mul<&Velocity<T>> for Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg * rhs.mps.clone()}
	}
}
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> core::ops::Mul<&Velocity<T>> for &Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg.clone() * rhs.mps.clone()}
	}
}

// Mass * AbsorbedDose -> Energy
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> core::ops::Mul<AbsorbedDose<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Gy}
	}
}
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> core::ops::Mul<AbsorbedDose<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Gy}
	}
}
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> core::ops::Mul<&AbsorbedDose<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Gy.clone()}
	}
}
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> core::ops::Mul<&AbsorbedDose<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Gy.clone()}
	}
}

// Mass * DoseEquivalent -> Energy
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> core::ops::Mul<DoseEquivalent<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Sv}
	}
}
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> core::ops::Mul<DoseEquivalent<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Sv}
	}
}
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> core::ops::Mul<&DoseEquivalent<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Sv.clone()}
	}
}
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> core::ops::Mul<&DoseEquivalent<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Sv.clone()}
	}
}

/// The temperature unit type, defined as degrees kelvin in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Temperature<T: NumLike>{
	/// The value of this Temperature in degrees kelvin
	pub K: T
}

impl<T> Temperature<T> where T: NumLike {

	/// Returns the standard unit name of temperature: "degrees kelvin"
	pub fn unit_name() -> &'static str { "degrees kelvin" }
	
	/// Returns the abbreviated name or symbol of temperature: "K" for degrees kelvin
	pub fn unit_symbol() -> &'static str { "K" }
	
	/// Returns a new temperature value from the given number of degrees kelvin
	///
	/// # Arguments
	/// * `K` - Any number-like type, representing a quantity of degrees kelvin
	pub fn from_K(K: T) -> Self { Temperature{K: K} }
	
	/// Returns a copy of this temperature value in degrees kelvin
	pub fn to_K(&self) -> T { self.K.clone() }

}

impl<T> fmt::Display for Temperature<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.K, Self::unit_symbol())
	}
}

impl<T> Temperature<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this temperature value in degrees celsius
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_C(&self) -> T {
		return (self.K.clone() * T::from(1.0_f64)) - T::from(273.15_f64);
	}

	/// Returns a new temperature value from the given number of degrees celsius
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `C` - Any number-like type, representing a quantity of degrees celsius
	pub fn from_C(C: T) -> Self {
		Temperature{K: (C + T::from(273.15_f64)) * T::from(1.0_f64)}
	}

	/// Returns a copy of this temperature value in degrees celsius
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_celsius(&self) -> T {
		return (self.K.clone() * T::from(1.0_f64)) - T::from(273.15_f64);
	}

	/// Returns a new temperature value from the given number of degrees celsius
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `celsius` - Any number-like type, representing a quantity of degrees celsius
	pub fn from_celsius(celsius: T) -> Self {
		Temperature{K: (celsius + T::from(273.15_f64)) * T::from(1.0_f64)}
	}

	/// Returns a copy of this temperature value in degrees fahrenheit
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_F(&self) -> T {
		return (self.K.clone() * T::from(1.8_f64)) - T::from(459.67_f64);
	}

	/// Returns a new temperature value from the given number of degrees fahrenheit
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `F` - Any number-like type, representing a quantity of degrees fahrenheit
	pub fn from_F(F: T) -> Self {
		Temperature{K: (F + T::from(459.67_f64)) * T::from(0.555555555555556_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Temperature<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Temperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Temperature<num_bigfloat::BigFloat>) -> Self::Output {
		Temperature{K: self * rhs.K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Temperature<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Temperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Temperature<num_bigfloat::BigFloat>) -> Self::Output {
		Temperature{K: self.clone() * rhs.K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Temperature<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Temperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Temperature<num_bigfloat::BigFloat>) -> Self::Output {
		Temperature{K: self * rhs.K.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Temperature<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Temperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Temperature<num_bigfloat::BigFloat>) -> Self::Output {
		Temperature{K: self.clone() * rhs.K.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Temperature<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Temperature<num_complex::Complex32>;
	fn mul(self, rhs: Temperature<num_complex::Complex32>) -> Self::Output {
		Temperature{K: self * rhs.K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Temperature<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Temperature<num_complex::Complex32>;
	fn mul(self, rhs: Temperature<num_complex::Complex32>) -> Self::Output {
		Temperature{K: self.clone() * rhs.K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Temperature<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Temperature<num_complex::Complex32>;
	fn mul(self, rhs: &Temperature<num_complex::Complex32>) -> Self::Output {
		Temperature{K: self * rhs.K.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Temperature<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Temperature<num_complex::Complex32>;
	fn mul(self, rhs: &Temperature<num_complex::Complex32>) -> Self::Output {
		Temperature{K: self.clone() * rhs.K.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Temperature<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Temperature<num_complex::Complex64>;
	fn mul(self, rhs: Temperature<num_complex::Complex64>) -> Self::Output {
		Temperature{K: self * rhs.K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Temperature<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Temperature<num_complex::Complex64>;
	fn mul(self, rhs: Temperature<num_complex::Complex64>) -> Self::Output {
		Temperature{K: self.clone() * rhs.K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Temperature<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Temperature<num_complex::Complex64>;
	fn mul(self, rhs: &Temperature<num_complex::Complex64>) -> Self::Output {
		Temperature{K: self * rhs.K.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Temperature<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Temperature<num_complex::Complex64>;
	fn mul(self, rhs: &Temperature<num_complex::Complex64>) -> Self::Output {
		Temperature{K: self.clone() * rhs.K.clone()}
	}
}



/// Converts a Temperature into the equivalent [uom](https://crates.io/crates/uom) type [ThermodynamicTemperature](https://docs.rs/uom/0.34.0/uom/si/f32/type.ThermodynamicTemperature.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ThermodynamicTemperature> for Temperature<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ThermodynamicTemperature {
		uom::si::f32::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(self.K.into())
	}
}

/// Creates a Temperature from the equivalent [uom](https://crates.io/crates/uom) type [ThermodynamicTemperature](https://docs.rs/uom/0.34.0/uom/si/f32/type.ThermodynamicTemperature.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ThermodynamicTemperature> for Temperature<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ThermodynamicTemperature) -> Self {
		Temperature{K: T::from(src.value)}
	}
}

/// Converts a Temperature into the equivalent [uom](https://crates.io/crates/uom) type [ThermodynamicTemperature](https://docs.rs/uom/0.34.0/uom/si/f64/type.ThermodynamicTemperature.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ThermodynamicTemperature> for Temperature<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ThermodynamicTemperature {
		uom::si::f64::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(self.K.into())
	}
}

/// Creates a Temperature from the equivalent [uom](https://crates.io/crates/uom) type [ThermodynamicTemperature](https://docs.rs/uom/0.34.0/uom/si/f64/type.ThermodynamicTemperature.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ThermodynamicTemperature> for Temperature<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ThermodynamicTemperature) -> Self {
		Temperature{K: T::from(src.value)}
	}
}


/// The time unit type, defined as seconds in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Time<T: NumLike>{
	/// The value of this Time in seconds
	pub s: T
}

impl<T> Time<T> where T: NumLike {

	/// Returns the standard unit name of time: "seconds"
	pub fn unit_name() -> &'static str { "seconds" }
	
	/// Returns the abbreviated name or symbol of time: "s" for seconds
	pub fn unit_symbol() -> &'static str { "s" }
	
	/// Returns a new time value from the given number of seconds
	///
	/// # Arguments
	/// * `s` - Any number-like type, representing a quantity of seconds
	pub fn from_s(s: T) -> Self { Time{s: s} }
	
	/// Returns a copy of this time value in seconds
	pub fn to_s(&self) -> T { self.s.clone() }

	/// Returns a new time value from the given number of seconds
	///
	/// # Arguments
	/// * `seconds` - Any number-like type, representing a quantity of seconds
	pub fn from_seconds(seconds: T) -> Self { Time{s: seconds} }
	
	/// Returns a copy of this time value in seconds
	pub fn to_seconds(&self) -> T { self.s.clone() }

}

impl<T> fmt::Display for Time<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.s, Self::unit_symbol())
	}
}

impl<T> Time<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this time value in milliseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ms(&self) -> T {
		return self.s.clone() * T::from(1000.0_f64);
	}

	/// Returns a new time value from the given number of milliseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ms` - Any number-like type, representing a quantity of milliseconds
	pub fn from_ms(ms: T) -> Self {
		Time{s: ms * T::from(0.001_f64)}
	}

	/// Returns a copy of this time value in microseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_us(&self) -> T {
		return self.s.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new time value from the given number of microseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `us` - Any number-like type, representing a quantity of microseconds
	pub fn from_us(us: T) -> Self {
		Time{s: us * T::from(1e-06_f64)}
	}

	/// Returns a copy of this time value in nanoseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ns(&self) -> T {
		return self.s.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new time value from the given number of nanoseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ns` - Any number-like type, representing a quantity of nanoseconds
	pub fn from_ns(ns: T) -> Self {
		Time{s: ns * T::from(1e-09_f64)}
	}

	/// Returns a copy of this time value in picoseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ps(&self) -> T {
		return self.s.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new time value from the given number of picoseconds
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ps` - Any number-like type, representing a quantity of picoseconds
	pub fn from_ps(ps: T) -> Self {
		Time{s: ps * T::from(1e-12_f64)}
	}

	/// Returns a copy of this time value in minutes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_min(&self) -> T {
		return self.s.clone() * T::from(0.0166666666666667_f64);
	}

	/// Returns a new time value from the given number of minutes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `min` - Any number-like type, representing a quantity of minutes
	pub fn from_min(min: T) -> Self {
		Time{s: min * T::from(60.0_f64)}
	}

	/// Returns a copy of this time value in hours
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_hr(&self) -> T {
		return self.s.clone() * T::from(0.0002777777777777_f64);
	}

	/// Returns a new time value from the given number of hours
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `hr` - Any number-like type, representing a quantity of hours
	pub fn from_hr(hr: T) -> Self {
		Time{s: hr * T::from(3600.0_f64)}
	}

	/// Returns a copy of this time value in days
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_days(&self) -> T {
		return self.s.clone() * T::from(1.15740740740741e-05_f64);
	}

	/// Returns a new time value from the given number of days
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `days` - Any number-like type, representing a quantity of days
	pub fn from_days(days: T) -> Self {
		Time{s: days * T::from(86400.0_f64)}
	}

	/// Returns a copy of this time value in weeks
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_weeks(&self) -> T {
		return self.s.clone() * T::from(1.65343915343915e-06_f64);
	}

	/// Returns a new time value from the given number of weeks
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `weeks` - Any number-like type, representing a quantity of weeks
	pub fn from_weeks(weeks: T) -> Self {
		Time{s: weeks * T::from(604800.0_f64)}
	}

	/// Returns a copy of this time value in years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_yr(&self) -> T {
		return self.s.clone() * T::from(3.16887654287165e-08_f64);
	}

	/// Returns a new time value from the given number of years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `yr` - Any number-like type, representing a quantity of years
	pub fn from_yr(yr: T) -> Self {
		Time{s: yr * T::from(31556925.19008_f64)}
	}

	/// Returns a copy of this time value in millennia
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kyr(&self) -> T {
		return self.s.clone() * T::from(3.16887654287165e-11_f64);
	}

	/// Returns a new time value from the given number of millennia
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kyr` - Any number-like type, representing a quantity of millennia
	pub fn from_kyr(kyr: T) -> Self {
		Time{s: kyr * T::from(31556925190.08_f64)}
	}

	/// Returns a copy of this time value in million years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Myr(&self) -> T {
		return self.s.clone() * T::from(3.16887654287165e-14_f64);
	}

	/// Returns a new time value from the given number of million years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Myr` - Any number-like type, representing a quantity of million years
	pub fn from_Myr(Myr: T) -> Self {
		Time{s: Myr * T::from(31556925190080.0_f64)}
	}

	/// Returns a copy of this time value in billion years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Gyr(&self) -> T {
		return self.s.clone() * T::from(3.16887654287165e-17_f64);
	}

	/// Returns a new time value from the given number of billion years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Gyr` - Any number-like type, representing a quantity of billion years
	pub fn from_Gyr(Gyr: T) -> Self {
		Time{s: Gyr * T::from(3.155692519008e+16_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Time<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Time<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Time<num_bigfloat::BigFloat>) -> Self::Output {
		Time{s: self * rhs.s}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Time<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Time<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Time<num_bigfloat::BigFloat>) -> Self::Output {
		Time{s: self.clone() * rhs.s}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Time<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Time<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Time<num_bigfloat::BigFloat>) -> Self::Output {
		Time{s: self * rhs.s.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Time<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Time<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Time<num_bigfloat::BigFloat>) -> Self::Output {
		Time{s: self.clone() * rhs.s.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Time<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Time<num_complex::Complex32>;
	fn mul(self, rhs: Time<num_complex::Complex32>) -> Self::Output {
		Time{s: self * rhs.s}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Time<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Time<num_complex::Complex32>;
	fn mul(self, rhs: Time<num_complex::Complex32>) -> Self::Output {
		Time{s: self.clone() * rhs.s}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Time<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Time<num_complex::Complex32>;
	fn mul(self, rhs: &Time<num_complex::Complex32>) -> Self::Output {
		Time{s: self * rhs.s.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Time<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Time<num_complex::Complex32>;
	fn mul(self, rhs: &Time<num_complex::Complex32>) -> Self::Output {
		Time{s: self.clone() * rhs.s.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Time<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Time<num_complex::Complex64>;
	fn mul(self, rhs: Time<num_complex::Complex64>) -> Self::Output {
		Time{s: self * rhs.s}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Time<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Time<num_complex::Complex64>;
	fn mul(self, rhs: Time<num_complex::Complex64>) -> Self::Output {
		Time{s: self.clone() * rhs.s}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Time<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Time<num_complex::Complex64>;
	fn mul(self, rhs: &Time<num_complex::Complex64>) -> Self::Output {
		Time{s: self * rhs.s.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Time<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Time<num_complex::Complex64>;
	fn mul(self, rhs: &Time<num_complex::Complex64>) -> Self::Output {
		Time{s: self.clone() * rhs.s.clone()}
	}
}



/// Converts a Time into the equivalent [uom](https://crates.io/crates/uom) type [Time](https://docs.rs/uom/0.34.0/uom/si/f32/type.Time.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Time> for Time<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Time {
		uom::si::f32::Time::new::<uom::si::time::second>(self.s.into())
	}
}

/// Creates a Time from the equivalent [uom](https://crates.io/crates/uom) type [Time](https://docs.rs/uom/0.34.0/uom/si/f32/type.Time.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Time> for Time<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Time) -> Self {
		Time{s: T::from(src.value)}
	}
}

/// Converts a Time into the equivalent [uom](https://crates.io/crates/uom) type [Time](https://docs.rs/uom/0.34.0/uom/si/f64/type.Time.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Time> for Time<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Time {
		uom::si::f64::Time::new::<uom::si::time::second>(self.s.into())
	}
}

/// Creates a Time from the equivalent [uom](https://crates.io/crates/uom) type [Time](https://docs.rs/uom/0.34.0/uom/si/f64/type.Time.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Time> for Time<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Time) -> Self {
		Time{s: T::from(src.value)}
	}
}


// Time * Current -> Charge
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> core::ops::Mul<Current<T>> for Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Charge{C: self.s * rhs.A}
	}
}
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> core::ops::Mul<Current<T>> for &Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Charge{C: self.s.clone() * rhs.A}
	}
}
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> core::ops::Mul<&Current<T>> for Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Charge{C: self.s * rhs.A.clone()}
	}
}
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> core::ops::Mul<&Current<T>> for &Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Charge{C: self.s.clone() * rhs.A.clone()}
	}
}

// Time * CatalyticActivity -> Amount
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> core::ops::Mul<CatalyticActivity<T>> for Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s * rhs.molps}
	}
}
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> core::ops::Mul<CatalyticActivity<T>> for &Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s.clone() * rhs.molps}
	}
}
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> core::ops::Mul<&CatalyticActivity<T>> for Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s * rhs.molps.clone()}
	}
}
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> core::ops::Mul<&CatalyticActivity<T>> for &Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s.clone() * rhs.molps.clone()}
	}
}

// Time / Capacitance -> Resistance
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> core::ops::Div<Capacitance<T>> for Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s / rhs.F}
	}
}
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> core::ops::Div<Capacitance<T>> for &Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s.clone() / rhs.F}
	}
}
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> core::ops::Div<&Capacitance<T>> for Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s / rhs.F.clone()}
	}
}
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> core::ops::Div<&Capacitance<T>> for &Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s.clone() / rhs.F.clone()}
	}
}

// Time * Conductance -> Capacitance
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> core::ops::Mul<Conductance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Capacitance{F: self.s * rhs.S}
	}
}
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> core::ops::Mul<Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() * rhs.S}
	}
}
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> core::ops::Mul<&Conductance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Capacitance{F: self.s * rhs.S.clone()}
	}
}
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> core::ops::Mul<&Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() * rhs.S.clone()}
	}
}

// Time / Conductance -> Inductance
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> core::ops::Div<Conductance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Inductance{H: self.s / rhs.S}
	}
}
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> core::ops::Div<Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Inductance{H: self.s.clone() / rhs.S}
	}
}
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> core::ops::Div<&Conductance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Inductance{H: self.s / rhs.S.clone()}
	}
}
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> core::ops::Div<&Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Inductance{H: self.s.clone() / rhs.S.clone()}
	}
}

// Time / Inductance -> Conductance
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> core::ops::Div<Inductance<T>> for Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Conductance{S: self.s / rhs.H}
	}
}
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> core::ops::Div<Inductance<T>> for &Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Conductance{S: self.s.clone() / rhs.H}
	}
}
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> core::ops::Div<&Inductance<T>> for Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Conductance{S: self.s / rhs.H.clone()}
	}
}
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> core::ops::Div<&Inductance<T>> for &Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Conductance{S: self.s.clone() / rhs.H.clone()}
	}
}

// Time * Resistance -> Inductance
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> core::ops::Mul<Resistance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Inductance{H: self.s * rhs.Ohm}
	}
}
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> core::ops::Mul<Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Inductance{H: self.s.clone() * rhs.Ohm}
	}
}
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> core::ops::Mul<&Resistance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Inductance{H: self.s * rhs.Ohm.clone()}
	}
}
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> core::ops::Mul<&Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Inductance{H: self.s.clone() * rhs.Ohm.clone()}
	}
}

// Time / Resistance -> Capacitance
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> core::ops::Div<Resistance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Capacitance{F: self.s / rhs.Ohm}
	}
}
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> core::ops::Div<Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() / rhs.Ohm}
	}
}
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> core::ops::Div<&Resistance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Capacitance{F: self.s / rhs.Ohm.clone()}
	}
}
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> core::ops::Div<&Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() / rhs.Ohm.clone()}
	}
}

// Time * Voltage -> MagneticFlux
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> core::ops::Mul<Voltage<T>> for Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s * rhs.V}
	}
}
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> core::ops::Mul<Voltage<T>> for &Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s.clone() * rhs.V}
	}
}
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Voltage<T>> for Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s * rhs.V.clone()}
	}
}
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Voltage<T>> for &Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s.clone() * rhs.V.clone()}
	}
}

// Time * Acceleration -> Velocity
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> core::ops::Mul<Acceleration<T>> for Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s * rhs.mps2}
	}
}
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> core::ops::Mul<Acceleration<T>> for &Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s.clone() * rhs.mps2}
	}
}
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> core::ops::Mul<&Acceleration<T>> for Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s * rhs.mps2.clone()}
	}
}
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> core::ops::Mul<&Acceleration<T>> for &Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s.clone() * rhs.mps2.clone()}
	}
}

// Time * AngularAcceleration -> AngularVelocity
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Mul<AngularAcceleration<T>> for Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s * rhs.radps2}
	}
}
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Mul<AngularAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s.clone() * rhs.radps2}
	}
}
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Mul<&AngularAcceleration<T>> for Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s * rhs.radps2.clone()}
	}
}
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Mul<&AngularAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s.clone() * rhs.radps2.clone()}
	}
}

// Time * AngularVelocity -> Angle
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> core::ops::Mul<AngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s * rhs.radps}
	}
}
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> core::ops::Mul<AngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s.clone() * rhs.radps}
	}
}
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> core::ops::Mul<&AngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s * rhs.radps.clone()}
	}
}
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> core::ops::Mul<&AngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s.clone() * rhs.radps.clone()}
	}
}

// Time * Force -> Momentum
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> core::ops::Mul<Force<T>> for Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Momentum{kgmps: self.s * rhs.N}
	}
}
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> core::ops::Mul<Force<T>> for &Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Momentum{kgmps: self.s.clone() * rhs.N}
	}
}
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> core::ops::Mul<&Force<T>> for Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Momentum{kgmps: self.s * rhs.N.clone()}
	}
}
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> core::ops::Mul<&Force<T>> for &Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Momentum{kgmps: self.s.clone() * rhs.N.clone()}
	}
}

// Time * Power -> Energy
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> core::ops::Mul<Power<T>> for Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Energy{J: self.s * rhs.W}
	}
}
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> core::ops::Mul<Power<T>> for &Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Energy{J: self.s.clone() * rhs.W}
	}
}
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> core::ops::Mul<&Power<T>> for Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Energy{J: self.s * rhs.W.clone()}
	}
}
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> core::ops::Mul<&Power<T>> for &Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Energy{J: self.s.clone() * rhs.W.clone()}
	}
}

// Time * Velocity -> Distance
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> core::ops::Mul<Velocity<T>> for Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Distance{m: self.s * rhs.mps}
	}
}
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> core::ops::Mul<Velocity<T>> for &Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Distance{m: self.s.clone() * rhs.mps}
	}
}
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> core::ops::Mul<&Velocity<T>> for Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Distance{m: self.s * rhs.mps.clone()}
	}
}
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> core::ops::Mul<&Velocity<T>> for &Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Distance{m: self.s.clone() * rhs.mps.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for f64 where T: NumLike+From<f64> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for f64 where T: NumLike+From<f64> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for f32 where T: NumLike+From<f32> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for f32 where T: NumLike+From<f32> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for i64 where T: NumLike+From<i64> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for i64 where T: NumLike+From<i64> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for i32 where T: NumLike+From<i32> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<Time<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for i32 where T: NumLike+From<i32> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
impl<T> core::ops::Div<&Time<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Time<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Time<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Time<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Time<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Time<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Time<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Time<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Time<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}

// 1/Time -> Frequency
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Time<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Time<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Frequency<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Time<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self) / rhs.s.clone()}
	}
}
/// Dividing a scalar value by a Time unit value returns a value of type Frequency
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Time<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Frequency<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Frequency{Hz: T::from(self.clone()) / rhs.s.clone()}
	}
}



