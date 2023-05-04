
//! This module provides base SI units, such as amount 
//! and distance (aka length).
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


// Amount * InverseMass -> Molality
/// Multiplying a Amount by a InverseMass returns a value of type Molality
impl<T> core::ops::Mul<InverseMass<T>> for Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: InverseMass<T>) -> Self::Output {
		Molality{molpkg: self.mol * rhs.per_kg}
	}
}
/// Multiplying a Amount by a InverseMass returns a value of type Molality
impl<T> core::ops::Mul<InverseMass<T>> for &Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: InverseMass<T>) -> Self::Output {
		Molality{molpkg: self.mol.clone() * rhs.per_kg}
	}
}
/// Multiplying a Amount by a InverseMass returns a value of type Molality
impl<T> core::ops::Mul<&InverseMass<T>> for Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: &InverseMass<T>) -> Self::Output {
		Molality{molpkg: self.mol * rhs.per_kg.clone()}
	}
}
/// Multiplying a Amount by a InverseMass returns a value of type Molality
impl<T> core::ops::Mul<&InverseMass<T>> for &Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: &InverseMass<T>) -> Self::Output {
		Molality{molpkg: self.mol.clone() * rhs.per_kg.clone()}
	}
}

// Amount / Mass -> Molality
/// Dividing a Amount by a Mass returns a value of type Molality
impl<T> core::ops::Div<Mass<T>> for Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Molality{molpkg: self.mol / rhs.kg}
	}
}
/// Dividing a Amount by a Mass returns a value of type Molality
impl<T> core::ops::Div<Mass<T>> for &Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Molality{molpkg: self.mol.clone() / rhs.kg}
	}
}
/// Dividing a Amount by a Mass returns a value of type Molality
impl<T> core::ops::Div<&Mass<T>> for Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Molality{molpkg: self.mol / rhs.kg.clone()}
	}
}
/// Dividing a Amount by a Mass returns a value of type Molality
impl<T> core::ops::Div<&Mass<T>> for &Amount<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Molality{molpkg: self.mol.clone() / rhs.kg.clone()}
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

// Amount * InverseCatalyticActivity -> Time
/// Multiplying a Amount by a InverseCatalyticActivity returns a value of type Time
impl<T> core::ops::Mul<InverseCatalyticActivity<T>> for Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseCatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol * rhs.s_per_mol}
	}
}
/// Multiplying a Amount by a InverseCatalyticActivity returns a value of type Time
impl<T> core::ops::Mul<InverseCatalyticActivity<T>> for &Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseCatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol.clone() * rhs.s_per_mol}
	}
}
/// Multiplying a Amount by a InverseCatalyticActivity returns a value of type Time
impl<T> core::ops::Mul<&InverseCatalyticActivity<T>> for Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseCatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol * rhs.s_per_mol.clone()}
	}
}
/// Multiplying a Amount by a InverseCatalyticActivity returns a value of type Time
impl<T> core::ops::Mul<&InverseCatalyticActivity<T>> for &Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseCatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol.clone() * rhs.s_per_mol.clone()}
	}
}

// Amount / Molality -> Mass
/// Dividing a Amount by a Molality returns a value of type Mass
impl<T> core::ops::Div<Molality<T>> for Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		Mass{kg: self.mol / rhs.molpkg}
	}
}
/// Dividing a Amount by a Molality returns a value of type Mass
impl<T> core::ops::Div<Molality<T>> for &Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		Mass{kg: self.mol.clone() / rhs.molpkg}
	}
}
/// Dividing a Amount by a Molality returns a value of type Mass
impl<T> core::ops::Div<&Molality<T>> for Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		Mass{kg: self.mol / rhs.molpkg.clone()}
	}
}
/// Dividing a Amount by a Molality returns a value of type Mass
impl<T> core::ops::Div<&Molality<T>> for &Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		Mass{kg: self.mol.clone() / rhs.molpkg.clone()}
	}
}

// Amount * MolarMass -> Mass
/// Multiplying a Amount by a MolarMass returns a value of type Mass
impl<T> core::ops::Mul<MolarMass<T>> for Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: MolarMass<T>) -> Self::Output {
		Mass{kg: self.mol * rhs.kgpmol}
	}
}
/// Multiplying a Amount by a MolarMass returns a value of type Mass
impl<T> core::ops::Mul<MolarMass<T>> for &Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: MolarMass<T>) -> Self::Output {
		Mass{kg: self.mol.clone() * rhs.kgpmol}
	}
}
/// Multiplying a Amount by a MolarMass returns a value of type Mass
impl<T> core::ops::Mul<&MolarMass<T>> for Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &MolarMass<T>) -> Self::Output {
		Mass{kg: self.mol * rhs.kgpmol.clone()}
	}
}
/// Multiplying a Amount by a MolarMass returns a value of type Mass
impl<T> core::ops::Mul<&MolarMass<T>> for &Amount<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &MolarMass<T>) -> Self::Output {
		Mass{kg: self.mol.clone() * rhs.kgpmol.clone()}
	}
}

// Amount * MolarVolume -> Volume
/// Multiplying a Amount by a MolarVolume returns a value of type Volume
impl<T> core::ops::Mul<MolarVolume<T>> for Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: MolarVolume<T>) -> Self::Output {
		Volume{m3: self.mol * rhs.m3_per_mol}
	}
}
/// Multiplying a Amount by a MolarVolume returns a value of type Volume
impl<T> core::ops::Mul<MolarVolume<T>> for &Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: MolarVolume<T>) -> Self::Output {
		Volume{m3: self.mol.clone() * rhs.m3_per_mol}
	}
}
/// Multiplying a Amount by a MolarVolume returns a value of type Volume
impl<T> core::ops::Mul<&MolarVolume<T>> for Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &MolarVolume<T>) -> Self::Output {
		Volume{m3: self.mol * rhs.m3_per_mol.clone()}
	}
}
/// Multiplying a Amount by a MolarVolume returns a value of type Volume
impl<T> core::ops::Mul<&MolarVolume<T>> for &Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &MolarVolume<T>) -> Self::Output {
		Volume{m3: self.mol.clone() * rhs.m3_per_mol.clone()}
	}
}

// Amount * InverseVolume -> Concentration
/// Multiplying a Amount by a InverseVolume returns a value of type Concentration
impl<T> core::ops::Mul<InverseVolume<T>> for Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		Concentration{molpm3: self.mol * rhs.per_m3}
	}
}
/// Multiplying a Amount by a InverseVolume returns a value of type Concentration
impl<T> core::ops::Mul<InverseVolume<T>> for &Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		Concentration{molpm3: self.mol.clone() * rhs.per_m3}
	}
}
/// Multiplying a Amount by a InverseVolume returns a value of type Concentration
impl<T> core::ops::Mul<&InverseVolume<T>> for Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		Concentration{molpm3: self.mol * rhs.per_m3.clone()}
	}
}
/// Multiplying a Amount by a InverseVolume returns a value of type Concentration
impl<T> core::ops::Mul<&InverseVolume<T>> for &Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		Concentration{molpm3: self.mol.clone() * rhs.per_m3.clone()}
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

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
	}
}

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
	}
}

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
	}
}

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<Amount<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
impl<T> core::ops::Div<&Amount<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
	}
}

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Amount<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Amount<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Amount<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Amount<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
	}
}

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Amount<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Amount<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Amount<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Amount<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
	}
}

// 1/Amount -> InverseAmount
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Amount<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Amount<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Amount<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self) / rhs.mol.clone()}
	}
}
/// Dividing a scalar value by a Amount unit value returns a value of type InverseAmount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Amount<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseAmount{per_mol: T::from(self.clone()) / rhs.mol.clone()}
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

// Current * InverseCharge -> Frequency
/// Multiplying a Current by a InverseCharge returns a value of type Frequency
impl<T> core::ops::Mul<InverseCharge<T>> for Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		Frequency{Hz: self.A * rhs.per_C}
	}
}
/// Multiplying a Current by a InverseCharge returns a value of type Frequency
impl<T> core::ops::Mul<InverseCharge<T>> for &Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		Frequency{Hz: self.A.clone() * rhs.per_C}
	}
}
/// Multiplying a Current by a InverseCharge returns a value of type Frequency
impl<T> core::ops::Mul<&InverseCharge<T>> for Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		Frequency{Hz: self.A * rhs.per_C.clone()}
	}
}
/// Multiplying a Current by a InverseCharge returns a value of type Frequency
impl<T> core::ops::Mul<&InverseCharge<T>> for &Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		Frequency{Hz: self.A.clone() * rhs.per_C.clone()}
	}
}

// Current / InverseInductance -> MagneticFlux
/// Dividing a Current by a InverseInductance returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseInductance<T>> for Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A / rhs.per_H}
	}
}
/// Dividing a Current by a InverseInductance returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseInductance<T>> for &Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A.clone() / rhs.per_H}
	}
}
/// Dividing a Current by a InverseInductance returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseInductance<T>> for Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A / rhs.per_H.clone()}
	}
}
/// Dividing a Current by a InverseInductance returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseInductance<T>> for &Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A.clone() / rhs.per_H.clone()}
	}
}

// Current * InverseMagneticFlux -> InverseInductance
/// Multiplying a Current by a InverseMagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A * rhs.per_Wb}
	}
}
/// Multiplying a Current by a InverseMagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Current by a InverseMagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Current by a InverseMagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A.clone() * rhs.per_Wb.clone()}
	}
}

// Current / InverseMagneticFlux -> Energy
/// Dividing a Current by a InverseMagneticFlux returns a value of type Energy
impl<T> core::ops::Div<InverseMagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Energy{J: self.A / rhs.per_Wb}
	}
}
/// Dividing a Current by a InverseMagneticFlux returns a value of type Energy
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Energy{J: self.A.clone() / rhs.per_Wb}
	}
}
/// Dividing a Current by a InverseMagneticFlux returns a value of type Energy
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Energy{J: self.A / rhs.per_Wb.clone()}
	}
}
/// Dividing a Current by a InverseMagneticFlux returns a value of type Energy
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Energy{J: self.A.clone() / rhs.per_Wb.clone()}
	}
}

// Current * InverseVoltage -> Conductance
/// Multiplying a Current by a InverseVoltage returns a value of type Conductance
impl<T> core::ops::Mul<InverseVoltage<T>> for Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		Conductance{S: self.A * rhs.per_V}
	}
}
/// Multiplying a Current by a InverseVoltage returns a value of type Conductance
impl<T> core::ops::Mul<InverseVoltage<T>> for &Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		Conductance{S: self.A.clone() * rhs.per_V}
	}
}
/// Multiplying a Current by a InverseVoltage returns a value of type Conductance
impl<T> core::ops::Mul<&InverseVoltage<T>> for Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Conductance{S: self.A * rhs.per_V.clone()}
	}
}
/// Multiplying a Current by a InverseVoltage returns a value of type Conductance
impl<T> core::ops::Mul<&InverseVoltage<T>> for &Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Conductance{S: self.A.clone() * rhs.per_V.clone()}
	}
}

// Current / InverseVoltage -> Power
/// Dividing a Current by a InverseVoltage returns a value of type Power
impl<T> core::ops::Div<InverseVoltage<T>> for Current<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Power{W: self.A / rhs.per_V}
	}
}
/// Dividing a Current by a InverseVoltage returns a value of type Power
impl<T> core::ops::Div<InverseVoltage<T>> for &Current<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Power{W: self.A.clone() / rhs.per_V}
	}
}
/// Dividing a Current by a InverseVoltage returns a value of type Power
impl<T> core::ops::Div<&InverseVoltage<T>> for Current<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Power{W: self.A / rhs.per_V.clone()}
	}
}
/// Dividing a Current by a InverseVoltage returns a value of type Power
impl<T> core::ops::Div<&InverseVoltage<T>> for &Current<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Power{W: self.A.clone() / rhs.per_V.clone()}
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

// Current / MagneticFlux -> InverseInductance
/// Dividing a Current by a MagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Div<MagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A / rhs.Wb}
	}
}
/// Dividing a Current by a MagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Div<MagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A.clone() / rhs.Wb}
	}
}
/// Dividing a Current by a MagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Div<&MagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A / rhs.Wb.clone()}
	}
}
/// Dividing a Current by a MagneticFlux returns a value of type InverseInductance
impl<T> core::ops::Div<&MagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseInductance{per_H: self.A.clone() / rhs.Wb.clone()}
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

// Current / Energy -> InverseMagneticFlux
/// Dividing a Current by a Energy returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Energy<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A / rhs.J}
	}
}
/// Dividing a Current by a Energy returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Energy<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() / rhs.J}
	}
}
/// Dividing a Current by a Energy returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Energy<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A / rhs.J.clone()}
	}
}
/// Dividing a Current by a Energy returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Energy<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() / rhs.J.clone()}
	}
}

// Current / Torque -> InverseMagneticFlux
/// Dividing a Current by a Torque returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Torque<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A / rhs.Nm}
	}
}
/// Dividing a Current by a Torque returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Torque<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() / rhs.Nm}
	}
}
/// Dividing a Current by a Torque returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Torque<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A / rhs.Nm.clone()}
	}
}
/// Dividing a Current by a Torque returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Torque<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() / rhs.Nm.clone()}
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

// Current * InverseEnergy -> InverseMagneticFlux
/// Multiplying a Current by a InverseEnergy returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseEnergy<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A * rhs.per_J}
	}
}
/// Multiplying a Current by a InverseEnergy returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseEnergy<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() * rhs.per_J}
	}
}
/// Multiplying a Current by a InverseEnergy returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseEnergy<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A * rhs.per_J.clone()}
	}
}
/// Multiplying a Current by a InverseEnergy returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseEnergy<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() * rhs.per_J.clone()}
	}
}

// Current * InverseTorque -> InverseMagneticFlux
/// Multiplying a Current by a InverseTorque returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseTorque<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A * rhs.per_Nm}
	}
}
/// Multiplying a Current by a InverseTorque returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseTorque<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() * rhs.per_Nm}
	}
}
/// Multiplying a Current by a InverseTorque returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseTorque<T>> for Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A * rhs.per_Nm.clone()}
	}
}
/// Multiplying a Current by a InverseTorque returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseTorque<T>> for &Current<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.A.clone() * rhs.per_Nm.clone()}
	}
}

// Current * InversePower -> InverseVoltage
/// Multiplying a Current by a InversePower returns a value of type InverseVoltage
impl<T> core::ops::Mul<InversePower<T>> for Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InversePower<T>) -> Self::Output {
		InverseVoltage{per_V: self.A * rhs.per_W}
	}
}
/// Multiplying a Current by a InversePower returns a value of type InverseVoltage
impl<T> core::ops::Mul<InversePower<T>> for &Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InversePower<T>) -> Self::Output {
		InverseVoltage{per_V: self.A.clone() * rhs.per_W}
	}
}
/// Multiplying a Current by a InversePower returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InversePower<T>> for Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InversePower<T>) -> Self::Output {
		InverseVoltage{per_V: self.A * rhs.per_W.clone()}
	}
}
/// Multiplying a Current by a InversePower returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InversePower<T>> for &Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InversePower<T>) -> Self::Output {
		InverseVoltage{per_V: self.A.clone() * rhs.per_W.clone()}
	}
}

// Current / Power -> InverseVoltage
/// Dividing a Current by a Power returns a value of type InverseVoltage
impl<T> core::ops::Div<Power<T>> for Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		InverseVoltage{per_V: self.A / rhs.W}
	}
}
/// Dividing a Current by a Power returns a value of type InverseVoltage
impl<T> core::ops::Div<Power<T>> for &Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		InverseVoltage{per_V: self.A.clone() / rhs.W}
	}
}
/// Dividing a Current by a Power returns a value of type InverseVoltage
impl<T> core::ops::Div<&Power<T>> for Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		InverseVoltage{per_V: self.A / rhs.W.clone()}
	}
}
/// Dividing a Current by a Power returns a value of type InverseVoltage
impl<T> core::ops::Div<&Power<T>> for &Current<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		InverseVoltage{per_V: self.A.clone() / rhs.W.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<Current<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
impl<T> core::ops::Div<&Current<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Current<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Current<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Current<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Current<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Current<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Current<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Current<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Current<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
	}
}

// 1/Current -> InverseCurrent
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Current<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Current<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Current<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self) / rhs.A.clone()}
	}
}
/// Dividing a scalar value by a Current unit value returns a value of type InverseCurrent
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Current<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseCurrent{per_A: T::from(self.clone()) / rhs.A.clone()}
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

// Distance / InverseDistance -> Area
/// Dividing a Distance by a InverseDistance returns a value of type Area
impl<T> core::ops::Div<InverseDistance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Area{m2: self.m / rhs.per_m}
	}
}
/// Dividing a Distance by a InverseDistance returns a value of type Area
impl<T> core::ops::Div<InverseDistance<T>> for &Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Area{m2: self.m.clone() / rhs.per_m}
	}
}
/// Dividing a Distance by a InverseDistance returns a value of type Area
impl<T> core::ops::Div<&InverseDistance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Area{m2: self.m / rhs.per_m.clone()}
	}
}
/// Dividing a Distance by a InverseDistance returns a value of type Area
impl<T> core::ops::Div<&InverseDistance<T>> for &Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Area{m2: self.m.clone() / rhs.per_m.clone()}
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

// Distance / Area -> InverseDistance
/// Dividing a Distance by a Area returns a value of type InverseDistance
impl<T> core::ops::Div<Area<T>> for Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseDistance{per_m: self.m / rhs.m2}
	}
}
/// Dividing a Distance by a Area returns a value of type InverseDistance
impl<T> core::ops::Div<Area<T>> for &Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseDistance{per_m: self.m.clone() / rhs.m2}
	}
}
/// Dividing a Distance by a Area returns a value of type InverseDistance
impl<T> core::ops::Div<&Area<T>> for Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseDistance{per_m: self.m / rhs.m2.clone()}
	}
}
/// Dividing a Distance by a Area returns a value of type InverseDistance
impl<T> core::ops::Div<&Area<T>> for &Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseDistance{per_m: self.m.clone() / rhs.m2.clone()}
	}
}

// Distance * InverseArea -> InverseDistance
/// Multiplying a Distance by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Mul<InverseArea<T>> for Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.m * rhs.per_m2}
	}
}
/// Multiplying a Distance by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Mul<InverseArea<T>> for &Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.m.clone() * rhs.per_m2}
	}
}
/// Multiplying a Distance by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Mul<&InverseArea<T>> for Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.m * rhs.per_m2.clone()}
	}
}
/// Multiplying a Distance by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Mul<&InverseArea<T>> for &Distance<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.m.clone() * rhs.per_m2.clone()}
	}
}

// Distance / InverseArea -> Volume
/// Dividing a Distance by a InverseArea returns a value of type Volume
impl<T> core::ops::Div<InverseArea<T>> for Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Volume{m3: self.m / rhs.per_m2}
	}
}
/// Dividing a Distance by a InverseArea returns a value of type Volume
impl<T> core::ops::Div<InverseArea<T>> for &Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Volume{m3: self.m.clone() / rhs.per_m2}
	}
}
/// Dividing a Distance by a InverseArea returns a value of type Volume
impl<T> core::ops::Div<&InverseArea<T>> for Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Volume{m3: self.m / rhs.per_m2.clone()}
	}
}
/// Dividing a Distance by a InverseArea returns a value of type Volume
impl<T> core::ops::Div<&InverseArea<T>> for &Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Volume{m3: self.m.clone() / rhs.per_m2.clone()}
	}
}

// Distance * InverseVolume -> InverseArea
/// Multiplying a Distance by a InverseVolume returns a value of type InverseArea
impl<T> core::ops::Mul<InverseVolume<T>> for Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		InverseArea{per_m2: self.m * rhs.per_m3}
	}
}
/// Multiplying a Distance by a InverseVolume returns a value of type InverseArea
impl<T> core::ops::Mul<InverseVolume<T>> for &Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		InverseArea{per_m2: self.m.clone() * rhs.per_m3}
	}
}
/// Multiplying a Distance by a InverseVolume returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseVolume<T>> for Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		InverseArea{per_m2: self.m * rhs.per_m3.clone()}
	}
}
/// Multiplying a Distance by a InverseVolume returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseVolume<T>> for &Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		InverseArea{per_m2: self.m.clone() * rhs.per_m3.clone()}
	}
}

// Distance / Volume -> InverseArea
/// Dividing a Distance by a Volume returns a value of type InverseArea
impl<T> core::ops::Div<Volume<T>> for Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseArea{per_m2: self.m / rhs.m3}
	}
}
/// Dividing a Distance by a Volume returns a value of type InverseArea
impl<T> core::ops::Div<Volume<T>> for &Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseArea{per_m2: self.m.clone() / rhs.m3}
	}
}
/// Dividing a Distance by a Volume returns a value of type InverseArea
impl<T> core::ops::Div<&Volume<T>> for Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseArea{per_m2: self.m / rhs.m3.clone()}
	}
}
/// Dividing a Distance by a Volume returns a value of type InverseArea
impl<T> core::ops::Div<&Volume<T>> for &Distance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseArea{per_m2: self.m.clone() / rhs.m3.clone()}
	}
}

// Distance / AreaDensity -> VolumePerMass
/// Dividing a Distance by a AreaDensity returns a value of type VolumePerMass
impl<T> core::ops::Div<AreaDensity<T>> for Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m / rhs.kgpm2}
	}
}
/// Dividing a Distance by a AreaDensity returns a value of type VolumePerMass
impl<T> core::ops::Div<AreaDensity<T>> for &Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m.clone() / rhs.kgpm2}
	}
}
/// Dividing a Distance by a AreaDensity returns a value of type VolumePerMass
impl<T> core::ops::Div<&AreaDensity<T>> for Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m / rhs.kgpm2.clone()}
	}
}
/// Dividing a Distance by a AreaDensity returns a value of type VolumePerMass
impl<T> core::ops::Div<&AreaDensity<T>> for &Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m.clone() / rhs.kgpm2.clone()}
	}
}

// Distance * AreaPerMass -> VolumePerMass
/// Multiplying a Distance by a AreaPerMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<AreaPerMass<T>> for Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: AreaPerMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m * rhs.m2_per_kg}
	}
}
/// Multiplying a Distance by a AreaPerMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<AreaPerMass<T>> for &Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: AreaPerMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m.clone() * rhs.m2_per_kg}
	}
}
/// Multiplying a Distance by a AreaPerMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<&AreaPerMass<T>> for Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: &AreaPerMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m * rhs.m2_per_kg.clone()}
	}
}
/// Multiplying a Distance by a AreaPerMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<&AreaPerMass<T>> for &Distance<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: &AreaPerMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m.clone() * rhs.m2_per_kg.clone()}
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

// Distance / Energy -> InverseForce
/// Dividing a Distance by a Energy returns a value of type InverseForce
impl<T> core::ops::Div<Energy<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseForce{per_N: self.m / rhs.J}
	}
}
/// Dividing a Distance by a Energy returns a value of type InverseForce
impl<T> core::ops::Div<Energy<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() / rhs.J}
	}
}
/// Dividing a Distance by a Energy returns a value of type InverseForce
impl<T> core::ops::Div<&Energy<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseForce{per_N: self.m / rhs.J.clone()}
	}
}
/// Dividing a Distance by a Energy returns a value of type InverseForce
impl<T> core::ops::Div<&Energy<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() / rhs.J.clone()}
	}
}

