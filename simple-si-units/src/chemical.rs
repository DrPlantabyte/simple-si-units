
//! This module provides chemical SI units, such as catalytic activity 
//! and molar mass.
use core::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::geometry::*;
use super::mechanical::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num-bigfloat")]
use num_bigfloat;
#[cfg(feature="num-complex")]
use num_complex;



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
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Nps(&self) -> T {
		return self.molps.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new catalytic activity value from the given number of count per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Nps` - Any number-like type, representing a quantity of count per second
	pub fn from_Nps(Nps: T) -> Self {
		CatalyticActivity{molps: Nps * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this catalytic activity value in millimoles per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mmolps(&self) -> T {
		return self.molps.clone() * T::from(1000.0_f64);
	}

	/// Returns a new catalytic activity value from the given number of millimoles per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mmolps` - Any number-like type, representing a quantity of millimoles per second
	pub fn from_mmolps(mmolps: T) -> Self {
		CatalyticActivity{molps: mmolps * T::from(0.001_f64)}
	}

	/// Returns a copy of this catalytic activity value in micromoles per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_umolps(&self) -> T {
		return self.molps.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new catalytic activity value from the given number of micromoles per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `umolps` - Any number-like type, representing a quantity of micromoles per second
	pub fn from_umolps(umolps: T) -> Self {
		CatalyticActivity{molps: umolps * T::from(1e-06_f64)}
	}

	/// Returns a copy of this catalytic activity value in nanomoles per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nmolps(&self) -> T {
		return self.molps.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new catalytic activity value from the given number of nanomoles per second
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nmolps` - Any number-like type, representing a quantity of nanomoles per second
	pub fn from_nmolps(nmolps: T) -> Self {
		CatalyticActivity{molps: nmolps * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<CatalyticActivity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = CatalyticActivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: CatalyticActivity<num_bigfloat::BigFloat>) -> Self::Output {
		CatalyticActivity{molps: self * rhs.molps}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<CatalyticActivity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = CatalyticActivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: CatalyticActivity<num_bigfloat::BigFloat>) -> Self::Output {
		CatalyticActivity{molps: self.clone() * rhs.molps}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&CatalyticActivity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = CatalyticActivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &CatalyticActivity<num_bigfloat::BigFloat>) -> Self::Output {
		CatalyticActivity{molps: self * rhs.molps.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&CatalyticActivity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = CatalyticActivity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &CatalyticActivity<num_bigfloat::BigFloat>) -> Self::Output {
		CatalyticActivity{molps: self.clone() * rhs.molps.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<CatalyticActivity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = CatalyticActivity<num_complex::Complex32>;
	fn mul(self, rhs: CatalyticActivity<num_complex::Complex32>) -> Self::Output {
		CatalyticActivity{molps: self * rhs.molps}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<CatalyticActivity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = CatalyticActivity<num_complex::Complex32>;
	fn mul(self, rhs: CatalyticActivity<num_complex::Complex32>) -> Self::Output {
		CatalyticActivity{molps: self.clone() * rhs.molps}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&CatalyticActivity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = CatalyticActivity<num_complex::Complex32>;
	fn mul(self, rhs: &CatalyticActivity<num_complex::Complex32>) -> Self::Output {
		CatalyticActivity{molps: self * rhs.molps.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&CatalyticActivity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = CatalyticActivity<num_complex::Complex32>;
	fn mul(self, rhs: &CatalyticActivity<num_complex::Complex32>) -> Self::Output {
		CatalyticActivity{molps: self.clone() * rhs.molps.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<CatalyticActivity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = CatalyticActivity<num_complex::Complex64>;
	fn mul(self, rhs: CatalyticActivity<num_complex::Complex64>) -> Self::Output {
		CatalyticActivity{molps: self * rhs.molps}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<CatalyticActivity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = CatalyticActivity<num_complex::Complex64>;
	fn mul(self, rhs: CatalyticActivity<num_complex::Complex64>) -> Self::Output {
		CatalyticActivity{molps: self.clone() * rhs.molps}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&CatalyticActivity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = CatalyticActivity<num_complex::Complex64>;
	fn mul(self, rhs: &CatalyticActivity<num_complex::Complex64>) -> Self::Output {
		CatalyticActivity{molps: self * rhs.molps.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&CatalyticActivity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = CatalyticActivity<num_complex::Complex64>;
	fn mul(self, rhs: &CatalyticActivity<num_complex::Complex64>) -> Self::Output {
		CatalyticActivity{molps: self.clone() * rhs.molps.clone()}
	}
}



/// Converts a CatalyticActivity into the equivalent [uom](https://crates.io/crates/uom) type [CatalyticActivity](https://docs.rs/uom/0.34.0/uom/si/f32/type.CatalyticActivity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::CatalyticActivity> for CatalyticActivity<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::CatalyticActivity {
		uom::si::f32::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(self.molps.into())
	}
}

/// Creates a CatalyticActivity from the equivalent [uom](https://crates.io/crates/uom) type [CatalyticActivity](https://docs.rs/uom/0.34.0/uom/si/f32/type.CatalyticActivity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::CatalyticActivity> for CatalyticActivity<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::CatalyticActivity) -> Self {
		CatalyticActivity{molps: T::from(src.value)}
	}
}

/// Converts a CatalyticActivity into the equivalent [uom](https://crates.io/crates/uom) type [CatalyticActivity](https://docs.rs/uom/0.34.0/uom/si/f64/type.CatalyticActivity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::CatalyticActivity> for CatalyticActivity<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::CatalyticActivity {
		uom::si::f64::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(self.molps.into())
	}
}

/// Creates a CatalyticActivity from the equivalent [uom](https://crates.io/crates/uom) type [CatalyticActivity](https://docs.rs/uom/0.34.0/uom/si/f64/type.CatalyticActivity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::CatalyticActivity> for CatalyticActivity<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::CatalyticActivity) -> Self {
		CatalyticActivity{molps: T::from(src.value)}
	}
}


// CatalyticActivity / Amount -> Frequency
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> core::ops::Div<Amount<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps / rhs.mol}
	}
}
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> core::ops::Div<Amount<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps.clone() / rhs.mol}
	}
}
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> core::ops::Div<&Amount<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps / rhs.mol.clone()}
	}
}
/// Dividing a CatalyticActivity by a Amount returns a value of type Frequency
impl<T> core::ops::Div<&Amount<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		Frequency{Hz: self.molps.clone() / rhs.mol.clone()}
	}
}

// CatalyticActivity * Time -> Amount
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> core::ops::Mul<Time<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Amount{mol: self.molps * rhs.s}
	}
}
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> core::ops::Mul<Time<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Amount{mol: self.molps.clone() * rhs.s}
	}
}
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> core::ops::Mul<&Time<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Amount{mol: self.molps * rhs.s.clone()}
	}
}
/// Multiplying a CatalyticActivity by a Time returns a value of type Amount
impl<T> core::ops::Mul<&Time<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Amount{mol: self.molps.clone() * rhs.s.clone()}
	}
}

// CatalyticActivity / Frequency -> Amount
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> core::ops::Div<Frequency<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Amount{mol: self.molps / rhs.Hz}
	}
}
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> core::ops::Div<Frequency<T>> for &CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Amount{mol: self.molps.clone() / rhs.Hz}
	}
}
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> core::ops::Div<&Frequency<T>> for CatalyticActivity<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Amount{mol: self.molps / rhs.Hz.clone()}
	}
}
/// Dividing a CatalyticActivity by a Frequency returns a value of type Amount
impl<T> core::ops::Div<&Frequency<T>> for &CatalyticActivity<T> where T: NumLike {
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
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Npm3(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic meter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Npm3` - Any number-like type, representing a quantity of count per cubic meter
	pub fn from_Npm3(Npm3: T) -> Self {
		Concentration{molpm3: Npm3 * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per cubic meter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_count_per_cubic_meter(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+23_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic meter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `count_per_cubic_meter` - Any number-like type, representing a quantity of count per cubic meter
	pub fn from_count_per_cubic_meter(count_per_cubic_meter: T) -> Self {
		Concentration{molpm3: count_per_cubic_meter * T::from(1.66053906717385e-24_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per liter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_NpL(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+26_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per liter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `NpL` - Any number-like type, representing a quantity of count per liter
	pub fn from_NpL(NpL: T) -> Self {
		Concentration{molpm3: NpL * T::from(1.66053906717385e-21_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per liter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_count_per_L(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+26_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per liter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `count_per_L` - Any number-like type, representing a quantity of count per liter
	pub fn from_count_per_L(count_per_L: T) -> Self {
		Concentration{molpm3: count_per_L * T::from(1.66053906717385e-21_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per cubic centimeter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Npcc(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+29_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic centimeter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Npcc` - Any number-like type, representing a quantity of count per cubic centimeter
	pub fn from_Npcc(Npcc: T) -> Self {
		Concentration{molpm3: Npcc * T::from(1.66053906717385e-18_f64)}
	}

	/// Returns a copy of this chemical concentration value in count per cubic centimeter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_count_per_cc(&self) -> T {
		return self.molpm3.clone() * T::from(6.02214076e+29_f64);
	}

	/// Returns a new chemical concentration value from the given number of count per cubic centimeter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `count_per_cc` - Any number-like type, representing a quantity of count per cubic centimeter
	pub fn from_count_per_cc(count_per_cc: T) -> Self {
		Concentration{molpm3: count_per_cc * T::from(1.66053906717385e-18_f64)}
	}

	/// Returns a copy of this chemical concentration value in moles per L
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_M(&self) -> T {
		return self.molpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new chemical concentration value from the given number of moles per L
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `M` - Any number-like type, representing a quantity of moles per L
	pub fn from_M(M: T) -> Self {
		Concentration{molpm3: M * T::from(1000.0_f64)}
	}

	/// Returns a copy of this chemical concentration value in moles per liter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_molarity(&self) -> T {
		return self.molpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new chemical concentration value from the given number of moles per liter
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `molarity` - Any number-like type, representing a quantity of moles per liter
	pub fn from_molarity(molarity: T) -> Self {
		Concentration{molpm3: molarity * T::from(1000.0_f64)}
	}

	/// Returns a copy of this chemical concentration value in micromolar
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uM(&self) -> T {
		return self.molpm3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new chemical concentration value from the given number of micromolar
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uM` - Any number-like type, representing a quantity of micromolar
	pub fn from_uM(uM: T) -> Self {
		Concentration{molpm3: uM * T::from(0.001_f64)}
	}

	/// Returns a copy of this chemical concentration value in nanomolar
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nM(&self) -> T {
		return self.molpm3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new chemical concentration value from the given number of nanomolar
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nM` - Any number-like type, representing a quantity of nanomolar
	pub fn from_nM(nM: T) -> Self {
		Concentration{molpm3: nM * T::from(1e-06_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Concentration<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Concentration<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Concentration<num_bigfloat::BigFloat>) -> Self::Output {
		Concentration{molpm3: self * rhs.molpm3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Concentration<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Concentration<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Concentration<num_bigfloat::BigFloat>) -> Self::Output {
		Concentration{molpm3: self.clone() * rhs.molpm3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Concentration<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Concentration<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Concentration<num_bigfloat::BigFloat>) -> Self::Output {
		Concentration{molpm3: self * rhs.molpm3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Concentration<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Concentration<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Concentration<num_bigfloat::BigFloat>) -> Self::Output {
		Concentration{molpm3: self.clone() * rhs.molpm3.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Concentration<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Concentration<num_complex::Complex32>;
	fn mul(self, rhs: Concentration<num_complex::Complex32>) -> Self::Output {
		Concentration{molpm3: self * rhs.molpm3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Concentration<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Concentration<num_complex::Complex32>;
	fn mul(self, rhs: Concentration<num_complex::Complex32>) -> Self::Output {
		Concentration{molpm3: self.clone() * rhs.molpm3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Concentration<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Concentration<num_complex::Complex32>;
	fn mul(self, rhs: &Concentration<num_complex::Complex32>) -> Self::Output {
		Concentration{molpm3: self * rhs.molpm3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Concentration<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Concentration<num_complex::Complex32>;
	fn mul(self, rhs: &Concentration<num_complex::Complex32>) -> Self::Output {
		Concentration{molpm3: self.clone() * rhs.molpm3.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Concentration<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Concentration<num_complex::Complex64>;
	fn mul(self, rhs: Concentration<num_complex::Complex64>) -> Self::Output {
		Concentration{molpm3: self * rhs.molpm3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Concentration<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Concentration<num_complex::Complex64>;
	fn mul(self, rhs: Concentration<num_complex::Complex64>) -> Self::Output {
		Concentration{molpm3: self.clone() * rhs.molpm3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Concentration<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Concentration<num_complex::Complex64>;
	fn mul(self, rhs: &Concentration<num_complex::Complex64>) -> Self::Output {
		Concentration{molpm3: self * rhs.molpm3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Concentration<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Concentration<num_complex::Complex64>;
	fn mul(self, rhs: &Concentration<num_complex::Complex64>) -> Self::Output {
		Concentration{molpm3: self.clone() * rhs.molpm3.clone()}
	}
}



/// Converts a Concentration into the equivalent [uom](https://crates.io/crates/uom) type [MolarConcentration](https://docs.rs/uom/0.34.0/uom/si/f32/type.MolarConcentration.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::MolarConcentration> for Concentration<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::MolarConcentration {
		uom::si::f32::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(self.molpm3.into())
	}
}

/// Creates a Concentration from the equivalent [uom](https://crates.io/crates/uom) type [MolarConcentration](https://docs.rs/uom/0.34.0/uom/si/f32/type.MolarConcentration.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::MolarConcentration> for Concentration<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::MolarConcentration) -> Self {
		Concentration{molpm3: T::from(src.value)}
	}
}

/// Converts a Concentration into the equivalent [uom](https://crates.io/crates/uom) type [MolarConcentration](https://docs.rs/uom/0.34.0/uom/si/f64/type.MolarConcentration.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::MolarConcentration> for Concentration<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::MolarConcentration {
		uom::si::f64::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(self.molpm3.into())
	}
}

/// Creates a Concentration from the equivalent [uom](https://crates.io/crates/uom) type [MolarConcentration](https://docs.rs/uom/0.34.0/uom/si/f64/type.MolarConcentration.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::MolarConcentration> for Concentration<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::MolarConcentration) -> Self {
		Concentration{molpm3: T::from(src.value)}
	}
}


// Concentration / Molality -> Density
/// Dividing a Concentration by a Molality returns a value of type Density
impl<T> core::ops::Div<Molality<T>> for Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		Density{kgpm3: self.molpm3 / rhs.molpkg}
	}
}
/// Dividing a Concentration by a Molality returns a value of type Density
impl<T> core::ops::Div<Molality<T>> for &Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		Density{kgpm3: self.molpm3.clone() / rhs.molpkg}
	}
}
/// Dividing a Concentration by a Molality returns a value of type Density
impl<T> core::ops::Div<&Molality<T>> for Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		Density{kgpm3: self.molpm3 / rhs.molpkg.clone()}
	}
}
/// Dividing a Concentration by a Molality returns a value of type Density
impl<T> core::ops::Div<&Molality<T>> for &Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		Density{kgpm3: self.molpm3.clone() / rhs.molpkg.clone()}
	}
}

// Concentration * MolarMass -> Density
/// Multiplying a Concentration by a MolarMass returns a value of type Density
impl<T> core::ops::Mul<MolarMass<T>> for Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: MolarMass<T>) -> Self::Output {
		Density{kgpm3: self.molpm3 * rhs.kgpmol}
	}
}
/// Multiplying a Concentration by a MolarMass returns a value of type Density
impl<T> core::ops::Mul<MolarMass<T>> for &Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: MolarMass<T>) -> Self::Output {
		Density{kgpm3: self.molpm3.clone() * rhs.kgpmol}
	}
}
/// Multiplying a Concentration by a MolarMass returns a value of type Density
impl<T> core::ops::Mul<&MolarMass<T>> for Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &MolarMass<T>) -> Self::Output {
		Density{kgpm3: self.molpm3 * rhs.kgpmol.clone()}
	}
}
/// Multiplying a Concentration by a MolarMass returns a value of type Density
impl<T> core::ops::Mul<&MolarMass<T>> for &Concentration<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &MolarMass<T>) -> Self::Output {
		Density{kgpm3: self.molpm3.clone() * rhs.kgpmol.clone()}
	}
}

// Concentration * Volume -> Amount
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> core::ops::Mul<Volume<T>> for Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3 * rhs.m3}
	}
}
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> core::ops::Mul<Volume<T>> for &Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3.clone() * rhs.m3}
	}
}
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> core::ops::Mul<&Volume<T>> for Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3 * rhs.m3.clone()}
	}
}
/// Multiplying a Concentration by a Volume returns a value of type Amount
impl<T> core::ops::Mul<&Volume<T>> for &Concentration<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Amount{mol: self.molpm3.clone() * rhs.m3.clone()}
	}
}

// Concentration / Density -> Molality
/// Dividing a Concentration by a Density returns a value of type Molality
impl<T> core::ops::Div<Density<T>> for Concentration<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		Molality{molpkg: self.molpm3 / rhs.kgpm3}
	}
}
/// Dividing a Concentration by a Density returns a value of type Molality
impl<T> core::ops::Div<Density<T>> for &Concentration<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		Molality{molpkg: self.molpm3.clone() / rhs.kgpm3}
	}
}
/// Dividing a Concentration by a Density returns a value of type Molality
impl<T> core::ops::Div<&Density<T>> for Concentration<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		Molality{molpkg: self.molpm3 / rhs.kgpm3.clone()}
	}
}
/// Dividing a Concentration by a Density returns a value of type Molality
impl<T> core::ops::Div<&Density<T>> for &Concentration<T> where T: NumLike {
	type Output = Molality<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		Molality{molpkg: self.molpm3.clone() / rhs.kgpm3.clone()}
	}
}

/// The molality unit type, defined as moles per kilogram in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Molality<T: NumLike>{
	/// The value of this Molality in moles per kilogram
	pub molpkg: T
}

impl<T> Molality<T> where T: NumLike {

	/// Returns the standard unit name of molality: "moles per kilogram"
	pub fn unit_name() -> &'static str { "moles per kilogram" }
	
	/// Returns the abbreviated name or symbol of molality: "mol/kg" for moles per kilogram
	pub fn unit_symbol() -> &'static str { "mol/kg" }
	
	/// Returns a new molality value from the given number of moles per kilogram
	///
	/// # Arguments
	/// * `molpkg` - Any number-like type, representing a quantity of moles per kilogram
	pub fn from_molpkg(molpkg: T) -> Self { Molality{molpkg: molpkg} }
	
	/// Returns a copy of this molality value in moles per kilogram
	pub fn to_molpkg(&self) -> T { self.molpkg.clone() }

	/// Returns a new molality value from the given number of moles per kilogram
	///
	/// # Arguments
	/// * `moles_per_kilogram` - Any number-like type, representing a quantity of moles per kilogram
	pub fn from_moles_per_kilogram(moles_per_kilogram: T) -> Self { Molality{molpkg: moles_per_kilogram} }
	
	/// Returns a copy of this molality value in moles per kilogram
	pub fn to_moles_per_kilogram(&self) -> T { self.molpkg.clone() }

	/// Returns a new molality value from the given number of millimoles per gram
	///
	/// # Arguments
	/// * `mmolpg` - Any number-like type, representing a quantity of moles per kilogram
	pub fn from_mmolpg(mmolpg: T) -> Self { Molality{molpkg: mmolpg} }
	
	/// Returns a copy of this molality value in millimoles per gram
	pub fn to_mmolpg(&self) -> T { self.molpkg.clone() }

}

impl<T> fmt::Display for Molality<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.molpkg, Self::unit_symbol())
	}
}

impl<T> Molality<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this molality value in millimoles per kilogram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mmolpkg(&self) -> T {
		return self.molpkg.clone() * T::from(1000.0_f64);
	}

	/// Returns a new molality value from the given number of millimoles per kilogram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mmolpkg` - Any number-like type, representing a quantity of millimoles per kilogram
	pub fn from_mmolpkg(mmolpkg: T) -> Self {
		Molality{molpkg: mmolpkg * T::from(0.001_f64)}
	}

	/// Returns a copy of this molality value in micromoles per kilogram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_umolpkg(&self) -> T {
		return self.molpkg.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new molality value from the given number of micromoles per kilogram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `umolpkg` - Any number-like type, representing a quantity of micromoles per kilogram
	pub fn from_umolpkg(umolpkg: T) -> Self {
		Molality{molpkg: umolpkg * T::from(1e-06_f64)}
	}

	/// Returns a copy of this molality value in nanomoles per kilogram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nmolpkg(&self) -> T {
		return self.molpkg.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new molality value from the given number of nanomoles per kilogram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nmolpkg` - Any number-like type, representing a quantity of nanomoles per kilogram
	pub fn from_nmolpkg(nmolpkg: T) -> Self {
		Molality{molpkg: nmolpkg * T::from(1e-09_f64)}
	}

	/// Returns a copy of this molality value in micromoles per gram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_umolpg(&self) -> T {
		return self.molpkg.clone() * T::from(1000.0_f64);
	}

	/// Returns a new molality value from the given number of micromoles per gram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `umolpg` - Any number-like type, representing a quantity of micromoles per gram
	pub fn from_umolpg(umolpg: T) -> Self {
		Molality{molpkg: umolpg * T::from(0.001_f64)}
	}

	/// Returns a copy of this molality value in nanomoles per gram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nmolpg(&self) -> T {
		return self.molpkg.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new molality value from the given number of nanomoles per gram
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nmolpg` - Any number-like type, representing a quantity of nanomoles per gram
	pub fn from_nmolpg(nmolpg: T) -> Self {
		Molality{molpkg: nmolpg * T::from(1e-06_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Molality<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Molality<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Molality<num_bigfloat::BigFloat>) -> Self::Output {
		Molality{molpkg: self * rhs.molpkg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Molality<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Molality<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Molality<num_bigfloat::BigFloat>) -> Self::Output {
		Molality{molpkg: self.clone() * rhs.molpkg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Molality<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Molality<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Molality<num_bigfloat::BigFloat>) -> Self::Output {
		Molality{molpkg: self * rhs.molpkg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Molality<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Molality<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Molality<num_bigfloat::BigFloat>) -> Self::Output {
		Molality{molpkg: self.clone() * rhs.molpkg.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Molality<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Molality<num_complex::Complex32>;
	fn mul(self, rhs: Molality<num_complex::Complex32>) -> Self::Output {
		Molality{molpkg: self * rhs.molpkg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Molality<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Molality<num_complex::Complex32>;
	fn mul(self, rhs: Molality<num_complex::Complex32>) -> Self::Output {
		Molality{molpkg: self.clone() * rhs.molpkg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Molality<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Molality<num_complex::Complex32>;
	fn mul(self, rhs: &Molality<num_complex::Complex32>) -> Self::Output {
		Molality{molpkg: self * rhs.molpkg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Molality<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Molality<num_complex::Complex32>;
	fn mul(self, rhs: &Molality<num_complex::Complex32>) -> Self::Output {
		Molality{molpkg: self.clone() * rhs.molpkg.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Molality<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Molality<num_complex::Complex64>;
	fn mul(self, rhs: Molality<num_complex::Complex64>) -> Self::Output {
		Molality{molpkg: self * rhs.molpkg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Molality<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Molality<num_complex::Complex64>;
	fn mul(self, rhs: Molality<num_complex::Complex64>) -> Self::Output {
		Molality{molpkg: self.clone() * rhs.molpkg}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Molality<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Molality<num_complex::Complex64>;
	fn mul(self, rhs: &Molality<num_complex::Complex64>) -> Self::Output {
		Molality{molpkg: self * rhs.molpkg.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Molality<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Molality<num_complex::Complex64>;
	fn mul(self, rhs: &Molality<num_complex::Complex64>) -> Self::Output {
		Molality{molpkg: self.clone() * rhs.molpkg.clone()}
	}
}



/// Converts a Molality into the equivalent [uom](https://crates.io/crates/uom) type [Molality](https://docs.rs/uom/0.34.0/uom/si/f32/type.Molality.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Molality> for Molality<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Molality {
		uom::si::f32::Molality::new::<uom::si::molality::mole_per_kilogram>(self.molpkg.into())
	}
}

/// Creates a Molality from the equivalent [uom](https://crates.io/crates/uom) type [Molality](https://docs.rs/uom/0.34.0/uom/si/f32/type.Molality.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Molality> for Molality<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Molality) -> Self {
		Molality{molpkg: T::from(src.value)}
	}
}

/// Converts a Molality into the equivalent [uom](https://crates.io/crates/uom) type [Molality](https://docs.rs/uom/0.34.0/uom/si/f64/type.Molality.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Molality> for Molality<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Molality {
		uom::si::f64::Molality::new::<uom::si::molality::mole_per_kilogram>(self.molpkg.into())
	}
}

/// Creates a Molality from the equivalent [uom](https://crates.io/crates/uom) type [Molality](https://docs.rs/uom/0.34.0/uom/si/f64/type.Molality.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Molality> for Molality<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Molality) -> Self {
		Molality{molpkg: T::from(src.value)}
	}
}


// Molality * Mass -> Amount
/// Multiplying a Molality by a Mass returns a value of type Amount
impl<T> core::ops::Mul<Mass<T>> for Molality<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Amount{mol: self.molpkg * rhs.kg}
	}
}
/// Multiplying a Molality by a Mass returns a value of type Amount
impl<T> core::ops::Mul<Mass<T>> for &Molality<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Amount{mol: self.molpkg.clone() * rhs.kg}
	}
}
/// Multiplying a Molality by a Mass returns a value of type Amount
impl<T> core::ops::Mul<&Mass<T>> for Molality<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Amount{mol: self.molpkg * rhs.kg.clone()}
	}
}
/// Multiplying a Molality by a Mass returns a value of type Amount
impl<T> core::ops::Mul<&Mass<T>> for &Molality<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Amount{mol: self.molpkg.clone() * rhs.kg.clone()}
	}
}

// Molality * Density -> Concentration
/// Multiplying a Molality by a Density returns a value of type Concentration
impl<T> core::ops::Mul<Density<T>> for Molality<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		Concentration{molpm3: self.molpkg * rhs.kgpm3}
	}
}
/// Multiplying a Molality by a Density returns a value of type Concentration
impl<T> core::ops::Mul<Density<T>> for &Molality<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		Concentration{molpm3: self.molpkg.clone() * rhs.kgpm3}
	}
}
/// Multiplying a Molality by a Density returns a value of type Concentration
impl<T> core::ops::Mul<&Density<T>> for Molality<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		Concentration{molpm3: self.molpkg * rhs.kgpm3.clone()}
	}
}
/// Multiplying a Molality by a Density returns a value of type Concentration
impl<T> core::ops::Mul<&Density<T>> for &Molality<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		Concentration{molpm3: self.molpkg.clone() * rhs.kgpm3.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for f64 where T: NumLike+From<f64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for &f64 where T: NumLike+From<f64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for f64 where T: NumLike+From<f64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for &f64 where T: NumLike+From<f64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for f32 where T: NumLike+From<f32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for &f32 where T: NumLike+From<f32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for f32 where T: NumLike+From<f32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for &f32 where T: NumLike+From<f32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for i64 where T: NumLike+From<i64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for &i64 where T: NumLike+From<i64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for i64 where T: NumLike+From<i64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for &i64 where T: NumLike+From<i64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for i32 where T: NumLike+From<i32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<Molality<T>> for &i32 where T: NumLike+From<i32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for i32 where T: NumLike+From<i32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
impl<T> core::ops::Div<&Molality<T>> for &i32 where T: NumLike+From<i32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Molality<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Molality<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Molality<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Molality<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Molality<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Molality<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Molality<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Molality<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

// 1/Molality -> MolarMass
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Molality<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Molality<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Molality<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self) / rhs.molpkg.clone()}
	}
}
/// Dividing a scalar value by a Molality unit value returns a value of type MolarMass
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Molality<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MolarMass<T>;
	fn div(self, rhs: &Molality<T>) -> Self::Output {
		MolarMass{kgpmol: T::from(self.clone()) / rhs.molpkg.clone()}
	}
}

/// The molar mass unit type, defined as kilograms per mole in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MolarMass<T: NumLike>{
	/// The value of this Molar mass in kilograms per mole
	pub kgpmol: T
}

impl<T> MolarMass<T> where T: NumLike {

	/// Returns the standard unit name of molar mass: "kilograms per mole"
	pub fn unit_name() -> &'static str { "kilograms per mole" }
	
	/// Returns the abbreviated name or symbol of molar mass: "kg/mol" for kilograms per mole
	pub fn unit_symbol() -> &'static str { "kg/mol" }
	
	/// Returns a new molar mass value from the given number of kilograms per mole
	///
	/// # Arguments
	/// * `kgpmol` - Any number-like type, representing a quantity of kilograms per mole
	pub fn from_kgpmol(kgpmol: T) -> Self { MolarMass{kgpmol: kgpmol} }
	
	/// Returns a copy of this molar mass value in kilograms per mole
	pub fn to_kgpmol(&self) -> T { self.kgpmol.clone() }

	/// Returns a new molar mass value from the given number of kilograms per mole
	///
	/// # Arguments
	/// * `kilograms_per_mole` - Any number-like type, representing a quantity of kilograms per mole
	pub fn from_kilograms_per_mole(kilograms_per_mole: T) -> Self { MolarMass{kgpmol: kilograms_per_mole} }
	
	/// Returns a copy of this molar mass value in kilograms per mole
	pub fn to_kilograms_per_mole(&self) -> T { self.kgpmol.clone() }

}

impl<T> fmt::Display for MolarMass<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgpmol, Self::unit_symbol())
	}
}

impl<T> MolarMass<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this molar mass value in grams per mole
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_gpmol(&self) -> T {
		return self.kgpmol.clone() * T::from(1000.0_f64);
	}

	/// Returns a new molar mass value from the given number of grams per mole
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `gpmol` - Any number-like type, representing a quantity of grams per mole
	pub fn from_gpmol(gpmol: T) -> Self {
		MolarMass{kgpmol: gpmol * T::from(0.001_f64)}
	}

	/// Returns a copy of this molar mass value in grams per mole
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_grams_per_mole(&self) -> T {
		return self.kgpmol.clone() * T::from(1000.0_f64);
	}

	/// Returns a new molar mass value from the given number of grams per mole
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `grams_per_mole` - Any number-like type, representing a quantity of grams per mole
	pub fn from_grams_per_mole(grams_per_mole: T) -> Self {
		MolarMass{kgpmol: grams_per_mole * T::from(0.001_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<MolarMass<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = MolarMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: MolarMass<num_bigfloat::BigFloat>) -> Self::Output {
		MolarMass{kgpmol: self * rhs.kgpmol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<MolarMass<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = MolarMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: MolarMass<num_bigfloat::BigFloat>) -> Self::Output {
		MolarMass{kgpmol: self.clone() * rhs.kgpmol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&MolarMass<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = MolarMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &MolarMass<num_bigfloat::BigFloat>) -> Self::Output {
		MolarMass{kgpmol: self * rhs.kgpmol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&MolarMass<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = MolarMass<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &MolarMass<num_bigfloat::BigFloat>) -> Self::Output {
		MolarMass{kgpmol: self.clone() * rhs.kgpmol.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MolarMass<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = MolarMass<num_complex::Complex32>;
	fn mul(self, rhs: MolarMass<num_complex::Complex32>) -> Self::Output {
		MolarMass{kgpmol: self * rhs.kgpmol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MolarMass<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = MolarMass<num_complex::Complex32>;
	fn mul(self, rhs: MolarMass<num_complex::Complex32>) -> Self::Output {
		MolarMass{kgpmol: self.clone() * rhs.kgpmol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MolarMass<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = MolarMass<num_complex::Complex32>;
	fn mul(self, rhs: &MolarMass<num_complex::Complex32>) -> Self::Output {
		MolarMass{kgpmol: self * rhs.kgpmol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MolarMass<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = MolarMass<num_complex::Complex32>;
	fn mul(self, rhs: &MolarMass<num_complex::Complex32>) -> Self::Output {
		MolarMass{kgpmol: self.clone() * rhs.kgpmol.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MolarMass<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = MolarMass<num_complex::Complex64>;
	fn mul(self, rhs: MolarMass<num_complex::Complex64>) -> Self::Output {
		MolarMass{kgpmol: self * rhs.kgpmol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MolarMass<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = MolarMass<num_complex::Complex64>;
	fn mul(self, rhs: MolarMass<num_complex::Complex64>) -> Self::Output {
		MolarMass{kgpmol: self.clone() * rhs.kgpmol}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MolarMass<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = MolarMass<num_complex::Complex64>;
	fn mul(self, rhs: &MolarMass<num_complex::Complex64>) -> Self::Output {
		MolarMass{kgpmol: self * rhs.kgpmol.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MolarMass<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = MolarMass<num_complex::Complex64>;
	fn mul(self, rhs: &MolarMass<num_complex::Complex64>) -> Self::Output {
		MolarMass{kgpmol: self.clone() * rhs.kgpmol.clone()}
	}
}



/// Converts a MolarMass into the equivalent [uom](https://crates.io/crates/uom) type [MolarMass](https://docs.rs/uom/0.34.0/uom/si/f32/type.MolarMass.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::MolarMass> for MolarMass<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::MolarMass {
		uom::si::f32::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(self.kgpmol.into())
	}
}

/// Creates a MolarMass from the equivalent [uom](https://crates.io/crates/uom) type [MolarMass](https://docs.rs/uom/0.34.0/uom/si/f32/type.MolarMass.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::MolarMass> for MolarMass<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::MolarMass) -> Self {
		MolarMass{kgpmol: T::from(src.value)}
	}
}

/// Converts a MolarMass into the equivalent [uom](https://crates.io/crates/uom) type [MolarMass](https://docs.rs/uom/0.34.0/uom/si/f64/type.MolarMass.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::MolarMass> for MolarMass<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::MolarMass {
		uom::si::f64::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(self.kgpmol.into())
	}
}

/// Creates a MolarMass from the equivalent [uom](https://crates.io/crates/uom) type [MolarMass](https://docs.rs/uom/0.34.0/uom/si/f64/type.MolarMass.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::MolarMass> for MolarMass<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::MolarMass) -> Self {
		MolarMass{kgpmol: T::from(src.value)}
	}
}


// MolarMass * Amount -> Mass
/// Multiplying a MolarMass by a Amount returns a value of type Mass
impl<T> core::ops::Mul<Amount<T>> for MolarMass<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		Mass{kg: self.kgpmol * rhs.mol}
	}
}
/// Multiplying a MolarMass by a Amount returns a value of type Mass
impl<T> core::ops::Mul<Amount<T>> for &MolarMass<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		Mass{kg: self.kgpmol.clone() * rhs.mol}
	}
}
/// Multiplying a MolarMass by a Amount returns a value of type Mass
impl<T> core::ops::Mul<&Amount<T>> for MolarMass<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		Mass{kg: self.kgpmol * rhs.mol.clone()}
	}
}
/// Multiplying a MolarMass by a Amount returns a value of type Mass
impl<T> core::ops::Mul<&Amount<T>> for &MolarMass<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		Mass{kg: self.kgpmol.clone() * rhs.mol.clone()}
	}
}

// MolarMass * Concentration -> Density
/// Multiplying a MolarMass by a Concentration returns a value of type Density
impl<T> core::ops::Mul<Concentration<T>> for MolarMass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		Density{kgpm3: self.kgpmol * rhs.molpm3}
	}
}
/// Multiplying a MolarMass by a Concentration returns a value of type Density
impl<T> core::ops::Mul<Concentration<T>> for &MolarMass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		Density{kgpm3: self.kgpmol.clone() * rhs.molpm3}
	}
}
/// Multiplying a MolarMass by a Concentration returns a value of type Density
impl<T> core::ops::Mul<&Concentration<T>> for MolarMass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		Density{kgpm3: self.kgpmol * rhs.molpm3.clone()}
	}
}
/// Multiplying a MolarMass by a Concentration returns a value of type Density
impl<T> core::ops::Mul<&Concentration<T>> for &MolarMass<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		Density{kgpm3: self.kgpmol.clone() * rhs.molpm3.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for f64 where T: NumLike+From<f64> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for f64 where T: NumLike+From<f64> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for f32 where T: NumLike+From<f32> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for f32 where T: NumLike+From<f32> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for i64 where T: NumLike+From<i64> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for i64 where T: NumLike+From<i64> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for i32 where T: NumLike+From<i32> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<MolarMass<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for i32 where T: NumLike+From<i32> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
impl<T> core::ops::Div<&MolarMass<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<MolarMass<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<MolarMass<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&MolarMass<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&MolarMass<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MolarMass<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MolarMass<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MolarMass<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MolarMass<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

// 1/MolarMass -> Molality
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MolarMass<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MolarMass<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Molality<T>;
	fn div(self, rhs: MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MolarMass<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self) / rhs.kgpmol.clone()}
	}
}
/// Dividing a scalar value by a MolarMass unit value returns a value of type Molality
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MolarMass<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Molality<T>;
	fn div(self, rhs: &MolarMass<T>) -> Self::Output {
		Molality{molpkg: T::from(self.clone()) / rhs.kgpmol.clone()}
	}
}