// Distance / Torque -> InverseForce
/// Dividing a Distance by a Torque returns a value of type InverseForce
impl<T> core::ops::Div<Torque<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseForce{per_N: self.m / rhs.Nm}
	}
}
/// Dividing a Distance by a Torque returns a value of type InverseForce
impl<T> core::ops::Div<Torque<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() / rhs.Nm}
	}
}
/// Dividing a Distance by a Torque returns a value of type InverseForce
impl<T> core::ops::Div<&Torque<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseForce{per_N: self.m / rhs.Nm.clone()}
	}
}
/// Dividing a Distance by a Torque returns a value of type InverseForce
impl<T> core::ops::Div<&Torque<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() / rhs.Nm.clone()}
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

// Distance * InverseEnergy -> InverseForce
/// Multiplying a Distance by a InverseEnergy returns a value of type InverseForce
impl<T> core::ops::Mul<InverseEnergy<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseForce{per_N: self.m * rhs.per_J}
	}
}
/// Multiplying a Distance by a InverseEnergy returns a value of type InverseForce
impl<T> core::ops::Mul<InverseEnergy<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() * rhs.per_J}
	}
}
/// Multiplying a Distance by a InverseEnergy returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseEnergy<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseForce{per_N: self.m * rhs.per_J.clone()}
	}
}
/// Multiplying a Distance by a InverseEnergy returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseEnergy<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() * rhs.per_J.clone()}
	}
}

// Distance * InverseTorque -> InverseForce
/// Multiplying a Distance by a InverseTorque returns a value of type InverseForce
impl<T> core::ops::Mul<InverseTorque<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseForce{per_N: self.m * rhs.per_Nm}
	}
}
/// Multiplying a Distance by a InverseTorque returns a value of type InverseForce
impl<T> core::ops::Mul<InverseTorque<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() * rhs.per_Nm}
	}
}
/// Multiplying a Distance by a InverseTorque returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseTorque<T>> for Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseForce{per_N: self.m * rhs.per_Nm.clone()}
	}
}
/// Multiplying a Distance by a InverseTorque returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseTorque<T>> for &Distance<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseForce{per_N: self.m.clone() * rhs.per_Nm.clone()}
	}
}

// Distance / InverseForce -> Energy
/// Dividing a Distance by a InverseForce returns a value of type Energy
impl<T> core::ops::Div<InverseForce<T>> for Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Energy{J: self.m / rhs.per_N}
	}
}
/// Dividing a Distance by a InverseForce returns a value of type Energy
impl<T> core::ops::Div<InverseForce<T>> for &Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Energy{J: self.m.clone() / rhs.per_N}
	}
}
/// Dividing a Distance by a InverseForce returns a value of type Energy
impl<T> core::ops::Div<&InverseForce<T>> for Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Energy{J: self.m / rhs.per_N.clone()}
	}
}
/// Dividing a Distance by a InverseForce returns a value of type Energy
impl<T> core::ops::Div<&InverseForce<T>> for &Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Energy{J: self.m.clone() / rhs.per_N.clone()}
	}
}

// Distance * TimePerDistance -> Time
/// Multiplying a Distance by a TimePerDistance returns a value of type Time
impl<T> core::ops::Mul<TimePerDistance<T>> for Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: TimePerDistance<T>) -> Self::Output {
		Time{s: self.m * rhs.spm}
	}
}
/// Multiplying a Distance by a TimePerDistance returns a value of type Time
impl<T> core::ops::Mul<TimePerDistance<T>> for &Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: TimePerDistance<T>) -> Self::Output {
		Time{s: self.m.clone() * rhs.spm}
	}
}
/// Multiplying a Distance by a TimePerDistance returns a value of type Time
impl<T> core::ops::Mul<&TimePerDistance<T>> for Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Time{s: self.m * rhs.spm.clone()}
	}
}
/// Multiplying a Distance by a TimePerDistance returns a value of type Time
impl<T> core::ops::Mul<&TimePerDistance<T>> for &Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Time{s: self.m.clone() * rhs.spm.clone()}
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

// Distance / VolumePerMass -> AreaDensity
/// Dividing a Distance by a VolumePerMass returns a value of type AreaDensity
impl<T> core::ops::Div<VolumePerMass<T>> for Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: VolumePerMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m / rhs.m3_per_kg}
	}
}
/// Dividing a Distance by a VolumePerMass returns a value of type AreaDensity
impl<T> core::ops::Div<VolumePerMass<T>> for &Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: VolumePerMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m.clone() / rhs.m3_per_kg}
	}
}
/// Dividing a Distance by a VolumePerMass returns a value of type AreaDensity
impl<T> core::ops::Div<&VolumePerMass<T>> for Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &VolumePerMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m / rhs.m3_per_kg.clone()}
	}
}
/// Dividing a Distance by a VolumePerMass returns a value of type AreaDensity
impl<T> core::ops::Div<&VolumePerMass<T>> for &Distance<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &VolumePerMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.m.clone() / rhs.m3_per_kg.clone()}
	}
}

// Distance * InverseAbsorbedDose -> InverseAcceleration
/// Multiplying a Distance by a InverseAbsorbedDose returns a value of type InverseAcceleration
impl<T> core::ops::Mul<InverseAbsorbedDose<T>> for Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m * rhs.per_Gy}
	}
}
/// Multiplying a Distance by a InverseAbsorbedDose returns a value of type InverseAcceleration
impl<T> core::ops::Mul<InverseAbsorbedDose<T>> for &Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m.clone() * rhs.per_Gy}
	}
}
/// Multiplying a Distance by a InverseAbsorbedDose returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&InverseAbsorbedDose<T>> for Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m * rhs.per_Gy.clone()}
	}
}
/// Multiplying a Distance by a InverseAbsorbedDose returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&InverseAbsorbedDose<T>> for &Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m.clone() * rhs.per_Gy.clone()}
	}
}

// Distance * InverseDoseEquivalent -> InverseAcceleration
/// Multiplying a Distance by a InverseDoseEquivalent returns a value of type InverseAcceleration
impl<T> core::ops::Mul<InverseDoseEquivalent<T>> for Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m * rhs.per_Sv}
	}
}
/// Multiplying a Distance by a InverseDoseEquivalent returns a value of type InverseAcceleration
impl<T> core::ops::Mul<InverseDoseEquivalent<T>> for &Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m.clone() * rhs.per_Sv}
	}
}
/// Multiplying a Distance by a InverseDoseEquivalent returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&InverseDoseEquivalent<T>> for Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m * rhs.per_Sv.clone()}
	}
}
/// Multiplying a Distance by a InverseDoseEquivalent returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&InverseDoseEquivalent<T>> for &Distance<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.m.clone() * rhs.per_Sv.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<Distance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
impl<T> core::ops::Div<&Distance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Distance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Distance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Distance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Distance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Distance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Distance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Distance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Distance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

// 1/Distance -> InverseDistance
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Distance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Distance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Distance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self) / rhs.m.clone()}
	}
}
/// Dividing a scalar value by a Distance unit value returns a value of type InverseDistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Distance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: T::from(self.clone()) / rhs.m.clone()}
	}
}

/// The inverse of amount unit type, defined as inverse moles in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseAmount<T: NumLike>{
	/// The value of this Inverse amount in inverse moles
	pub per_mol: T
}

impl<T> InverseAmount<T> where T: NumLike {

	/// Returns the standard unit name of inverse amount: "inverse moles"
	pub fn unit_name() -> &'static str { "inverse moles" }
	
	/// Returns the abbreviated name or symbol of inverse amount: "1/mol" for inverse moles
	pub fn unit_symbol() -> &'static str { "1/mol" }
	
	/// Returns a new inverse amount value from the given number of inverse moles
	///
	/// # Arguments
	/// * `per_mole` - Any number-like type, representing a quantity of inverse moles
	pub fn from_per_mole(per_mole: T) -> Self { InverseAmount{per_mol: per_mole} }
	
	/// Returns a copy of this inverse amount value in inverse moles
	pub fn to_per_mole(&self) -> T { self.per_mol.clone() }

	/// Returns a new inverse amount value from the given number of inverse moles
	///
	/// # Arguments
	/// * `per_mol` - Any number-like type, representing a quantity of inverse moles
	pub fn from_per_mol(per_mol: T) -> Self { InverseAmount{per_mol: per_mol} }
	
	/// Returns a copy of this inverse amount value in inverse moles
	pub fn to_per_mol(&self) -> T { self.per_mol.clone() }

}

impl<T> fmt::Display for InverseAmount<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_mol, Self::unit_symbol())
	}
}