/// The specific heat capacity unit type, defined as joules per kilogram per kelvin in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct SpecificHeatCapacity<T: NumLike>{
	/// The value of this Specific heat capacity in joules per kilogram per kelvin
	pub JpkgK: T
}

impl<T> SpecificHeatCapacity<T> where T: NumLike {

	/// Returns the standard unit name of specific heat capacity: "joules per kilogram per kelvin"
	pub fn unit_name() -> &'static str { "joules per kilogram per kelvin" }
	
	/// Returns the abbreviated name or symbol of specific heat capacity: "J/kg·K" for joules per kilogram per kelvin
	pub fn unit_symbol() -> &'static str { "J/kg·K" }
	
	/// Returns a new specific heat capacity value from the given number of joules per kilogram per kelvin
	///
	/// # Arguments
	/// * `joules_per_kilogram_kelvin` - Any number-like type, representing a quantity of joules per kilogram per kelvin
	pub fn from_joules_per_kilogram_kelvin(joules_per_kilogram_kelvin: T) -> Self { SpecificHeatCapacity{JpkgK: joules_per_kilogram_kelvin} }
	
	/// Returns a copy of this specific heat capacity value in joules per kilogram per kelvin
	pub fn to_joules_per_kilogram_kelvin(&self) -> T { self.JpkgK.clone() }