impl<T> InverseAmount<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse amount value in inverse count
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_count(&self) -> T {
		return self.per_mol.clone() * T::from(1.66e-24_f64);
	}

	/// Returns a new inverse amount value from the given number of inverse count
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_count` - Any number-like type, representing a quantity of inverse count
	pub fn from_per_count(per_count: T) -> Self {
		InverseAmount{per_mol: per_count * T::from(6.02e+23_f64)}
	}

	/// Returns a copy of this inverse amount value in inverse millimoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mmol(&self) -> T {
		return self.per_mol.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse amount value from the given number of inverse millimoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mmol` - Any number-like type, representing a quantity of inverse millimoles
	pub fn from_per_mmol(per_mmol: T) -> Self {
		InverseAmount{per_mol: per_mmol * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse amount value in inverse micromoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_umol(&self) -> T {
		return self.per_mol.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse amount value from the given number of inverse micromoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_umol` - Any number-like type, representing a quantity of inverse micromoles
	pub fn from_per_umol(per_umol: T) -> Self {
		InverseAmount{per_mol: per_umol * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse amount value in inverse nanomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nmol(&self) -> T {
		return self.per_mol.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse amount value from the given number of inverse nanomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nmol` - Any number-like type, representing a quantity of inverse nanomoles
	pub fn from_per_nmol(per_nmol: T) -> Self {
		InverseAmount{per_mol: per_nmol * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse amount value in inverse picomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_pmol(&self) -> T {
		return self.per_mol.clone() * T::from(1e-12_f64);
	}

	/// Returns a new inverse amount value from the given number of inverse picomoles
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_pmol` - Any number-like type, representing a quantity of inverse picomoles
	pub fn from_per_pmol(per_pmol: T) -> Self {
		InverseAmount{per_mol: per_pmol * T::from(1000000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseAmount<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseAmount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseAmount<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAmount{per_mol: self * rhs.per_mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseAmount<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseAmount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseAmount<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAmount{per_mol: self.clone() * rhs.per_mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseAmount<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseAmount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseAmount<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAmount{per_mol: self * rhs.per_mol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseAmount<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseAmount<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseAmount<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAmount{per_mol: self.clone() * rhs.per_mol.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAmount<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseAmount<num_complex::Complex32>;
	fn mul(self, rhs: InverseAmount<num_complex::Complex32>) -> Self::Output {
		InverseAmount{per_mol: self * rhs.per_mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAmount<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseAmount<num_complex::Complex32>;
	fn mul(self, rhs: InverseAmount<num_complex::Complex32>) -> Self::Output {
		InverseAmount{per_mol: self.clone() * rhs.per_mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAmount<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseAmount<num_complex::Complex32>;
	fn mul(self, rhs: &InverseAmount<num_complex::Complex32>) -> Self::Output {
		InverseAmount{per_mol: self * rhs.per_mol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAmount<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseAmount<num_complex::Complex32>;
	fn mul(self, rhs: &InverseAmount<num_complex::Complex32>) -> Self::Output {
		InverseAmount{per_mol: self.clone() * rhs.per_mol.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAmount<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseAmount<num_complex::Complex64>;
	fn mul(self, rhs: InverseAmount<num_complex::Complex64>) -> Self::Output {
		InverseAmount{per_mol: self * rhs.per_mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAmount<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseAmount<num_complex::Complex64>;
	fn mul(self, rhs: InverseAmount<num_complex::Complex64>) -> Self::Output {
		InverseAmount{per_mol: self.clone() * rhs.per_mol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAmount<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseAmount<num_complex::Complex64>;
	fn mul(self, rhs: &InverseAmount<num_complex::Complex64>) -> Self::Output {
		InverseAmount{per_mol: self * rhs.per_mol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAmount<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseAmount<num_complex::Complex64>;
	fn mul(self, rhs: &InverseAmount<num_complex::Complex64>) -> Self::Output {
		InverseAmount{per_mol: self.clone() * rhs.per_mol.clone()}
	}
}




// InverseAmount / InverseMass -> MolarMass
/// Dividing a InverseAmount by a InverseMass returns a value of type MolarMass
impl<T> core::ops::Div<InverseMass<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol / rhs.per_kg}
	}
}
/// Dividing a InverseAmount by a InverseMass returns a value of type MolarMass
impl<T> core::ops::Div<InverseMass<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol.clone() / rhs.per_kg}
	}
}
/// Dividing a InverseAmount by a InverseMass returns a value of type MolarMass
impl<T> core::ops::Div<&InverseMass<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol / rhs.per_kg.clone()}
	}
}
/// Dividing a InverseAmount by a InverseMass returns a value of type MolarMass
impl<T> core::ops::Div<&InverseMass<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol.clone() / rhs.per_kg.clone()}
	}
}

// InverseAmount * Mass -> MolarMass
/// Multiplying a InverseAmount by a Mass returns a value of type MolarMass
impl<T> core::ops::Mul<Mass<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol * rhs.kg}
	}
}
/// Multiplying a InverseAmount by a Mass returns a value of type MolarMass
impl<T> core::ops::Mul<Mass<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol.clone() * rhs.kg}
	}
}
/// Multiplying a InverseAmount by a Mass returns a value of type MolarMass
impl<T> core::ops::Mul<&Mass<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol * rhs.kg.clone()}
	}
}
/// Multiplying a InverseAmount by a Mass returns a value of type MolarMass
impl<T> core::ops::Mul<&Mass<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		MolarMass{kgpmol: self.per_mol.clone() * rhs.kg.clone()}
	}
}

// InverseAmount * Time -> InverseCatalyticActivity
/// Multiplying a InverseAmount by a Time returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<Time<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol * rhs.s}
	}
}
/// Multiplying a InverseAmount by a Time returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<Time<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol.clone() * rhs.s}
	}
}
/// Multiplying a InverseAmount by a Time returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<&Time<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol * rhs.s.clone()}
	}
}
/// Multiplying a InverseAmount by a Time returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<&Time<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol.clone() * rhs.s.clone()}
	}
}

// InverseAmount * CatalyticActivity -> Frequency
/// Multiplying a InverseAmount by a CatalyticActivity returns a value of type Frequency
impl<T> core::ops::Mul<CatalyticActivity<T>> for InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol * rhs.molps}
	}
}
/// Multiplying a InverseAmount by a CatalyticActivity returns a value of type Frequency
impl<T> core::ops::Mul<CatalyticActivity<T>> for &InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol.clone() * rhs.molps}
	}
}
/// Multiplying a InverseAmount by a CatalyticActivity returns a value of type Frequency
impl<T> core::ops::Mul<&CatalyticActivity<T>> for InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol * rhs.molps.clone()}
	}
}
/// Multiplying a InverseAmount by a CatalyticActivity returns a value of type Frequency
impl<T> core::ops::Mul<&CatalyticActivity<T>> for &InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol.clone() * rhs.molps.clone()}
	}
}

// InverseAmount * Concentration -> InverseVolume
/// Multiplying a InverseAmount by a Concentration returns a value of type InverseVolume
impl<T> core::ops::Mul<Concentration<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol * rhs.molpm3}
	}
}
/// Multiplying a InverseAmount by a Concentration returns a value of type InverseVolume
impl<T> core::ops::Mul<Concentration<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol.clone() * rhs.molpm3}
	}
}
/// Multiplying a InverseAmount by a Concentration returns a value of type InverseVolume
impl<T> core::ops::Mul<&Concentration<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol * rhs.molpm3.clone()}
	}
}
/// Multiplying a InverseAmount by a Concentration returns a value of type InverseVolume
impl<T> core::ops::Mul<&Concentration<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol.clone() * rhs.molpm3.clone()}
	}
}

// InverseAmount / InverseCatalyticActivity -> Frequency
/// Dividing a InverseAmount by a InverseCatalyticActivity returns a value of type Frequency
impl<T> core::ops::Div<InverseCatalyticActivity<T>> for InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseCatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol / rhs.s_per_mol}
	}
}
/// Dividing a InverseAmount by a InverseCatalyticActivity returns a value of type Frequency
impl<T> core::ops::Div<InverseCatalyticActivity<T>> for &InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseCatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol.clone() / rhs.s_per_mol}
	}
}
/// Dividing a InverseAmount by a InverseCatalyticActivity returns a value of type Frequency
impl<T> core::ops::Div<&InverseCatalyticActivity<T>> for InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseCatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol / rhs.s_per_mol.clone()}
	}
}
/// Dividing a InverseAmount by a InverseCatalyticActivity returns a value of type Frequency
impl<T> core::ops::Div<&InverseCatalyticActivity<T>> for &InverseAmount<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseCatalyticActivity<T>) -> Self::Output {
		Frequency{Hz: self.per_mol.clone() / rhs.s_per_mol.clone()}
	}
}

// InverseAmount * Molality -> InverseMass
/// Multiplying a InverseAmount by a Molality returns a value of type InverseMass
impl<T> core::ops::Mul<Molality<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: Molality<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol * rhs.molpkg}
	}
}
/// Multiplying a InverseAmount by a Molality returns a value of type InverseMass
impl<T> core::ops::Mul<Molality<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: Molality<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol.clone() * rhs.molpkg}
	}
}
/// Multiplying a InverseAmount by a Molality returns a value of type InverseMass
impl<T> core::ops::Mul<&Molality<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &Molality<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol * rhs.molpkg.clone()}
	}
}
/// Multiplying a InverseAmount by a Molality returns a value of type InverseMass
impl<T> core::ops::Mul<&Molality<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &Molality<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol.clone() * rhs.molpkg.clone()}
	}
}

// InverseAmount / MolarMass -> InverseMass
/// Dividing a InverseAmount by a MolarMass returns a value of type InverseMass
impl<T> core::ops::Div<MolarMass<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol / rhs.kgpmol}
	}
}
/// Dividing a InverseAmount by a MolarMass returns a value of type InverseMass
impl<T> core::ops::Div<MolarMass<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol.clone() / rhs.kgpmol}
	}
}
/// Dividing a InverseAmount by a MolarMass returns a value of type InverseMass
impl<T> core::ops::Div<&MolarMass<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol / rhs.kgpmol.clone()}
	}
}
/// Dividing a InverseAmount by a MolarMass returns a value of type InverseMass
impl<T> core::ops::Div<&MolarMass<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_mol.clone() / rhs.kgpmol.clone()}
	}
}

// InverseAmount / MolarVolume -> InverseVolume
/// Dividing a InverseAmount by a MolarVolume returns a value of type InverseVolume
impl<T> core::ops::Div<MolarVolume<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: MolarVolume<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol / rhs.m3_per_mol}
	}
}
/// Dividing a InverseAmount by a MolarVolume returns a value of type InverseVolume
impl<T> core::ops::Div<MolarVolume<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: MolarVolume<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol.clone() / rhs.m3_per_mol}
	}
}
/// Dividing a InverseAmount by a MolarVolume returns a value of type InverseVolume
impl<T> core::ops::Div<&MolarVolume<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &MolarVolume<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol / rhs.m3_per_mol.clone()}
	}
}
/// Dividing a InverseAmount by a MolarVolume returns a value of type InverseVolume
impl<T> core::ops::Div<&MolarVolume<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &MolarVolume<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_mol.clone() / rhs.m3_per_mol.clone()}
	}
}

// InverseAmount / InverseVolume -> MolarVolume
/// Dividing a InverseAmount by a InverseVolume returns a value of type MolarVolume
impl<T> core::ops::Div<InverseVolume<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol / rhs.per_m3}
	}
}
/// Dividing a InverseAmount by a InverseVolume returns a value of type MolarVolume
impl<T> core::ops::Div<InverseVolume<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol.clone() / rhs.per_m3}
	}
}
/// Dividing a InverseAmount by a InverseVolume returns a value of type MolarVolume
impl<T> core::ops::Div<&InverseVolume<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol / rhs.per_m3.clone()}
	}
}
/// Dividing a InverseAmount by a InverseVolume returns a value of type MolarVolume
impl<T> core::ops::Div<&InverseVolume<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol.clone() / rhs.per_m3.clone()}
	}
}

// InverseAmount * Volume -> MolarVolume
/// Multiplying a InverseAmount by a Volume returns a value of type MolarVolume
impl<T> core::ops::Mul<Volume<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol * rhs.m3}
	}
}
/// Multiplying a InverseAmount by a Volume returns a value of type MolarVolume
impl<T> core::ops::Mul<Volume<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol.clone() * rhs.m3}
	}
}
/// Multiplying a InverseAmount by a Volume returns a value of type MolarVolume
impl<T> core::ops::Mul<&Volume<T>> for InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol * rhs.m3.clone()}
	}
}
/// Multiplying a InverseAmount by a Volume returns a value of type MolarVolume
impl<T> core::ops::Mul<&Volume<T>> for &InverseAmount<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.per_mol.clone() * rhs.m3.clone()}
	}
}

// InverseAmount / Frequency -> InverseCatalyticActivity
/// Dividing a InverseAmount by a Frequency returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<Frequency<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol / rhs.Hz}
	}
}
/// Dividing a InverseAmount by a Frequency returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<Frequency<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol.clone() / rhs.Hz}
	}
}
/// Dividing a InverseAmount by a Frequency returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<&Frequency<T>> for InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol / rhs.Hz.clone()}
	}
}
/// Dividing a InverseAmount by a Frequency returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<&Frequency<T>> for &InverseAmount<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.per_mol.clone() / rhs.Hz.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for f64 where T: NumLike+From<f64> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for f64 where T: NumLike+From<f64> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for f32 where T: NumLike+From<f32> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for f32 where T: NumLike+From<f32> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for i64 where T: NumLike+From<i64> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for i64 where T: NumLike+From<i64> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for i32 where T: NumLike+From<i32> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<InverseAmount<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for i32 where T: NumLike+From<i32> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
impl<T> core::ops::Div<&InverseAmount<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseAmount<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseAmount<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseAmount<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseAmount<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAmount<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAmount<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAmount<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAmount<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

// 1/InverseAmount -> Amount
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAmount<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAmount<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Amount<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAmount<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self) / rhs.per_mol.clone()}
	}
}
/// Dividing a scalar value by a InverseAmount unit value returns a value of type Amount
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAmount<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Amount{mol: T::from(self.clone()) / rhs.per_mol.clone()}
	}
}

/// The inverse of electrical current unit type, defined as inverse amperes in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseCurrent<T: NumLike>{
	/// The value of this Inverse electrical current in inverse amperes
	pub per_A: T
}

impl<T> InverseCurrent<T> where T: NumLike {

	/// Returns the standard unit name of inverse electrical current: "inverse amperes"
	pub fn unit_name() -> &'static str { "inverse amperes" }
	
	/// Returns the abbreviated name or symbol of inverse electrical current: "1/A" for inverse amperes
	pub fn unit_symbol() -> &'static str { "1/A" }
	
	/// Returns a new inverse electrical current value from the given number of inverse amperes
	///
	/// # Arguments
	/// * `per_A` - Any number-like type, representing a quantity of inverse amperes
	pub fn from_per_A(per_A: T) -> Self { InverseCurrent{per_A: per_A} }
	
	/// Returns a copy of this inverse electrical current value in inverse amperes
	pub fn to_per_A(&self) -> T { self.per_A.clone() }

	/// Returns a new inverse electrical current value from the given number of inverse amperes
	///
	/// # Arguments
	/// * `per_ampere` - Any number-like type, representing a quantity of inverse amperes
	pub fn from_per_ampere(per_ampere: T) -> Self { InverseCurrent{per_A: per_ampere} }
	
	/// Returns a copy of this inverse electrical current value in inverse amperes
	pub fn to_per_ampere(&self) -> T { self.per_A.clone() }

}

impl<T> fmt::Display for InverseCurrent<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_A, Self::unit_symbol())
	}
}

impl<T> InverseCurrent<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse electrical current value in inverse milliamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mA(&self) -> T {
		return self.per_A.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse electrical current value from the given number of inverse milliamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mA` - Any number-like type, representing a quantity of inverse milliamperes
	pub fn from_per_mA(per_mA: T) -> Self {
		InverseCurrent{per_A: per_mA * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse electrical current value in inverse microamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uA(&self) -> T {
		return self.per_A.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse electrical current value from the given number of inverse microamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uA` - Any number-like type, representing a quantity of inverse microamperes
	pub fn from_per_uA(per_uA: T) -> Self {
		InverseCurrent{per_A: per_uA * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse electrical current value in inverse nanoamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nA(&self) -> T {
		return self.per_A.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse electrical current value from the given number of inverse nanoamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nA` - Any number-like type, representing a quantity of inverse nanoamperes
	pub fn from_per_nA(per_nA: T) -> Self {
		InverseCurrent{per_A: per_nA * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse electrical current value in inverse kiloamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kA(&self) -> T {
		return self.per_A.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse electrical current value from the given number of inverse kiloamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kA` - Any number-like type, representing a quantity of inverse kiloamperes
	pub fn from_per_kA(per_kA: T) -> Self {
		InverseCurrent{per_A: per_kA * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse electrical current value in inverse megaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_MA(&self) -> T {
		return self.per_A.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse electrical current value from the given number of inverse megaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_MA` - Any number-like type, representing a quantity of inverse megaamperes
	pub fn from_per_MA(per_MA: T) -> Self {
		InverseCurrent{per_A: per_MA * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse electrical current value in inverse gigaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GA(&self) -> T {
		return self.per_A.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse electrical current value from the given number of inverse gigaamperes
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GA` - Any number-like type, representing a quantity of inverse gigaamperes
	pub fn from_per_GA(per_GA: T) -> Self {
		InverseCurrent{per_A: per_GA * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseCurrent<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseCurrent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseCurrent<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCurrent{per_A: self * rhs.per_A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseCurrent<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseCurrent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseCurrent<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCurrent{per_A: self.clone() * rhs.per_A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseCurrent<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseCurrent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseCurrent<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCurrent{per_A: self * rhs.per_A.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseCurrent<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseCurrent<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseCurrent<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCurrent{per_A: self.clone() * rhs.per_A.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCurrent<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseCurrent<num_complex::Complex32>;
	fn mul(self, rhs: InverseCurrent<num_complex::Complex32>) -> Self::Output {
		InverseCurrent{per_A: self * rhs.per_A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCurrent<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseCurrent<num_complex::Complex32>;
	fn mul(self, rhs: InverseCurrent<num_complex::Complex32>) -> Self::Output {
		InverseCurrent{per_A: self.clone() * rhs.per_A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCurrent<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseCurrent<num_complex::Complex32>;
	fn mul(self, rhs: &InverseCurrent<num_complex::Complex32>) -> Self::Output {
		InverseCurrent{per_A: self * rhs.per_A.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCurrent<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseCurrent<num_complex::Complex32>;
	fn mul(self, rhs: &InverseCurrent<num_complex::Complex32>) -> Self::Output {
		InverseCurrent{per_A: self.clone() * rhs.per_A.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCurrent<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseCurrent<num_complex::Complex64>;
	fn mul(self, rhs: InverseCurrent<num_complex::Complex64>) -> Self::Output {
		InverseCurrent{per_A: self * rhs.per_A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCurrent<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseCurrent<num_complex::Complex64>;
	fn mul(self, rhs: InverseCurrent<num_complex::Complex64>) -> Self::Output {
		InverseCurrent{per_A: self.clone() * rhs.per_A}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCurrent<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseCurrent<num_complex::Complex64>;
	fn mul(self, rhs: &InverseCurrent<num_complex::Complex64>) -> Self::Output {
		InverseCurrent{per_A: self * rhs.per_A.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCurrent<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseCurrent<num_complex::Complex64>;
	fn mul(self, rhs: &InverseCurrent<num_complex::Complex64>) -> Self::Output {
		InverseCurrent{per_A: self.clone() * rhs.per_A.clone()}
	}
}




// InverseCurrent / Time -> InverseCharge
/// Dividing a InverseCurrent by a Time returns a value of type InverseCharge
impl<T> core::ops::Div<Time<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A / rhs.s}
	}
}
/// Dividing a InverseCurrent by a Time returns a value of type InverseCharge
impl<T> core::ops::Div<Time<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A.clone() / rhs.s}
	}
}
/// Dividing a InverseCurrent by a Time returns a value of type InverseCharge
impl<T> core::ops::Div<&Time<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A / rhs.s.clone()}
	}
}
/// Dividing a InverseCurrent by a Time returns a value of type InverseCharge
impl<T> core::ops::Div<&Time<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A.clone() / rhs.s.clone()}
	}
}

// InverseCurrent * Charge -> Time
/// Multiplying a InverseCurrent by a Charge returns a value of type Time
impl<T> core::ops::Mul<Charge<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Time{s: self.per_A * rhs.C}
	}
}
/// Multiplying a InverseCurrent by a Charge returns a value of type Time
impl<T> core::ops::Mul<Charge<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Time{s: self.per_A.clone() * rhs.C}
	}
}
/// Multiplying a InverseCurrent by a Charge returns a value of type Time
impl<T> core::ops::Mul<&Charge<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Time{s: self.per_A * rhs.C.clone()}
	}
}
/// Multiplying a InverseCurrent by a Charge returns a value of type Time
impl<T> core::ops::Mul<&Charge<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Time{s: self.per_A.clone() * rhs.C.clone()}
	}
}

// InverseCurrent * Conductance -> InverseVoltage
/// Multiplying a InverseCurrent by a Conductance returns a value of type InverseVoltage
impl<T> core::ops::Mul<Conductance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A * rhs.S}
	}
}
/// Multiplying a InverseCurrent by a Conductance returns a value of type InverseVoltage
impl<T> core::ops::Mul<Conductance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A.clone() * rhs.S}
	}
}
/// Multiplying a InverseCurrent by a Conductance returns a value of type InverseVoltage
impl<T> core::ops::Mul<&Conductance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A * rhs.S.clone()}
	}
}
/// Multiplying a InverseCurrent by a Conductance returns a value of type InverseVoltage
impl<T> core::ops::Mul<&Conductance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A.clone() * rhs.S.clone()}
	}
}

// InverseCurrent / Inductance -> InverseMagneticFlux
/// Dividing a InverseCurrent by a Inductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Inductance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A / rhs.H}
	}
}
/// Dividing a InverseCurrent by a Inductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Inductance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A.clone() / rhs.H}
	}
}
/// Dividing a InverseCurrent by a Inductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Inductance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A / rhs.H.clone()}
	}
}
/// Dividing a InverseCurrent by a Inductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Inductance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A.clone() / rhs.H.clone()}
	}
}

// InverseCurrent / InverseCharge -> Time
/// Dividing a InverseCurrent by a InverseCharge returns a value of type Time
impl<T> core::ops::Div<InverseCharge<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Time{s: self.per_A / rhs.per_C}
	}
}
/// Dividing a InverseCurrent by a InverseCharge returns a value of type Time
impl<T> core::ops::Div<InverseCharge<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Time{s: self.per_A.clone() / rhs.per_C}
	}
}
/// Dividing a InverseCurrent by a InverseCharge returns a value of type Time
impl<T> core::ops::Div<&InverseCharge<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Time{s: self.per_A / rhs.per_C.clone()}
	}
}
/// Dividing a InverseCurrent by a InverseCharge returns a value of type Time
impl<T> core::ops::Div<&InverseCharge<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Time{s: self.per_A.clone() / rhs.per_C.clone()}
	}
}

// InverseCurrent * InverseInductance -> InverseMagneticFlux
/// Multiplying a InverseCurrent by a InverseInductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseInductance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A * rhs.per_H}
	}
}
/// Multiplying a InverseCurrent by a InverseInductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseInductance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A.clone() * rhs.per_H}
	}
}
/// Multiplying a InverseCurrent by a InverseInductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseInductance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A * rhs.per_H.clone()}
	}
}
/// Multiplying a InverseCurrent by a InverseInductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseInductance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_A.clone() * rhs.per_H.clone()}
	}
}

// InverseCurrent * InverseMagneticFlux -> InverseEnergy
/// Multiplying a InverseCurrent by a InverseMagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A * rhs.per_Wb}
	}
}
/// Multiplying a InverseCurrent by a InverseMagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A.clone() * rhs.per_Wb}
	}
}
/// Multiplying a InverseCurrent by a InverseMagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A * rhs.per_Wb.clone()}
	}
}
/// Multiplying a InverseCurrent by a InverseMagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A.clone() * rhs.per_Wb.clone()}
	}
}

// InverseCurrent / InverseMagneticFlux -> Inductance
/// Dividing a InverseCurrent by a InverseMagneticFlux returns a value of type Inductance
impl<T> core::ops::Div<InverseMagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A / rhs.per_Wb}
	}
}
/// Dividing a InverseCurrent by a InverseMagneticFlux returns a value of type Inductance
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A.clone() / rhs.per_Wb}
	}
}
/// Dividing a InverseCurrent by a InverseMagneticFlux returns a value of type Inductance
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A / rhs.per_Wb.clone()}
	}
}
/// Dividing a InverseCurrent by a InverseMagneticFlux returns a value of type Inductance
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A.clone() / rhs.per_Wb.clone()}
	}
}

// InverseCurrent * InverseVoltage -> InversePower
/// Multiplying a InverseCurrent by a InverseVoltage returns a value of type InversePower
impl<T> core::ops::Mul<InverseVoltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A * rhs.per_V}
	}
}
/// Multiplying a InverseCurrent by a InverseVoltage returns a value of type InversePower
impl<T> core::ops::Mul<InverseVoltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A.clone() * rhs.per_V}
	}
}
/// Multiplying a InverseCurrent by a InverseVoltage returns a value of type InversePower
impl<T> core::ops::Mul<&InverseVoltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A * rhs.per_V.clone()}
	}
}
/// Multiplying a InverseCurrent by a InverseVoltage returns a value of type InversePower
impl<T> core::ops::Mul<&InverseVoltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A.clone() * rhs.per_V.clone()}
	}
}

// InverseCurrent / InverseVoltage -> Resistance
/// Dividing a InverseCurrent by a InverseVoltage returns a value of type Resistance
impl<T> core::ops::Div<InverseVoltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A / rhs.per_V}
	}
}
/// Dividing a InverseCurrent by a InverseVoltage returns a value of type Resistance
impl<T> core::ops::Div<InverseVoltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A.clone() / rhs.per_V}
	}
}
/// Dividing a InverseCurrent by a InverseVoltage returns a value of type Resistance
impl<T> core::ops::Div<&InverseVoltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A / rhs.per_V.clone()}
	}
}
/// Dividing a InverseCurrent by a InverseVoltage returns a value of type Resistance
impl<T> core::ops::Div<&InverseVoltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A.clone() / rhs.per_V.clone()}
	}
}

// InverseCurrent * MagneticFlux -> Inductance
/// Multiplying a InverseCurrent by a MagneticFlux returns a value of type Inductance
impl<T> core::ops::Mul<MagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A * rhs.Wb}
	}
}
/// Multiplying a InverseCurrent by a MagneticFlux returns a value of type Inductance
impl<T> core::ops::Mul<MagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A.clone() * rhs.Wb}
	}
}
/// Multiplying a InverseCurrent by a MagneticFlux returns a value of type Inductance
impl<T> core::ops::Mul<&MagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A * rhs.Wb.clone()}
	}
}
/// Multiplying a InverseCurrent by a MagneticFlux returns a value of type Inductance
impl<T> core::ops::Mul<&MagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Inductance{H: self.per_A.clone() * rhs.Wb.clone()}
	}
}

// InverseCurrent / MagneticFlux -> InverseEnergy
/// Dividing a InverseCurrent by a MagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Div<MagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A / rhs.Wb}
	}
}
/// Dividing a InverseCurrent by a MagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Div<MagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A.clone() / rhs.Wb}
	}
}
/// Dividing a InverseCurrent by a MagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Div<&MagneticFlux<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A / rhs.Wb.clone()}
	}
}
/// Dividing a InverseCurrent by a MagneticFlux returns a value of type InverseEnergy
impl<T> core::ops::Div<&MagneticFlux<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_A.clone() / rhs.Wb.clone()}
	}
}

// InverseCurrent / Resistance -> InverseVoltage
/// Dividing a InverseCurrent by a Resistance returns a value of type InverseVoltage
impl<T> core::ops::Div<Resistance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A / rhs.Ohm}
	}
}
/// Dividing a InverseCurrent by a Resistance returns a value of type InverseVoltage
impl<T> core::ops::Div<Resistance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A.clone() / rhs.Ohm}
	}
}
/// Dividing a InverseCurrent by a Resistance returns a value of type InverseVoltage
impl<T> core::ops::Div<&Resistance<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A / rhs.Ohm.clone()}
	}
}
/// Dividing a InverseCurrent by a Resistance returns a value of type InverseVoltage
impl<T> core::ops::Div<&Resistance<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_A.clone() / rhs.Ohm.clone()}
	}
}

// InverseCurrent * Voltage -> Resistance
/// Multiplying a InverseCurrent by a Voltage returns a value of type Resistance
impl<T> core::ops::Mul<Voltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A * rhs.V}
	}
}
/// Multiplying a InverseCurrent by a Voltage returns a value of type Resistance
impl<T> core::ops::Mul<Voltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A.clone() * rhs.V}
	}
}
/// Multiplying a InverseCurrent by a Voltage returns a value of type Resistance
impl<T> core::ops::Mul<&Voltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A * rhs.V.clone()}
	}
}
/// Multiplying a InverseCurrent by a Voltage returns a value of type Resistance
impl<T> core::ops::Mul<&Voltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Resistance{Ohm: self.per_A.clone() * rhs.V.clone()}
	}
}

// InverseCurrent / Voltage -> InversePower
/// Dividing a InverseCurrent by a Voltage returns a value of type InversePower
impl<T> core::ops::Div<Voltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A / rhs.V}
	}
}
/// Dividing a InverseCurrent by a Voltage returns a value of type InversePower
impl<T> core::ops::Div<Voltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A.clone() / rhs.V}
	}
}
/// Dividing a InverseCurrent by a Voltage returns a value of type InversePower
impl<T> core::ops::Div<&Voltage<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A / rhs.V.clone()}
	}
}
/// Dividing a InverseCurrent by a Voltage returns a value of type InversePower
impl<T> core::ops::Div<&Voltage<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InversePower{per_W: self.per_A.clone() / rhs.V.clone()}
	}
}

// InverseCurrent * Energy -> MagneticFlux
/// Multiplying a InverseCurrent by a Energy returns a value of type MagneticFlux
impl<T> core::ops::Mul<Energy<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A * rhs.J}
	}
}
/// Multiplying a InverseCurrent by a Energy returns a value of type MagneticFlux
impl<T> core::ops::Mul<Energy<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() * rhs.J}
	}
}
/// Multiplying a InverseCurrent by a Energy returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Energy<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A * rhs.J.clone()}
	}
}
/// Multiplying a InverseCurrent by a Energy returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Energy<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() * rhs.J.clone()}
	}
}

// InverseCurrent * Torque -> MagneticFlux
/// Multiplying a InverseCurrent by a Torque returns a value of type MagneticFlux
impl<T> core::ops::Mul<Torque<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A * rhs.Nm}
	}
}
/// Multiplying a InverseCurrent by a Torque returns a value of type MagneticFlux
impl<T> core::ops::Mul<Torque<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() * rhs.Nm}
	}
}
/// Multiplying a InverseCurrent by a Torque returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Torque<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A * rhs.Nm.clone()}
	}
}
/// Multiplying a InverseCurrent by a Torque returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Torque<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() * rhs.Nm.clone()}
	}
}

// InverseCurrent * Frequency -> InverseCharge
/// Multiplying a InverseCurrent by a Frequency returns a value of type InverseCharge
impl<T> core::ops::Mul<Frequency<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A * rhs.Hz}
	}
}
/// Multiplying a InverseCurrent by a Frequency returns a value of type InverseCharge
impl<T> core::ops::Mul<Frequency<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A.clone() * rhs.Hz}
	}
}
/// Multiplying a InverseCurrent by a Frequency returns a value of type InverseCharge
impl<T> core::ops::Mul<&Frequency<T>> for InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A * rhs.Hz.clone()}
	}
}
/// Multiplying a InverseCurrent by a Frequency returns a value of type InverseCharge
impl<T> core::ops::Mul<&Frequency<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		InverseCharge{per_C: self.per_A.clone() * rhs.Hz.clone()}
	}
}

// InverseCurrent / InverseEnergy -> MagneticFlux
/// Dividing a InverseCurrent by a InverseEnergy returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseEnergy<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A / rhs.per_J}
	}
}
/// Dividing a InverseCurrent by a InverseEnergy returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseEnergy<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() / rhs.per_J}
	}
}
/// Dividing a InverseCurrent by a InverseEnergy returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseEnergy<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A / rhs.per_J.clone()}
	}
}
/// Dividing a InverseCurrent by a InverseEnergy returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseEnergy<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() / rhs.per_J.clone()}
	}
}

// InverseCurrent / InverseTorque -> MagneticFlux
/// Dividing a InverseCurrent by a InverseTorque returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseTorque<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A / rhs.per_Nm}
	}
}
/// Dividing a InverseCurrent by a InverseTorque returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseTorque<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() / rhs.per_Nm}
	}
}
/// Dividing a InverseCurrent by a InverseTorque returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseTorque<T>> for InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A / rhs.per_Nm.clone()}
	}
}
/// Dividing a InverseCurrent by a InverseTorque returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseTorque<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		MagneticFlux{Wb: self.per_A.clone() / rhs.per_Nm.clone()}
	}
}

// InverseCurrent / InversePower -> Voltage
/// Dividing a InverseCurrent by a InversePower returns a value of type Voltage
impl<T> core::ops::Div<InversePower<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InversePower<T>) -> Self::Output {
		Voltage{V: self.per_A / rhs.per_W}
	}
}
/// Dividing a InverseCurrent by a InversePower returns a value of type Voltage
impl<T> core::ops::Div<InversePower<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InversePower<T>) -> Self::Output {
		Voltage{V: self.per_A.clone() / rhs.per_W}
	}
}
/// Dividing a InverseCurrent by a InversePower returns a value of type Voltage
impl<T> core::ops::Div<&InversePower<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InversePower<T>) -> Self::Output {
		Voltage{V: self.per_A / rhs.per_W.clone()}
	}
}
/// Dividing a InverseCurrent by a InversePower returns a value of type Voltage
impl<T> core::ops::Div<&InversePower<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InversePower<T>) -> Self::Output {
		Voltage{V: self.per_A.clone() / rhs.per_W.clone()}
	}
}

// InverseCurrent * Power -> Voltage
/// Multiplying a InverseCurrent by a Power returns a value of type Voltage
impl<T> core::ops::Mul<Power<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Voltage{V: self.per_A * rhs.W}
	}
}
/// Multiplying a InverseCurrent by a Power returns a value of type Voltage
impl<T> core::ops::Mul<Power<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Voltage{V: self.per_A.clone() * rhs.W}
	}
}
/// Multiplying a InverseCurrent by a Power returns a value of type Voltage
impl<T> core::ops::Mul<&Power<T>> for InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Voltage{V: self.per_A * rhs.W.clone()}
	}
}
/// Multiplying a InverseCurrent by a Power returns a value of type Voltage
impl<T> core::ops::Mul<&Power<T>> for &InverseCurrent<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Voltage{V: self.per_A.clone() * rhs.W.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for f64 where T: NumLike+From<f64> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for f64 where T: NumLike+From<f64> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for f32 where T: NumLike+From<f32> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for f32 where T: NumLike+From<f32> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for i64 where T: NumLike+From<i64> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for i64 where T: NumLike+From<i64> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for i32 where T: NumLike+From<i32> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<InverseCurrent<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for i32 where T: NumLike+From<i32> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
impl<T> core::ops::Div<&InverseCurrent<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseCurrent<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseCurrent<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseCurrent<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseCurrent<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCurrent<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCurrent<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCurrent<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCurrent<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

// 1/InverseCurrent -> Current
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCurrent<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCurrent<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Current<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCurrent<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self) / rhs.per_A.clone()}
	}
}
/// Dividing a scalar value by a InverseCurrent unit value returns a value of type Current
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCurrent<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Current<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Current{A: T::from(self.clone()) / rhs.per_A.clone()}
	}
}

/// The inverse of distance unit type, defined as inverse meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseDistance<T: NumLike>{
	/// The value of this Inverse distance in inverse meters
	pub per_m: T
}

impl<T> InverseDistance<T> where T: NumLike {

	/// Returns the standard unit name of inverse distance: "inverse meters"
	pub fn unit_name() -> &'static str { "inverse meters" }
	
	/// Returns the abbreviated name or symbol of inverse distance: "1/m" for inverse meters
	pub fn unit_symbol() -> &'static str { "1/m" }
	
	/// Returns a new inverse distance value from the given number of inverse meters
	///
	/// # Arguments
	/// * `per_m` - Any number-like type, representing a quantity of inverse meters
	pub fn from_per_m(per_m: T) -> Self { InverseDistance{per_m: per_m} }
	
	/// Returns a copy of this inverse distance value in inverse meters
	pub fn to_per_m(&self) -> T { self.per_m.clone() }

	/// Returns a new inverse distance value from the given number of inverse meters
	///
	/// # Arguments
	/// * `per_meter` - Any number-like type, representing a quantity of inverse meters
	pub fn from_per_meter(per_meter: T) -> Self { InverseDistance{per_m: per_meter} }
	
	/// Returns a copy of this inverse distance value in inverse meters
	pub fn to_per_meter(&self) -> T { self.per_m.clone() }

}

impl<T> fmt::Display for InverseDistance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_m, Self::unit_symbol())
	}
}

impl<T> InverseDistance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse distance value in inverse millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_cm(&self) -> T {
		return self.per_m.clone() * T::from(0.01_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_cm` - Any number-like type, representing a quantity of inverse millimeters
	pub fn from_per_cm(per_cm: T) -> Self {
		InverseDistance{per_m: per_cm * T::from(100.0_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mm(&self) -> T {
		return self.per_m.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse millimeters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mm` - Any number-like type, representing a quantity of inverse millimeters
	pub fn from_per_mm(per_mm: T) -> Self {
		InverseDistance{per_m: per_mm * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse micrometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_um(&self) -> T {
		return self.per_m.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse micrometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_um` - Any number-like type, representing a quantity of inverse micrometers
	pub fn from_per_um(per_um: T) -> Self {
		InverseDistance{per_m: per_um * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse nanometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nm(&self) -> T {
		return self.per_m.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse nanometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nm` - Any number-like type, representing a quantity of inverse nanometers
	pub fn from_per_nm(per_nm: T) -> Self {
		InverseDistance{per_m: per_nm * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse picometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_pm(&self) -> T {
		return self.per_m.clone() * T::from(1e-12_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse picometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_pm` - Any number-like type, representing a quantity of inverse picometers
	pub fn from_per_pm(per_pm: T) -> Self {
		InverseDistance{per_m: per_pm * T::from(1000000000000.0_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse kilometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_km(&self) -> T {
		return self.per_m.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse kilometers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_km` - Any number-like type, representing a quantity of inverse kilometers
	pub fn from_per_km(per_km: T) -> Self {
		InverseDistance{per_m: per_km * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse astronomical units
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_au(&self) -> T {
		return self.per_m.clone() * T::from(149597870700.0_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse astronomical units
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_au` - Any number-like type, representing a quantity of inverse astronomical units
	pub fn from_per_au(per_au: T) -> Self {
		InverseDistance{per_m: per_au * T::from(6.68e-12_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse parsecs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_parsec(&self) -> T {
		return self.per_m.clone() * T::from(3.09e+16_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse parsecs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_parsec` - Any number-like type, representing a quantity of inverse parsecs
	pub fn from_per_parsec(per_parsec: T) -> Self {
		InverseDistance{per_m: per_parsec * T::from(3.24e-17_f64)}
	}

	/// Returns a copy of this inverse distance value in inverse light-years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_lyr(&self) -> T {
		return self.per_m.clone() * T::from(9460528169656200.0_f64);
	}

	/// Returns a new inverse distance value from the given number of inverse light-years
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_lyr` - Any number-like type, representing a quantity of inverse light-years
	pub fn from_per_lyr(per_lyr: T) -> Self {
		InverseDistance{per_m: per_lyr * T::from(1.06e-16_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseDistance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseDistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseDistance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseDistance{per_m: self * rhs.per_m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseDistance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseDistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseDistance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseDistance{per_m: self.clone() * rhs.per_m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseDistance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseDistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseDistance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseDistance{per_m: self * rhs.per_m.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseDistance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseDistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseDistance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseDistance{per_m: self.clone() * rhs.per_m.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseDistance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseDistance<num_complex::Complex32>;
	fn mul(self, rhs: InverseDistance<num_complex::Complex32>) -> Self::Output {
		InverseDistance{per_m: self * rhs.per_m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseDistance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseDistance<num_complex::Complex32>;
	fn mul(self, rhs: InverseDistance<num_complex::Complex32>) -> Self::Output {
		InverseDistance{per_m: self.clone() * rhs.per_m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseDistance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseDistance<num_complex::Complex32>;
	fn mul(self, rhs: &InverseDistance<num_complex::Complex32>) -> Self::Output {
		InverseDistance{per_m: self * rhs.per_m.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseDistance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseDistance<num_complex::Complex32>;
	fn mul(self, rhs: &InverseDistance<num_complex::Complex32>) -> Self::Output {
		InverseDistance{per_m: self.clone() * rhs.per_m.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseDistance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseDistance<num_complex::Complex64>;
	fn mul(self, rhs: InverseDistance<num_complex::Complex64>) -> Self::Output {
		InverseDistance{per_m: self * rhs.per_m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseDistance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseDistance<num_complex::Complex64>;
	fn mul(self, rhs: InverseDistance<num_complex::Complex64>) -> Self::Output {
		InverseDistance{per_m: self.clone() * rhs.per_m}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseDistance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseDistance<num_complex::Complex64>;
	fn mul(self, rhs: &InverseDistance<num_complex::Complex64>) -> Self::Output {
		InverseDistance{per_m: self * rhs.per_m.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseDistance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseDistance<num_complex::Complex64>;
	fn mul(self, rhs: &InverseDistance<num_complex::Complex64>) -> Self::Output {
		InverseDistance{per_m: self.clone() * rhs.per_m.clone()}
	}
}



/// Converts a InverseDistance into the equivalent [uom](https://crates.io/crates/uom) type [LinearNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.LinearNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::LinearNumberDensity> for InverseDistance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::LinearNumberDensity {
		uom::si::f32::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(self.per_m.into())
	}
}

/// Creates a InverseDistance from the equivalent [uom](https://crates.io/crates/uom) type [LinearNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.LinearNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::LinearNumberDensity> for InverseDistance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::LinearNumberDensity) -> Self {
		InverseDistance{per_m: T::from(src.value)}
	}
}

/// Converts a InverseDistance into the equivalent [uom](https://crates.io/crates/uom) type [LinearNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.LinearNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::LinearNumberDensity> for InverseDistance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::LinearNumberDensity {
		uom::si::f64::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(self.per_m.into())
	}
}

/// Creates a InverseDistance from the equivalent [uom](https://crates.io/crates/uom) type [LinearNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.LinearNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::LinearNumberDensity> for InverseDistance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::LinearNumberDensity) -> Self {
		InverseDistance{per_m: T::from(src.value)}
	}
}


// InverseDistance / Distance -> InverseArea
/// Dividing a InverseDistance by a Distance returns a value of type InverseArea
impl<T> core::ops::Div<Distance<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m / rhs.m}
	}
}
/// Dividing a InverseDistance by a Distance returns a value of type InverseArea
impl<T> core::ops::Div<Distance<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m.clone() / rhs.m}
	}
}
/// Dividing a InverseDistance by a Distance returns a value of type InverseArea
impl<T> core::ops::Div<&Distance<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m / rhs.m.clone()}
	}
}
/// Dividing a InverseDistance by a Distance returns a value of type InverseArea
impl<T> core::ops::Div<&Distance<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m.clone() / rhs.m.clone()}
	}
}

// InverseDistance * InverseDistance -> InverseArea
/// Multiplying a InverseDistance by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Mul<InverseDistance<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m * rhs.per_m}
	}
}
/// Multiplying a InverseDistance by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Mul<InverseDistance<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m.clone() * rhs.per_m}
	}
}
/// Multiplying a InverseDistance by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseDistance<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m * rhs.per_m.clone()}
	}
}
/// Multiplying a InverseDistance by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseDistance<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m.clone() * rhs.per_m.clone()}
	}
}

// InverseDistance * Time -> TimePerDistance
/// Multiplying a InverseDistance by a Time returns a value of type TimePerDistance
impl<T> core::ops::Mul<Time<T>> for InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m * rhs.s}
	}
}
/// Multiplying a InverseDistance by a Time returns a value of type TimePerDistance
impl<T> core::ops::Mul<Time<T>> for &InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m.clone() * rhs.s}
	}
}
/// Multiplying a InverseDistance by a Time returns a value of type TimePerDistance
impl<T> core::ops::Mul<&Time<T>> for InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m * rhs.s.clone()}
	}
}
/// Multiplying a InverseDistance by a Time returns a value of type TimePerDistance
impl<T> core::ops::Mul<&Time<T>> for &InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m.clone() * rhs.s.clone()}
	}
}

// InverseDistance * Area -> Distance
/// Multiplying a InverseDistance by a Area returns a value of type Distance
impl<T> core::ops::Mul<Area<T>> for InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Distance{m: self.per_m * rhs.m2}
	}
}
/// Multiplying a InverseDistance by a Area returns a value of type Distance
impl<T> core::ops::Mul<Area<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Distance{m: self.per_m.clone() * rhs.m2}
	}
}
/// Multiplying a InverseDistance by a Area returns a value of type Distance
impl<T> core::ops::Mul<&Area<T>> for InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Distance{m: self.per_m * rhs.m2.clone()}
	}
}
/// Multiplying a InverseDistance by a Area returns a value of type Distance
impl<T> core::ops::Mul<&Area<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Distance{m: self.per_m.clone() * rhs.m2.clone()}
	}
}

// InverseDistance / Area -> InverseVolume
/// Dividing a InverseDistance by a Area returns a value of type InverseVolume
impl<T> core::ops::Div<Area<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m / rhs.m2}
	}
}
/// Dividing a InverseDistance by a Area returns a value of type InverseVolume
impl<T> core::ops::Div<Area<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m.clone() / rhs.m2}
	}
}
/// Dividing a InverseDistance by a Area returns a value of type InverseVolume
impl<T> core::ops::Div<&Area<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m / rhs.m2.clone()}
	}
}
/// Dividing a InverseDistance by a Area returns a value of type InverseVolume
impl<T> core::ops::Div<&Area<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m.clone() / rhs.m2.clone()}
	}
}

// InverseDistance * InverseArea -> InverseVolume
/// Multiplying a InverseDistance by a InverseArea returns a value of type InverseVolume
impl<T> core::ops::Mul<InverseArea<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m * rhs.per_m2}
	}
}
/// Multiplying a InverseDistance by a InverseArea returns a value of type InverseVolume
impl<T> core::ops::Mul<InverseArea<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m.clone() * rhs.per_m2}
	}
}
/// Multiplying a InverseDistance by a InverseArea returns a value of type InverseVolume
impl<T> core::ops::Mul<&InverseArea<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m * rhs.per_m2.clone()}
	}
}
/// Multiplying a InverseDistance by a InverseArea returns a value of type InverseVolume
impl<T> core::ops::Mul<&InverseArea<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m.clone() * rhs.per_m2.clone()}
	}
}

// InverseDistance / InverseArea -> Distance
/// Dividing a InverseDistance by a InverseArea returns a value of type Distance
impl<T> core::ops::Div<InverseArea<T>> for InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Distance{m: self.per_m / rhs.per_m2}
	}
}
/// Dividing a InverseDistance by a InverseArea returns a value of type Distance
impl<T> core::ops::Div<InverseArea<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Distance{m: self.per_m.clone() / rhs.per_m2}
	}
}
/// Dividing a InverseDistance by a InverseArea returns a value of type Distance
impl<T> core::ops::Div<&InverseArea<T>> for InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Distance{m: self.per_m / rhs.per_m2.clone()}
	}
}
/// Dividing a InverseDistance by a InverseArea returns a value of type Distance
impl<T> core::ops::Div<&InverseArea<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Distance{m: self.per_m.clone() / rhs.per_m2.clone()}
	}
}

// InverseDistance / InverseVolume -> Area
/// Dividing a InverseDistance by a InverseVolume returns a value of type Area
impl<T> core::ops::Div<InverseVolume<T>> for InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Area{m2: self.per_m / rhs.per_m3}
	}
}
/// Dividing a InverseDistance by a InverseVolume returns a value of type Area
impl<T> core::ops::Div<InverseVolume<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Area{m2: self.per_m.clone() / rhs.per_m3}
	}
}
/// Dividing a InverseDistance by a InverseVolume returns a value of type Area
impl<T> core::ops::Div<&InverseVolume<T>> for InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Area{m2: self.per_m / rhs.per_m3.clone()}
	}
}
/// Dividing a InverseDistance by a InverseVolume returns a value of type Area
impl<T> core::ops::Div<&InverseVolume<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Area{m2: self.per_m.clone() / rhs.per_m3.clone()}
	}
}

// InverseDistance * Volume -> Area
/// Multiplying a InverseDistance by a Volume returns a value of type Area
impl<T> core::ops::Mul<Volume<T>> for InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Area{m2: self.per_m * rhs.m3}
	}
}
/// Multiplying a InverseDistance by a Volume returns a value of type Area
impl<T> core::ops::Mul<Volume<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Area{m2: self.per_m.clone() * rhs.m3}
	}
}
/// Multiplying a InverseDistance by a Volume returns a value of type Area
impl<T> core::ops::Mul<&Volume<T>> for InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Area{m2: self.per_m * rhs.m3.clone()}
	}
}
/// Multiplying a InverseDistance by a Volume returns a value of type Area
impl<T> core::ops::Mul<&Volume<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Area{m2: self.per_m.clone() * rhs.m3.clone()}
	}
}

// InverseDistance * AreaDensity -> Density
/// Multiplying a InverseDistance by a AreaDensity returns a value of type Density
impl<T> core::ops::Mul<AreaDensity<T>> for InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		Density{kgpm3: self.per_m * rhs.kgpm2}
	}
}
/// Multiplying a InverseDistance by a AreaDensity returns a value of type Density
impl<T> core::ops::Mul<AreaDensity<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		Density{kgpm3: self.per_m.clone() * rhs.kgpm2}
	}
}
/// Multiplying a InverseDistance by a AreaDensity returns a value of type Density
impl<T> core::ops::Mul<&AreaDensity<T>> for InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		Density{kgpm3: self.per_m * rhs.kgpm2.clone()}
	}
}
/// Multiplying a InverseDistance by a AreaDensity returns a value of type Density
impl<T> core::ops::Mul<&AreaDensity<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		Density{kgpm3: self.per_m.clone() * rhs.kgpm2.clone()}
	}
}

// InverseDistance / AreaPerMass -> Density
/// Dividing a InverseDistance by a AreaPerMass returns a value of type Density
impl<T> core::ops::Div<AreaPerMass<T>> for InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: AreaPerMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m / rhs.m2_per_kg}
	}
}
/// Dividing a InverseDistance by a AreaPerMass returns a value of type Density
impl<T> core::ops::Div<AreaPerMass<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: AreaPerMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m.clone() / rhs.m2_per_kg}
	}
}
/// Dividing a InverseDistance by a AreaPerMass returns a value of type Density
impl<T> core::ops::Div<&AreaPerMass<T>> for InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &AreaPerMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m / rhs.m2_per_kg.clone()}
	}
}
/// Dividing a InverseDistance by a AreaPerMass returns a value of type Density
impl<T> core::ops::Div<&AreaPerMass<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &AreaPerMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m.clone() / rhs.m2_per_kg.clone()}
	}
}

// InverseDistance / Density -> AreaPerMass
/// Dividing a InverseDistance by a Density returns a value of type AreaPerMass
impl<T> core::ops::Div<Density<T>> for InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m / rhs.kgpm3}
	}
}
/// Dividing a InverseDistance by a Density returns a value of type AreaPerMass
impl<T> core::ops::Div<Density<T>> for &InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m.clone() / rhs.kgpm3}
	}
}
/// Dividing a InverseDistance by a Density returns a value of type AreaPerMass
impl<T> core::ops::Div<&Density<T>> for InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m / rhs.kgpm3.clone()}
	}
}
/// Dividing a InverseDistance by a Density returns a value of type AreaPerMass
impl<T> core::ops::Div<&Density<T>> for &InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m.clone() / rhs.kgpm3.clone()}
	}
}

// InverseDistance * Energy -> Force
/// Multiplying a InverseDistance by a Energy returns a value of type Force
impl<T> core::ops::Mul<Energy<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Force{N: self.per_m * rhs.J}
	}
}
/// Multiplying a InverseDistance by a Energy returns a value of type Force
impl<T> core::ops::Mul<Energy<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Force{N: self.per_m.clone() * rhs.J}
	}
}
/// Multiplying a InverseDistance by a Energy returns a value of type Force
impl<T> core::ops::Mul<&Energy<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Force{N: self.per_m * rhs.J.clone()}
	}
}
/// Multiplying a InverseDistance by a Energy returns a value of type Force
impl<T> core::ops::Mul<&Energy<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Force{N: self.per_m.clone() * rhs.J.clone()}
	}
}

// InverseDistance * Torque -> Force
/// Multiplying a InverseDistance by a Torque returns a value of type Force
impl<T> core::ops::Mul<Torque<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Force{N: self.per_m * rhs.Nm}
	}
}
/// Multiplying a InverseDistance by a Torque returns a value of type Force
impl<T> core::ops::Mul<Torque<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Force{N: self.per_m.clone() * rhs.Nm}
	}
}
/// Multiplying a InverseDistance by a Torque returns a value of type Force
impl<T> core::ops::Mul<&Torque<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Force{N: self.per_m * rhs.Nm.clone()}
	}
}
/// Multiplying a InverseDistance by a Torque returns a value of type Force
impl<T> core::ops::Mul<&Torque<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Force{N: self.per_m.clone() * rhs.Nm.clone()}
	}
}

// InverseDistance / Force -> InverseEnergy
/// Dividing a InverseDistance by a Force returns a value of type InverseEnergy
impl<T> core::ops::Div<Force<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m / rhs.N}
	}
}
/// Dividing a InverseDistance by a Force returns a value of type InverseEnergy
impl<T> core::ops::Div<Force<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m.clone() / rhs.N}
	}
}
/// Dividing a InverseDistance by a Force returns a value of type InverseEnergy
impl<T> core::ops::Div<&Force<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m / rhs.N.clone()}
	}
}
/// Dividing a InverseDistance by a Force returns a value of type InverseEnergy
impl<T> core::ops::Div<&Force<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m.clone() / rhs.N.clone()}
	}
}

// InverseDistance / Frequency -> TimePerDistance
/// Dividing a InverseDistance by a Frequency returns a value of type TimePerDistance
impl<T> core::ops::Div<Frequency<T>> for InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m / rhs.Hz}
	}
}
/// Dividing a InverseDistance by a Frequency returns a value of type TimePerDistance
impl<T> core::ops::Div<Frequency<T>> for &InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m.clone() / rhs.Hz}
	}
}
/// Dividing a InverseDistance by a Frequency returns a value of type TimePerDistance
impl<T> core::ops::Div<&Frequency<T>> for InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m / rhs.Hz.clone()}
	}
}
/// Dividing a InverseDistance by a Frequency returns a value of type TimePerDistance
impl<T> core::ops::Div<&Frequency<T>> for &InverseDistance<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		TimePerDistance{spm: self.per_m.clone() / rhs.Hz.clone()}
	}
}

// InverseDistance / InverseEnergy -> Force
/// Dividing a InverseDistance by a InverseEnergy returns a value of type Force
impl<T> core::ops::Div<InverseEnergy<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Force{N: self.per_m / rhs.per_J}
	}
}
/// Dividing a InverseDistance by a InverseEnergy returns a value of type Force
impl<T> core::ops::Div<InverseEnergy<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Force{N: self.per_m.clone() / rhs.per_J}
	}
}
/// Dividing a InverseDistance by a InverseEnergy returns a value of type Force
impl<T> core::ops::Div<&InverseEnergy<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Force{N: self.per_m / rhs.per_J.clone()}
	}
}
/// Dividing a InverseDistance by a InverseEnergy returns a value of type Force
impl<T> core::ops::Div<&InverseEnergy<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Force{N: self.per_m.clone() / rhs.per_J.clone()}
	}
}

// InverseDistance / InverseTorque -> Force
/// Dividing a InverseDistance by a InverseTorque returns a value of type Force
impl<T> core::ops::Div<InverseTorque<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Force{N: self.per_m / rhs.per_Nm}
	}
}
/// Dividing a InverseDistance by a InverseTorque returns a value of type Force
impl<T> core::ops::Div<InverseTorque<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Force{N: self.per_m.clone() / rhs.per_Nm}
	}
}
/// Dividing a InverseDistance by a InverseTorque returns a value of type Force
impl<T> core::ops::Div<&InverseTorque<T>> for InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Force{N: self.per_m / rhs.per_Nm.clone()}
	}
}
/// Dividing a InverseDistance by a InverseTorque returns a value of type Force
impl<T> core::ops::Div<&InverseTorque<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Force{N: self.per_m.clone() / rhs.per_Nm.clone()}
	}
}

// InverseDistance * InverseForce -> InverseEnergy
/// Multiplying a InverseDistance by a InverseForce returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseForce<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseForce<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m * rhs.per_N}
	}
}
/// Multiplying a InverseDistance by a InverseForce returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseForce<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseForce<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m.clone() * rhs.per_N}
	}
}
/// Multiplying a InverseDistance by a InverseForce returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseForce<T>> for InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseForce<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m * rhs.per_N.clone()}
	}
}
/// Multiplying a InverseDistance by a InverseForce returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseForce<T>> for &InverseDistance<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseForce<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m.clone() * rhs.per_N.clone()}
	}
}

// InverseDistance / TimePerDistance -> Frequency
/// Dividing a InverseDistance by a TimePerDistance returns a value of type Frequency
impl<T> core::ops::Div<TimePerDistance<T>> for InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: TimePerDistance<T>) -> Self::Output {
		Frequency{Hz: self.per_m / rhs.spm}
	}
}
/// Dividing a InverseDistance by a TimePerDistance returns a value of type Frequency
impl<T> core::ops::Div<TimePerDistance<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: TimePerDistance<T>) -> Self::Output {
		Frequency{Hz: self.per_m.clone() / rhs.spm}
	}
}
/// Dividing a InverseDistance by a TimePerDistance returns a value of type Frequency
impl<T> core::ops::Div<&TimePerDistance<T>> for InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Frequency{Hz: self.per_m / rhs.spm.clone()}
	}
}
/// Dividing a InverseDistance by a TimePerDistance returns a value of type Frequency
impl<T> core::ops::Div<&TimePerDistance<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Frequency{Hz: self.per_m.clone() / rhs.spm.clone()}
	}
}

// InverseDistance * Velocity -> Frequency
/// Multiplying a InverseDistance by a Velocity returns a value of type Frequency
impl<T> core::ops::Mul<Velocity<T>> for InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Frequency{Hz: self.per_m * rhs.mps}
	}
}
/// Multiplying a InverseDistance by a Velocity returns a value of type Frequency
impl<T> core::ops::Mul<Velocity<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Frequency{Hz: self.per_m.clone() * rhs.mps}
	}
}
/// Multiplying a InverseDistance by a Velocity returns a value of type Frequency
impl<T> core::ops::Mul<&Velocity<T>> for InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Frequency{Hz: self.per_m * rhs.mps.clone()}
	}
}
/// Multiplying a InverseDistance by a Velocity returns a value of type Frequency
impl<T> core::ops::Mul<&Velocity<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Frequency{Hz: self.per_m.clone() * rhs.mps.clone()}
	}
}

// InverseDistance * VolumePerMass -> AreaPerMass
/// Multiplying a InverseDistance by a VolumePerMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<VolumePerMass<T>> for InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: VolumePerMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m * rhs.m3_per_kg}
	}
}
/// Multiplying a InverseDistance by a VolumePerMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<VolumePerMass<T>> for &InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: VolumePerMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m.clone() * rhs.m3_per_kg}
	}
}
/// Multiplying a InverseDistance by a VolumePerMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<&VolumePerMass<T>> for InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: &VolumePerMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m * rhs.m3_per_kg.clone()}
	}
}
/// Multiplying a InverseDistance by a VolumePerMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<&VolumePerMass<T>> for &InverseDistance<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: &VolumePerMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_m.clone() * rhs.m3_per_kg.clone()}
	}
}