	/// Returns a new specific heat capacity value from the given number of joules per kilogram per kelvin
	///
	/// # Arguments
	/// * `JpkgK ` - Any number-like type, representing a quantity of joules per kilogram per kelvin
	pub fn from_JpkgK (JpkgK : T) -> Self { SpecificHeatCapacity{JpkgK: JpkgK } }
	
	/// Returns a copy of this specific heat capacity value in joules per kilogram per kelvin
	pub fn to_JpkgK (&self) -> T { self.JpkgK.clone() }

}

impl<T> fmt::Display for SpecificHeatCapacity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.JpkgK, Self::unit_symbol())
	}
}

impl<T> SpecificHeatCapacity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this specific heat capacity value in joules per gram per kelvin
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_joules_per_gram_kelvin(&self) -> T {
		return self.JpkgK.clone() * T::from(0.001_f64);
	}

	/// Returns a new specific heat capacity value from the given number of joules per gram per kelvin
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `joules_per_gram_kelvin` - Any number-like type, representing a quantity of joules per gram per kelvin
	pub fn from_joules_per_gram_kelvin(joules_per_gram_kelvin: T) -> Self {
		SpecificHeatCapacity{JpkgK: joules_per_gram_kelvin * T::from(1000.0_f64)}
	}

	/// Returns a copy of this specific heat capacity value in joules per gram per kelvin
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_JpgK (&self) -> T {
		return self.JpkgK.clone() * T::from(0.001_f64);
	}

	/// Returns a new specific heat capacity value from the given number of joules per gram per kelvin
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `JpgK ` - Any number-like type, representing a quantity of joules per gram per kelvin
	pub fn from_JpgK (JpgK : T) -> Self {
		SpecificHeatCapacity{JpkgK: JpgK  * T::from(0.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<SpecificHeatCapacity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = SpecificHeatCapacity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: SpecificHeatCapacity<num_bigfloat::BigFloat>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self * rhs.JpkgK}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<SpecificHeatCapacity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = SpecificHeatCapacity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: SpecificHeatCapacity<num_bigfloat::BigFloat>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self.clone() * rhs.JpkgK}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&SpecificHeatCapacity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = SpecificHeatCapacity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &SpecificHeatCapacity<num_bigfloat::BigFloat>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self * rhs.JpkgK.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&SpecificHeatCapacity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = SpecificHeatCapacity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &SpecificHeatCapacity<num_bigfloat::BigFloat>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self.clone() * rhs.JpkgK.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SpecificHeatCapacity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = SpecificHeatCapacity<num_complex::Complex32>;
	fn mul(self, rhs: SpecificHeatCapacity<num_complex::Complex32>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self * rhs.JpkgK}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SpecificHeatCapacity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = SpecificHeatCapacity<num_complex::Complex32>;
	fn mul(self, rhs: SpecificHeatCapacity<num_complex::Complex32>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self.clone() * rhs.JpkgK}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SpecificHeatCapacity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = SpecificHeatCapacity<num_complex::Complex32>;
	fn mul(self, rhs: &SpecificHeatCapacity<num_complex::Complex32>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self * rhs.JpkgK.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SpecificHeatCapacity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = SpecificHeatCapacity<num_complex::Complex32>;
	fn mul(self, rhs: &SpecificHeatCapacity<num_complex::Complex32>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self.clone() * rhs.JpkgK.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SpecificHeatCapacity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = SpecificHeatCapacity<num_complex::Complex64>;
	fn mul(self, rhs: SpecificHeatCapacity<num_complex::Complex64>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self * rhs.JpkgK}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SpecificHeatCapacity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = SpecificHeatCapacity<num_complex::Complex64>;
	fn mul(self, rhs: SpecificHeatCapacity<num_complex::Complex64>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self.clone() * rhs.JpkgK}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SpecificHeatCapacity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = SpecificHeatCapacity<num_complex::Complex64>;
	fn mul(self, rhs: &SpecificHeatCapacity<num_complex::Complex64>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self * rhs.JpkgK.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SpecificHeatCapacity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = SpecificHeatCapacity<num_complex::Complex64>;
	fn mul(self, rhs: &SpecificHeatCapacity<num_complex::Complex64>) -> Self::Output {
		SpecificHeatCapacity{JpkgK: self.clone() * rhs.JpkgK.clone()}
	}
}



/// Converts a SpecificHeatCapacity into the equivalent [uom](https://crates.io/crates/uom) type [SpecificHeatCapacity](https://docs.rs/uom/0.34.0/uom/si/f32/type.SpecificHeatCapacity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::SpecificHeatCapacity> for SpecificHeatCapacity<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::SpecificHeatCapacity {
		uom::si::f32::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(self.JpkgK.into())
	}
}

/// Creates a SpecificHeatCapacity from the equivalent [uom](https://crates.io/crates/uom) type [SpecificHeatCapacity](https://docs.rs/uom/0.34.0/uom/si/f32/type.SpecificHeatCapacity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::SpecificHeatCapacity> for SpecificHeatCapacity<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::SpecificHeatCapacity) -> Self {
		SpecificHeatCapacity{JpkgK: T::from(src.value)}
	}
}

/// Converts a SpecificHeatCapacity into the equivalent [uom](https://crates.io/crates/uom) type [SpecificHeatCapacity](https://docs.rs/uom/0.34.0/uom/si/f64/type.SpecificHeatCapacity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::SpecificHeatCapacity> for SpecificHeatCapacity<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::SpecificHeatCapacity {
		uom::si::f64::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(self.JpkgK.into())
	}
}

/// Creates a SpecificHeatCapacity from the equivalent [uom](https://crates.io/crates/uom) type [SpecificHeatCapacity](https://docs.rs/uom/0.34.0/uom/si/f64/type.SpecificHeatCapacity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::SpecificHeatCapacity> for SpecificHeatCapacity<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::SpecificHeatCapacity) -> Self {
		SpecificHeatCapacity{JpkgK: T::from(src.value)}
	}
}