// InverseDistance / InverseAbsorbedDose -> Acceleration
/// Dividing a InverseDistance by a InverseAbsorbedDose returns a value of type Acceleration
impl<T> core::ops::Div<InverseAbsorbedDose<T>> for InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		Acceleration{mps2: self.per_m / rhs.per_Gy}
	}
}
/// Dividing a InverseDistance by a InverseAbsorbedDose returns a value of type Acceleration
impl<T> core::ops::Div<InverseAbsorbedDose<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		Acceleration{mps2: self.per_m.clone() / rhs.per_Gy}
	}
}
/// Dividing a InverseDistance by a InverseAbsorbedDose returns a value of type Acceleration
impl<T> core::ops::Div<&InverseAbsorbedDose<T>> for InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		Acceleration{mps2: self.per_m / rhs.per_Gy.clone()}
	}
}
/// Dividing a InverseDistance by a InverseAbsorbedDose returns a value of type Acceleration
impl<T> core::ops::Div<&InverseAbsorbedDose<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		Acceleration{mps2: self.per_m.clone() / rhs.per_Gy.clone()}
	}
}

// InverseDistance / InverseDoseEquivalent -> Acceleration
/// Dividing a InverseDistance by a InverseDoseEquivalent returns a value of type Acceleration
impl<T> core::ops::Div<InverseDoseEquivalent<T>> for InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		Acceleration{mps2: self.per_m / rhs.per_Sv}
	}
}
/// Dividing a InverseDistance by a InverseDoseEquivalent returns a value of type Acceleration
impl<T> core::ops::Div<InverseDoseEquivalent<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		Acceleration{mps2: self.per_m.clone() / rhs.per_Sv}
	}
}
/// Dividing a InverseDistance by a InverseDoseEquivalent returns a value of type Acceleration
impl<T> core::ops::Div<&InverseDoseEquivalent<T>> for InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		Acceleration{mps2: self.per_m / rhs.per_Sv.clone()}
	}
}
/// Dividing a InverseDistance by a InverseDoseEquivalent returns a value of type Acceleration
impl<T> core::ops::Div<&InverseDoseEquivalent<T>> for &InverseDistance<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		Acceleration{mps2: self.per_m.clone() / rhs.per_Sv.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<InverseDistance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
impl<T> core::ops::Div<&InverseDistance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseDistance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseDistance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseDistance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseDistance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseDistance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseDistance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseDistance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseDistance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

// 1/InverseDistance -> Distance
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseDistance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseDistance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Distance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseDistance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self) / rhs.per_m.clone()}
	}
}
/// Dividing a scalar value by a InverseDistance unit value returns a value of type Distance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseDistance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: T::from(self.clone()) / rhs.per_m.clone()}
	}
}

/// The inverse of luminosity unit type, defined as inverse candela in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseLuminosity<T: NumLike>{
	/// The value of this Inverse luminosity in inverse candela
	pub per_cd: T
}

impl<T> InverseLuminosity<T> where T: NumLike {

	/// Returns the standard unit name of inverse luminosity: "inverse candela"
	pub fn unit_name() -> &'static str { "inverse candela" }
	
	/// Returns the abbreviated name or symbol of inverse luminosity: "1/cd" for inverse candela
	pub fn unit_symbol() -> &'static str { "1/cd" }
	
	/// Returns a new inverse luminosity value from the given number of inverse candela
	///
	/// # Arguments
	/// * `per_cd` - Any number-like type, representing a quantity of inverse candela
	pub fn from_per_cd(per_cd: T) -> Self { InverseLuminosity{per_cd: per_cd} }
	
	/// Returns a copy of this inverse luminosity value in inverse candela
	pub fn to_per_cd(&self) -> T { self.per_cd.clone() }

	/// Returns a new inverse luminosity value from the given number of inverse candela
	///
	/// # Arguments
	/// * `per_candela` - Any number-like type, representing a quantity of inverse candela
	pub fn from_per_candela(per_candela: T) -> Self { InverseLuminosity{per_cd: per_candela} }
	
	/// Returns a copy of this inverse luminosity value in inverse candela
	pub fn to_per_candela(&self) -> T { self.per_cd.clone() }

}

impl<T> fmt::Display for InverseLuminosity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_cd, Self::unit_symbol())
	}
}

impl<T> InverseLuminosity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse luminosity value in inverse millicandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mcd(&self) -> T {
		return self.per_cd.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse luminosity value from the given number of inverse millicandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mcd` - Any number-like type, representing a quantity of inverse millicandela
	pub fn from_per_mcd(per_mcd: T) -> Self {
		InverseLuminosity{per_cd: per_mcd * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse luminosity value in inverse microcandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_ucd(&self) -> T {
		return self.per_cd.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse luminosity value from the given number of inverse microcandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_ucd` - Any number-like type, representing a quantity of inverse microcandela
	pub fn from_per_ucd(per_ucd: T) -> Self {
		InverseLuminosity{per_cd: per_ucd * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse luminosity value in inverse nanocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_ncd(&self) -> T {
		return self.per_cd.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse luminosity value from the given number of inverse nanocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_ncd` - Any number-like type, representing a quantity of inverse nanocandela
	pub fn from_per_ncd(per_ncd: T) -> Self {
		InverseLuminosity{per_cd: per_ncd * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse luminosity value in inverse kilocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kcd(&self) -> T {
		return self.per_cd.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse luminosity value from the given number of inverse kilocandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kcd` - Any number-like type, representing a quantity of inverse kilocandela
	pub fn from_per_kcd(per_kcd: T) -> Self {
		InverseLuminosity{per_cd: per_kcd * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse luminosity value in inverse megacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_Mcd(&self) -> T {
		return self.per_cd.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse luminosity value from the given number of inverse megacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_Mcd` - Any number-like type, representing a quantity of inverse megacandela
	pub fn from_per_Mcd(per_Mcd: T) -> Self {
		InverseLuminosity{per_cd: per_Mcd * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse luminosity value in inverse gigacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_Gcd(&self) -> T {
		return self.per_cd.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse luminosity value from the given number of inverse gigacandela
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_Gcd` - Any number-like type, representing a quantity of inverse gigacandela
	pub fn from_per_Gcd(per_Gcd: T) -> Self {
		InverseLuminosity{per_cd: per_Gcd * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseLuminosity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseLuminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseLuminosity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminosity{per_cd: self * rhs.per_cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseLuminosity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseLuminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseLuminosity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminosity{per_cd: self.clone() * rhs.per_cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseLuminosity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseLuminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseLuminosity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminosity{per_cd: self * rhs.per_cd.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseLuminosity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseLuminosity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseLuminosity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminosity{per_cd: self.clone() * rhs.per_cd.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminosity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseLuminosity<num_complex::Complex32>;
	fn mul(self, rhs: InverseLuminosity<num_complex::Complex32>) -> Self::Output {
		InverseLuminosity{per_cd: self * rhs.per_cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminosity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseLuminosity<num_complex::Complex32>;
	fn mul(self, rhs: InverseLuminosity<num_complex::Complex32>) -> Self::Output {
		InverseLuminosity{per_cd: self.clone() * rhs.per_cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminosity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseLuminosity<num_complex::Complex32>;
	fn mul(self, rhs: &InverseLuminosity<num_complex::Complex32>) -> Self::Output {
		InverseLuminosity{per_cd: self * rhs.per_cd.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminosity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseLuminosity<num_complex::Complex32>;
	fn mul(self, rhs: &InverseLuminosity<num_complex::Complex32>) -> Self::Output {
		InverseLuminosity{per_cd: self.clone() * rhs.per_cd.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminosity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseLuminosity<num_complex::Complex64>;
	fn mul(self, rhs: InverseLuminosity<num_complex::Complex64>) -> Self::Output {
		InverseLuminosity{per_cd: self * rhs.per_cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminosity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseLuminosity<num_complex::Complex64>;
	fn mul(self, rhs: InverseLuminosity<num_complex::Complex64>) -> Self::Output {
		InverseLuminosity{per_cd: self.clone() * rhs.per_cd}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminosity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseLuminosity<num_complex::Complex64>;
	fn mul(self, rhs: &InverseLuminosity<num_complex::Complex64>) -> Self::Output {
		InverseLuminosity{per_cd: self * rhs.per_cd.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminosity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseLuminosity<num_complex::Complex64>;
	fn mul(self, rhs: &InverseLuminosity<num_complex::Complex64>) -> Self::Output {
		InverseLuminosity{per_cd: self.clone() * rhs.per_cd.clone()}
	}
}




// InverseLuminosity / InverseLuminousFlux -> SolidAngle
/// Dividing a InverseLuminosity by a InverseLuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Div<InverseLuminousFlux<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd / rhs.per_lm}
	}
}
/// Dividing a InverseLuminosity by a InverseLuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd.clone() / rhs.per_lm}
	}
}
/// Dividing a InverseLuminosity by a InverseLuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd / rhs.per_lm.clone()}
	}
}
/// Dividing a InverseLuminosity by a InverseLuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd.clone() / rhs.per_lm.clone()}
	}
}

// InverseLuminosity * LuminousFlux -> SolidAngle
/// Multiplying a InverseLuminosity by a LuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Mul<LuminousFlux<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd * rhs.lm}
	}
}
/// Multiplying a InverseLuminosity by a LuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Mul<LuminousFlux<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd.clone() * rhs.lm}
	}
}
/// Multiplying a InverseLuminosity by a LuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Mul<&LuminousFlux<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd * rhs.lm.clone()}
	}
}
/// Multiplying a InverseLuminosity by a LuminousFlux returns a value of type SolidAngle
impl<T> core::ops::Mul<&LuminousFlux<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		SolidAngle{sr: self.per_cd.clone() * rhs.lm.clone()}
	}
}

// InverseLuminosity * InverseSolidAngle -> InverseLuminousFlux
/// Multiplying a InverseLuminosity by a InverseSolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<InverseSolidAngle<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd * rhs.per_sr}
	}
}
/// Multiplying a InverseLuminosity by a InverseSolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<InverseSolidAngle<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd.clone() * rhs.per_sr}
	}
}
/// Multiplying a InverseLuminosity by a InverseSolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&InverseSolidAngle<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd * rhs.per_sr.clone()}
	}
}
/// Multiplying a InverseLuminosity by a InverseSolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&InverseSolidAngle<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd.clone() * rhs.per_sr.clone()}
	}
}

// InverseLuminosity / SolidAngle -> InverseLuminousFlux
/// Dividing a InverseLuminosity by a SolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<SolidAngle<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd / rhs.sr}
	}
}
/// Dividing a InverseLuminosity by a SolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<SolidAngle<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd.clone() / rhs.sr}
	}
}
/// Dividing a InverseLuminosity by a SolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&SolidAngle<T>> for InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd / rhs.sr.clone()}
	}
}
/// Dividing a InverseLuminosity by a SolidAngle returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&SolidAngle<T>> for &InverseLuminosity<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_cd.clone() / rhs.sr.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for f64 where T: NumLike+From<f64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for f64 where T: NumLike+From<f64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for f32 where T: NumLike+From<f32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for f32 where T: NumLike+From<f32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for i64 where T: NumLike+From<i64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for i64 where T: NumLike+From<i64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for i32 where T: NumLike+From<i32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminosity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for i32 where T: NumLike+From<i32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminosity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseLuminosity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseLuminosity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseLuminosity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseLuminosity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminosity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminosity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminosity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminosity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

// 1/InverseLuminosity -> Luminosity
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminosity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminosity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminosity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self) / rhs.per_cd.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminosity unit value returns a value of type Luminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminosity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		Luminosity{cd: T::from(self.clone()) / rhs.per_cd.clone()}
	}
}

/// The inverse of mass unit type, defined as inverse kilograms in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseMass<T: NumLike>{
	/// The value of this Inverse mass in inverse kilograms
	pub per_kg: T
}

impl<T> InverseMass<T> where T: NumLike {

	/// Returns the standard unit name of inverse mass: "inverse kilograms"
	pub fn unit_name() -> &'static str { "inverse kilograms" }
	
	/// Returns the abbreviated name or symbol of inverse mass: "1/kg" for inverse kilograms
	pub fn unit_symbol() -> &'static str { "1/kg" }
	
	/// Returns a new inverse mass value from the given number of inverse kilograms
	///
	/// # Arguments
	/// * `per_kg` - Any number-like type, representing a quantity of inverse kilograms
	pub fn from_per_kg(per_kg: T) -> Self { InverseMass{per_kg: per_kg} }
	
	/// Returns a copy of this inverse mass value in inverse kilograms
	pub fn to_per_kg(&self) -> T { self.per_kg.clone() }

	/// Returns a new inverse mass value from the given number of inverse kilograms
	///
	/// # Arguments
	/// * `per_kilograms` - Any number-like type, representing a quantity of inverse kilograms
	pub fn from_per_kilograms(per_kilograms: T) -> Self { InverseMass{per_kg: per_kilograms} }
	
	/// Returns a copy of this inverse mass value in inverse kilograms
	pub fn to_per_kilograms(&self) -> T { self.per_kg.clone() }

}

impl<T> fmt::Display for InverseMass<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_kg, Self::unit_symbol())
	}
}

impl<T> InverseMass<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse mass value in inverse grams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_g(&self) -> T {
		return self.per_kg.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse grams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_g` - Any number-like type, representing a quantity of inverse grams
	pub fn from_per_g(per_g: T) -> Self {
		InverseMass{per_kg: per_g * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse milligrams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mg(&self) -> T {
		return self.per_kg.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse milligrams
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mg` - Any number-like type, representing a quantity of inverse milligrams
	pub fn from_per_mg(per_mg: T) -> Self {
		InverseMass{per_kg: per_mg * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse micrograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_ug(&self) -> T {
		return self.per_kg.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse micrograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_ug` - Any number-like type, representing a quantity of inverse micrograms
	pub fn from_per_ug(per_ug: T) -> Self {
		InverseMass{per_kg: per_ug * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse nanograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_ng(&self) -> T {
		return self.per_kg.clone() * T::from(1e-12_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse nanograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_ng` - Any number-like type, representing a quantity of inverse nanograms
	pub fn from_per_ng(per_ng: T) -> Self {
		InverseMass{per_kg: per_ng * T::from(1000000000000.0_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse picograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_pg(&self) -> T {
		return self.per_kg.clone() * T::from(1e-15_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse picograms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_pg` - Any number-like type, representing a quantity of inverse picograms
	pub fn from_per_pg(per_pg: T) -> Self {
		InverseMass{per_kg: per_pg * T::from(1000000000000000.0_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse tons
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_tons(&self) -> T {
		return self.per_kg.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse tons
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_tons` - Any number-like type, representing a quantity of inverse tons
	pub fn from_per_tons(per_tons: T) -> Self {
		InverseMass{per_kg: per_tons * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse earth masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_earth_mass(&self) -> T {
		return self.per_kg.clone() * T::from(5.97e+24_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse earth masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_earth_mass` - Any number-like type, representing a quantity of inverse earth masses
	pub fn from_per_earth_mass(per_earth_mass: T) -> Self {
		InverseMass{per_kg: per_earth_mass * T::from(1.6699999999999999e-25_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse jupiter masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_jupiter_mass(&self) -> T {
		return self.per_kg.clone() * T::from(1.9e+27_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse jupiter masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_jupiter_mass` - Any number-like type, representing a quantity of inverse jupiter masses
	pub fn from_per_jupiter_mass(per_jupiter_mass: T) -> Self {
		InverseMass{per_kg: per_jupiter_mass * T::from(5.27e-28_f64)}
	}

	/// Returns a copy of this inverse mass value in inverse solar masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_solar_mass(&self) -> T {
		return self.per_kg.clone() * T::from(1.99e+30_f64);
	}

	/// Returns a new inverse mass value from the given number of inverse solar masses
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_solar_mass` - Any number-like type, representing a quantity of inverse solar masses
	pub fn from_per_solar_mass(per_solar_mass: T) -> Self {
		InverseMass{per_kg: per_solar_mass * T::from(5.03e-31_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseMass<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseMass<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMass{per_kg: self * rhs.per_kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseMass<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseMass<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMass{per_kg: self.clone() * rhs.per_kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseMass<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseMass<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMass{per_kg: self * rhs.per_kg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseMass<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseMass<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMass{per_kg: self.clone() * rhs.per_kg.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMass<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseMass<num_complex::Complex32>;
	fn mul(self, rhs: InverseMass<num_complex::Complex32>) -> Self::Output {
		InverseMass{per_kg: self * rhs.per_kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMass<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseMass<num_complex::Complex32>;
	fn mul(self, rhs: InverseMass<num_complex::Complex32>) -> Self::Output {
		InverseMass{per_kg: self.clone() * rhs.per_kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMass<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseMass<num_complex::Complex32>;
	fn mul(self, rhs: &InverseMass<num_complex::Complex32>) -> Self::Output {
		InverseMass{per_kg: self * rhs.per_kg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMass<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseMass<num_complex::Complex32>;
	fn mul(self, rhs: &InverseMass<num_complex::Complex32>) -> Self::Output {
		InverseMass{per_kg: self.clone() * rhs.per_kg.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMass<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseMass<num_complex::Complex64>;
	fn mul(self, rhs: InverseMass<num_complex::Complex64>) -> Self::Output {
		InverseMass{per_kg: self * rhs.per_kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMass<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseMass<num_complex::Complex64>;
	fn mul(self, rhs: InverseMass<num_complex::Complex64>) -> Self::Output {
		InverseMass{per_kg: self.clone() * rhs.per_kg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMass<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseMass<num_complex::Complex64>;
	fn mul(self, rhs: &InverseMass<num_complex::Complex64>) -> Self::Output {
		InverseMass{per_kg: self * rhs.per_kg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMass<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseMass<num_complex::Complex64>;
	fn mul(self, rhs: &InverseMass<num_complex::Complex64>) -> Self::Output {
		InverseMass{per_kg: self.clone() * rhs.per_kg.clone()}
	}
}




// InverseMass * Amount -> Molality
/// Multiplying a InverseMass by a Amount returns a value of type Molality
impl<T> core::ops::Mul<Amount<T>> for InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg * rhs.mol}
	}
}
/// Multiplying a InverseMass by a Amount returns a value of type Molality
impl<T> core::ops::Mul<Amount<T>> for &InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg.clone() * rhs.mol}
	}
}
/// Multiplying a InverseMass by a Amount returns a value of type Molality
impl<T> core::ops::Mul<&Amount<T>> for InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg * rhs.mol.clone()}
	}
}
/// Multiplying a InverseMass by a Amount returns a value of type Molality
impl<T> core::ops::Mul<&Amount<T>> for &InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg.clone() * rhs.mol.clone()}
	}
}

// InverseMass / InverseAmount -> Molality
/// Dividing a InverseMass by a InverseAmount returns a value of type Molality
impl<T> core::ops::Div<InverseAmount<T>> for InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg / rhs.per_mol}
	}
}
/// Dividing a InverseMass by a InverseAmount returns a value of type Molality
impl<T> core::ops::Div<InverseAmount<T>> for &InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg.clone() / rhs.per_mol}
	}
}
/// Dividing a InverseMass by a InverseAmount returns a value of type Molality
impl<T> core::ops::Div<&InverseAmount<T>> for InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg / rhs.per_mol.clone()}
	}
}
/// Dividing a InverseMass by a InverseAmount returns a value of type Molality
impl<T> core::ops::Div<&InverseAmount<T>> for &InverseMass<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Molality{molpkg: self.per_kg.clone() / rhs.per_mol.clone()}
	}
}

// InverseMass / Molality -> InverseAmount
/// Dividing a InverseMass by a Molality returns a value of type InverseAmount
impl<T> core::ops::Div<Molality<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg / rhs.molpkg}
	}
}
/// Dividing a InverseMass by a Molality returns a value of type InverseAmount
impl<T> core::ops::Div<Molality<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg.clone() / rhs.molpkg}
	}
}
/// Dividing a InverseMass by a Molality returns a value of type InverseAmount
impl<T> core::ops::Div<&Molality<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg / rhs.molpkg.clone()}
	}
}
/// Dividing a InverseMass by a Molality returns a value of type InverseAmount
impl<T> core::ops::Div<&Molality<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg.clone() / rhs.molpkg.clone()}
	}
}

// InverseMass * MolarMass -> InverseAmount
/// Multiplying a InverseMass by a MolarMass returns a value of type InverseAmount
impl<T> core::ops::Mul<MolarMass<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: MolarMass<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg * rhs.kgpmol}
	}
}
/// Multiplying a InverseMass by a MolarMass returns a value of type InverseAmount
impl<T> core::ops::Mul<MolarMass<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: MolarMass<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg.clone() * rhs.kgpmol}
	}
}
/// Multiplying a InverseMass by a MolarMass returns a value of type InverseAmount
impl<T> core::ops::Mul<&MolarMass<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: &MolarMass<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg * rhs.kgpmol.clone()}
	}
}
/// Multiplying a InverseMass by a MolarMass returns a value of type InverseAmount
impl<T> core::ops::Mul<&MolarMass<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: &MolarMass<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_kg.clone() * rhs.kgpmol.clone()}
	}
}

// InverseMass * Area -> AreaPerMass
/// Multiplying a InverseMass by a Area returns a value of type AreaPerMass
impl<T> core::ops::Mul<Area<T>> for InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg * rhs.m2}
	}
}
/// Multiplying a InverseMass by a Area returns a value of type AreaPerMass
impl<T> core::ops::Mul<Area<T>> for &InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg.clone() * rhs.m2}
	}
}
/// Multiplying a InverseMass by a Area returns a value of type AreaPerMass
impl<T> core::ops::Mul<&Area<T>> for InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg * rhs.m2.clone()}
	}
}
/// Multiplying a InverseMass by a Area returns a value of type AreaPerMass
impl<T> core::ops::Mul<&Area<T>> for &InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg.clone() * rhs.m2.clone()}
	}
}

// InverseMass / InverseArea -> AreaPerMass
/// Dividing a InverseMass by a InverseArea returns a value of type AreaPerMass
impl<T> core::ops::Div<InverseArea<T>> for InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg / rhs.per_m2}
	}
}
/// Dividing a InverseMass by a InverseArea returns a value of type AreaPerMass
impl<T> core::ops::Div<InverseArea<T>> for &InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg.clone() / rhs.per_m2}
	}
}
/// Dividing a InverseMass by a InverseArea returns a value of type AreaPerMass
impl<T> core::ops::Div<&InverseArea<T>> for InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg / rhs.per_m2.clone()}
	}
}
/// Dividing a InverseMass by a InverseArea returns a value of type AreaPerMass
impl<T> core::ops::Div<&InverseArea<T>> for &InverseMass<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.per_kg.clone() / rhs.per_m2.clone()}
	}
}

// InverseMass / InverseVolume -> VolumePerMass
/// Dividing a InverseMass by a InverseVolume returns a value of type VolumePerMass
impl<T> core::ops::Div<InverseVolume<T>> for InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg / rhs.per_m3}
	}
}
/// Dividing a InverseMass by a InverseVolume returns a value of type VolumePerMass
impl<T> core::ops::Div<InverseVolume<T>> for &InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg.clone() / rhs.per_m3}
	}
}
/// Dividing a InverseMass by a InverseVolume returns a value of type VolumePerMass
impl<T> core::ops::Div<&InverseVolume<T>> for InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg / rhs.per_m3.clone()}
	}
}
/// Dividing a InverseMass by a InverseVolume returns a value of type VolumePerMass
impl<T> core::ops::Div<&InverseVolume<T>> for &InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg.clone() / rhs.per_m3.clone()}
	}
}

// InverseMass * Volume -> VolumePerMass
/// Multiplying a InverseMass by a Volume returns a value of type VolumePerMass
impl<T> core::ops::Mul<Volume<T>> for InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg * rhs.m3}
	}
}
/// Multiplying a InverseMass by a Volume returns a value of type VolumePerMass
impl<T> core::ops::Mul<Volume<T>> for &InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg.clone() * rhs.m3}
	}
}
/// Multiplying a InverseMass by a Volume returns a value of type VolumePerMass
impl<T> core::ops::Mul<&Volume<T>> for InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg * rhs.m3.clone()}
	}
}
/// Multiplying a InverseMass by a Volume returns a value of type VolumePerMass
impl<T> core::ops::Mul<&Volume<T>> for &InverseMass<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.per_kg.clone() * rhs.m3.clone()}
	}
}

// InverseMass / Acceleration -> InverseForce
/// Dividing a InverseMass by a Acceleration returns a value of type InverseForce
impl<T> core::ops::Div<Acceleration<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg / rhs.mps2}
	}
}
/// Dividing a InverseMass by a Acceleration returns a value of type InverseForce
impl<T> core::ops::Div<Acceleration<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg.clone() / rhs.mps2}
	}
}
/// Dividing a InverseMass by a Acceleration returns a value of type InverseForce
impl<T> core::ops::Div<&Acceleration<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg / rhs.mps2.clone()}
	}
}
/// Dividing a InverseMass by a Acceleration returns a value of type InverseForce
impl<T> core::ops::Div<&Acceleration<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg.clone() / rhs.mps2.clone()}
	}
}

// InverseMass * AreaDensity -> InverseArea
/// Multiplying a InverseMass by a AreaDensity returns a value of type InverseArea
impl<T> core::ops::Mul<AreaDensity<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg * rhs.kgpm2}
	}
}
/// Multiplying a InverseMass by a AreaDensity returns a value of type InverseArea
impl<T> core::ops::Mul<AreaDensity<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg.clone() * rhs.kgpm2}
	}
}
/// Multiplying a InverseMass by a AreaDensity returns a value of type InverseArea
impl<T> core::ops::Mul<&AreaDensity<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg * rhs.kgpm2.clone()}
	}
}
/// Multiplying a InverseMass by a AreaDensity returns a value of type InverseArea
impl<T> core::ops::Mul<&AreaDensity<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg.clone() * rhs.kgpm2.clone()}
	}
}

// InverseMass / AreaPerMass -> InverseArea
/// Dividing a InverseMass by a AreaPerMass returns a value of type InverseArea
impl<T> core::ops::Div<AreaPerMass<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: AreaPerMass<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg / rhs.m2_per_kg}
	}
}
/// Dividing a InverseMass by a AreaPerMass returns a value of type InverseArea
impl<T> core::ops::Div<AreaPerMass<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: AreaPerMass<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg.clone() / rhs.m2_per_kg}
	}
}
/// Dividing a InverseMass by a AreaPerMass returns a value of type InverseArea
impl<T> core::ops::Div<&AreaPerMass<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &AreaPerMass<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg / rhs.m2_per_kg.clone()}
	}
}
/// Dividing a InverseMass by a AreaPerMass returns a value of type InverseArea
impl<T> core::ops::Div<&AreaPerMass<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &AreaPerMass<T>) -> Self::Output {
		InverseArea{per_m2: self.per_kg.clone() / rhs.m2_per_kg.clone()}
	}
}

// InverseMass * Density -> InverseVolume
/// Multiplying a InverseMass by a Density returns a value of type InverseVolume
impl<T> core::ops::Mul<Density<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg * rhs.kgpm3}
	}
}
/// Multiplying a InverseMass by a Density returns a value of type InverseVolume
impl<T> core::ops::Mul<Density<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg.clone() * rhs.kgpm3}
	}
}
/// Multiplying a InverseMass by a Density returns a value of type InverseVolume
impl<T> core::ops::Mul<&Density<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg * rhs.kgpm3.clone()}
	}
}
/// Multiplying a InverseMass by a Density returns a value of type InverseVolume
impl<T> core::ops::Mul<&Density<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg.clone() * rhs.kgpm3.clone()}
	}
}

// InverseMass * Force -> Acceleration
/// Multiplying a InverseMass by a Force returns a value of type Acceleration
impl<T> core::ops::Mul<Force<T>> for InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg * rhs.N}
	}
}
/// Multiplying a InverseMass by a Force returns a value of type Acceleration
impl<T> core::ops::Mul<Force<T>> for &InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg.clone() * rhs.N}
	}
}
/// Multiplying a InverseMass by a Force returns a value of type Acceleration
impl<T> core::ops::Mul<&Force<T>> for InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg * rhs.N.clone()}
	}
}
/// Multiplying a InverseMass by a Force returns a value of type Acceleration
impl<T> core::ops::Mul<&Force<T>> for &InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg.clone() * rhs.N.clone()}
	}
}

// InverseMass * InverseAcceleration -> InverseForce
/// Multiplying a InverseMass by a InverseAcceleration returns a value of type InverseForce
impl<T> core::ops::Mul<InverseAcceleration<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseAcceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg * rhs.s2pm}
	}
}
/// Multiplying a InverseMass by a InverseAcceleration returns a value of type InverseForce
impl<T> core::ops::Mul<InverseAcceleration<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseAcceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg.clone() * rhs.s2pm}
	}
}
/// Multiplying a InverseMass by a InverseAcceleration returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseAcceleration<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseAcceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg * rhs.s2pm.clone()}
	}
}
/// Multiplying a InverseMass by a InverseAcceleration returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseAcceleration<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseAcceleration<T>) -> Self::Output {
		InverseForce{per_N: self.per_kg.clone() * rhs.s2pm.clone()}
	}
}

// InverseMass / InverseForce -> Acceleration
/// Dividing a InverseMass by a InverseForce returns a value of type Acceleration
impl<T> core::ops::Div<InverseForce<T>> for InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg / rhs.per_N}
	}
}
/// Dividing a InverseMass by a InverseForce returns a value of type Acceleration
impl<T> core::ops::Div<InverseForce<T>> for &InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg.clone() / rhs.per_N}
	}
}
/// Dividing a InverseMass by a InverseForce returns a value of type Acceleration
impl<T> core::ops::Div<&InverseForce<T>> for InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg / rhs.per_N.clone()}
	}
}
/// Dividing a InverseMass by a InverseForce returns a value of type Acceleration
impl<T> core::ops::Div<&InverseForce<T>> for &InverseMass<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Acceleration{mps2: self.per_kg.clone() / rhs.per_N.clone()}
	}
}

// InverseMass / InverseMomentOfInertia -> Area
/// Dividing a InverseMass by a InverseMomentOfInertia returns a value of type Area
impl<T> core::ops::Div<InverseMomentOfInertia<T>> for InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg / rhs.per_kgm2}
	}
}
/// Dividing a InverseMass by a InverseMomentOfInertia returns a value of type Area
impl<T> core::ops::Div<InverseMomentOfInertia<T>> for &InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg.clone() / rhs.per_kgm2}
	}
}
/// Dividing a InverseMass by a InverseMomentOfInertia returns a value of type Area
impl<T> core::ops::Div<&InverseMomentOfInertia<T>> for InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg / rhs.per_kgm2.clone()}
	}
}
/// Dividing a InverseMass by a InverseMomentOfInertia returns a value of type Area
impl<T> core::ops::Div<&InverseMomentOfInertia<T>> for &InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg.clone() / rhs.per_kgm2.clone()}
	}
}

// InverseMass / InverseMomentum -> Velocity
/// Dividing a InverseMass by a InverseMomentum returns a value of type Velocity
impl<T> core::ops::Div<InverseMomentum<T>> for InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: InverseMomentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg / rhs.s_per_kgm}
	}
}
/// Dividing a InverseMass by a InverseMomentum returns a value of type Velocity
impl<T> core::ops::Div<InverseMomentum<T>> for &InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: InverseMomentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg.clone() / rhs.s_per_kgm}
	}
}
/// Dividing a InverseMass by a InverseMomentum returns a value of type Velocity
impl<T> core::ops::Div<&InverseMomentum<T>> for InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &InverseMomentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg / rhs.s_per_kgm.clone()}
	}
}
/// Dividing a InverseMass by a InverseMomentum returns a value of type Velocity
impl<T> core::ops::Div<&InverseMomentum<T>> for &InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &InverseMomentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg.clone() / rhs.s_per_kgm.clone()}
	}
}

// InverseMass * MomentOfInertia -> Area
/// Multiplying a InverseMass by a MomentOfInertia returns a value of type Area
impl<T> core::ops::Mul<MomentOfInertia<T>> for InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg * rhs.kgm2}
	}
}
/// Multiplying a InverseMass by a MomentOfInertia returns a value of type Area
impl<T> core::ops::Mul<MomentOfInertia<T>> for &InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg.clone() * rhs.kgm2}
	}
}
/// Multiplying a InverseMass by a MomentOfInertia returns a value of type Area
impl<T> core::ops::Mul<&MomentOfInertia<T>> for InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg * rhs.kgm2.clone()}
	}
}
/// Multiplying a InverseMass by a MomentOfInertia returns a value of type Area
impl<T> core::ops::Mul<&MomentOfInertia<T>> for &InverseMass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		Area{m2: self.per_kg.clone() * rhs.kgm2.clone()}
	}
}

// InverseMass * Momentum -> Velocity
/// Multiplying a InverseMass by a Momentum returns a value of type Velocity
impl<T> core::ops::Mul<Momentum<T>> for InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg * rhs.kgmps}
	}
}
/// Multiplying a InverseMass by a Momentum returns a value of type Velocity
impl<T> core::ops::Mul<Momentum<T>> for &InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg.clone() * rhs.kgmps}
	}
}
/// Multiplying a InverseMass by a Momentum returns a value of type Velocity
impl<T> core::ops::Mul<&Momentum<T>> for InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg * rhs.kgmps.clone()}
	}
}
/// Multiplying a InverseMass by a Momentum returns a value of type Velocity
impl<T> core::ops::Mul<&Momentum<T>> for &InverseMass<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Velocity{mps: self.per_kg.clone() * rhs.kgmps.clone()}
	}
}

// InverseMass * TimePerDistance -> InverseMomentum
/// Multiplying a InverseMass by a TimePerDistance returns a value of type InverseMomentum
impl<T> core::ops::Mul<TimePerDistance<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn mul(self, rhs: TimePerDistance<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg * rhs.spm}
	}
}
/// Multiplying a InverseMass by a TimePerDistance returns a value of type InverseMomentum
impl<T> core::ops::Mul<TimePerDistance<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn mul(self, rhs: TimePerDistance<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg.clone() * rhs.spm}
	}
}
/// Multiplying a InverseMass by a TimePerDistance returns a value of type InverseMomentum
impl<T> core::ops::Mul<&TimePerDistance<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn mul(self, rhs: &TimePerDistance<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg * rhs.spm.clone()}
	}
}
/// Multiplying a InverseMass by a TimePerDistance returns a value of type InverseMomentum
impl<T> core::ops::Mul<&TimePerDistance<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn mul(self, rhs: &TimePerDistance<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg.clone() * rhs.spm.clone()}
	}
}

// InverseMass / Velocity -> InverseMomentum
/// Dividing a InverseMass by a Velocity returns a value of type InverseMomentum
impl<T> core::ops::Div<Velocity<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg / rhs.mps}
	}
}
/// Dividing a InverseMass by a Velocity returns a value of type InverseMomentum
impl<T> core::ops::Div<Velocity<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg.clone() / rhs.mps}
	}
}
/// Dividing a InverseMass by a Velocity returns a value of type InverseMomentum
impl<T> core::ops::Div<&Velocity<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg / rhs.mps.clone()}
	}
}
/// Dividing a InverseMass by a Velocity returns a value of type InverseMomentum
impl<T> core::ops::Div<&Velocity<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseMomentum<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		InverseMomentum{s_per_kgm: self.per_kg.clone() / rhs.mps.clone()}
	}
}

// InverseMass / VolumePerMass -> InverseVolume
/// Dividing a InverseMass by a VolumePerMass returns a value of type InverseVolume
impl<T> core::ops::Div<VolumePerMass<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: VolumePerMass<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg / rhs.m3_per_kg}
	}
}
/// Dividing a InverseMass by a VolumePerMass returns a value of type InverseVolume
impl<T> core::ops::Div<VolumePerMass<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: VolumePerMass<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg.clone() / rhs.m3_per_kg}
	}
}
/// Dividing a InverseMass by a VolumePerMass returns a value of type InverseVolume
impl<T> core::ops::Div<&VolumePerMass<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &VolumePerMass<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg / rhs.m3_per_kg.clone()}
	}
}
/// Dividing a InverseMass by a VolumePerMass returns a value of type InverseVolume
impl<T> core::ops::Div<&VolumePerMass<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &VolumePerMass<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_kg.clone() / rhs.m3_per_kg.clone()}
	}
}

// InverseMass * InverseAbsorbedDose -> InverseEnergy
/// Multiplying a InverseMass by a InverseAbsorbedDose returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseAbsorbedDose<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg * rhs.per_Gy}
	}
}
/// Multiplying a InverseMass by a InverseAbsorbedDose returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseAbsorbedDose<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg.clone() * rhs.per_Gy}
	}
}
/// Multiplying a InverseMass by a InverseAbsorbedDose returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseAbsorbedDose<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg * rhs.per_Gy.clone()}
	}
}
/// Multiplying a InverseMass by a InverseAbsorbedDose returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseAbsorbedDose<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg.clone() * rhs.per_Gy.clone()}
	}
}

// InverseMass * InverseDoseEquivalent -> InverseEnergy
/// Multiplying a InverseMass by a InverseDoseEquivalent returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseDoseEquivalent<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg * rhs.per_Sv}
	}
}
/// Multiplying a InverseMass by a InverseDoseEquivalent returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseDoseEquivalent<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg.clone() * rhs.per_Sv}
	}
}
/// Multiplying a InverseMass by a InverseDoseEquivalent returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseDoseEquivalent<T>> for InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg * rhs.per_Sv.clone()}
	}
}
/// Multiplying a InverseMass by a InverseDoseEquivalent returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseDoseEquivalent<T>> for &InverseMass<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_kg.clone() * rhs.per_Sv.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for f64 where T: NumLike+From<f64> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for f64 where T: NumLike+From<f64> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for f32 where T: NumLike+From<f32> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for f32 where T: NumLike+From<f32> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for i64 where T: NumLike+From<i64> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for i64 where T: NumLike+From<i64> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for i32 where T: NumLike+From<i32> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<InverseMass<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for i32 where T: NumLike+From<i32> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
impl<T> core::ops::Div<&InverseMass<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseMass<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseMass<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseMass<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseMass<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMass<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMass<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMass<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMass<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

// 1/InverseMass -> Mass
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMass<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMass<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMass<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self) / rhs.per_kg.clone()}
	}
}
/// Dividing a scalar value by a InverseMass unit value returns a value of type Mass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMass<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Mass{kg: T::from(self.clone()) / rhs.per_kg.clone()}
	}
}

/// The inverse of temperature unit type, defined as inverse degrees kelvin in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseTemperature<T: NumLike>{
	/// The value of this Inverse temperature in inverse degrees kelvin
	pub per_K: T
}

impl<T> InverseTemperature<T> where T: NumLike {

	/// Returns the standard unit name of inverse temperature: "inverse degrees kelvin"
	pub fn unit_name() -> &'static str { "inverse degrees kelvin" }
	
	/// Returns the abbreviated name or symbol of inverse temperature: "1/K" for inverse degrees kelvin
	pub fn unit_symbol() -> &'static str { "1/K" }
	
	/// Returns a new inverse temperature value from the given number of inverse degrees kelvin
	///
	/// # Arguments
	/// * `per_K` - Any number-like type, representing a quantity of inverse degrees kelvin
	pub fn from_per_K(per_K: T) -> Self { InverseTemperature{per_K: per_K} }
	
	/// Returns a copy of this inverse temperature value in inverse degrees kelvin
	pub fn to_per_K(&self) -> T { self.per_K.clone() }

}

impl<T> fmt::Display for InverseTemperature<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_K, Self::unit_symbol())
	}
}

impl<T> InverseTemperature<T> where T: NumLike+From<f64> {
	
}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseTemperature<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseTemperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseTemperature<num_bigfloat::BigFloat>) -> Self::Output {
		InverseTemperature{per_K: self * rhs.per_K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseTemperature<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseTemperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseTemperature<num_bigfloat::BigFloat>) -> Self::Output {
		InverseTemperature{per_K: self.clone() * rhs.per_K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseTemperature<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseTemperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseTemperature<num_bigfloat::BigFloat>) -> Self::Output {
		InverseTemperature{per_K: self * rhs.per_K.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseTemperature<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseTemperature<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseTemperature<num_bigfloat::BigFloat>) -> Self::Output {
		InverseTemperature{per_K: self.clone() * rhs.per_K.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseTemperature<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseTemperature<num_complex::Complex32>;
	fn mul(self, rhs: InverseTemperature<num_complex::Complex32>) -> Self::Output {
		InverseTemperature{per_K: self * rhs.per_K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseTemperature<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseTemperature<num_complex::Complex32>;
	fn mul(self, rhs: InverseTemperature<num_complex::Complex32>) -> Self::Output {
		InverseTemperature{per_K: self.clone() * rhs.per_K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseTemperature<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseTemperature<num_complex::Complex32>;
	fn mul(self, rhs: &InverseTemperature<num_complex::Complex32>) -> Self::Output {
		InverseTemperature{per_K: self * rhs.per_K.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseTemperature<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseTemperature<num_complex::Complex32>;
	fn mul(self, rhs: &InverseTemperature<num_complex::Complex32>) -> Self::Output {
		InverseTemperature{per_K: self.clone() * rhs.per_K.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseTemperature<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseTemperature<num_complex::Complex64>;
	fn mul(self, rhs: InverseTemperature<num_complex::Complex64>) -> Self::Output {
		InverseTemperature{per_K: self * rhs.per_K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseTemperature<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseTemperature<num_complex::Complex64>;
	fn mul(self, rhs: InverseTemperature<num_complex::Complex64>) -> Self::Output {
		InverseTemperature{per_K: self.clone() * rhs.per_K}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseTemperature<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseTemperature<num_complex::Complex64>;
	fn mul(self, rhs: &InverseTemperature<num_complex::Complex64>) -> Self::Output {
		InverseTemperature{per_K: self * rhs.per_K.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseTemperature<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseTemperature<num_complex::Complex64>;
	fn mul(self, rhs: &InverseTemperature<num_complex::Complex64>) -> Self::Output {
		InverseTemperature{per_K: self.clone() * rhs.per_K.clone()}
	}
}



/// Converts a InverseTemperature into the equivalent [uom](https://crates.io/crates/uom) type [TemperatureCoefficient](https://docs.rs/uom/0.34.0/uom/si/f32/type.TemperatureCoefficient.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::TemperatureCoefficient> for InverseTemperature<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::TemperatureCoefficient {
		uom::si::f32::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(self.per_K.into())
	}
}

/// Creates a InverseTemperature from the equivalent [uom](https://crates.io/crates/uom) type [TemperatureCoefficient](https://docs.rs/uom/0.34.0/uom/si/f32/type.TemperatureCoefficient.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::TemperatureCoefficient> for InverseTemperature<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::TemperatureCoefficient) -> Self {
		InverseTemperature{per_K: T::from(src.value)}
	}
}

/// Converts a InverseTemperature into the equivalent [uom](https://crates.io/crates/uom) type [TemperatureCoefficient](https://docs.rs/uom/0.34.0/uom/si/f64/type.TemperatureCoefficient.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::TemperatureCoefficient> for InverseTemperature<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::TemperatureCoefficient {
		uom::si::f64::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(self.per_K.into())
	}
}

/// Creates a InverseTemperature from the equivalent [uom](https://crates.io/crates/uom) type [TemperatureCoefficient](https://docs.rs/uom/0.34.0/uom/si/f64/type.TemperatureCoefficient.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::TemperatureCoefficient> for InverseTemperature<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::TemperatureCoefficient) -> Self {
		InverseTemperature{per_K: T::from(src.value)}
	}
}


// InverseTemperature / InverseAbsorbedDose -> SpecificHeatCapacity
/// Dividing a InverseTemperature by a InverseAbsorbedDose returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<InverseAbsorbedDose<T>> for InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K / rhs.per_Gy}
	}
}
/// Dividing a InverseTemperature by a InverseAbsorbedDose returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<InverseAbsorbedDose<T>> for &InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K.clone() / rhs.per_Gy}
	}
}
/// Dividing a InverseTemperature by a InverseAbsorbedDose returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<&InverseAbsorbedDose<T>> for InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K / rhs.per_Gy.clone()}
	}
}
/// Dividing a InverseTemperature by a InverseAbsorbedDose returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<&InverseAbsorbedDose<T>> for &InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K.clone() / rhs.per_Gy.clone()}
	}
}

// InverseTemperature / InverseDoseEquivalent -> SpecificHeatCapacity
/// Dividing a InverseTemperature by a InverseDoseEquivalent returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<InverseDoseEquivalent<T>> for InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K / rhs.per_Sv}
	}
}
/// Dividing a InverseTemperature by a InverseDoseEquivalent returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<InverseDoseEquivalent<T>> for &InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K.clone() / rhs.per_Sv}
	}
}
/// Dividing a InverseTemperature by a InverseDoseEquivalent returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<&InverseDoseEquivalent<T>> for InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K / rhs.per_Sv.clone()}
	}
}
/// Dividing a InverseTemperature by a InverseDoseEquivalent returns a value of type SpecificHeatCapacity
impl<T> core::ops::Div<&InverseDoseEquivalent<T>> for &InverseTemperature<T> where T: NumLike {
	type Output = SpecificHeatCapacity<T>;
	fn div(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		SpecificHeatCapacity{J_per_kgK: self.per_K.clone() / rhs.per_Sv.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for f64 where T: NumLike+From<f64> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for f64 where T: NumLike+From<f64> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for f32 where T: NumLike+From<f32> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for f32 where T: NumLike+From<f32> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for i64 where T: NumLike+From<i64> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for i64 where T: NumLike+From<i64> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for i32 where T: NumLike+From<i32> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<InverseTemperature<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for i32 where T: NumLike+From<i32> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
impl<T> core::ops::Div<&InverseTemperature<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseTemperature<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseTemperature<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseTemperature<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseTemperature<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseTemperature<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseTemperature<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseTemperature<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseTemperature<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
	}
}

// 1/InverseTemperature -> Temperature
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseTemperature<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseTemperature<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Temperature<T>;
	fn div(self, rhs: InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseTemperature<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self) / rhs.per_K.clone()}
	}
}
/// Dividing a scalar value by a InverseTemperature unit value returns a value of type Temperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseTemperature<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Temperature<T>;
	fn div(self, rhs: &InverseTemperature<T>) -> Self::Output {
		Temperature{K: T::from(self.clone()) / rhs.per_K.clone()}
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


// Luminosity * InverseLuminousFlux -> InverseSolidAngle
/// Multiplying a Luminosity by a InverseLuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd * rhs.per_lm}
	}
}
/// Multiplying a Luminosity by a InverseLuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for &Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd.clone() * rhs.per_lm}
	}
}
/// Multiplying a Luminosity by a InverseLuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd * rhs.per_lm.clone()}
	}
}
/// Multiplying a Luminosity by a InverseLuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for &Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd.clone() * rhs.per_lm.clone()}
	}
}

// Luminosity / LuminousFlux -> InverseSolidAngle
/// Dividing a Luminosity by a LuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Div<LuminousFlux<T>> for Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd / rhs.lm}
	}
}
/// Dividing a Luminosity by a LuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Div<LuminousFlux<T>> for &Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd.clone() / rhs.lm}
	}
}
/// Dividing a Luminosity by a LuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&LuminousFlux<T>> for Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd / rhs.lm.clone()}
	}
}
/// Dividing a Luminosity by a LuminousFlux returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&LuminousFlux<T>> for &Luminosity<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.cd.clone() / rhs.lm.clone()}
	}
}

// Luminosity / InverseSolidAngle -> LuminousFlux
/// Dividing a Luminosity by a InverseSolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseSolidAngle<T>> for Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd / rhs.per_sr}
	}
}
/// Dividing a Luminosity by a InverseSolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseSolidAngle<T>> for &Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd.clone() / rhs.per_sr}
	}
}
/// Dividing a Luminosity by a InverseSolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseSolidAngle<T>> for Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd / rhs.per_sr.clone()}
	}
}
/// Dividing a Luminosity by a InverseSolidAngle returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd.clone() / rhs.per_sr.clone()}
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

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
	}
}

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
	}
}

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
	}
}

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<Luminosity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
impl<T> core::ops::Div<&Luminosity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
	}
}

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Luminosity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Luminosity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Luminosity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Luminosity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
	}
}

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Luminosity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Luminosity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Luminosity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Luminosity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
	}
}

// 1/Luminosity -> InverseLuminosity
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Luminosity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Luminosity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Luminosity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self) / rhs.cd.clone()}
	}
}
/// Dividing a scalar value by a Luminosity unit value returns a value of type InverseLuminosity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Luminosity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminosity{per_cd: T::from(self.clone()) / rhs.cd.clone()}
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


// Mass / Amount -> MolarMass
/// Dividing a Mass by a Amount returns a value of type MolarMass
impl<T> core::ops::Div<Amount<T>> for Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg / rhs.mol}
	}
}
/// Dividing a Mass by a Amount returns a value of type MolarMass
impl<T> core::ops::Div<Amount<T>> for &Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg.clone() / rhs.mol}
	}
}
/// Dividing a Mass by a Amount returns a value of type MolarMass
impl<T> core::ops::Div<&Amount<T>> for Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg / rhs.mol.clone()}
	}
}
/// Dividing a Mass by a Amount returns a value of type MolarMass
impl<T> core::ops::Div<&Amount<T>> for &Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg.clone() / rhs.mol.clone()}
	}
}

// Mass * InverseAmount -> MolarMass
/// Multiplying a Mass by a InverseAmount returns a value of type MolarMass
impl<T> core::ops::Mul<InverseAmount<T>> for Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: InverseAmount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg * rhs.per_mol}
	}
}
/// Multiplying a Mass by a InverseAmount returns a value of type MolarMass
impl<T> core::ops::Mul<InverseAmount<T>> for &Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: InverseAmount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg.clone() * rhs.per_mol}
	}
}
/// Multiplying a Mass by a InverseAmount returns a value of type MolarMass
impl<T> core::ops::Mul<&InverseAmount<T>> for Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: &InverseAmount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg * rhs.per_mol.clone()}
	}
}
/// Multiplying a Mass by a InverseAmount returns a value of type MolarMass
impl<T> core::ops::Mul<&InverseAmount<T>> for &Mass<T> where T: NumLike {
	type Output = MolarMass<T>;
	fn mul(self, rhs: &InverseAmount<T>) -> Self::Output {
		MolarMass{kgpmol: self.kg.clone() * rhs.per_mol.clone()}
	}
}

// Mass * Molality -> Amount
/// Multiplying a Mass by a Molality returns a value of type Amount
impl<T> core::ops::Mul<Molality<T>> for Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Molality<T>) -> Self::Output {
		Amount{mol: self.kg * rhs.molpkg}
	}
}
/// Multiplying a Mass by a Molality returns a value of type Amount
impl<T> core::ops::Mul<Molality<T>> for &Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Molality<T>) -> Self::Output {
		Amount{mol: self.kg.clone() * rhs.molpkg}
	}
}
/// Multiplying a Mass by a Molality returns a value of type Amount
impl<T> core::ops::Mul<&Molality<T>> for Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Molality<T>) -> Self::Output {
		Amount{mol: self.kg * rhs.molpkg.clone()}
	}
}
/// Multiplying a Mass by a Molality returns a value of type Amount
impl<T> core::ops::Mul<&Molality<T>> for &Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Molality<T>) -> Self::Output {
		Amount{mol: self.kg.clone() * rhs.molpkg.clone()}
	}
}

// Mass / MolarMass -> Amount
/// Dividing a Mass by a MolarMass returns a value of type Amount
impl<T> core::ops::Div<MolarMass<T>> for Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Amount{mol: self.kg / rhs.kgpmol}
	}
}
/// Dividing a Mass by a MolarMass returns a value of type Amount
impl<T> core::ops::Div<MolarMass<T>> for &Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Amount{mol: self.kg.clone() / rhs.kgpmol}
	}
}
/// Dividing a Mass by a MolarMass returns a value of type Amount
impl<T> core::ops::Div<&MolarMass<T>> for Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Amount{mol: self.kg / rhs.kgpmol.clone()}
	}
}
/// Dividing a Mass by a MolarMass returns a value of type Amount
impl<T> core::ops::Div<&MolarMass<T>> for &Mass<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Amount{mol: self.kg.clone() / rhs.kgpmol.clone()}
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

// Mass * InverseArea -> AreaDensity
/// Multiplying a Mass by a InverseArea returns a value of type AreaDensity
impl<T> core::ops::Mul<InverseArea<T>> for Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg * rhs.per_m2}
	}
}
/// Multiplying a Mass by a InverseArea returns a value of type AreaDensity
impl<T> core::ops::Mul<InverseArea<T>> for &Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg.clone() * rhs.per_m2}
	}
}
/// Multiplying a Mass by a InverseArea returns a value of type AreaDensity
impl<T> core::ops::Mul<&InverseArea<T>> for Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg * rhs.per_m2.clone()}
	}
}
/// Multiplying a Mass by a InverseArea returns a value of type AreaDensity
impl<T> core::ops::Mul<&InverseArea<T>> for &Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		AreaDensity{kgpm2: self.kg.clone() * rhs.per_m2.clone()}
	}
}

// Mass * InverseVolume -> Density
/// Multiplying a Mass by a InverseVolume returns a value of type Density
impl<T> core::ops::Mul<InverseVolume<T>> for Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		Density{kgpm3: self.kg * rhs.per_m3}
	}
}
/// Multiplying a Mass by a InverseVolume returns a value of type Density
impl<T> core::ops::Mul<InverseVolume<T>> for &Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		Density{kgpm3: self.kg.clone() * rhs.per_m3}
	}
}
/// Multiplying a Mass by a InverseVolume returns a value of type Density
impl<T> core::ops::Mul<&InverseVolume<T>> for Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		Density{kgpm3: self.kg * rhs.per_m3.clone()}
	}
}
/// Multiplying a Mass by a InverseVolume returns a value of type Density
impl<T> core::ops::Mul<&InverseVolume<T>> for &Mass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		Density{kgpm3: self.kg.clone() * rhs.per_m3.clone()}
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

// Mass * AreaPerMass -> Area
/// Multiplying a Mass by a AreaPerMass returns a value of type Area
impl<T> core::ops::Mul<AreaPerMass<T>> for Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: AreaPerMass<T>) -> Self::Output {
		Area{m2: self.kg * rhs.m2_per_kg}
	}
}
/// Multiplying a Mass by a AreaPerMass returns a value of type Area
impl<T> core::ops::Mul<AreaPerMass<T>> for &Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: AreaPerMass<T>) -> Self::Output {
		Area{m2: self.kg.clone() * rhs.m2_per_kg}
	}
}
/// Multiplying a Mass by a AreaPerMass returns a value of type Area
impl<T> core::ops::Mul<&AreaPerMass<T>> for Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &AreaPerMass<T>) -> Self::Output {
		Area{m2: self.kg * rhs.m2_per_kg.clone()}
	}
}
/// Multiplying a Mass by a AreaPerMass returns a value of type Area
impl<T> core::ops::Mul<&AreaPerMass<T>> for &Mass<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &AreaPerMass<T>) -> Self::Output {
		Area{m2: self.kg.clone() * rhs.m2_per_kg.clone()}
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

// Mass / Force -> InverseAcceleration
/// Dividing a Mass by a Force returns a value of type InverseAcceleration
impl<T> core::ops::Div<Force<T>> for Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg / rhs.N}
	}
}
/// Dividing a Mass by a Force returns a value of type InverseAcceleration
impl<T> core::ops::Div<Force<T>> for &Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg.clone() / rhs.N}
	}
}
/// Dividing a Mass by a Force returns a value of type InverseAcceleration
impl<T> core::ops::Div<&Force<T>> for Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg / rhs.N.clone()}
	}
}
/// Dividing a Mass by a Force returns a value of type InverseAcceleration
impl<T> core::ops::Div<&Force<T>> for &Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg.clone() / rhs.N.clone()}
	}
}

// Mass / InverseAcceleration -> Force
/// Dividing a Mass by a InverseAcceleration returns a value of type Force
impl<T> core::ops::Div<InverseAcceleration<T>> for Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InverseAcceleration<T>) -> Self::Output {
		Force{N: self.kg / rhs.s2pm}
	}
}
/// Dividing a Mass by a InverseAcceleration returns a value of type Force
impl<T> core::ops::Div<InverseAcceleration<T>> for &Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InverseAcceleration<T>) -> Self::Output {
		Force{N: self.kg.clone() / rhs.s2pm}
	}
}
/// Dividing a Mass by a InverseAcceleration returns a value of type Force
impl<T> core::ops::Div<&InverseAcceleration<T>> for Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InverseAcceleration<T>) -> Self::Output {
		Force{N: self.kg / rhs.s2pm.clone()}
	}
}
/// Dividing a Mass by a InverseAcceleration returns a value of type Force
impl<T> core::ops::Div<&InverseAcceleration<T>> for &Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InverseAcceleration<T>) -> Self::Output {
		Force{N: self.kg.clone() / rhs.s2pm.clone()}
	}
}

// Mass * InverseForce -> InverseAcceleration
/// Multiplying a Mass by a InverseForce returns a value of type InverseAcceleration
impl<T> core::ops::Mul<InverseForce<T>> for Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: InverseForce<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg * rhs.per_N}
	}
}
/// Multiplying a Mass by a InverseForce returns a value of type InverseAcceleration
impl<T> core::ops::Mul<InverseForce<T>> for &Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: InverseForce<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg.clone() * rhs.per_N}
	}
}
/// Multiplying a Mass by a InverseForce returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&InverseForce<T>> for Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &InverseForce<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg * rhs.per_N.clone()}
	}
}
/// Multiplying a Mass by a InverseForce returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&InverseForce<T>> for &Mass<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &InverseForce<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.kg.clone() * rhs.per_N.clone()}
	}
}

// Mass * InverseMomentOfInertia -> InverseArea
/// Multiplying a Mass by a InverseMomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Mul<InverseMomentOfInertia<T>> for Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg * rhs.per_kgm2}
	}
}
/// Multiplying a Mass by a InverseMomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Mul<InverseMomentOfInertia<T>> for &Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg.clone() * rhs.per_kgm2}
	}
}
/// Multiplying a Mass by a InverseMomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseMomentOfInertia<T>> for Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg * rhs.per_kgm2.clone()}
	}
}
/// Multiplying a Mass by a InverseMomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseMomentOfInertia<T>> for &Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg.clone() * rhs.per_kgm2.clone()}
	}
}

// Mass * InverseMomentum -> TimePerDistance
/// Multiplying a Mass by a InverseMomentum returns a value of type TimePerDistance
impl<T> core::ops::Mul<InverseMomentum<T>> for Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: InverseMomentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg * rhs.s_per_kgm}
	}
}
/// Multiplying a Mass by a InverseMomentum returns a value of type TimePerDistance
impl<T> core::ops::Mul<InverseMomentum<T>> for &Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: InverseMomentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg.clone() * rhs.s_per_kgm}
	}
}
/// Multiplying a Mass by a InverseMomentum returns a value of type TimePerDistance
impl<T> core::ops::Mul<&InverseMomentum<T>> for Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: &InverseMomentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg * rhs.s_per_kgm.clone()}
	}
}
/// Multiplying a Mass by a InverseMomentum returns a value of type TimePerDistance
impl<T> core::ops::Mul<&InverseMomentum<T>> for &Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: &InverseMomentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg.clone() * rhs.s_per_kgm.clone()}
	}
}

// Mass / MomentOfInertia -> InverseArea
/// Dividing a Mass by a MomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Div<MomentOfInertia<T>> for Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg / rhs.kgm2}
	}
}
/// Dividing a Mass by a MomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Div<MomentOfInertia<T>> for &Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg.clone() / rhs.kgm2}
	}
}
/// Dividing a Mass by a MomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Div<&MomentOfInertia<T>> for Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg / rhs.kgm2.clone()}
	}
}
/// Dividing a Mass by a MomentOfInertia returns a value of type InverseArea
impl<T> core::ops::Div<&MomentOfInertia<T>> for &Mass<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		InverseArea{per_m2: self.kg.clone() / rhs.kgm2.clone()}
	}
}

// Mass / Momentum -> TimePerDistance
/// Dividing a Mass by a Momentum returns a value of type TimePerDistance
impl<T> core::ops::Div<Momentum<T>> for Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg / rhs.kgmps}
	}
}
/// Dividing a Mass by a Momentum returns a value of type TimePerDistance
impl<T> core::ops::Div<Momentum<T>> for &Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg.clone() / rhs.kgmps}
	}
}
/// Dividing a Mass by a Momentum returns a value of type TimePerDistance
impl<T> core::ops::Div<&Momentum<T>> for Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg / rhs.kgmps.clone()}
	}
}
/// Dividing a Mass by a Momentum returns a value of type TimePerDistance
impl<T> core::ops::Div<&Momentum<T>> for &Mass<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		TimePerDistance{spm: self.kg.clone() / rhs.kgmps.clone()}
	}
}

// Mass / TimePerDistance -> Momentum
/// Dividing a Mass by a TimePerDistance returns a value of type Momentum
impl<T> core::ops::Div<TimePerDistance<T>> for Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: TimePerDistance<T>) -> Self::Output {
		Momentum{kgmps: self.kg / rhs.spm}
	}
}
/// Dividing a Mass by a TimePerDistance returns a value of type Momentum
impl<T> core::ops::Div<TimePerDistance<T>> for &Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: TimePerDistance<T>) -> Self::Output {
		Momentum{kgmps: self.kg.clone() / rhs.spm}
	}
}
/// Dividing a Mass by a TimePerDistance returns a value of type Momentum
impl<T> core::ops::Div<&TimePerDistance<T>> for Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Momentum{kgmps: self.kg / rhs.spm.clone()}
	}
}
/// Dividing a Mass by a TimePerDistance returns a value of type Momentum
impl<T> core::ops::Div<&TimePerDistance<T>> for &Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Momentum{kgmps: self.kg.clone() / rhs.spm.clone()}
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

// Mass * VolumePerMass -> Volume
/// Multiplying a Mass by a VolumePerMass returns a value of type Volume
impl<T> core::ops::Mul<VolumePerMass<T>> for Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: VolumePerMass<T>) -> Self::Output {
		Volume{m3: self.kg * rhs.m3_per_kg}
	}
}
/// Multiplying a Mass by a VolumePerMass returns a value of type Volume
impl<T> core::ops::Mul<VolumePerMass<T>> for &Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: VolumePerMass<T>) -> Self::Output {
		Volume{m3: self.kg.clone() * rhs.m3_per_kg}
	}
}
/// Multiplying a Mass by a VolumePerMass returns a value of type Volume
impl<T> core::ops::Mul<&VolumePerMass<T>> for Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &VolumePerMass<T>) -> Self::Output {
		Volume{m3: self.kg * rhs.m3_per_kg.clone()}
	}
}
/// Multiplying a Mass by a VolumePerMass returns a value of type Volume
impl<T> core::ops::Mul<&VolumePerMass<T>> for &Mass<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &VolumePerMass<T>) -> Self::Output {
		Volume{m3: self.kg.clone() * rhs.m3_per_kg.clone()}
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

// Mass / InverseAbsorbedDose -> Energy
/// Dividing a Mass by a InverseAbsorbedDose returns a value of type Energy
impl<T> core::ops::Div<InverseAbsorbedDose<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg / rhs.per_Gy}
	}
}
/// Dividing a Mass by a InverseAbsorbedDose returns a value of type Energy
impl<T> core::ops::Div<InverseAbsorbedDose<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg.clone() / rhs.per_Gy}
	}
}
/// Dividing a Mass by a InverseAbsorbedDose returns a value of type Energy
impl<T> core::ops::Div<&InverseAbsorbedDose<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg / rhs.per_Gy.clone()}
	}
}
/// Dividing a Mass by a InverseAbsorbedDose returns a value of type Energy
impl<T> core::ops::Div<&InverseAbsorbedDose<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg.clone() / rhs.per_Gy.clone()}
	}
}

// Mass / InverseDoseEquivalent -> Energy
/// Dividing a Mass by a InverseDoseEquivalent returns a value of type Energy
impl<T> core::ops::Div<InverseDoseEquivalent<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg / rhs.per_Sv}
	}
}
/// Dividing a Mass by a InverseDoseEquivalent returns a value of type Energy
impl<T> core::ops::Div<InverseDoseEquivalent<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg.clone() / rhs.per_Sv}
	}
}
/// Dividing a Mass by a InverseDoseEquivalent returns a value of type Energy
impl<T> core::ops::Div<&InverseDoseEquivalent<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg / rhs.per_Sv.clone()}
	}
}
/// Dividing a Mass by a InverseDoseEquivalent returns a value of type Energy
impl<T> core::ops::Div<&InverseDoseEquivalent<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg.clone() / rhs.per_Sv.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<Mass<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
impl<T> core::ops::Div<&Mass<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Mass<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Mass<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Mass<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Mass<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Mass<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Mass<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Mass<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Mass<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
	}
}

// 1/Mass -> InverseMass
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Mass<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Mass<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Mass<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self) / rhs.kg.clone()}
	}
}
/// Dividing a scalar value by a Mass unit value returns a value of type InverseMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Mass<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		InverseMass{per_kg: T::from(self.clone()) / rhs.kg.clone()}
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


// Temperature * InverseAbsorbedDose -> InverseSpecificHeatCapacity
/// Multiplying a Temperature by a InverseAbsorbedDose returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<InverseAbsorbedDose<T>> for Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K * rhs.per_Gy}
	}
}
/// Multiplying a Temperature by a InverseAbsorbedDose returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<InverseAbsorbedDose<T>> for &Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: InverseAbsorbedDose<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K.clone() * rhs.per_Gy}
	}
}
/// Multiplying a Temperature by a InverseAbsorbedDose returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<&InverseAbsorbedDose<T>> for Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K * rhs.per_Gy.clone()}
	}
}
/// Multiplying a Temperature by a InverseAbsorbedDose returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<&InverseAbsorbedDose<T>> for &Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: &InverseAbsorbedDose<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K.clone() * rhs.per_Gy.clone()}
	}
}

// Temperature * InverseDoseEquivalent -> InverseSpecificHeatCapacity
/// Multiplying a Temperature by a InverseDoseEquivalent returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<InverseDoseEquivalent<T>> for Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K * rhs.per_Sv}
	}
}
/// Multiplying a Temperature by a InverseDoseEquivalent returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<InverseDoseEquivalent<T>> for &Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: InverseDoseEquivalent<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K.clone() * rhs.per_Sv}
	}
}
/// Multiplying a Temperature by a InverseDoseEquivalent returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<&InverseDoseEquivalent<T>> for Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K * rhs.per_Sv.clone()}
	}
}
/// Multiplying a Temperature by a InverseDoseEquivalent returns a value of type InverseSpecificHeatCapacity
impl<T> core::ops::Mul<&InverseDoseEquivalent<T>> for &Temperature<T> where T: NumLike {
	type Output = InverseSpecificHeatCapacity<T>;
	fn mul(self, rhs: &InverseDoseEquivalent<T>) -> Self::Output {
		InverseSpecificHeatCapacity{kgK_per_J: self.K.clone() * rhs.per_Sv.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<Temperature<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
impl<T> core::ops::Div<&Temperature<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Temperature<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Temperature<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Temperature<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Temperature<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Temperature<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Temperature<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Temperature<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Temperature<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
	}
}

// 1/Temperature -> InverseTemperature
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Temperature<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Temperature<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Temperature<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self) / rhs.K.clone()}
	}
}
/// Dividing a scalar value by a Temperature unit value returns a value of type InverseTemperature
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Temperature<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseTemperature<T>;
	fn div(self, rhs: &Temperature<T>) -> Self::Output {
		InverseTemperature{per_K: T::from(self.clone()) / rhs.K.clone()}
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


// Time / Amount -> InverseCatalyticActivity
/// Dividing a Time by a Amount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<Amount<T>> for Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s / rhs.mol}
	}
}
/// Dividing a Time by a Amount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<Amount<T>> for &Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s.clone() / rhs.mol}
	}
}
/// Dividing a Time by a Amount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<&Amount<T>> for Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s / rhs.mol.clone()}
	}
}
/// Dividing a Time by a Amount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Div<&Amount<T>> for &Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s.clone() / rhs.mol.clone()}
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

// Time / Distance -> TimePerDistance
/// Dividing a Time by a Distance returns a value of type TimePerDistance
impl<T> core::ops::Div<Distance<T>> for Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		TimePerDistance{spm: self.s / rhs.m}
	}
}
/// Dividing a Time by a Distance returns a value of type TimePerDistance
impl<T> core::ops::Div<Distance<T>> for &Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		TimePerDistance{spm: self.s.clone() / rhs.m}
	}
}
/// Dividing a Time by a Distance returns a value of type TimePerDistance
impl<T> core::ops::Div<&Distance<T>> for Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		TimePerDistance{spm: self.s / rhs.m.clone()}
	}
}
/// Dividing a Time by a Distance returns a value of type TimePerDistance
impl<T> core::ops::Div<&Distance<T>> for &Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		TimePerDistance{spm: self.s.clone() / rhs.m.clone()}
	}
}

// Time * InverseAmount -> InverseCatalyticActivity
/// Multiplying a Time by a InverseAmount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<InverseAmount<T>> for Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: InverseAmount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s * rhs.per_mol}
	}
}
/// Multiplying a Time by a InverseAmount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<InverseAmount<T>> for &Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: InverseAmount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s.clone() * rhs.per_mol}
	}
}
/// Multiplying a Time by a InverseAmount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<&InverseAmount<T>> for Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: &InverseAmount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s * rhs.per_mol.clone()}
	}
}
/// Multiplying a Time by a InverseAmount returns a value of type InverseCatalyticActivity
impl<T> core::ops::Mul<&InverseAmount<T>> for &Time<T> where T: NumLike {
	type Output = InverseCatalyticActivity<T>;
	fn mul(self, rhs: &InverseAmount<T>) -> Self::Output {
		InverseCatalyticActivity{s_per_mol: self.s.clone() * rhs.per_mol.clone()}
	}
}

// Time / InverseCurrent -> Charge
/// Dividing a Time by a InverseCurrent returns a value of type Charge
impl<T> core::ops::Div<InverseCurrent<T>> for Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Charge{C: self.s / rhs.per_A}
	}
}
/// Dividing a Time by a InverseCurrent returns a value of type Charge
impl<T> core::ops::Div<InverseCurrent<T>> for &Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Charge{C: self.s.clone() / rhs.per_A}
	}
}
/// Dividing a Time by a InverseCurrent returns a value of type Charge
impl<T> core::ops::Div<&InverseCurrent<T>> for Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Charge{C: self.s / rhs.per_A.clone()}
	}
}
/// Dividing a Time by a InverseCurrent returns a value of type Charge
impl<T> core::ops::Div<&InverseCurrent<T>> for &Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Charge{C: self.s.clone() / rhs.per_A.clone()}
	}
}

// Time * InverseDistance -> TimePerDistance
/// Multiplying a Time by a InverseDistance returns a value of type TimePerDistance
impl<T> core::ops::Mul<InverseDistance<T>> for Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		TimePerDistance{spm: self.s * rhs.per_m}
	}
}
/// Multiplying a Time by a InverseDistance returns a value of type TimePerDistance
impl<T> core::ops::Mul<InverseDistance<T>> for &Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		TimePerDistance{spm: self.s.clone() * rhs.per_m}
	}
}
/// Multiplying a Time by a InverseDistance returns a value of type TimePerDistance
impl<T> core::ops::Mul<&InverseDistance<T>> for Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		TimePerDistance{spm: self.s * rhs.per_m.clone()}
	}
}
/// Multiplying a Time by a InverseDistance returns a value of type TimePerDistance
impl<T> core::ops::Mul<&InverseDistance<T>> for &Time<T> where T: NumLike {
	type Output = TimePerDistance<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		TimePerDistance{spm: self.s.clone() * rhs.per_m.clone()}
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

// Time / InverseCatalyticActivity -> Amount
/// Dividing a Time by a InverseCatalyticActivity returns a value of type Amount
impl<T> core::ops::Div<InverseCatalyticActivity<T>> for Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: InverseCatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s / rhs.s_per_mol}
	}
}
/// Dividing a Time by a InverseCatalyticActivity returns a value of type Amount
impl<T> core::ops::Div<InverseCatalyticActivity<T>> for &Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: InverseCatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s.clone() / rhs.s_per_mol}
	}
}
/// Dividing a Time by a InverseCatalyticActivity returns a value of type Amount
impl<T> core::ops::Div<&InverseCatalyticActivity<T>> for Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseCatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s / rhs.s_per_mol.clone()}
	}
}
/// Dividing a Time by a InverseCatalyticActivity returns a value of type Amount
impl<T> core::ops::Div<&InverseCatalyticActivity<T>> for &Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &InverseCatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s.clone() / rhs.s_per_mol.clone()}
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

// Time / Charge -> InverseCurrent
/// Dividing a Time by a Charge returns a value of type InverseCurrent
impl<T> core::ops::Div<Charge<T>> for Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s / rhs.C}
	}
}
/// Dividing a Time by a Charge returns a value of type InverseCurrent
impl<T> core::ops::Div<Charge<T>> for &Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s.clone() / rhs.C}
	}
}
/// Dividing a Time by a Charge returns a value of type InverseCurrent
impl<T> core::ops::Div<&Charge<T>> for Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s / rhs.C.clone()}
	}
}
/// Dividing a Time by a Charge returns a value of type InverseCurrent
impl<T> core::ops::Div<&Charge<T>> for &Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s.clone() / rhs.C.clone()}
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

// Time * Elastance -> Resistance
/// Multiplying a Time by a Elastance returns a value of type Resistance
impl<T> core::ops::Mul<Elastance<T>> for Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		Resistance{Ohm: self.s * rhs.per_F}
	}
}
/// Multiplying a Time by a Elastance returns a value of type Resistance
impl<T> core::ops::Mul<Elastance<T>> for &Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		Resistance{Ohm: self.s.clone() * rhs.per_F}
	}
}
/// Multiplying a Time by a Elastance returns a value of type Resistance
impl<T> core::ops::Mul<&Elastance<T>> for Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		Resistance{Ohm: self.s * rhs.per_F.clone()}
	}
}
/// Multiplying a Time by a Elastance returns a value of type Resistance
impl<T> core::ops::Mul<&Elastance<T>> for &Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		Resistance{Ohm: self.s.clone() * rhs.per_F.clone()}
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

// Time * InverseCharge -> InverseCurrent
/// Multiplying a Time by a InverseCharge returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseCharge<T>> for Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s * rhs.per_C}
	}
}
/// Multiplying a Time by a InverseCharge returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseCharge<T>> for &Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s.clone() * rhs.per_C}
	}
}
/// Multiplying a Time by a InverseCharge returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseCharge<T>> for Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s * rhs.per_C.clone()}
	}
}
/// Multiplying a Time by a InverseCharge returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseCharge<T>> for &Time<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseCurrent{per_A: self.s.clone() * rhs.per_C.clone()}
	}
}

// Time * InverseInductance -> Conductance
/// Multiplying a Time by a InverseInductance returns a value of type Conductance
impl<T> core::ops::Mul<InverseInductance<T>> for Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		Conductance{S: self.s * rhs.per_H}
	}
}
/// Multiplying a Time by a InverseInductance returns a value of type Conductance
impl<T> core::ops::Mul<InverseInductance<T>> for &Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		Conductance{S: self.s.clone() * rhs.per_H}
	}
}
/// Multiplying a Time by a InverseInductance returns a value of type Conductance
impl<T> core::ops::Mul<&InverseInductance<T>> for Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		Conductance{S: self.s * rhs.per_H.clone()}
	}
}
/// Multiplying a Time by a InverseInductance returns a value of type Conductance
impl<T> core::ops::Mul<&InverseInductance<T>> for &Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		Conductance{S: self.s.clone() * rhs.per_H.clone()}
	}
}

// Time * InverseMagneticFlux -> InverseVoltage
/// Multiplying a Time by a InverseMagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s * rhs.per_Wb}
	}
}
/// Multiplying a Time by a InverseMagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Time by a InverseMagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Time by a InverseMagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s.clone() * rhs.per_Wb.clone()}
	}
}

// Time / InverseVoltage -> MagneticFlux
/// Dividing a Time by a InverseVoltage returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseVoltage<T>> for Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s / rhs.per_V}
	}
}
/// Dividing a Time by a InverseVoltage returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseVoltage<T>> for &Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s.clone() / rhs.per_V}
	}
}
/// Dividing a Time by a InverseVoltage returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseVoltage<T>> for Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s / rhs.per_V.clone()}
	}
}
/// Dividing a Time by a InverseVoltage returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseVoltage<T>> for &Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s.clone() / rhs.per_V.clone()}
	}
}

// Time / MagneticFlux -> InverseVoltage
/// Dividing a Time by a MagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Div<MagneticFlux<T>> for Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s / rhs.Wb}
	}
}
/// Dividing a Time by a MagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Div<MagneticFlux<T>> for &Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s.clone() / rhs.Wb}
	}
}
/// Dividing a Time by a MagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Div<&MagneticFlux<T>> for Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s / rhs.Wb.clone()}
	}
}
/// Dividing a Time by a MagneticFlux returns a value of type InverseVoltage
impl<T> core::ops::Div<&MagneticFlux<T>> for &Time<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseVoltage{per_V: self.s.clone() / rhs.Wb.clone()}
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

// Time / Angle -> InverseAngularVelocity
/// Dividing a Time by a Angle returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<Angle<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s / rhs.rad}
	}
}
/// Dividing a Time by a Angle returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<Angle<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s.clone() / rhs.rad}
	}
}
/// Dividing a Time by a Angle returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<&Angle<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s / rhs.rad.clone()}
	}
}
/// Dividing a Time by a Angle returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<&Angle<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s.clone() / rhs.rad.clone()}
	}
}

// Time * InverseAngle -> InverseAngularVelocity
/// Multiplying a Time by a InverseAngle returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<InverseAngle<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: InverseAngle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s * rhs.per_rad}
	}
}
/// Multiplying a Time by a InverseAngle returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<InverseAngle<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: InverseAngle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s.clone() * rhs.per_rad}
	}
}
/// Multiplying a Time by a InverseAngle returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<&InverseAngle<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: &InverseAngle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s * rhs.per_rad.clone()}
	}
}
/// Multiplying a Time by a InverseAngle returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<&InverseAngle<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: &InverseAngle<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.s.clone() * rhs.per_rad.clone()}
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

// Time / AngularVelocity -> InverseAngularAcceleration
/// Dividing a Time by a AngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Div<AngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s / rhs.radps}
	}
}
/// Dividing a Time by a AngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Div<AngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s.clone() / rhs.radps}
	}
}
/// Dividing a Time by a AngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Div<&AngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s / rhs.radps.clone()}
	}
}
/// Dividing a Time by a AngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Div<&AngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s.clone() / rhs.radps.clone()}
	}
}

// Time / Energy -> InversePower
/// Dividing a Time by a Energy returns a value of type InversePower
impl<T> core::ops::Div<Energy<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InversePower{per_W: self.s / rhs.J}
	}
}
/// Dividing a Time by a Energy returns a value of type InversePower
impl<T> core::ops::Div<Energy<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() / rhs.J}
	}
}
/// Dividing a Time by a Energy returns a value of type InversePower
impl<T> core::ops::Div<&Energy<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InversePower{per_W: self.s / rhs.J.clone()}
	}
}
/// Dividing a Time by a Energy returns a value of type InversePower
impl<T> core::ops::Div<&Energy<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() / rhs.J.clone()}
	}
}

// Time / Torque -> InversePower
/// Dividing a Time by a Torque returns a value of type InversePower
impl<T> core::ops::Div<Torque<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InversePower{per_W: self.s / rhs.Nm}
	}
}
/// Dividing a Time by a Torque returns a value of type InversePower
impl<T> core::ops::Div<Torque<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() / rhs.Nm}
	}
}
/// Dividing a Time by a Torque returns a value of type InversePower
impl<T> core::ops::Div<&Torque<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InversePower{per_W: self.s / rhs.Nm.clone()}
	}
}
/// Dividing a Time by a Torque returns a value of type InversePower
impl<T> core::ops::Div<&Torque<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() / rhs.Nm.clone()}
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

// Time / InverseAcceleration -> Velocity
/// Dividing a Time by a InverseAcceleration returns a value of type Velocity
impl<T> core::ops::Div<InverseAcceleration<T>> for Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: InverseAcceleration<T>) -> Self::Output {
		Velocity{mps: self.s / rhs.s2pm}
	}
}
/// Dividing a Time by a InverseAcceleration returns a value of type Velocity
impl<T> core::ops::Div<InverseAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: InverseAcceleration<T>) -> Self::Output {
		Velocity{mps: self.s.clone() / rhs.s2pm}
	}
}
/// Dividing a Time by a InverseAcceleration returns a value of type Velocity
impl<T> core::ops::Div<&InverseAcceleration<T>> for Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &InverseAcceleration<T>) -> Self::Output {
		Velocity{mps: self.s / rhs.s2pm.clone()}
	}
}
/// Dividing a Time by a InverseAcceleration returns a value of type Velocity
impl<T> core::ops::Div<&InverseAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &InverseAcceleration<T>) -> Self::Output {
		Velocity{mps: self.s.clone() / rhs.s2pm.clone()}
	}
}

// Time / InverseAngularAcceleration -> AngularVelocity
/// Dividing a Time by a InverseAngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Div<InverseAngularAcceleration<T>> for Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: InverseAngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s / rhs.s2prad}
	}
}
/// Dividing a Time by a InverseAngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Div<InverseAngularAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: InverseAngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s.clone() / rhs.s2prad}
	}
}
/// Dividing a Time by a InverseAngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Div<&InverseAngularAcceleration<T>> for Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &InverseAngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s / rhs.s2prad.clone()}
	}
}
/// Dividing a Time by a InverseAngularAcceleration returns a value of type AngularVelocity
impl<T> core::ops::Div<&InverseAngularAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &InverseAngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s.clone() / rhs.s2prad.clone()}
	}
}

// Time * InverseAngularVelocity -> InverseAngularAcceleration
/// Multiplying a Time by a InverseAngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Mul<InverseAngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn mul(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s * rhs.s_per_rad}
	}
}
/// Multiplying a Time by a InverseAngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Mul<InverseAngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn mul(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s.clone() * rhs.s_per_rad}
	}
}
/// Multiplying a Time by a InverseAngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Mul<&InverseAngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn mul(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s * rhs.s_per_rad.clone()}
	}
}
/// Multiplying a Time by a InverseAngularVelocity returns a value of type InverseAngularAcceleration
impl<T> core::ops::Mul<&InverseAngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = InverseAngularAcceleration<T>;
	fn mul(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		InverseAngularAcceleration{s2prad: self.s.clone() * rhs.s_per_rad.clone()}
	}
}

// Time / InverseAngularVelocity -> Angle
/// Dividing a Time by a InverseAngularVelocity returns a value of type Angle
impl<T> core::ops::Div<InverseAngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s / rhs.s_per_rad}
	}
}
/// Dividing a Time by a InverseAngularVelocity returns a value of type Angle
impl<T> core::ops::Div<InverseAngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s.clone() / rhs.s_per_rad}
	}
}
/// Dividing a Time by a InverseAngularVelocity returns a value of type Angle
impl<T> core::ops::Div<&InverseAngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s / rhs.s_per_rad.clone()}
	}
}
/// Dividing a Time by a InverseAngularVelocity returns a value of type Angle
impl<T> core::ops::Div<&InverseAngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s.clone() / rhs.s_per_rad.clone()}
	}
}

// Time * InverseEnergy -> InversePower
/// Multiplying a Time by a InverseEnergy returns a value of type InversePower
impl<T> core::ops::Mul<InverseEnergy<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InversePower{per_W: self.s * rhs.per_J}
	}
}
/// Multiplying a Time by a InverseEnergy returns a value of type InversePower
impl<T> core::ops::Mul<InverseEnergy<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() * rhs.per_J}
	}
}
/// Multiplying a Time by a InverseEnergy returns a value of type InversePower
impl<T> core::ops::Mul<&InverseEnergy<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InversePower{per_W: self.s * rhs.per_J.clone()}
	}
}
/// Multiplying a Time by a InverseEnergy returns a value of type InversePower
impl<T> core::ops::Mul<&InverseEnergy<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() * rhs.per_J.clone()}
	}
}

// Time * InverseTorque -> InversePower
/// Multiplying a Time by a InverseTorque returns a value of type InversePower
impl<T> core::ops::Mul<InverseTorque<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InversePower{per_W: self.s * rhs.per_Nm}
	}
}
/// Multiplying a Time by a InverseTorque returns a value of type InversePower
impl<T> core::ops::Mul<InverseTorque<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() * rhs.per_Nm}
	}
}
/// Multiplying a Time by a InverseTorque returns a value of type InversePower
impl<T> core::ops::Mul<&InverseTorque<T>> for Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InversePower{per_W: self.s * rhs.per_Nm.clone()}
	}
}
/// Multiplying a Time by a InverseTorque returns a value of type InversePower
impl<T> core::ops::Mul<&InverseTorque<T>> for &Time<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InversePower{per_W: self.s.clone() * rhs.per_Nm.clone()}
	}
}

// Time / InverseForce -> Momentum
/// Dividing a Time by a InverseForce returns a value of type Momentum
impl<T> core::ops::Div<InverseForce<T>> for Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Momentum{kgmps: self.s / rhs.per_N}
	}
}
/// Dividing a Time by a InverseForce returns a value of type Momentum
impl<T> core::ops::Div<InverseForce<T>> for &Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Momentum{kgmps: self.s.clone() / rhs.per_N}
	}
}
/// Dividing a Time by a InverseForce returns a value of type Momentum
impl<T> core::ops::Div<&InverseForce<T>> for Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Momentum{kgmps: self.s / rhs.per_N.clone()}
	}
}
/// Dividing a Time by a InverseForce returns a value of type Momentum
impl<T> core::ops::Div<&InverseForce<T>> for &Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Momentum{kgmps: self.s.clone() / rhs.per_N.clone()}
	}
}

// Time * InverseMomentum -> InverseForce
/// Multiplying a Time by a InverseMomentum returns a value of type InverseForce
impl<T> core::ops::Mul<InverseMomentum<T>> for Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseMomentum<T>) -> Self::Output {
		InverseForce{per_N: self.s * rhs.s_per_kgm}
	}
}
/// Multiplying a Time by a InverseMomentum returns a value of type InverseForce
impl<T> core::ops::Mul<InverseMomentum<T>> for &Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InverseMomentum<T>) -> Self::Output {
		InverseForce{per_N: self.s.clone() * rhs.s_per_kgm}
	}
}
/// Multiplying a Time by a InverseMomentum returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseMomentum<T>> for Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseMomentum<T>) -> Self::Output {
		InverseForce{per_N: self.s * rhs.s_per_kgm.clone()}
	}
}
/// Multiplying a Time by a InverseMomentum returns a value of type InverseForce
impl<T> core::ops::Mul<&InverseMomentum<T>> for &Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InverseMomentum<T>) -> Self::Output {
		InverseForce{per_N: self.s.clone() * rhs.s_per_kgm.clone()}
	}
}

// Time / InversePower -> Energy
/// Dividing a Time by a InversePower returns a value of type Energy
impl<T> core::ops::Div<InversePower<T>> for Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InversePower<T>) -> Self::Output {
		Energy{J: self.s / rhs.per_W}
	}
}
/// Dividing a Time by a InversePower returns a value of type Energy
impl<T> core::ops::Div<InversePower<T>> for &Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InversePower<T>) -> Self::Output {
		Energy{J: self.s.clone() / rhs.per_W}
	}
}
/// Dividing a Time by a InversePower returns a value of type Energy
impl<T> core::ops::Div<&InversePower<T>> for Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InversePower<T>) -> Self::Output {
		Energy{J: self.s / rhs.per_W.clone()}
	}
}
/// Dividing a Time by a InversePower returns a value of type Energy
impl<T> core::ops::Div<&InversePower<T>> for &Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InversePower<T>) -> Self::Output {
		Energy{J: self.s.clone() / rhs.per_W.clone()}
	}
}

// Time / Momentum -> InverseForce
/// Dividing a Time by a Momentum returns a value of type InverseForce
impl<T> core::ops::Div<Momentum<T>> for Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		InverseForce{per_N: self.s / rhs.kgmps}
	}
}
/// Dividing a Time by a Momentum returns a value of type InverseForce
impl<T> core::ops::Div<Momentum<T>> for &Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		InverseForce{per_N: self.s.clone() / rhs.kgmps}
	}
}
/// Dividing a Time by a Momentum returns a value of type InverseForce
impl<T> core::ops::Div<&Momentum<T>> for Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		InverseForce{per_N: self.s / rhs.kgmps.clone()}
	}
}
/// Dividing a Time by a Momentum returns a value of type InverseForce
impl<T> core::ops::Div<&Momentum<T>> for &Time<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		InverseForce{per_N: self.s.clone() / rhs.kgmps.clone()}
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

// Time * TimePerDistance -> InverseAcceleration
/// Multiplying a Time by a TimePerDistance returns a value of type InverseAcceleration
impl<T> core::ops::Mul<TimePerDistance<T>> for Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: TimePerDistance<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s * rhs.spm}
	}
}
/// Multiplying a Time by a TimePerDistance returns a value of type InverseAcceleration
impl<T> core::ops::Mul<TimePerDistance<T>> for &Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: TimePerDistance<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s.clone() * rhs.spm}
	}
}
/// Multiplying a Time by a TimePerDistance returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&TimePerDistance<T>> for Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &TimePerDistance<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s * rhs.spm.clone()}
	}
}
/// Multiplying a Time by a TimePerDistance returns a value of type InverseAcceleration
impl<T> core::ops::Mul<&TimePerDistance<T>> for &Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn mul(self, rhs: &TimePerDistance<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s.clone() * rhs.spm.clone()}
	}
}

// Time / TimePerDistance -> Distance
/// Dividing a Time by a TimePerDistance returns a value of type Distance
impl<T> core::ops::Div<TimePerDistance<T>> for Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: TimePerDistance<T>) -> Self::Output {
		Distance{m: self.s / rhs.spm}
	}
}
/// Dividing a Time by a TimePerDistance returns a value of type Distance
impl<T> core::ops::Div<TimePerDistance<T>> for &Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: TimePerDistance<T>) -> Self::Output {
		Distance{m: self.s.clone() / rhs.spm}
	}
}
/// Dividing a Time by a TimePerDistance returns a value of type Distance
impl<T> core::ops::Div<&TimePerDistance<T>> for Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Distance{m: self.s / rhs.spm.clone()}
	}
}
/// Dividing a Time by a TimePerDistance returns a value of type Distance
impl<T> core::ops::Div<&TimePerDistance<T>> for &Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &TimePerDistance<T>) -> Self::Output {
		Distance{m: self.s.clone() / rhs.spm.clone()}
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

// Time / Velocity -> InverseAcceleration
/// Dividing a Time by a Velocity returns a value of type InverseAcceleration
impl<T> core::ops::Div<Velocity<T>> for Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s / rhs.mps}
	}
}
/// Dividing a Time by a Velocity returns a value of type InverseAcceleration
impl<T> core::ops::Div<Velocity<T>> for &Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s.clone() / rhs.mps}
	}
}
/// Dividing a Time by a Velocity returns a value of type InverseAcceleration
impl<T> core::ops::Div<&Velocity<T>> for Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s / rhs.mps.clone()}
	}
}
/// Dividing a Time by a Velocity returns a value of type InverseAcceleration
impl<T> core::ops::Div<&Velocity<T>> for &Time<T> where T: NumLike {
	type Output = InverseAcceleration<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		InverseAcceleration{s2pm: self.s.clone() / rhs.mps.clone()}
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



