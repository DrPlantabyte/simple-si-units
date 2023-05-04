
//! This module provides electromagnetic SI units, such as inverse of illuminance 
//! and electric charge (aka coulombs).
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



/// The inverse of illuminance unit type, defined as square meters per lumen in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AreaPerLumen<T: NumLike>{
	/// The value of this Area per lumen in square meters per lumen
	pub m2_per_lm: T
}

impl<T> AreaPerLumen<T> where T: NumLike {

	/// Returns the standard unit name of area per lumen: "square meters per lumen"
	pub fn unit_name() -> &'static str { "square meters per lumen" }
	
	/// Returns the abbreviated name or symbol of area per lumen: "m²/lm" for square meters per lumen
	pub fn unit_symbol() -> &'static str { "m²/lm" }
	
	/// Returns a new area per lumen value from the given number of square meters per lumen
	///
	/// # Arguments
	/// * `m2_per_lm` - Any number-like type, representing a quantity of square meters per lumen
	pub fn from_m2_per_lm(m2_per_lm: T) -> Self { AreaPerLumen{m2_per_lm: m2_per_lm} }
	
	/// Returns a copy of this area per lumen value in square meters per lumen
	pub fn to_m2_per_lm(&self) -> T { self.m2_per_lm.clone() }

	/// Returns a new area per lumen value from the given number of square meters per lumen
	///
	/// # Arguments
	/// * `square_meters_per_lumen` - Any number-like type, representing a quantity of square meters per lumen
	pub fn from_square_meters_per_lumen(square_meters_per_lumen: T) -> Self { AreaPerLumen{m2_per_lm: square_meters_per_lumen} }
	
	/// Returns a copy of this area per lumen value in square meters per lumen
	pub fn to_square_meters_per_lumen(&self) -> T { self.m2_per_lm.clone() }

	/// Returns a new area per lumen value from the given number of inverse lux
	///
	/// # Arguments
	/// * `per_lux` - Any number-like type, representing a quantity of square meters per lumen
	pub fn from_per_lux(per_lux: T) -> Self { AreaPerLumen{m2_per_lm: per_lux} }
	
	/// Returns a copy of this area per lumen value in inverse lux
	pub fn to_per_lux(&self) -> T { self.m2_per_lm.clone() }

}

impl<T> fmt::Display for AreaPerLumen<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m2_per_lm, Self::unit_symbol())
	}
}

impl<T> AreaPerLumen<T> where T: NumLike+From<f64> {
	
}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<AreaPerLumen<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = AreaPerLumen<num_bigfloat::BigFloat>;
	fn mul(self, rhs: AreaPerLumen<num_bigfloat::BigFloat>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self * rhs.m2_per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<AreaPerLumen<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = AreaPerLumen<num_bigfloat::BigFloat>;
	fn mul(self, rhs: AreaPerLumen<num_bigfloat::BigFloat>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.clone() * rhs.m2_per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&AreaPerLumen<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = AreaPerLumen<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &AreaPerLumen<num_bigfloat::BigFloat>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self * rhs.m2_per_lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&AreaPerLumen<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = AreaPerLumen<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &AreaPerLumen<num_bigfloat::BigFloat>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.clone() * rhs.m2_per_lm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<AreaPerLumen<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = AreaPerLumen<num_complex::Complex32>;
	fn mul(self, rhs: AreaPerLumen<num_complex::Complex32>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self * rhs.m2_per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<AreaPerLumen<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = AreaPerLumen<num_complex::Complex32>;
	fn mul(self, rhs: AreaPerLumen<num_complex::Complex32>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.clone() * rhs.m2_per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&AreaPerLumen<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = AreaPerLumen<num_complex::Complex32>;
	fn mul(self, rhs: &AreaPerLumen<num_complex::Complex32>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self * rhs.m2_per_lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&AreaPerLumen<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = AreaPerLumen<num_complex::Complex32>;
	fn mul(self, rhs: &AreaPerLumen<num_complex::Complex32>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.clone() * rhs.m2_per_lm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<AreaPerLumen<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = AreaPerLumen<num_complex::Complex64>;
	fn mul(self, rhs: AreaPerLumen<num_complex::Complex64>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self * rhs.m2_per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<AreaPerLumen<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = AreaPerLumen<num_complex::Complex64>;
	fn mul(self, rhs: AreaPerLumen<num_complex::Complex64>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.clone() * rhs.m2_per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&AreaPerLumen<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = AreaPerLumen<num_complex::Complex64>;
	fn mul(self, rhs: &AreaPerLumen<num_complex::Complex64>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self * rhs.m2_per_lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&AreaPerLumen<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = AreaPerLumen<num_complex::Complex64>;
	fn mul(self, rhs: &AreaPerLumen<num_complex::Complex64>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.clone() * rhs.m2_per_lm.clone()}
	}
}




// AreaPerLumen / InverseLuminousFlux -> Area
/// Dividing a AreaPerLumen by a InverseLuminousFlux returns a value of type Area
impl<T> core::ops::Div<InverseLuminousFlux<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm / rhs.per_lm}
	}
}
/// Dividing a AreaPerLumen by a InverseLuminousFlux returns a value of type Area
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm.clone() / rhs.per_lm}
	}
}
/// Dividing a AreaPerLumen by a InverseLuminousFlux returns a value of type Area
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm / rhs.per_lm.clone()}
	}
}
/// Dividing a AreaPerLumen by a InverseLuminousFlux returns a value of type Area
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm.clone() / rhs.per_lm.clone()}
	}
}

// AreaPerLumen * LuminousFlux -> Area
/// Multiplying a AreaPerLumen by a LuminousFlux returns a value of type Area
impl<T> core::ops::Mul<LuminousFlux<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm * rhs.lm}
	}
}
/// Multiplying a AreaPerLumen by a LuminousFlux returns a value of type Area
impl<T> core::ops::Mul<LuminousFlux<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm.clone() * rhs.lm}
	}
}
/// Multiplying a AreaPerLumen by a LuminousFlux returns a value of type Area
impl<T> core::ops::Mul<&LuminousFlux<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm * rhs.lm.clone()}
	}
}
/// Multiplying a AreaPerLumen by a LuminousFlux returns a value of type Area
impl<T> core::ops::Mul<&LuminousFlux<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_lm.clone() * rhs.lm.clone()}
	}
}

// AreaPerLumen / Area -> InverseLuminousFlux
/// Dividing a AreaPerLumen by a Area returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<Area<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm / rhs.m2}
	}
}
/// Dividing a AreaPerLumen by a Area returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<Area<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm.clone() / rhs.m2}
	}
}
/// Dividing a AreaPerLumen by a Area returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&Area<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm / rhs.m2.clone()}
	}
}
/// Dividing a AreaPerLumen by a Area returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&Area<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm.clone() / rhs.m2.clone()}
	}
}

// AreaPerLumen * InverseArea -> InverseLuminousFlux
/// Multiplying a AreaPerLumen by a InverseArea returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<InverseArea<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm * rhs.per_m2}
	}
}
/// Multiplying a AreaPerLumen by a InverseArea returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<InverseArea<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm.clone() * rhs.per_m2}
	}
}
/// Multiplying a AreaPerLumen by a InverseArea returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&InverseArea<T>> for AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm * rhs.per_m2.clone()}
	}
}
/// Multiplying a AreaPerLumen by a InverseArea returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&InverseArea<T>> for &AreaPerLumen<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.m2_per_lm.clone() * rhs.per_m2.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for f64 where T: NumLike+From<f64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for f64 where T: NumLike+From<f64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for f32 where T: NumLike+From<f32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for f32 where T: NumLike+From<f32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for i64 where T: NumLike+From<i64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for i64 where T: NumLike+From<i64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for i32 where T: NumLike+From<i32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<AreaPerLumen<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for i32 where T: NumLike+From<i32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
impl<T> core::ops::Div<&AreaPerLumen<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<AreaPerLumen<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<AreaPerLumen<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&AreaPerLumen<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&AreaPerLumen<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<AreaPerLumen<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<AreaPerLumen<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&AreaPerLumen<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&AreaPerLumen<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

// 1/AreaPerLumen -> Illuminance
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<AreaPerLumen<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<AreaPerLumen<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&AreaPerLumen<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self) / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a scalar value by a AreaPerLumen unit value returns a value of type Illuminance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&AreaPerLumen<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Illuminance<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Illuminance{lux: T::from(self.clone()) / rhs.m2_per_lm.clone()}
	}
}

/// The electrical capacitance unit type, defined as farads in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Capacitance<T: NumLike>{
	/// The value of this Electrical capacitance in farads
	pub F: T
}

impl<T> Capacitance<T> where T: NumLike {

	/// Returns the standard unit name of electrical capacitance: "farads"
	pub fn unit_name() -> &'static str { "farads" }
	
	/// Returns the abbreviated name or symbol of electrical capacitance: "F" for farads
	pub fn unit_symbol() -> &'static str { "F" }
	
	/// Returns a new electrical capacitance value from the given number of farads
	///
	/// # Arguments
	/// * `F` - Any number-like type, representing a quantity of farads
	pub fn from_F(F: T) -> Self { Capacitance{F: F} }
	
	/// Returns a copy of this electrical capacitance value in farads
	pub fn to_F(&self) -> T { self.F.clone() }

	/// Returns a new electrical capacitance value from the given number of farads
	///
	/// # Arguments
	/// * `farads` - Any number-like type, representing a quantity of farads
	pub fn from_farads(farads: T) -> Self { Capacitance{F: farads} }
	
	/// Returns a copy of this electrical capacitance value in farads
	pub fn to_farads(&self) -> T { self.F.clone() }

}

impl<T> fmt::Display for Capacitance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.F, Self::unit_symbol())
	}
}

impl<T> Capacitance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical capacitance value in millifarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mF(&self) -> T {
		return self.F.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of millifarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mF` - Any number-like type, representing a quantity of millifarads
	pub fn from_mF(mF: T) -> Self {
		Capacitance{F: mF * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical capacitance value in microfarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uF(&self) -> T {
		return self.F.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of microfarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uF` - Any number-like type, representing a quantity of microfarads
	pub fn from_uF(uF: T) -> Self {
		Capacitance{F: uF * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical capacitance value in nanofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nF(&self) -> T {
		return self.F.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of nanofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nF` - Any number-like type, representing a quantity of nanofarads
	pub fn from_nF(nF: T) -> Self {
		Capacitance{F: nF * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical capacitance value in picofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_pF(&self) -> T {
		return self.F.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of picofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `pF` - Any number-like type, representing a quantity of picofarads
	pub fn from_pF(pF: T) -> Self {
		Capacitance{F: pF * T::from(1e-12_f64)}
	}

	/// Returns a copy of this electrical capacitance value in kilofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kF(&self) -> T {
		return self.F.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical capacitance value from the given number of kilofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kF` - Any number-like type, representing a quantity of kilofarads
	pub fn from_kF(kF: T) -> Self {
		Capacitance{F: kF * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical capacitance value in megafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MF(&self) -> T {
		return self.F.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical capacitance value from the given number of megafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MF` - Any number-like type, representing a quantity of megafarads
	pub fn from_MF(MF: T) -> Self {
		Capacitance{F: MF * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical capacitance value in gigafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GF(&self) -> T {
		return self.F.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical capacitance value from the given number of gigafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GF` - Any number-like type, representing a quantity of gigafarads
	pub fn from_GF(GF: T) -> Self {
		Capacitance{F: GF * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Capacitance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Capacitance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Capacitance<num_bigfloat::BigFloat>) -> Self::Output {
		Capacitance{F: self * rhs.F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Capacitance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Capacitance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Capacitance<num_bigfloat::BigFloat>) -> Self::Output {
		Capacitance{F: self.clone() * rhs.F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Capacitance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Capacitance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Capacitance<num_bigfloat::BigFloat>) -> Self::Output {
		Capacitance{F: self * rhs.F.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Capacitance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Capacitance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Capacitance<num_bigfloat::BigFloat>) -> Self::Output {
		Capacitance{F: self.clone() * rhs.F.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Capacitance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Capacitance<num_complex::Complex32>;
	fn mul(self, rhs: Capacitance<num_complex::Complex32>) -> Self::Output {
		Capacitance{F: self * rhs.F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Capacitance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Capacitance<num_complex::Complex32>;
	fn mul(self, rhs: Capacitance<num_complex::Complex32>) -> Self::Output {
		Capacitance{F: self.clone() * rhs.F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Capacitance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Capacitance<num_complex::Complex32>;
	fn mul(self, rhs: &Capacitance<num_complex::Complex32>) -> Self::Output {
		Capacitance{F: self * rhs.F.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Capacitance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Capacitance<num_complex::Complex32>;
	fn mul(self, rhs: &Capacitance<num_complex::Complex32>) -> Self::Output {
		Capacitance{F: self.clone() * rhs.F.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Capacitance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Capacitance<num_complex::Complex64>;
	fn mul(self, rhs: Capacitance<num_complex::Complex64>) -> Self::Output {
		Capacitance{F: self * rhs.F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Capacitance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Capacitance<num_complex::Complex64>;
	fn mul(self, rhs: Capacitance<num_complex::Complex64>) -> Self::Output {
		Capacitance{F: self.clone() * rhs.F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Capacitance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Capacitance<num_complex::Complex64>;
	fn mul(self, rhs: &Capacitance<num_complex::Complex64>) -> Self::Output {
		Capacitance{F: self * rhs.F.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Capacitance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Capacitance<num_complex::Complex64>;
	fn mul(self, rhs: &Capacitance<num_complex::Complex64>) -> Self::Output {
		Capacitance{F: self.clone() * rhs.F.clone()}
	}
}



/// Converts a Capacitance into the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Capacitance> for Capacitance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Capacitance {
		uom::si::f32::Capacitance::new::<uom::si::capacitance::farad>(self.F.into())
	}
}

/// Creates a Capacitance from the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Capacitance> for Capacitance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Capacitance) -> Self {
		Capacitance{F: T::from(src.value)}
	}
}

/// Converts a Capacitance into the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Capacitance> for Capacitance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Capacitance {
		uom::si::f64::Capacitance::new::<uom::si::capacitance::farad>(self.F.into())
	}
}

/// Creates a Capacitance from the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Capacitance> for Capacitance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Capacitance) -> Self {
		Capacitance{F: T::from(src.value)}
	}
}


// Capacitance / Time -> Conductance
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> core::ops::Div<Time<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Conductance{S: self.F / rhs.s}
	}
}
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> core::ops::Div<Time<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Conductance{S: self.F.clone() / rhs.s}
	}
}
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> core::ops::Div<&Time<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Conductance{S: self.F / rhs.s.clone()}
	}
}
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> core::ops::Div<&Time<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Conductance{S: self.F.clone() / rhs.s.clone()}
	}
}

// Capacitance / Charge -> InverseVoltage
/// Dividing a Capacitance by a Charge returns a value of type InverseVoltage
impl<T> core::ops::Div<Charge<T>> for Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F / rhs.C}
	}
}
/// Dividing a Capacitance by a Charge returns a value of type InverseVoltage
impl<T> core::ops::Div<Charge<T>> for &Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F.clone() / rhs.C}
	}
}
/// Dividing a Capacitance by a Charge returns a value of type InverseVoltage
impl<T> core::ops::Div<&Charge<T>> for Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F / rhs.C.clone()}
	}
}
/// Dividing a Capacitance by a Charge returns a value of type InverseVoltage
impl<T> core::ops::Div<&Charge<T>> for &Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F.clone() / rhs.C.clone()}
	}
}

// Capacitance / Conductance -> Time
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> core::ops::Div<Conductance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.F / rhs.S}
	}
}
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> core::ops::Div<Conductance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.F.clone() / rhs.S}
	}
}
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> core::ops::Div<&Conductance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.F / rhs.S.clone()}
	}
}
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> core::ops::Div<&Conductance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.F.clone() / rhs.S.clone()}
	}
}

// Capacitance * InverseCharge -> InverseVoltage
/// Multiplying a Capacitance by a InverseCharge returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseCharge<T>> for Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F * rhs.per_C}
	}
}
/// Multiplying a Capacitance by a InverseCharge returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseCharge<T>> for &Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F.clone() * rhs.per_C}
	}
}
/// Multiplying a Capacitance by a InverseCharge returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseCharge<T>> for Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F * rhs.per_C.clone()}
	}
}
/// Multiplying a Capacitance by a InverseCharge returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseCharge<T>> for &Capacitance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseVoltage{per_V: self.F.clone() * rhs.per_C.clone()}
	}
}

// Capacitance / InverseVoltage -> Charge
/// Dividing a Capacitance by a InverseVoltage returns a value of type Charge
impl<T> core::ops::Div<InverseVoltage<T>> for Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Charge{C: self.F / rhs.per_V}
	}
}
/// Dividing a Capacitance by a InverseVoltage returns a value of type Charge
impl<T> core::ops::Div<InverseVoltage<T>> for &Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Charge{C: self.F.clone() / rhs.per_V}
	}
}
/// Dividing a Capacitance by a InverseVoltage returns a value of type Charge
impl<T> core::ops::Div<&InverseVoltage<T>> for Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Charge{C: self.F / rhs.per_V.clone()}
	}
}
/// Dividing a Capacitance by a InverseVoltage returns a value of type Charge
impl<T> core::ops::Div<&InverseVoltage<T>> for &Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Charge{C: self.F.clone() / rhs.per_V.clone()}
	}
}

// Capacitance * Resistance -> Time
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> core::ops::Mul<Resistance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.F * rhs.Ohm}
	}
}
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> core::ops::Mul<Resistance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.F.clone() * rhs.Ohm}
	}
}
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> core::ops::Mul<&Resistance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.F * rhs.Ohm.clone()}
	}
}
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> core::ops::Mul<&Resistance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.F.clone() * rhs.Ohm.clone()}
	}
}

// Capacitance * Voltage -> Charge
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> core::ops::Mul<Voltage<T>> for Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.F * rhs.V}
	}
}
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> core::ops::Mul<Voltage<T>> for &Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.F.clone() * rhs.V}
	}
}
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> core::ops::Mul<&Voltage<T>> for Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.F * rhs.V.clone()}
	}
}
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> core::ops::Mul<&Voltage<T>> for &Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.F.clone() * rhs.V.clone()}
	}
}

// Capacitance * Frequency -> Conductance
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> core::ops::Mul<Frequency<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Conductance{S: self.F * rhs.Hz}
	}
}
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> core::ops::Mul<Frequency<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Conductance{S: self.F.clone() * rhs.Hz}
	}
}
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> core::ops::Mul<&Frequency<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Conductance{S: self.F * rhs.Hz.clone()}
	}
}
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> core::ops::Mul<&Frequency<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Conductance{S: self.F.clone() * rhs.Hz.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<Capacitance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
impl<T> core::ops::Div<&Capacitance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Capacitance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Capacitance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Capacitance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Capacitance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Capacitance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Capacitance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Capacitance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Capacitance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

// 1/Capacitance -> Elastance
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Capacitance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Capacitance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Elastance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Capacitance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self) / rhs.F.clone()}
	}
}
/// Dividing a scalar value by a Capacitance unit value returns a value of type Elastance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Capacitance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Elastance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Elastance{per_F: T::from(self.clone()) / rhs.F.clone()}
	}
}

/// The electric charge (aka coulombs) unit type, defined as coulombs in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Charge<T: NumLike>{
	/// The value of this Electric charge in coulombs
	pub C: T
}

impl<T> Charge<T> where T: NumLike {

	/// Returns the standard unit name of electric charge: "coulombs"
	pub fn unit_name() -> &'static str { "coulombs" }
	
	/// Returns the abbreviated name or symbol of electric charge: "C" for coulombs
	pub fn unit_symbol() -> &'static str { "C" }
	
	/// Returns a new electric charge value from the given number of coulombs
	///
	/// # Arguments
	/// * `C` - Any number-like type, representing a quantity of coulombs
	pub fn from_C(C: T) -> Self { Charge{C: C} }
	
	/// Returns a copy of this electric charge value in coulombs
	pub fn to_C(&self) -> T { self.C.clone() }

	/// Returns a new electric charge value from the given number of coulombs
	///
	/// # Arguments
	/// * `coulombs` - Any number-like type, representing a quantity of coulombs
	pub fn from_coulombs(coulombs: T) -> Self { Charge{C: coulombs} }
	
	/// Returns a copy of this electric charge value in coulombs
	pub fn to_coulombs(&self) -> T { self.C.clone() }

}

impl<T> fmt::Display for Charge<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.C, Self::unit_symbol())
	}
}

impl<T> Charge<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electric charge value in millicoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mC(&self) -> T {
		return self.C.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electric charge value from the given number of millicoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mC` - Any number-like type, representing a quantity of millicoulombs
	pub fn from_mC(mC: T) -> Self {
		Charge{C: mC * T::from(0.001_f64)}
	}

	/// Returns a copy of this electric charge value in microcoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uC(&self) -> T {
		return self.C.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electric charge value from the given number of microcoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uC` - Any number-like type, representing a quantity of microcoulombs
	pub fn from_uC(uC: T) -> Self {
		Charge{C: uC * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electric charge value in nanocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nC(&self) -> T {
		return self.C.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electric charge value from the given number of nanocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nC` - Any number-like type, representing a quantity of nanocoulombs
	pub fn from_nC(nC: T) -> Self {
		Charge{C: nC * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electric charge value in kilocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kC(&self) -> T {
		return self.C.clone() * T::from(0.001_f64);
	}

	/// Returns a new electric charge value from the given number of kilocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kC` - Any number-like type, representing a quantity of kilocoulombs
	pub fn from_kC(kC: T) -> Self {
		Charge{C: kC * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electric charge value in megacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MC(&self) -> T {
		return self.C.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electric charge value from the given number of megacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MC` - Any number-like type, representing a quantity of megacoulombs
	pub fn from_MC(MC: T) -> Self {
		Charge{C: MC * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electric charge value in gigacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GC(&self) -> T {
		return self.C.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electric charge value from the given number of gigacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GC` - Any number-like type, representing a quantity of gigacoulombs
	pub fn from_GC(GC: T) -> Self {
		Charge{C: GC * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this electric charge value in proton
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_p(&self) -> T {
		return self.C.clone() * T::from(6.24150907446076e+18_f64);
	}

	/// Returns a new electric charge value from the given number of proton
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `p` - Any number-like type, representing a quantity of proton
	pub fn from_p(p: T) -> Self {
		Charge{C: p * T::from(1.6021766340000001e-19_f64)}
	}

	/// Returns a copy of this electric charge value in electron
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_e(&self) -> T {
		return self.C.clone() * T::from(-6.24150907446076e+18_f64);
	}

	/// Returns a new electric charge value from the given number of electron
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `e` - Any number-like type, representing a quantity of electron
	pub fn from_e(e: T) -> Self {
		Charge{C: e * T::from(-1.6021766340000001e-19_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Charge<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Charge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Charge<num_bigfloat::BigFloat>) -> Self::Output {
		Charge{C: self * rhs.C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Charge<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Charge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Charge<num_bigfloat::BigFloat>) -> Self::Output {
		Charge{C: self.clone() * rhs.C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Charge<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Charge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Charge<num_bigfloat::BigFloat>) -> Self::Output {
		Charge{C: self * rhs.C.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Charge<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Charge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Charge<num_bigfloat::BigFloat>) -> Self::Output {
		Charge{C: self.clone() * rhs.C.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Charge<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Charge<num_complex::Complex32>;
	fn mul(self, rhs: Charge<num_complex::Complex32>) -> Self::Output {
		Charge{C: self * rhs.C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Charge<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Charge<num_complex::Complex32>;
	fn mul(self, rhs: Charge<num_complex::Complex32>) -> Self::Output {
		Charge{C: self.clone() * rhs.C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Charge<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Charge<num_complex::Complex32>;
	fn mul(self, rhs: &Charge<num_complex::Complex32>) -> Self::Output {
		Charge{C: self * rhs.C.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Charge<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Charge<num_complex::Complex32>;
	fn mul(self, rhs: &Charge<num_complex::Complex32>) -> Self::Output {
		Charge{C: self.clone() * rhs.C.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Charge<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Charge<num_complex::Complex64>;
	fn mul(self, rhs: Charge<num_complex::Complex64>) -> Self::Output {
		Charge{C: self * rhs.C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Charge<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Charge<num_complex::Complex64>;
	fn mul(self, rhs: Charge<num_complex::Complex64>) -> Self::Output {
		Charge{C: self.clone() * rhs.C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Charge<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Charge<num_complex::Complex64>;
	fn mul(self, rhs: &Charge<num_complex::Complex64>) -> Self::Output {
		Charge{C: self * rhs.C.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Charge<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Charge<num_complex::Complex64>;
	fn mul(self, rhs: &Charge<num_complex::Complex64>) -> Self::Output {
		Charge{C: self.clone() * rhs.C.clone()}
	}
}



/// Converts a Charge into the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricCharge> for Charge<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricCharge {
		uom::si::f32::ElectricCharge::new::<uom::si::electric_charge::coulomb>(self.C.into())
	}
}

/// Creates a Charge from the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricCharge> for Charge<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricCharge) -> Self {
		Charge{C: T::from(src.value)}
	}
}

/// Converts a Charge into the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricCharge> for Charge<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricCharge {
		uom::si::f64::ElectricCharge::new::<uom::si::electric_charge::coulomb>(self.C.into())
	}
}

/// Creates a Charge from the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricCharge> for Charge<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricCharge) -> Self {
		Charge{C: T::from(src.value)}
	}
}


// Charge / Current -> Time
/// Dividing a Charge by a Current returns a value of type Time
impl<T> core::ops::Div<Current<T>> for Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Time{s: self.C / rhs.A}
	}
}
/// Dividing a Charge by a Current returns a value of type Time
impl<T> core::ops::Div<Current<T>> for &Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Time{s: self.C.clone() / rhs.A}
	}
}
/// Dividing a Charge by a Current returns a value of type Time
impl<T> core::ops::Div<&Current<T>> for Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Time{s: self.C / rhs.A.clone()}
	}
}
/// Dividing a Charge by a Current returns a value of type Time
impl<T> core::ops::Div<&Current<T>> for &Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Time{s: self.C.clone() / rhs.A.clone()}
	}
}

// Charge * InverseCurrent -> Time
/// Multiplying a Charge by a InverseCurrent returns a value of type Time
impl<T> core::ops::Mul<InverseCurrent<T>> for Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		Time{s: self.C * rhs.per_A}
	}
}
/// Multiplying a Charge by a InverseCurrent returns a value of type Time
impl<T> core::ops::Mul<InverseCurrent<T>> for &Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		Time{s: self.C.clone() * rhs.per_A}
	}
}
/// Multiplying a Charge by a InverseCurrent returns a value of type Time
impl<T> core::ops::Mul<&InverseCurrent<T>> for Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Time{s: self.C * rhs.per_A.clone()}
	}
}
/// Multiplying a Charge by a InverseCurrent returns a value of type Time
impl<T> core::ops::Mul<&InverseCurrent<T>> for &Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Time{s: self.C.clone() * rhs.per_A.clone()}
	}
}

// Charge / Time -> Current
/// Dividing a Charge by a Time returns a value of type Current
impl<T> core::ops::Div<Time<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Current{A: self.C / rhs.s}
	}
}
/// Dividing a Charge by a Time returns a value of type Current
impl<T> core::ops::Div<Time<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Current{A: self.C.clone() / rhs.s}
	}
}
/// Dividing a Charge by a Time returns a value of type Current
impl<T> core::ops::Div<&Time<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Current{A: self.C / rhs.s.clone()}
	}
}
/// Dividing a Charge by a Time returns a value of type Current
impl<T> core::ops::Div<&Time<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Current{A: self.C.clone() / rhs.s.clone()}
	}
}

// Charge / Capacitance -> Voltage
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> core::ops::Div<Capacitance<T>> for Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Voltage{V: self.C / rhs.F}
	}
}
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> core::ops::Div<Capacitance<T>> for &Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Voltage{V: self.C.clone() / rhs.F}
	}
}
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> core::ops::Div<&Capacitance<T>> for Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Voltage{V: self.C / rhs.F.clone()}
	}
}
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> core::ops::Div<&Capacitance<T>> for &Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Voltage{V: self.C.clone() / rhs.F.clone()}
	}
}

// Charge / Conductance -> MagneticFlux
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> core::ops::Div<Conductance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C / rhs.S}
	}
}
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> core::ops::Div<Conductance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() / rhs.S}
	}
}
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> core::ops::Div<&Conductance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C / rhs.S.clone()}
	}
}
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> core::ops::Div<&Conductance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() / rhs.S.clone()}
	}
}

// Charge * Elastance -> Voltage
/// Multiplying a Charge by a Elastance returns a value of type Voltage
impl<T> core::ops::Mul<Elastance<T>> for Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		Voltage{V: self.C * rhs.per_F}
	}
}
/// Multiplying a Charge by a Elastance returns a value of type Voltage
impl<T> core::ops::Mul<Elastance<T>> for &Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		Voltage{V: self.C.clone() * rhs.per_F}
	}
}
/// Multiplying a Charge by a Elastance returns a value of type Voltage
impl<T> core::ops::Mul<&Elastance<T>> for Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		Voltage{V: self.C * rhs.per_F.clone()}
	}
}
/// Multiplying a Charge by a Elastance returns a value of type Voltage
impl<T> core::ops::Mul<&Elastance<T>> for &Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		Voltage{V: self.C.clone() * rhs.per_F.clone()}
	}
}

// Charge * InverseMagneticFlux -> Conductance
/// Multiplying a Charge by a InverseMagneticFlux returns a value of type Conductance
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C * rhs.per_Wb}
	}
}
/// Multiplying a Charge by a InverseMagneticFlux returns a value of type Conductance
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Charge by a InverseMagneticFlux returns a value of type Conductance
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Charge by a InverseMagneticFlux returns a value of type Conductance
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C.clone() * rhs.per_Wb.clone()}
	}
}

// Charge * InverseVoltage -> Capacitance
/// Multiplying a Charge by a InverseVoltage returns a value of type Capacitance
impl<T> core::ops::Mul<InverseVoltage<T>> for Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		Capacitance{F: self.C * rhs.per_V}
	}
}
/// Multiplying a Charge by a InverseVoltage returns a value of type Capacitance
impl<T> core::ops::Mul<InverseVoltage<T>> for &Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		Capacitance{F: self.C.clone() * rhs.per_V}
	}
}
/// Multiplying a Charge by a InverseVoltage returns a value of type Capacitance
impl<T> core::ops::Mul<&InverseVoltage<T>> for Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Capacitance{F: self.C * rhs.per_V.clone()}
	}
}
/// Multiplying a Charge by a InverseVoltage returns a value of type Capacitance
impl<T> core::ops::Mul<&InverseVoltage<T>> for &Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Capacitance{F: self.C.clone() * rhs.per_V.clone()}
	}
}

// Charge / InverseVoltage -> Energy
/// Dividing a Charge by a InverseVoltage returns a value of type Energy
impl<T> core::ops::Div<InverseVoltage<T>> for Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Energy{J: self.C / rhs.per_V}
	}
}
/// Dividing a Charge by a InverseVoltage returns a value of type Energy
impl<T> core::ops::Div<InverseVoltage<T>> for &Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Energy{J: self.C.clone() / rhs.per_V}
	}
}
/// Dividing a Charge by a InverseVoltage returns a value of type Energy
impl<T> core::ops::Div<&InverseVoltage<T>> for Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Energy{J: self.C / rhs.per_V.clone()}
	}
}
/// Dividing a Charge by a InverseVoltage returns a value of type Energy
impl<T> core::ops::Div<&InverseVoltage<T>> for &Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Energy{J: self.C.clone() / rhs.per_V.clone()}
	}
}

// Charge / MagneticFlux -> Conductance
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> core::ops::Div<MagneticFlux<T>> for Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C / rhs.Wb}
	}
}
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> core::ops::Div<MagneticFlux<T>> for &Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C.clone() / rhs.Wb}
	}
}
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> core::ops::Div<&MagneticFlux<T>> for Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C / rhs.Wb.clone()}
	}
}
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> core::ops::Div<&MagneticFlux<T>> for &Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C.clone() / rhs.Wb.clone()}
	}
}

// Charge * Resistance -> MagneticFlux
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> core::ops::Mul<Resistance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C * rhs.Ohm}
	}
}
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> core::ops::Mul<Resistance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() * rhs.Ohm}
	}
}
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Resistance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C * rhs.Ohm.clone()}
	}
}
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Resistance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() * rhs.Ohm.clone()}
	}
}

// Charge * Voltage -> Energy
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> core::ops::Mul<Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Energy{J: self.C * rhs.V}
	}
}
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> core::ops::Mul<Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Energy{J: self.C.clone() * rhs.V}
	}
}
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> core::ops::Mul<&Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Energy{J: self.C * rhs.V.clone()}
	}
}
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> core::ops::Mul<&Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Energy{J: self.C.clone() * rhs.V.clone()}
	}
}

// Charge / Voltage -> Capacitance
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> core::ops::Div<Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Capacitance{F: self.C / rhs.V}
	}
}
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> core::ops::Div<Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Capacitance{F: self.C.clone() / rhs.V}
	}
}
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> core::ops::Div<&Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Capacitance{F: self.C / rhs.V.clone()}
	}
}
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> core::ops::Div<&Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Capacitance{F: self.C.clone() / rhs.V.clone()}
	}
}

// Charge / Energy -> InverseVoltage
/// Dividing a Charge by a Energy returns a value of type InverseVoltage
impl<T> core::ops::Div<Energy<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C / rhs.J}
	}
}
/// Dividing a Charge by a Energy returns a value of type InverseVoltage
impl<T> core::ops::Div<Energy<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() / rhs.J}
	}
}
/// Dividing a Charge by a Energy returns a value of type InverseVoltage
impl<T> core::ops::Div<&Energy<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C / rhs.J.clone()}
	}
}
/// Dividing a Charge by a Energy returns a value of type InverseVoltage
impl<T> core::ops::Div<&Energy<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() / rhs.J.clone()}
	}
}

// Charge / Torque -> InverseVoltage
/// Dividing a Charge by a Torque returns a value of type InverseVoltage
impl<T> core::ops::Div<Torque<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C / rhs.Nm}
	}
}
/// Dividing a Charge by a Torque returns a value of type InverseVoltage
impl<T> core::ops::Div<Torque<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() / rhs.Nm}
	}
}
/// Dividing a Charge by a Torque returns a value of type InverseVoltage
impl<T> core::ops::Div<&Torque<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C / rhs.Nm.clone()}
	}
}
/// Dividing a Charge by a Torque returns a value of type InverseVoltage
impl<T> core::ops::Div<&Torque<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() / rhs.Nm.clone()}
	}
}

// Charge * Frequency -> Current
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> core::ops::Mul<Frequency<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Current{A: self.C * rhs.Hz}
	}
}
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> core::ops::Mul<Frequency<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Current{A: self.C.clone() * rhs.Hz}
	}
}
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> core::ops::Mul<&Frequency<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Current{A: self.C * rhs.Hz.clone()}
	}
}
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> core::ops::Mul<&Frequency<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Current{A: self.C.clone() * rhs.Hz.clone()}
	}
}

// Charge * InverseEnergy -> InverseVoltage
/// Multiplying a Charge by a InverseEnergy returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseEnergy<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C * rhs.per_J}
	}
}
/// Multiplying a Charge by a InverseEnergy returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseEnergy<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() * rhs.per_J}
	}
}
/// Multiplying a Charge by a InverseEnergy returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseEnergy<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C * rhs.per_J.clone()}
	}
}
/// Multiplying a Charge by a InverseEnergy returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseEnergy<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() * rhs.per_J.clone()}
	}
}

// Charge * InverseTorque -> InverseVoltage
/// Multiplying a Charge by a InverseTorque returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseTorque<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C * rhs.per_Nm}
	}
}
/// Multiplying a Charge by a InverseTorque returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseTorque<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() * rhs.per_Nm}
	}
}
/// Multiplying a Charge by a InverseTorque returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseTorque<T>> for Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C * rhs.per_Nm.clone()}
	}
}
/// Multiplying a Charge by a InverseTorque returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseTorque<T>> for &Charge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseVoltage{per_V: self.C.clone() * rhs.per_Nm.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<Charge<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
impl<T> core::ops::Div<&Charge<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Charge<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Charge<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Charge<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Charge<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Charge<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Charge<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Charge<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Charge<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

// 1/Charge -> InverseCharge
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Charge<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Charge<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Charge<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self) / rhs.C.clone()}
	}
}
/// Dividing a scalar value by a Charge unit value returns a value of type InverseCharge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Charge<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseCharge{per_C: T::from(self.clone()) / rhs.C.clone()}
	}
}

/// The electrical conductance unit type, defined as siemens in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Conductance<T: NumLike>{
	/// The value of this Electrical conductance in siemens
	pub S: T
}

impl<T> Conductance<T> where T: NumLike {

	/// Returns the standard unit name of electrical conductance: "siemens"
	pub fn unit_name() -> &'static str { "siemens" }
	
	/// Returns the abbreviated name or symbol of electrical conductance: "S" for siemens
	pub fn unit_symbol() -> &'static str { "S" }
	
	/// Returns a new electrical conductance value from the given number of siemens
	///
	/// # Arguments
	/// * `S` - Any number-like type, representing a quantity of siemens
	pub fn from_S(S: T) -> Self { Conductance{S: S} }
	
	/// Returns a copy of this electrical conductance value in siemens
	pub fn to_S(&self) -> T { self.S.clone() }

	/// Returns a new electrical conductance value from the given number of siemens
	///
	/// # Arguments
	/// * `siemens` - Any number-like type, representing a quantity of siemens
	pub fn from_siemens(siemens: T) -> Self { Conductance{S: siemens} }
	
	/// Returns a copy of this electrical conductance value in siemens
	pub fn to_siemens(&self) -> T { self.S.clone() }

}

impl<T> fmt::Display for Conductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.S, Self::unit_symbol())
	}
}

impl<T> Conductance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical conductance value in millisiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mS(&self) -> T {
		return self.S.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical conductance value from the given number of millisiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mS` - Any number-like type, representing a quantity of millisiemens
	pub fn from_mS(mS: T) -> Self {
		Conductance{S: mS * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical conductance value in microsiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uS(&self) -> T {
		return self.S.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical conductance value from the given number of microsiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uS` - Any number-like type, representing a quantity of microsiemens
	pub fn from_uS(uS: T) -> Self {
		Conductance{S: uS * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical conductance value in nanosiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nS(&self) -> T {
		return self.S.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical conductance value from the given number of nanosiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nS` - Any number-like type, representing a quantity of nanosiemens
	pub fn from_nS(nS: T) -> Self {
		Conductance{S: nS * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical conductance value in kilosiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kS(&self) -> T {
		return self.S.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical conductance value from the given number of kilosiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kS` - Any number-like type, representing a quantity of kilosiemens
	pub fn from_kS(kS: T) -> Self {
		Conductance{S: kS * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical conductance value in megasiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MS(&self) -> T {
		return self.S.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical conductance value from the given number of megasiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MS` - Any number-like type, representing a quantity of megasiemens
	pub fn from_MS(MS: T) -> Self {
		Conductance{S: MS * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical conductance value in gigasiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GS(&self) -> T {
		return self.S.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical conductance value from the given number of gigasiemens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GS` - Any number-like type, representing a quantity of gigasiemens
	pub fn from_GS(GS: T) -> Self {
		Conductance{S: GS * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Conductance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Conductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Conductance<num_bigfloat::BigFloat>) -> Self::Output {
		Conductance{S: self * rhs.S}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Conductance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Conductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Conductance<num_bigfloat::BigFloat>) -> Self::Output {
		Conductance{S: self.clone() * rhs.S}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Conductance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Conductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Conductance<num_bigfloat::BigFloat>) -> Self::Output {
		Conductance{S: self * rhs.S.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Conductance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Conductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Conductance<num_bigfloat::BigFloat>) -> Self::Output {
		Conductance{S: self.clone() * rhs.S.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Conductance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Conductance<num_complex::Complex32>;
	fn mul(self, rhs: Conductance<num_complex::Complex32>) -> Self::Output {
		Conductance{S: self * rhs.S}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Conductance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Conductance<num_complex::Complex32>;
	fn mul(self, rhs: Conductance<num_complex::Complex32>) -> Self::Output {
		Conductance{S: self.clone() * rhs.S}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Conductance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Conductance<num_complex::Complex32>;
	fn mul(self, rhs: &Conductance<num_complex::Complex32>) -> Self::Output {
		Conductance{S: self * rhs.S.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Conductance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Conductance<num_complex::Complex32>;
	fn mul(self, rhs: &Conductance<num_complex::Complex32>) -> Self::Output {
		Conductance{S: self.clone() * rhs.S.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Conductance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Conductance<num_complex::Complex64>;
	fn mul(self, rhs: Conductance<num_complex::Complex64>) -> Self::Output {
		Conductance{S: self * rhs.S}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Conductance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Conductance<num_complex::Complex64>;
	fn mul(self, rhs: Conductance<num_complex::Complex64>) -> Self::Output {
		Conductance{S: self.clone() * rhs.S}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Conductance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Conductance<num_complex::Complex64>;
	fn mul(self, rhs: &Conductance<num_complex::Complex64>) -> Self::Output {
		Conductance{S: self * rhs.S.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Conductance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Conductance<num_complex::Complex64>;
	fn mul(self, rhs: &Conductance<num_complex::Complex64>) -> Self::Output {
		Conductance{S: self.clone() * rhs.S.clone()}
	}
}



/// Converts a Conductance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricalConductance> for Conductance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricalConductance {
		uom::si::f32::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(self.S.into())
	}
}

/// Creates a Conductance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricalConductance> for Conductance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricalConductance) -> Self {
		Conductance{S: T::from(src.value)}
	}
}

/// Converts a Conductance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricalConductance> for Conductance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricalConductance {
		uom::si::f64::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(self.S.into())
	}
}

/// Creates a Conductance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricalConductance> for Conductance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricalConductance) -> Self {
		Conductance{S: T::from(src.value)}
	}
}


// Conductance / Current -> InverseVoltage
/// Dividing a Conductance by a Current returns a value of type InverseVoltage
impl<T> core::ops::Div<Current<T>> for Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseVoltage{per_V: self.S / rhs.A}
	}
}
/// Dividing a Conductance by a Current returns a value of type InverseVoltage
impl<T> core::ops::Div<Current<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseVoltage{per_V: self.S.clone() / rhs.A}
	}
}
/// Dividing a Conductance by a Current returns a value of type InverseVoltage
impl<T> core::ops::Div<&Current<T>> for Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseVoltage{per_V: self.S / rhs.A.clone()}
	}
}
/// Dividing a Conductance by a Current returns a value of type InverseVoltage
impl<T> core::ops::Div<&Current<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseVoltage{per_V: self.S.clone() / rhs.A.clone()}
	}
}

// Conductance * InverseCurrent -> InverseVoltage
/// Multiplying a Conductance by a InverseCurrent returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseCurrent<T>> for Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseVoltage{per_V: self.S * rhs.per_A}
	}
}
/// Multiplying a Conductance by a InverseCurrent returns a value of type InverseVoltage
impl<T> core::ops::Mul<InverseCurrent<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseVoltage{per_V: self.S.clone() * rhs.per_A}
	}
}
/// Multiplying a Conductance by a InverseCurrent returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseCurrent<T>> for Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseVoltage{per_V: self.S * rhs.per_A.clone()}
	}
}
/// Multiplying a Conductance by a InverseCurrent returns a value of type InverseVoltage
impl<T> core::ops::Mul<&InverseCurrent<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseVoltage{per_V: self.S.clone() * rhs.per_A.clone()}
	}
}

// Conductance * Time -> Capacitance
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> core::ops::Mul<Time<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Capacitance{F: self.S * rhs.s}
	}
}
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> core::ops::Mul<Time<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Capacitance{F: self.S.clone() * rhs.s}
	}
}
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> core::ops::Mul<&Time<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Capacitance{F: self.S * rhs.s.clone()}
	}
}
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> core::ops::Mul<&Time<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Capacitance{F: self.S.clone() * rhs.s.clone()}
	}
}

// Conductance / Time -> InverseInductance
/// Dividing a Conductance by a Time returns a value of type InverseInductance
impl<T> core::ops::Div<Time<T>> for Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		InverseInductance{per_H: self.S / rhs.s}
	}
}
/// Dividing a Conductance by a Time returns a value of type InverseInductance
impl<T> core::ops::Div<Time<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		InverseInductance{per_H: self.S.clone() / rhs.s}
	}
}
/// Dividing a Conductance by a Time returns a value of type InverseInductance
impl<T> core::ops::Div<&Time<T>> for Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		InverseInductance{per_H: self.S / rhs.s.clone()}
	}
}
/// Dividing a Conductance by a Time returns a value of type InverseInductance
impl<T> core::ops::Div<&Time<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		InverseInductance{per_H: self.S.clone() / rhs.s.clone()}
	}
}

// Conductance / Capacitance -> Frequency
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> core::ops::Div<Capacitance<T>> for Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S / rhs.F}
	}
}
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> core::ops::Div<Capacitance<T>> for &Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S.clone() / rhs.F}
	}
}
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> core::ops::Div<&Capacitance<T>> for Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S / rhs.F.clone()}
	}
}
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> core::ops::Div<&Capacitance<T>> for &Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S.clone() / rhs.F.clone()}
	}
}

// Conductance / Charge -> InverseMagneticFlux
/// Dividing a Conductance by a Charge returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Charge<T>> for Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S / rhs.C}
	}
}
/// Dividing a Conductance by a Charge returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Charge<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S.clone() / rhs.C}
	}
}
/// Dividing a Conductance by a Charge returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Charge<T>> for Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S / rhs.C.clone()}
	}
}
/// Dividing a Conductance by a Charge returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Charge<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S.clone() / rhs.C.clone()}
	}
}

// Conductance * Elastance -> Frequency
/// Multiplying a Conductance by a Elastance returns a value of type Frequency
impl<T> core::ops::Mul<Elastance<T>> for Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		Frequency{Hz: self.S * rhs.per_F}
	}
}
/// Multiplying a Conductance by a Elastance returns a value of type Frequency
impl<T> core::ops::Mul<Elastance<T>> for &Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		Frequency{Hz: self.S.clone() * rhs.per_F}
	}
}
/// Multiplying a Conductance by a Elastance returns a value of type Frequency
impl<T> core::ops::Mul<&Elastance<T>> for Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		Frequency{Hz: self.S * rhs.per_F.clone()}
	}
}
/// Multiplying a Conductance by a Elastance returns a value of type Frequency
impl<T> core::ops::Mul<&Elastance<T>> for &Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		Frequency{Hz: self.S.clone() * rhs.per_F.clone()}
	}
}

// Conductance * Inductance -> Time
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> core::ops::Mul<Inductance<T>> for Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		Time{s: self.S * rhs.H}
	}
}
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> core::ops::Mul<Inductance<T>> for &Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		Time{s: self.S.clone() * rhs.H}
	}
}
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> core::ops::Mul<&Inductance<T>> for Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		Time{s: self.S * rhs.H.clone()}
	}
}
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> core::ops::Mul<&Inductance<T>> for &Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		Time{s: self.S.clone() * rhs.H.clone()}
	}
}

// Conductance * InverseCharge -> InverseMagneticFlux
/// Multiplying a Conductance by a InverseCharge returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseCharge<T>> for Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S * rhs.per_C}
	}
}
/// Multiplying a Conductance by a InverseCharge returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseCharge<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S.clone() * rhs.per_C}
	}
}
/// Multiplying a Conductance by a InverseCharge returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseCharge<T>> for Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S * rhs.per_C.clone()}
	}
}
/// Multiplying a Conductance by a InverseCharge returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseCharge<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.S.clone() * rhs.per_C.clone()}
	}
}

// Conductance / InverseInductance -> Time
/// Dividing a Conductance by a InverseInductance returns a value of type Time
impl<T> core::ops::Div<InverseInductance<T>> for Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Time{s: self.S / rhs.per_H}
	}
}
/// Dividing a Conductance by a InverseInductance returns a value of type Time
impl<T> core::ops::Div<InverseInductance<T>> for &Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Time{s: self.S.clone() / rhs.per_H}
	}
}
/// Dividing a Conductance by a InverseInductance returns a value of type Time
impl<T> core::ops::Div<&InverseInductance<T>> for Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Time{s: self.S / rhs.per_H.clone()}
	}
}
/// Dividing a Conductance by a InverseInductance returns a value of type Time
impl<T> core::ops::Div<&InverseInductance<T>> for &Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Time{s: self.S.clone() / rhs.per_H.clone()}
	}
}

// Conductance / InverseMagneticFlux -> Charge
/// Dividing a Conductance by a InverseMagneticFlux returns a value of type Charge
impl<T> core::ops::Div<InverseMagneticFlux<T>> for Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Charge{C: self.S / rhs.per_Wb}
	}
}
/// Dividing a Conductance by a InverseMagneticFlux returns a value of type Charge
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Charge{C: self.S.clone() / rhs.per_Wb}
	}
}
/// Dividing a Conductance by a InverseMagneticFlux returns a value of type Charge
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Charge{C: self.S / rhs.per_Wb.clone()}
	}
}
/// Dividing a Conductance by a InverseMagneticFlux returns a value of type Charge
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Charge{C: self.S.clone() / rhs.per_Wb.clone()}
	}
}

// Conductance / InverseVoltage -> Current
/// Dividing a Conductance by a InverseVoltage returns a value of type Current
impl<T> core::ops::Div<InverseVoltage<T>> for Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Current{A: self.S / rhs.per_V}
	}
}
/// Dividing a Conductance by a InverseVoltage returns a value of type Current
impl<T> core::ops::Div<InverseVoltage<T>> for &Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Current{A: self.S.clone() / rhs.per_V}
	}
}
/// Dividing a Conductance by a InverseVoltage returns a value of type Current
impl<T> core::ops::Div<&InverseVoltage<T>> for Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Current{A: self.S / rhs.per_V.clone()}
	}
}
/// Dividing a Conductance by a InverseVoltage returns a value of type Current
impl<T> core::ops::Div<&InverseVoltage<T>> for &Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Current{A: self.S.clone() / rhs.per_V.clone()}
	}
}

// Conductance * MagneticFlux -> Charge
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> core::ops::Mul<MagneticFlux<T>> for Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S * rhs.Wb}
	}
}
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> core::ops::Mul<MagneticFlux<T>> for &Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S.clone() * rhs.Wb}
	}
}
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> core::ops::Mul<&MagneticFlux<T>> for Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S * rhs.Wb.clone()}
	}
}
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> core::ops::Mul<&MagneticFlux<T>> for &Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S.clone() * rhs.Wb.clone()}
	}
}

// Conductance * Voltage -> Current
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> core::ops::Mul<Voltage<T>> for Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Current{A: self.S * rhs.V}
	}
}
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> core::ops::Mul<Voltage<T>> for &Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Current{A: self.S.clone() * rhs.V}
	}
}
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> core::ops::Mul<&Voltage<T>> for Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Current{A: self.S * rhs.V.clone()}
	}
}
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> core::ops::Mul<&Voltage<T>> for &Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Current{A: self.S.clone() * rhs.V.clone()}
	}
}

// Conductance * Frequency -> InverseInductance
/// Multiplying a Conductance by a Frequency returns a value of type InverseInductance
impl<T> core::ops::Mul<Frequency<T>> for Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		InverseInductance{per_H: self.S * rhs.Hz}
	}
}
/// Multiplying a Conductance by a Frequency returns a value of type InverseInductance
impl<T> core::ops::Mul<Frequency<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		InverseInductance{per_H: self.S.clone() * rhs.Hz}
	}
}
/// Multiplying a Conductance by a Frequency returns a value of type InverseInductance
impl<T> core::ops::Mul<&Frequency<T>> for Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		InverseInductance{per_H: self.S * rhs.Hz.clone()}
	}
}
/// Multiplying a Conductance by a Frequency returns a value of type InverseInductance
impl<T> core::ops::Mul<&Frequency<T>> for &Conductance<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		InverseInductance{per_H: self.S.clone() * rhs.Hz.clone()}
	}
}

// Conductance / Frequency -> Capacitance
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> core::ops::Div<Frequency<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Capacitance{F: self.S / rhs.Hz}
	}
}
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> core::ops::Div<Frequency<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Capacitance{F: self.S.clone() / rhs.Hz}
	}
}
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> core::ops::Div<&Frequency<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Capacitance{F: self.S / rhs.Hz.clone()}
	}
}
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> core::ops::Div<&Frequency<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Capacitance{F: self.S.clone() / rhs.Hz.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<Conductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> core::ops::Div<&Conductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Conductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Conductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Conductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Conductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Conductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Conductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Conductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Conductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Conductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Conductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Conductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Conductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

/// The electrical elastance unit type, defined as inverse farads in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Elastance<T: NumLike>{
	/// The value of this Electrical elastance in inverse farads
	pub per_F: T
}

impl<T> Elastance<T> where T: NumLike {

	/// Returns the standard unit name of electrical elastance: "inverse farads"
	pub fn unit_name() -> &'static str { "inverse farads" }
	
	/// Returns the abbreviated name or symbol of electrical elastance: "1/F" for inverse farads
	pub fn unit_symbol() -> &'static str { "1/F" }
	
	/// Returns a new electrical elastance value from the given number of inverse farads
	///
	/// # Arguments
	/// * `per_F` - Any number-like type, representing a quantity of inverse farads
	pub fn from_per_F(per_F: T) -> Self { Elastance{per_F: per_F} }
	
	/// Returns a copy of this electrical elastance value in inverse farads
	pub fn to_per_F(&self) -> T { self.per_F.clone() }

	/// Returns a new electrical elastance value from the given number of inverse farads
	///
	/// # Arguments
	/// * `per_farads` - Any number-like type, representing a quantity of inverse farads
	pub fn from_per_farads(per_farads: T) -> Self { Elastance{per_F: per_farads} }
	
	/// Returns a copy of this electrical elastance value in inverse farads
	pub fn to_per_farads(&self) -> T { self.per_F.clone() }

}

impl<T> fmt::Display for Elastance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_F, Self::unit_symbol())
	}
}

impl<T> Elastance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical elastance value in inverse millifarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mF(&self) -> T {
		return self.per_F.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse millifarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mF` - Any number-like type, representing a quantity of inverse millifarads
	pub fn from_per_mF(per_mF: T) -> Self {
		Elastance{per_F: per_mF * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical elastance value in inverse microfarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uF(&self) -> T {
		return self.per_F.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse microfarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uF` - Any number-like type, representing a quantity of inverse microfarads
	pub fn from_per_uF(per_uF: T) -> Self {
		Elastance{per_F: per_uF * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical elastance value in inverse nanofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nF(&self) -> T {
		return self.per_F.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse nanofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nF` - Any number-like type, representing a quantity of inverse nanofarads
	pub fn from_per_nF(per_nF: T) -> Self {
		Elastance{per_F: per_nF * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this electrical elastance value in inverse picofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_pF(&self) -> T {
		return self.per_F.clone() * T::from(1e-12_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse picofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_pF` - Any number-like type, representing a quantity of inverse picofarads
	pub fn from_per_pF(per_pF: T) -> Self {
		Elastance{per_F: per_pF * T::from(1000000000000.0_f64)}
	}

	/// Returns a copy of this electrical elastance value in inverse kilofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kF(&self) -> T {
		return self.per_F.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse kilofarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kF` - Any number-like type, representing a quantity of inverse kilofarads
	pub fn from_per_kF(per_kF: T) -> Self {
		Elastance{per_F: per_kF * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical elastance value in inverse megafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_MF(&self) -> T {
		return self.per_F.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse megafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_MF` - Any number-like type, representing a quantity of inverse megafarads
	pub fn from_per_MF(per_MF: T) -> Self {
		Elastance{per_F: per_MF * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical elastance value in inverse gigafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GF(&self) -> T {
		return self.per_F.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical elastance value from the given number of inverse gigafarads
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GF` - Any number-like type, representing a quantity of inverse gigafarads
	pub fn from_per_GF(per_GF: T) -> Self {
		Elastance{per_F: per_GF * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Elastance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Elastance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Elastance<num_bigfloat::BigFloat>) -> Self::Output {
		Elastance{per_F: self * rhs.per_F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Elastance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Elastance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Elastance<num_bigfloat::BigFloat>) -> Self::Output {
		Elastance{per_F: self.clone() * rhs.per_F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Elastance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Elastance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Elastance<num_bigfloat::BigFloat>) -> Self::Output {
		Elastance{per_F: self * rhs.per_F.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Elastance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Elastance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Elastance<num_bigfloat::BigFloat>) -> Self::Output {
		Elastance{per_F: self.clone() * rhs.per_F.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Elastance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Elastance<num_complex::Complex32>;
	fn mul(self, rhs: Elastance<num_complex::Complex32>) -> Self::Output {
		Elastance{per_F: self * rhs.per_F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Elastance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Elastance<num_complex::Complex32>;
	fn mul(self, rhs: Elastance<num_complex::Complex32>) -> Self::Output {
		Elastance{per_F: self.clone() * rhs.per_F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Elastance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Elastance<num_complex::Complex32>;
	fn mul(self, rhs: &Elastance<num_complex::Complex32>) -> Self::Output {
		Elastance{per_F: self * rhs.per_F.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Elastance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Elastance<num_complex::Complex32>;
	fn mul(self, rhs: &Elastance<num_complex::Complex32>) -> Self::Output {
		Elastance{per_F: self.clone() * rhs.per_F.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Elastance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Elastance<num_complex::Complex64>;
	fn mul(self, rhs: Elastance<num_complex::Complex64>) -> Self::Output {
		Elastance{per_F: self * rhs.per_F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Elastance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Elastance<num_complex::Complex64>;
	fn mul(self, rhs: Elastance<num_complex::Complex64>) -> Self::Output {
		Elastance{per_F: self.clone() * rhs.per_F}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Elastance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Elastance<num_complex::Complex64>;
	fn mul(self, rhs: &Elastance<num_complex::Complex64>) -> Self::Output {
		Elastance{per_F: self * rhs.per_F.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Elastance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Elastance<num_complex::Complex64>;
	fn mul(self, rhs: &Elastance<num_complex::Complex64>) -> Self::Output {
		Elastance{per_F: self.clone() * rhs.per_F.clone()}
	}
}




// Elastance * Time -> Resistance
/// Multiplying a Elastance by a Time returns a value of type Resistance
impl<T> core::ops::Mul<Time<T>> for Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Resistance{Ohm: self.per_F * rhs.s}
	}
}
/// Multiplying a Elastance by a Time returns a value of type Resistance
impl<T> core::ops::Mul<Time<T>> for &Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Resistance{Ohm: self.per_F.clone() * rhs.s}
	}
}
/// Multiplying a Elastance by a Time returns a value of type Resistance
impl<T> core::ops::Mul<&Time<T>> for Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Resistance{Ohm: self.per_F * rhs.s.clone()}
	}
}
/// Multiplying a Elastance by a Time returns a value of type Resistance
impl<T> core::ops::Mul<&Time<T>> for &Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Resistance{Ohm: self.per_F.clone() * rhs.s.clone()}
	}
}

// Elastance * Charge -> Voltage
/// Multiplying a Elastance by a Charge returns a value of type Voltage
impl<T> core::ops::Mul<Charge<T>> for Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Voltage{V: self.per_F * rhs.C}
	}
}
/// Multiplying a Elastance by a Charge returns a value of type Voltage
impl<T> core::ops::Mul<Charge<T>> for &Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Voltage{V: self.per_F.clone() * rhs.C}
	}
}
/// Multiplying a Elastance by a Charge returns a value of type Voltage
impl<T> core::ops::Mul<&Charge<T>> for Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Voltage{V: self.per_F * rhs.C.clone()}
	}
}
/// Multiplying a Elastance by a Charge returns a value of type Voltage
impl<T> core::ops::Mul<&Charge<T>> for &Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Voltage{V: self.per_F.clone() * rhs.C.clone()}
	}
}

// Elastance * Conductance -> Frequency
/// Multiplying a Elastance by a Conductance returns a value of type Frequency
impl<T> core::ops::Mul<Conductance<T>> for Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_F * rhs.S}
	}
}
/// Multiplying a Elastance by a Conductance returns a value of type Frequency
impl<T> core::ops::Mul<Conductance<T>> for &Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_F.clone() * rhs.S}
	}
}
/// Multiplying a Elastance by a Conductance returns a value of type Frequency
impl<T> core::ops::Mul<&Conductance<T>> for Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_F * rhs.S.clone()}
	}
}
/// Multiplying a Elastance by a Conductance returns a value of type Frequency
impl<T> core::ops::Mul<&Conductance<T>> for &Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_F.clone() * rhs.S.clone()}
	}
}

// Elastance / InverseCharge -> Voltage
/// Dividing a Elastance by a InverseCharge returns a value of type Voltage
impl<T> core::ops::Div<InverseCharge<T>> for Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Voltage{V: self.per_F / rhs.per_C}
	}
}
/// Dividing a Elastance by a InverseCharge returns a value of type Voltage
impl<T> core::ops::Div<InverseCharge<T>> for &Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Voltage{V: self.per_F.clone() / rhs.per_C}
	}
}
/// Dividing a Elastance by a InverseCharge returns a value of type Voltage
impl<T> core::ops::Div<&InverseCharge<T>> for Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Voltage{V: self.per_F / rhs.per_C.clone()}
	}
}
/// Dividing a Elastance by a InverseCharge returns a value of type Voltage
impl<T> core::ops::Div<&InverseCharge<T>> for &Elastance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Voltage{V: self.per_F.clone() / rhs.per_C.clone()}
	}
}

// Elastance * InverseVoltage -> InverseCharge
/// Multiplying a Elastance by a InverseVoltage returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseVoltage<T>> for Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F * rhs.per_V}
	}
}
/// Multiplying a Elastance by a InverseVoltage returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseVoltage<T>> for &Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F.clone() * rhs.per_V}
	}
}
/// Multiplying a Elastance by a InverseVoltage returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseVoltage<T>> for Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F * rhs.per_V.clone()}
	}
}
/// Multiplying a Elastance by a InverseVoltage returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseVoltage<T>> for &Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F.clone() * rhs.per_V.clone()}
	}
}

// Elastance / Resistance -> Frequency
/// Dividing a Elastance by a Resistance returns a value of type Frequency
impl<T> core::ops::Div<Resistance<T>> for Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_F / rhs.Ohm}
	}
}
/// Dividing a Elastance by a Resistance returns a value of type Frequency
impl<T> core::ops::Div<Resistance<T>> for &Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_F.clone() / rhs.Ohm}
	}
}
/// Dividing a Elastance by a Resistance returns a value of type Frequency
impl<T> core::ops::Div<&Resistance<T>> for Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_F / rhs.Ohm.clone()}
	}
}
/// Dividing a Elastance by a Resistance returns a value of type Frequency
impl<T> core::ops::Div<&Resistance<T>> for &Elastance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_F.clone() / rhs.Ohm.clone()}
	}
}

// Elastance / Voltage -> InverseCharge
/// Dividing a Elastance by a Voltage returns a value of type InverseCharge
impl<T> core::ops::Div<Voltage<T>> for Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F / rhs.V}
	}
}
/// Dividing a Elastance by a Voltage returns a value of type InverseCharge
impl<T> core::ops::Div<Voltage<T>> for &Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F.clone() / rhs.V}
	}
}
/// Dividing a Elastance by a Voltage returns a value of type InverseCharge
impl<T> core::ops::Div<&Voltage<T>> for Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F / rhs.V.clone()}
	}
}
/// Dividing a Elastance by a Voltage returns a value of type InverseCharge
impl<T> core::ops::Div<&Voltage<T>> for &Elastance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseCharge{per_C: self.per_F.clone() / rhs.V.clone()}
	}
}

// Elastance / Frequency -> Resistance
/// Dividing a Elastance by a Frequency returns a value of type Resistance
impl<T> core::ops::Div<Frequency<T>> for Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.per_F / rhs.Hz}
	}
}
/// Dividing a Elastance by a Frequency returns a value of type Resistance
impl<T> core::ops::Div<Frequency<T>> for &Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.per_F.clone() / rhs.Hz}
	}
}
/// Dividing a Elastance by a Frequency returns a value of type Resistance
impl<T> core::ops::Div<&Frequency<T>> for Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.per_F / rhs.Hz.clone()}
	}
}
/// Dividing a Elastance by a Frequency returns a value of type Resistance
impl<T> core::ops::Div<&Frequency<T>> for &Elastance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.per_F.clone() / rhs.Hz.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<Elastance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
impl<T> core::ops::Div<&Elastance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Elastance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Elastance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Elastance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Elastance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Elastance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Elastance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Elastance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Elastance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

// 1/Elastance -> Capacitance
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Elastance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Elastance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Elastance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self) / rhs.per_F.clone()}
	}
}
/// Dividing a scalar value by a Elastance unit value returns a value of type Capacitance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Elastance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Capacitance{F: T::from(self.clone()) / rhs.per_F.clone()}
	}
}

/// The illuminance unit type, defined as lux in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Illuminance<T: NumLike>{
	/// The value of this Illuminance in lux
	pub lux: T
}

impl<T> Illuminance<T> where T: NumLike {

	/// Returns the standard unit name of illuminance: "lux"
	pub fn unit_name() -> &'static str { "lux" }
	
	/// Returns the abbreviated name or symbol of illuminance: "lux" for lux
	pub fn unit_symbol() -> &'static str { "lux" }
	
	/// Returns a new illuminance value from the given number of lux
	///
	/// # Arguments
	/// * `lux` - Any number-like type, representing a quantity of lux
	pub fn from_lux(lux: T) -> Self { Illuminance{lux: lux} }
	
	/// Returns a copy of this illuminance value in lux
	pub fn to_lux(&self) -> T { self.lux.clone() }

}

impl<T> fmt::Display for Illuminance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.lux, Self::unit_symbol())
	}
}

impl<T> Illuminance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this illuminance value in millilux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mlux(&self) -> T {
		return self.lux.clone() * T::from(1000.0_f64);
	}

	/// Returns a new illuminance value from the given number of millilux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mlux` - Any number-like type, representing a quantity of millilux
	pub fn from_mlux(mlux: T) -> Self {
		Illuminance{lux: mlux * T::from(0.001_f64)}
	}

	/// Returns a copy of this illuminance value in microlux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ulux(&self) -> T {
		return self.lux.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new illuminance value from the given number of microlux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ulux` - Any number-like type, representing a quantity of microlux
	pub fn from_ulux(ulux: T) -> Self {
		Illuminance{lux: ulux * T::from(1e-06_f64)}
	}

	/// Returns a copy of this illuminance value in nanolux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nlux(&self) -> T {
		return self.lux.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new illuminance value from the given number of nanolux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nlux` - Any number-like type, representing a quantity of nanolux
	pub fn from_nlux(nlux: T) -> Self {
		Illuminance{lux: nlux * T::from(1e-09_f64)}
	}

	/// Returns a copy of this illuminance value in kilolux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_klux(&self) -> T {
		return self.lux.clone() * T::from(0.001_f64);
	}

	/// Returns a new illuminance value from the given number of kilolux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `klux` - Any number-like type, representing a quantity of kilolux
	pub fn from_klux(klux: T) -> Self {
		Illuminance{lux: klux * T::from(1000.0_f64)}
	}

	/// Returns a copy of this illuminance value in megalux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Mlux(&self) -> T {
		return self.lux.clone() * T::from(1e-06_f64);
	}

	/// Returns a new illuminance value from the given number of megalux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Mlux` - Any number-like type, representing a quantity of megalux
	pub fn from_Mlux(Mlux: T) -> Self {
		Illuminance{lux: Mlux * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this illuminance value in gigalux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Glux(&self) -> T {
		return self.lux.clone() * T::from(1e-09_f64);
	}

	/// Returns a new illuminance value from the given number of gigalux
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Glux` - Any number-like type, representing a quantity of gigalux
	pub fn from_Glux(Glux: T) -> Self {
		Illuminance{lux: Glux * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Illuminance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Illuminance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Illuminance<num_bigfloat::BigFloat>) -> Self::Output {
		Illuminance{lux: self * rhs.lux}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Illuminance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Illuminance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Illuminance<num_bigfloat::BigFloat>) -> Self::Output {
		Illuminance{lux: self.clone() * rhs.lux}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Illuminance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Illuminance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Illuminance<num_bigfloat::BigFloat>) -> Self::Output {
		Illuminance{lux: self * rhs.lux.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Illuminance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Illuminance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Illuminance<num_bigfloat::BigFloat>) -> Self::Output {
		Illuminance{lux: self.clone() * rhs.lux.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Illuminance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Illuminance<num_complex::Complex32>;
	fn mul(self, rhs: Illuminance<num_complex::Complex32>) -> Self::Output {
		Illuminance{lux: self * rhs.lux}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Illuminance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Illuminance<num_complex::Complex32>;
	fn mul(self, rhs: Illuminance<num_complex::Complex32>) -> Self::Output {
		Illuminance{lux: self.clone() * rhs.lux}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Illuminance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Illuminance<num_complex::Complex32>;
	fn mul(self, rhs: &Illuminance<num_complex::Complex32>) -> Self::Output {
		Illuminance{lux: self * rhs.lux.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Illuminance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Illuminance<num_complex::Complex32>;
	fn mul(self, rhs: &Illuminance<num_complex::Complex32>) -> Self::Output {
		Illuminance{lux: self.clone() * rhs.lux.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Illuminance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Illuminance<num_complex::Complex64>;
	fn mul(self, rhs: Illuminance<num_complex::Complex64>) -> Self::Output {
		Illuminance{lux: self * rhs.lux}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Illuminance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Illuminance<num_complex::Complex64>;
	fn mul(self, rhs: Illuminance<num_complex::Complex64>) -> Self::Output {
		Illuminance{lux: self.clone() * rhs.lux}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Illuminance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Illuminance<num_complex::Complex64>;
	fn mul(self, rhs: &Illuminance<num_complex::Complex64>) -> Self::Output {
		Illuminance{lux: self * rhs.lux.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Illuminance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Illuminance<num_complex::Complex64>;
	fn mul(self, rhs: &Illuminance<num_complex::Complex64>) -> Self::Output {
		Illuminance{lux: self.clone() * rhs.lux.clone()}
	}
}



/// Converts a Illuminance into the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Luminance> for Illuminance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Luminance {
		uom::si::f32::Luminance::new::<uom::si::luminance::candela_per_square_meter>(self.lux.into())
	}
}

/// Creates a Illuminance from the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Luminance> for Illuminance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Luminance) -> Self {
		Illuminance{lux: T::from(src.value)}
	}
}

/// Converts a Illuminance into the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Luminance> for Illuminance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Luminance {
		uom::si::f64::Luminance::new::<uom::si::luminance::candela_per_square_meter>(self.lux.into())
	}
}

/// Creates a Illuminance from the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Luminance> for Illuminance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Luminance) -> Self {
		Illuminance{lux: T::from(src.value)}
	}
}


// Illuminance * InverseLuminousFlux -> InverseArea
/// Multiplying a Illuminance by a InverseLuminousFlux returns a value of type InverseArea
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux * rhs.per_lm}
	}
}
/// Multiplying a Illuminance by a InverseLuminousFlux returns a value of type InverseArea
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for &Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux.clone() * rhs.per_lm}
	}
}
/// Multiplying a Illuminance by a InverseLuminousFlux returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux * rhs.per_lm.clone()}
	}
}
/// Multiplying a Illuminance by a InverseLuminousFlux returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for &Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux.clone() * rhs.per_lm.clone()}
	}
}

// Illuminance / LuminousFlux -> InverseArea
/// Dividing a Illuminance by a LuminousFlux returns a value of type InverseArea
impl<T> core::ops::Div<LuminousFlux<T>> for Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux / rhs.lm}
	}
}
/// Dividing a Illuminance by a LuminousFlux returns a value of type InverseArea
impl<T> core::ops::Div<LuminousFlux<T>> for &Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux.clone() / rhs.lm}
	}
}
/// Dividing a Illuminance by a LuminousFlux returns a value of type InverseArea
impl<T> core::ops::Div<&LuminousFlux<T>> for Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux / rhs.lm.clone()}
	}
}
/// Dividing a Illuminance by a LuminousFlux returns a value of type InverseArea
impl<T> core::ops::Div<&LuminousFlux<T>> for &Illuminance<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.lux.clone() / rhs.lm.clone()}
	}
}

// Illuminance * Area -> LuminousFlux
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> core::ops::Mul<Area<T>> for Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux * rhs.m2}
	}
}
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> core::ops::Mul<Area<T>> for &Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux.clone() * rhs.m2}
	}
}
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> core::ops::Mul<&Area<T>> for Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux * rhs.m2.clone()}
	}
}
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> core::ops::Mul<&Area<T>> for &Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux.clone() * rhs.m2.clone()}
	}
}

// Illuminance / InverseArea -> LuminousFlux
/// Dividing a Illuminance by a InverseArea returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseArea<T>> for Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		LuminousFlux{lm: self.lux / rhs.per_m2}
	}
}
/// Dividing a Illuminance by a InverseArea returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseArea<T>> for &Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		LuminousFlux{lm: self.lux.clone() / rhs.per_m2}
	}
}
/// Dividing a Illuminance by a InverseArea returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseArea<T>> for Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		LuminousFlux{lm: self.lux / rhs.per_m2.clone()}
	}
}
/// Dividing a Illuminance by a InverseArea returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseArea<T>> for &Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		LuminousFlux{lm: self.lux.clone() / rhs.per_m2.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for f64 where T: NumLike+From<f64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for f64 where T: NumLike+From<f64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for f32 where T: NumLike+From<f32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for f32 where T: NumLike+From<f32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for i64 where T: NumLike+From<i64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for i64 where T: NumLike+From<i64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for i32 where T: NumLike+From<i32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<Illuminance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for i32 where T: NumLike+From<i32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
impl<T> core::ops::Div<&Illuminance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Illuminance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Illuminance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Illuminance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Illuminance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Illuminance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Illuminance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Illuminance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Illuminance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

// 1/Illuminance -> AreaPerLumen
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Illuminance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Illuminance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Illuminance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self) / rhs.lux.clone()}
	}
}
/// Dividing a scalar value by a Illuminance unit value returns a value of type AreaPerLumen
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Illuminance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: T::from(self.clone()) / rhs.lux.clone()}
	}
}

/// The inductance unit type, defined as henries in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Inductance<T: NumLike>{
	/// The value of this Inductance in henries
	pub H: T
}

impl<T> Inductance<T> where T: NumLike {

	/// Returns the standard unit name of inductance: "henries"
	pub fn unit_name() -> &'static str { "henries" }
	
	/// Returns the abbreviated name or symbol of inductance: "H" for henries
	pub fn unit_symbol() -> &'static str { "H" }
	
	/// Returns a new inductance value from the given number of henries
	///
	/// # Arguments
	/// * `H` - Any number-like type, representing a quantity of henries
	pub fn from_H(H: T) -> Self { Inductance{H: H} }
	
	/// Returns a copy of this inductance value in henries
	pub fn to_H(&self) -> T { self.H.clone() }

	/// Returns a new inductance value from the given number of henries
	///
	/// # Arguments
	/// * `henries` - Any number-like type, representing a quantity of henries
	pub fn from_henries(henries: T) -> Self { Inductance{H: henries} }
	
	/// Returns a copy of this inductance value in henries
	pub fn to_henries(&self) -> T { self.H.clone() }

}

impl<T> fmt::Display for Inductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.H, Self::unit_symbol())
	}
}

impl<T> Inductance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inductance value in millihenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mH(&self) -> T {
		return self.H.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inductance value from the given number of millihenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mH` - Any number-like type, representing a quantity of millihenries
	pub fn from_mH(mH: T) -> Self {
		Inductance{H: mH * T::from(0.001_f64)}
	}

	/// Returns a copy of this inductance value in microhenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uH(&self) -> T {
		return self.H.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inductance value from the given number of microhenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uH` - Any number-like type, representing a quantity of microhenries
	pub fn from_uH(uH: T) -> Self {
		Inductance{H: uH * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inductance value in nanohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nH(&self) -> T {
		return self.H.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inductance value from the given number of nanohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nH` - Any number-like type, representing a quantity of nanohenries
	pub fn from_nH(nH: T) -> Self {
		Inductance{H: nH * T::from(1e-09_f64)}
	}

	/// Returns a copy of this inductance value in kilohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kH(&self) -> T {
		return self.H.clone() * T::from(0.001_f64);
	}

	/// Returns a new inductance value from the given number of kilohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kH` - Any number-like type, representing a quantity of kilohenries
	pub fn from_kH(kH: T) -> Self {
		Inductance{H: kH * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inductance value in megahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MH(&self) -> T {
		return self.H.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inductance value from the given number of megahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MH` - Any number-like type, representing a quantity of megahenries
	pub fn from_MH(MH: T) -> Self {
		Inductance{H: MH * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inductance value in gigahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GH(&self) -> T {
		return self.H.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inductance value from the given number of gigahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GH` - Any number-like type, representing a quantity of gigahenries
	pub fn from_GH(GH: T) -> Self {
		Inductance{H: GH * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Inductance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Inductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Inductance<num_bigfloat::BigFloat>) -> Self::Output {
		Inductance{H: self * rhs.H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Inductance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Inductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Inductance<num_bigfloat::BigFloat>) -> Self::Output {
		Inductance{H: self.clone() * rhs.H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Inductance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Inductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Inductance<num_bigfloat::BigFloat>) -> Self::Output {
		Inductance{H: self * rhs.H.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Inductance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Inductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Inductance<num_bigfloat::BigFloat>) -> Self::Output {
		Inductance{H: self.clone() * rhs.H.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Inductance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Inductance<num_complex::Complex32>;
	fn mul(self, rhs: Inductance<num_complex::Complex32>) -> Self::Output {
		Inductance{H: self * rhs.H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Inductance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Inductance<num_complex::Complex32>;
	fn mul(self, rhs: Inductance<num_complex::Complex32>) -> Self::Output {
		Inductance{H: self.clone() * rhs.H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Inductance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Inductance<num_complex::Complex32>;
	fn mul(self, rhs: &Inductance<num_complex::Complex32>) -> Self::Output {
		Inductance{H: self * rhs.H.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Inductance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Inductance<num_complex::Complex32>;
	fn mul(self, rhs: &Inductance<num_complex::Complex32>) -> Self::Output {
		Inductance{H: self.clone() * rhs.H.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Inductance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Inductance<num_complex::Complex64>;
	fn mul(self, rhs: Inductance<num_complex::Complex64>) -> Self::Output {
		Inductance{H: self * rhs.H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Inductance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Inductance<num_complex::Complex64>;
	fn mul(self, rhs: Inductance<num_complex::Complex64>) -> Self::Output {
		Inductance{H: self.clone() * rhs.H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Inductance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Inductance<num_complex::Complex64>;
	fn mul(self, rhs: &Inductance<num_complex::Complex64>) -> Self::Output {
		Inductance{H: self * rhs.H.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Inductance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Inductance<num_complex::Complex64>;
	fn mul(self, rhs: &Inductance<num_complex::Complex64>) -> Self::Output {
		Inductance{H: self.clone() * rhs.H.clone()}
	}
}



/// Converts a Inductance into the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Inductance> for Inductance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Inductance {
		uom::si::f32::Inductance::new::<uom::si::inductance::henry>(self.H.into())
	}
}

/// Creates a Inductance from the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Inductance> for Inductance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Inductance) -> Self {
		Inductance{H: T::from(src.value)}
	}
}

/// Converts a Inductance into the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Inductance> for Inductance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Inductance {
		uom::si::f64::Inductance::new::<uom::si::inductance::henry>(self.H.into())
	}
}

/// Creates a Inductance from the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Inductance> for Inductance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Inductance) -> Self {
		Inductance{H: T::from(src.value)}
	}
}


// Inductance * Current -> MagneticFlux
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> core::ops::Mul<Current<T>> for Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H * rhs.A}
	}
}
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> core::ops::Mul<Current<T>> for &Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H.clone() * rhs.A}
	}
}
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Current<T>> for Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H * rhs.A.clone()}
	}
}
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Current<T>> for &Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H.clone() * rhs.A.clone()}
	}
}

// Inductance / InverseCurrent -> MagneticFlux
/// Dividing a Inductance by a InverseCurrent returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseCurrent<T>> for Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		MagneticFlux{Wb: self.H / rhs.per_A}
	}
}
/// Dividing a Inductance by a InverseCurrent returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseCurrent<T>> for &Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		MagneticFlux{Wb: self.H.clone() / rhs.per_A}
	}
}
/// Dividing a Inductance by a InverseCurrent returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseCurrent<T>> for Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		MagneticFlux{Wb: self.H / rhs.per_A.clone()}
	}
}
/// Dividing a Inductance by a InverseCurrent returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseCurrent<T>> for &Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		MagneticFlux{Wb: self.H.clone() / rhs.per_A.clone()}
	}
}

// Inductance / Time -> Resistance
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> core::ops::Div<Time<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Resistance{Ohm: self.H / rhs.s}
	}
}
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> core::ops::Div<Time<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() / rhs.s}
	}
}
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> core::ops::Div<&Time<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Resistance{Ohm: self.H / rhs.s.clone()}
	}
}
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> core::ops::Div<&Time<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() / rhs.s.clone()}
	}
}

// Inductance * Conductance -> Time
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> core::ops::Mul<Conductance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.H * rhs.S}
	}
}
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> core::ops::Mul<Conductance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.H.clone() * rhs.S}
	}
}
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> core::ops::Mul<&Conductance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.H * rhs.S.clone()}
	}
}
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> core::ops::Mul<&Conductance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.H.clone() * rhs.S.clone()}
	}
}

// Inductance * InverseMagneticFlux -> InverseCurrent
/// Multiplying a Inductance by a InverseMagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H * rhs.per_Wb}
	}
}
/// Multiplying a Inductance by a InverseMagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Inductance by a InverseMagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Inductance by a InverseMagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H.clone() * rhs.per_Wb.clone()}
	}
}

// Inductance / MagneticFlux -> InverseCurrent
/// Dividing a Inductance by a MagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Div<MagneticFlux<T>> for Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H / rhs.Wb}
	}
}
/// Dividing a Inductance by a MagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Div<MagneticFlux<T>> for &Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H.clone() / rhs.Wb}
	}
}
/// Dividing a Inductance by a MagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Div<&MagneticFlux<T>> for Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H / rhs.Wb.clone()}
	}
}
/// Dividing a Inductance by a MagneticFlux returns a value of type InverseCurrent
impl<T> core::ops::Div<&MagneticFlux<T>> for &Inductance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseCurrent{per_A: self.H.clone() / rhs.Wb.clone()}
	}
}

// Inductance / Resistance -> Time
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> core::ops::Div<Resistance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.H / rhs.Ohm}
	}
}
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> core::ops::Div<Resistance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.H.clone() / rhs.Ohm}
	}
}
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> core::ops::Div<&Resistance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.H / rhs.Ohm.clone()}
	}
}
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> core::ops::Div<&Resistance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.H.clone() / rhs.Ohm.clone()}
	}
}

// Inductance * Frequency -> Resistance
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> core::ops::Mul<Frequency<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H * rhs.Hz}
	}
}
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> core::ops::Mul<Frequency<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() * rhs.Hz}
	}
}
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> core::ops::Mul<&Frequency<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H * rhs.Hz.clone()}
	}
}
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> core::ops::Mul<&Frequency<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() * rhs.Hz.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<Inductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
impl<T> core::ops::Div<&Inductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Inductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Inductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Inductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Inductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Inductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Inductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Inductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Inductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

// 1/Inductance -> InverseInductance
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Inductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Inductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Inductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self) / rhs.H.clone()}
	}
}
/// Dividing a scalar value by a Inductance unit value returns a value of type InverseInductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Inductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		InverseInductance{per_H: T::from(self.clone()) / rhs.H.clone()}
	}
}

/// The inverse of electric charge (aka coulombs) unit type, defined as inverse coulombs in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseCharge<T: NumLike>{
	/// The value of this Inverse electric charge in inverse coulombs
	pub per_C: T
}

impl<T> InverseCharge<T> where T: NumLike {

	/// Returns the standard unit name of inverse electric charge: "inverse coulombs"
	pub fn unit_name() -> &'static str { "inverse coulombs" }
	
	/// Returns the abbreviated name or symbol of inverse electric charge: "1/C" for inverse coulombs
	pub fn unit_symbol() -> &'static str { "1/C" }
	
	/// Returns a new inverse electric charge value from the given number of inverse coulombs
	///
	/// # Arguments
	/// * `per_C` - Any number-like type, representing a quantity of inverse coulombs
	pub fn from_per_C(per_C: T) -> Self { InverseCharge{per_C: per_C} }
	
	/// Returns a copy of this inverse electric charge value in inverse coulombs
	pub fn to_per_C(&self) -> T { self.per_C.clone() }

	/// Returns a new inverse electric charge value from the given number of inverse coulombs
	///
	/// # Arguments
	/// * `per_coulombs` - Any number-like type, representing a quantity of inverse coulombs
	pub fn from_per_coulombs(per_coulombs: T) -> Self { InverseCharge{per_C: per_coulombs} }
	
	/// Returns a copy of this inverse electric charge value in inverse coulombs
	pub fn to_per_coulombs(&self) -> T { self.per_C.clone() }

}

impl<T> fmt::Display for InverseCharge<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_C, Self::unit_symbol())
	}
}

impl<T> InverseCharge<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse electric charge value in inverse millicoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mC(&self) -> T {
		return self.per_C.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse electric charge value from the given number of inverse millicoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mC` - Any number-like type, representing a quantity of inverse millicoulombs
	pub fn from_per_mC(per_mC: T) -> Self {
		InverseCharge{per_C: per_mC * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse electric charge value in inverse microcoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uC(&self) -> T {
		return self.per_C.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse electric charge value from the given number of inverse microcoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uC` - Any number-like type, representing a quantity of inverse microcoulombs
	pub fn from_per_uC(per_uC: T) -> Self {
		InverseCharge{per_C: per_uC * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse electric charge value in inverse nanocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nC(&self) -> T {
		return self.per_C.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse electric charge value from the given number of inverse nanocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nC` - Any number-like type, representing a quantity of inverse nanocoulombs
	pub fn from_per_nC(per_nC: T) -> Self {
		InverseCharge{per_C: per_nC * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse electric charge value in inverse kilocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kC(&self) -> T {
		return self.per_C.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse electric charge value from the given number of inverse kilocoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kC` - Any number-like type, representing a quantity of inverse kilocoulombs
	pub fn from_per_kC(per_kC: T) -> Self {
		InverseCharge{per_C: per_kC * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse electric charge value in inverse megacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_MC(&self) -> T {
		return self.per_C.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse electric charge value from the given number of inverse megacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_MC` - Any number-like type, representing a quantity of inverse megacoulombs
	pub fn from_per_MC(per_MC: T) -> Self {
		InverseCharge{per_C: per_MC * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse electric charge value in inverse gigacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GC(&self) -> T {
		return self.per_C.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse electric charge value from the given number of inverse gigacoulombs
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GC` - Any number-like type, representing a quantity of inverse gigacoulombs
	pub fn from_per_GC(per_GC: T) -> Self {
		InverseCharge{per_C: per_GC * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseCharge<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseCharge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseCharge<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCharge{per_C: self * rhs.per_C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseCharge<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseCharge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseCharge<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCharge{per_C: self.clone() * rhs.per_C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseCharge<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseCharge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseCharge<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCharge{per_C: self * rhs.per_C.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseCharge<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseCharge<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseCharge<num_bigfloat::BigFloat>) -> Self::Output {
		InverseCharge{per_C: self.clone() * rhs.per_C.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCharge<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseCharge<num_complex::Complex32>;
	fn mul(self, rhs: InverseCharge<num_complex::Complex32>) -> Self::Output {
		InverseCharge{per_C: self * rhs.per_C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCharge<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseCharge<num_complex::Complex32>;
	fn mul(self, rhs: InverseCharge<num_complex::Complex32>) -> Self::Output {
		InverseCharge{per_C: self.clone() * rhs.per_C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCharge<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseCharge<num_complex::Complex32>;
	fn mul(self, rhs: &InverseCharge<num_complex::Complex32>) -> Self::Output {
		InverseCharge{per_C: self * rhs.per_C.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCharge<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseCharge<num_complex::Complex32>;
	fn mul(self, rhs: &InverseCharge<num_complex::Complex32>) -> Self::Output {
		InverseCharge{per_C: self.clone() * rhs.per_C.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCharge<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseCharge<num_complex::Complex64>;
	fn mul(self, rhs: InverseCharge<num_complex::Complex64>) -> Self::Output {
		InverseCharge{per_C: self * rhs.per_C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseCharge<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseCharge<num_complex::Complex64>;
	fn mul(self, rhs: InverseCharge<num_complex::Complex64>) -> Self::Output {
		InverseCharge{per_C: self.clone() * rhs.per_C}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCharge<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseCharge<num_complex::Complex64>;
	fn mul(self, rhs: &InverseCharge<num_complex::Complex64>) -> Self::Output {
		InverseCharge{per_C: self * rhs.per_C.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseCharge<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseCharge<num_complex::Complex64>;
	fn mul(self, rhs: &InverseCharge<num_complex::Complex64>) -> Self::Output {
		InverseCharge{per_C: self.clone() * rhs.per_C.clone()}
	}
}




// InverseCharge * Current -> Frequency
/// Multiplying a InverseCharge by a Current returns a value of type Frequency
impl<T> core::ops::Mul<Current<T>> for InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Frequency{Hz: self.per_C * rhs.A}
	}
}
/// Multiplying a InverseCharge by a Current returns a value of type Frequency
impl<T> core::ops::Mul<Current<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Frequency{Hz: self.per_C.clone() * rhs.A}
	}
}
/// Multiplying a InverseCharge by a Current returns a value of type Frequency
impl<T> core::ops::Mul<&Current<T>> for InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Frequency{Hz: self.per_C * rhs.A.clone()}
	}
}
/// Multiplying a InverseCharge by a Current returns a value of type Frequency
impl<T> core::ops::Mul<&Current<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Frequency{Hz: self.per_C.clone() * rhs.A.clone()}
	}
}

// InverseCharge / InverseCurrent -> Frequency
/// Dividing a InverseCharge by a InverseCurrent returns a value of type Frequency
impl<T> core::ops::Div<InverseCurrent<T>> for InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Frequency{Hz: self.per_C / rhs.per_A}
	}
}
/// Dividing a InverseCharge by a InverseCurrent returns a value of type Frequency
impl<T> core::ops::Div<InverseCurrent<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Frequency{Hz: self.per_C.clone() / rhs.per_A}
	}
}
/// Dividing a InverseCharge by a InverseCurrent returns a value of type Frequency
impl<T> core::ops::Div<&InverseCurrent<T>> for InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Frequency{Hz: self.per_C / rhs.per_A.clone()}
	}
}
/// Dividing a InverseCharge by a InverseCurrent returns a value of type Frequency
impl<T> core::ops::Div<&InverseCurrent<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Frequency{Hz: self.per_C.clone() / rhs.per_A.clone()}
	}
}

// InverseCharge * Time -> InverseCurrent
/// Multiplying a InverseCharge by a Time returns a value of type InverseCurrent
impl<T> core::ops::Mul<Time<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C * rhs.s}
	}
}
/// Multiplying a InverseCharge by a Time returns a value of type InverseCurrent
impl<T> core::ops::Mul<Time<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C.clone() * rhs.s}
	}
}
/// Multiplying a InverseCharge by a Time returns a value of type InverseCurrent
impl<T> core::ops::Mul<&Time<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C * rhs.s.clone()}
	}
}
/// Multiplying a InverseCharge by a Time returns a value of type InverseCurrent
impl<T> core::ops::Mul<&Time<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C.clone() * rhs.s.clone()}
	}
}

// InverseCharge * Capacitance -> InverseVoltage
/// Multiplying a InverseCharge by a Capacitance returns a value of type InverseVoltage
impl<T> core::ops::Mul<Capacitance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C * rhs.F}
	}
}
/// Multiplying a InverseCharge by a Capacitance returns a value of type InverseVoltage
impl<T> core::ops::Mul<Capacitance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C.clone() * rhs.F}
	}
}
/// Multiplying a InverseCharge by a Capacitance returns a value of type InverseVoltage
impl<T> core::ops::Mul<&Capacitance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C * rhs.F.clone()}
	}
}
/// Multiplying a InverseCharge by a Capacitance returns a value of type InverseVoltage
impl<T> core::ops::Mul<&Capacitance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C.clone() * rhs.F.clone()}
	}
}

// InverseCharge * Conductance -> InverseMagneticFlux
/// Multiplying a InverseCharge by a Conductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<Conductance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C * rhs.S}
	}
}
/// Multiplying a InverseCharge by a Conductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<Conductance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C.clone() * rhs.S}
	}
}
/// Multiplying a InverseCharge by a Conductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&Conductance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C * rhs.S.clone()}
	}
}
/// Multiplying a InverseCharge by a Conductance returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&Conductance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C.clone() * rhs.S.clone()}
	}
}

// InverseCharge / Elastance -> InverseVoltage
/// Dividing a InverseCharge by a Elastance returns a value of type InverseVoltage
impl<T> core::ops::Div<Elastance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C / rhs.per_F}
	}
}
/// Dividing a InverseCharge by a Elastance returns a value of type InverseVoltage
impl<T> core::ops::Div<Elastance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C.clone() / rhs.per_F}
	}
}
/// Dividing a InverseCharge by a Elastance returns a value of type InverseVoltage
impl<T> core::ops::Div<&Elastance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C / rhs.per_F.clone()}
	}
}
/// Dividing a InverseCharge by a Elastance returns a value of type InverseVoltage
impl<T> core::ops::Div<&Elastance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_C.clone() / rhs.per_F.clone()}
	}
}

// InverseCharge / InverseMagneticFlux -> Resistance
/// Dividing a InverseCharge by a InverseMagneticFlux returns a value of type Resistance
impl<T> core::ops::Div<InverseMagneticFlux<T>> for InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C / rhs.per_Wb}
	}
}
/// Dividing a InverseCharge by a InverseMagneticFlux returns a value of type Resistance
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C.clone() / rhs.per_Wb}
	}
}
/// Dividing a InverseCharge by a InverseMagneticFlux returns a value of type Resistance
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C / rhs.per_Wb.clone()}
	}
}
/// Dividing a InverseCharge by a InverseMagneticFlux returns a value of type Resistance
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C.clone() / rhs.per_Wb.clone()}
	}
}

// InverseCharge * InverseVoltage -> InverseEnergy
/// Multiplying a InverseCharge by a InverseVoltage returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseVoltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C * rhs.per_V}
	}
}
/// Multiplying a InverseCharge by a InverseVoltage returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseVoltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C.clone() * rhs.per_V}
	}
}
/// Multiplying a InverseCharge by a InverseVoltage returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseVoltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C * rhs.per_V.clone()}
	}
}
/// Multiplying a InverseCharge by a InverseVoltage returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseVoltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C.clone() * rhs.per_V.clone()}
	}
}

// InverseCharge / InverseVoltage -> Elastance
/// Dividing a InverseCharge by a InverseVoltage returns a value of type Elastance
impl<T> core::ops::Div<InverseVoltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C / rhs.per_V}
	}
}
/// Dividing a InverseCharge by a InverseVoltage returns a value of type Elastance
impl<T> core::ops::Div<InverseVoltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C.clone() / rhs.per_V}
	}
}
/// Dividing a InverseCharge by a InverseVoltage returns a value of type Elastance
impl<T> core::ops::Div<&InverseVoltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C / rhs.per_V.clone()}
	}
}
/// Dividing a InverseCharge by a InverseVoltage returns a value of type Elastance
impl<T> core::ops::Div<&InverseVoltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C.clone() / rhs.per_V.clone()}
	}
}

// InverseCharge * MagneticFlux -> Resistance
/// Multiplying a InverseCharge by a MagneticFlux returns a value of type Resistance
impl<T> core::ops::Mul<MagneticFlux<T>> for InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C * rhs.Wb}
	}
}
/// Multiplying a InverseCharge by a MagneticFlux returns a value of type Resistance
impl<T> core::ops::Mul<MagneticFlux<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C.clone() * rhs.Wb}
	}
}
/// Multiplying a InverseCharge by a MagneticFlux returns a value of type Resistance
impl<T> core::ops::Mul<&MagneticFlux<T>> for InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C * rhs.Wb.clone()}
	}
}
/// Multiplying a InverseCharge by a MagneticFlux returns a value of type Resistance
impl<T> core::ops::Mul<&MagneticFlux<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Resistance{Ohm: self.per_C.clone() * rhs.Wb.clone()}
	}
}

// InverseCharge / Resistance -> InverseMagneticFlux
/// Dividing a InverseCharge by a Resistance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Resistance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C / rhs.Ohm}
	}
}
/// Dividing a InverseCharge by a Resistance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Resistance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C.clone() / rhs.Ohm}
	}
}
/// Dividing a InverseCharge by a Resistance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Resistance<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C / rhs.Ohm.clone()}
	}
}
/// Dividing a InverseCharge by a Resistance returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Resistance<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_C.clone() / rhs.Ohm.clone()}
	}
}

// InverseCharge * Voltage -> Elastance
/// Multiplying a InverseCharge by a Voltage returns a value of type Elastance
impl<T> core::ops::Mul<Voltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C * rhs.V}
	}
}
/// Multiplying a InverseCharge by a Voltage returns a value of type Elastance
impl<T> core::ops::Mul<Voltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C.clone() * rhs.V}
	}
}
/// Multiplying a InverseCharge by a Voltage returns a value of type Elastance
impl<T> core::ops::Mul<&Voltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C * rhs.V.clone()}
	}
}
/// Multiplying a InverseCharge by a Voltage returns a value of type Elastance
impl<T> core::ops::Mul<&Voltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Elastance{per_F: self.per_C.clone() * rhs.V.clone()}
	}
}

// InverseCharge / Voltage -> InverseEnergy
/// Dividing a InverseCharge by a Voltage returns a value of type InverseEnergy
impl<T> core::ops::Div<Voltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C / rhs.V}
	}
}
/// Dividing a InverseCharge by a Voltage returns a value of type InverseEnergy
impl<T> core::ops::Div<Voltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C.clone() / rhs.V}
	}
}
/// Dividing a InverseCharge by a Voltage returns a value of type InverseEnergy
impl<T> core::ops::Div<&Voltage<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C / rhs.V.clone()}
	}
}
/// Dividing a InverseCharge by a Voltage returns a value of type InverseEnergy
impl<T> core::ops::Div<&Voltage<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_C.clone() / rhs.V.clone()}
	}
}

// InverseCharge * Energy -> Voltage
/// Multiplying a InverseCharge by a Energy returns a value of type Voltage
impl<T> core::ops::Mul<Energy<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Voltage{V: self.per_C * rhs.J}
	}
}
/// Multiplying a InverseCharge by a Energy returns a value of type Voltage
impl<T> core::ops::Mul<Energy<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() * rhs.J}
	}
}
/// Multiplying a InverseCharge by a Energy returns a value of type Voltage
impl<T> core::ops::Mul<&Energy<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Voltage{V: self.per_C * rhs.J.clone()}
	}
}
/// Multiplying a InverseCharge by a Energy returns a value of type Voltage
impl<T> core::ops::Mul<&Energy<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() * rhs.J.clone()}
	}
}

// InverseCharge * Torque -> Voltage
/// Multiplying a InverseCharge by a Torque returns a value of type Voltage
impl<T> core::ops::Mul<Torque<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Voltage{V: self.per_C * rhs.Nm}
	}
}
/// Multiplying a InverseCharge by a Torque returns a value of type Voltage
impl<T> core::ops::Mul<Torque<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() * rhs.Nm}
	}
}
/// Multiplying a InverseCharge by a Torque returns a value of type Voltage
impl<T> core::ops::Mul<&Torque<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Voltage{V: self.per_C * rhs.Nm.clone()}
	}
}
/// Multiplying a InverseCharge by a Torque returns a value of type Voltage
impl<T> core::ops::Mul<&Torque<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() * rhs.Nm.clone()}
	}
}

// InverseCharge / Frequency -> InverseCurrent
/// Dividing a InverseCharge by a Frequency returns a value of type InverseCurrent
impl<T> core::ops::Div<Frequency<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C / rhs.Hz}
	}
}
/// Dividing a InverseCharge by a Frequency returns a value of type InverseCurrent
impl<T> core::ops::Div<Frequency<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C.clone() / rhs.Hz}
	}
}
/// Dividing a InverseCharge by a Frequency returns a value of type InverseCurrent
impl<T> core::ops::Div<&Frequency<T>> for InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C / rhs.Hz.clone()}
	}
}
/// Dividing a InverseCharge by a Frequency returns a value of type InverseCurrent
impl<T> core::ops::Div<&Frequency<T>> for &InverseCharge<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_C.clone() / rhs.Hz.clone()}
	}
}

// InverseCharge / InverseEnergy -> Voltage
/// Dividing a InverseCharge by a InverseEnergy returns a value of type Voltage
impl<T> core::ops::Div<InverseEnergy<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Voltage{V: self.per_C / rhs.per_J}
	}
}
/// Dividing a InverseCharge by a InverseEnergy returns a value of type Voltage
impl<T> core::ops::Div<InverseEnergy<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() / rhs.per_J}
	}
}
/// Dividing a InverseCharge by a InverseEnergy returns a value of type Voltage
impl<T> core::ops::Div<&InverseEnergy<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Voltage{V: self.per_C / rhs.per_J.clone()}
	}
}
/// Dividing a InverseCharge by a InverseEnergy returns a value of type Voltage
impl<T> core::ops::Div<&InverseEnergy<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() / rhs.per_J.clone()}
	}
}

// InverseCharge / InverseTorque -> Voltage
/// Dividing a InverseCharge by a InverseTorque returns a value of type Voltage
impl<T> core::ops::Div<InverseTorque<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Voltage{V: self.per_C / rhs.per_Nm}
	}
}
/// Dividing a InverseCharge by a InverseTorque returns a value of type Voltage
impl<T> core::ops::Div<InverseTorque<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() / rhs.per_Nm}
	}
}
/// Dividing a InverseCharge by a InverseTorque returns a value of type Voltage
impl<T> core::ops::Div<&InverseTorque<T>> for InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Voltage{V: self.per_C / rhs.per_Nm.clone()}
	}
}
/// Dividing a InverseCharge by a InverseTorque returns a value of type Voltage
impl<T> core::ops::Div<&InverseTorque<T>> for &InverseCharge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Voltage{V: self.per_C.clone() / rhs.per_Nm.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for f64 where T: NumLike+From<f64> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for f64 where T: NumLike+From<f64> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for f32 where T: NumLike+From<f32> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for f32 where T: NumLike+From<f32> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for i64 where T: NumLike+From<i64> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for i64 where T: NumLike+From<i64> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for i32 where T: NumLike+From<i32> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<InverseCharge<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for i32 where T: NumLike+From<i32> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
impl<T> core::ops::Div<&InverseCharge<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseCharge<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseCharge<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseCharge<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseCharge<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCharge<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCharge<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCharge<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCharge<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

// 1/InverseCharge -> Charge
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCharge<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseCharge<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Charge<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCharge<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self) / rhs.per_C.clone()}
	}
}
/// Dividing a scalar value by a InverseCharge unit value returns a value of type Charge
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseCharge<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Charge{C: T::from(self.clone()) / rhs.per_C.clone()}
	}
}

/// The inverse of inductance unit type, defined as inverse henries in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseInductance<T: NumLike>{
	/// The value of this Inverse inductance in inverse henries
	pub per_H: T
}

impl<T> InverseInductance<T> where T: NumLike {

	/// Returns the standard unit name of inverse inductance: "inverse henries"
	pub fn unit_name() -> &'static str { "inverse henries" }
	
	/// Returns the abbreviated name or symbol of inverse inductance: "1/H" for inverse henries
	pub fn unit_symbol() -> &'static str { "1/H" }
	
	/// Returns a new inverse inductance value from the given number of inverse henries
	///
	/// # Arguments
	/// * `per_H` - Any number-like type, representing a quantity of inverse henries
	pub fn from_per_H(per_H: T) -> Self { InverseInductance{per_H: per_H} }
	
	/// Returns a copy of this inverse inductance value in inverse henries
	pub fn to_per_H(&self) -> T { self.per_H.clone() }

	/// Returns a new inverse inductance value from the given number of inverse henries
	///
	/// # Arguments
	/// * `per_henry` - Any number-like type, representing a quantity of inverse henries
	pub fn from_per_henry(per_henry: T) -> Self { InverseInductance{per_H: per_henry} }
	
	/// Returns a copy of this inverse inductance value in inverse henries
	pub fn to_per_henry(&self) -> T { self.per_H.clone() }

}

impl<T> fmt::Display for InverseInductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_H, Self::unit_symbol())
	}
}

impl<T> InverseInductance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse inductance value in inverse millihenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mH(&self) -> T {
		return self.per_H.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse inductance value from the given number of inverse millihenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mH` - Any number-like type, representing a quantity of inverse millihenries
	pub fn from_per_mH(per_mH: T) -> Self {
		InverseInductance{per_H: per_mH * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse inductance value in inverse microhenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uH(&self) -> T {
		return self.per_H.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse inductance value from the given number of inverse microhenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uH` - Any number-like type, representing a quantity of inverse microhenries
	pub fn from_per_uH(per_uH: T) -> Self {
		InverseInductance{per_H: per_uH * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse inductance value in inverse nanohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nH(&self) -> T {
		return self.per_H.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse inductance value from the given number of inverse nanohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nH` - Any number-like type, representing a quantity of inverse nanohenries
	pub fn from_per_nH(per_nH: T) -> Self {
		InverseInductance{per_H: per_nH * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse inductance value in inverse kilohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kH(&self) -> T {
		return self.per_H.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse inductance value from the given number of inverse kilohenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kH` - Any number-like type, representing a quantity of inverse kilohenries
	pub fn from_per_kH(per_kH: T) -> Self {
		InverseInductance{per_H: per_kH * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse inductance value in inverse megahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_MH(&self) -> T {
		return self.per_H.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse inductance value from the given number of inverse megahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_MH` - Any number-like type, representing a quantity of inverse megahenries
	pub fn from_per_MH(per_MH: T) -> Self {
		InverseInductance{per_H: per_MH * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse inductance value in inverse gigahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GH(&self) -> T {
		return self.per_H.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse inductance value from the given number of inverse gigahenries
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GH` - Any number-like type, representing a quantity of inverse gigahenries
	pub fn from_per_GH(per_GH: T) -> Self {
		InverseInductance{per_H: per_GH * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseInductance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseInductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseInductance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseInductance{per_H: self * rhs.per_H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseInductance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseInductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseInductance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseInductance{per_H: self.clone() * rhs.per_H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseInductance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseInductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseInductance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseInductance{per_H: self * rhs.per_H.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseInductance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseInductance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseInductance<num_bigfloat::BigFloat>) -> Self::Output {
		InverseInductance{per_H: self.clone() * rhs.per_H.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseInductance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseInductance<num_complex::Complex32>;
	fn mul(self, rhs: InverseInductance<num_complex::Complex32>) -> Self::Output {
		InverseInductance{per_H: self * rhs.per_H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseInductance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseInductance<num_complex::Complex32>;
	fn mul(self, rhs: InverseInductance<num_complex::Complex32>) -> Self::Output {
		InverseInductance{per_H: self.clone() * rhs.per_H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseInductance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseInductance<num_complex::Complex32>;
	fn mul(self, rhs: &InverseInductance<num_complex::Complex32>) -> Self::Output {
		InverseInductance{per_H: self * rhs.per_H.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseInductance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseInductance<num_complex::Complex32>;
	fn mul(self, rhs: &InverseInductance<num_complex::Complex32>) -> Self::Output {
		InverseInductance{per_H: self.clone() * rhs.per_H.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseInductance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseInductance<num_complex::Complex64>;
	fn mul(self, rhs: InverseInductance<num_complex::Complex64>) -> Self::Output {
		InverseInductance{per_H: self * rhs.per_H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseInductance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseInductance<num_complex::Complex64>;
	fn mul(self, rhs: InverseInductance<num_complex::Complex64>) -> Self::Output {
		InverseInductance{per_H: self.clone() * rhs.per_H}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseInductance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseInductance<num_complex::Complex64>;
	fn mul(self, rhs: &InverseInductance<num_complex::Complex64>) -> Self::Output {
		InverseInductance{per_H: self * rhs.per_H.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseInductance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseInductance<num_complex::Complex64>;
	fn mul(self, rhs: &InverseInductance<num_complex::Complex64>) -> Self::Output {
		InverseInductance{per_H: self.clone() * rhs.per_H.clone()}
	}
}




// InverseInductance / Current -> InverseMagneticFlux
/// Dividing a InverseInductance by a Current returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Current<T>> for InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H / rhs.A}
	}
}
/// Dividing a InverseInductance by a Current returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Current<T>> for &InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H.clone() / rhs.A}
	}
}
/// Dividing a InverseInductance by a Current returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Current<T>> for InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H / rhs.A.clone()}
	}
}
/// Dividing a InverseInductance by a Current returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Current<T>> for &InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H.clone() / rhs.A.clone()}
	}
}

// InverseInductance * InverseCurrent -> InverseMagneticFlux
/// Multiplying a InverseInductance by a InverseCurrent returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseCurrent<T>> for InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H * rhs.per_A}
	}
}
/// Multiplying a InverseInductance by a InverseCurrent returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseCurrent<T>> for &InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H.clone() * rhs.per_A}
	}
}
/// Multiplying a InverseInductance by a InverseCurrent returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseCurrent<T>> for InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H * rhs.per_A.clone()}
	}
}
/// Multiplying a InverseInductance by a InverseCurrent returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseCurrent<T>> for &InverseInductance<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_H.clone() * rhs.per_A.clone()}
	}
}

// InverseInductance * Time -> Conductance
/// Multiplying a InverseInductance by a Time returns a value of type Conductance
impl<T> core::ops::Mul<Time<T>> for InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Conductance{S: self.per_H * rhs.s}
	}
}
/// Multiplying a InverseInductance by a Time returns a value of type Conductance
impl<T> core::ops::Mul<Time<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Conductance{S: self.per_H.clone() * rhs.s}
	}
}
/// Multiplying a InverseInductance by a Time returns a value of type Conductance
impl<T> core::ops::Mul<&Time<T>> for InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Conductance{S: self.per_H * rhs.s.clone()}
	}
}
/// Multiplying a InverseInductance by a Time returns a value of type Conductance
impl<T> core::ops::Mul<&Time<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Conductance{S: self.per_H.clone() * rhs.s.clone()}
	}
}

// InverseInductance / Conductance -> Frequency
/// Dividing a InverseInductance by a Conductance returns a value of type Frequency
impl<T> core::ops::Div<Conductance<T>> for InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_H / rhs.S}
	}
}
/// Dividing a InverseInductance by a Conductance returns a value of type Frequency
impl<T> core::ops::Div<Conductance<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_H.clone() / rhs.S}
	}
}
/// Dividing a InverseInductance by a Conductance returns a value of type Frequency
impl<T> core::ops::Div<&Conductance<T>> for InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_H / rhs.S.clone()}
	}
}
/// Dividing a InverseInductance by a Conductance returns a value of type Frequency
impl<T> core::ops::Div<&Conductance<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Frequency{Hz: self.per_H.clone() / rhs.S.clone()}
	}
}

// InverseInductance / InverseMagneticFlux -> Current
/// Dividing a InverseInductance by a InverseMagneticFlux returns a value of type Current
impl<T> core::ops::Div<InverseMagneticFlux<T>> for InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H / rhs.per_Wb}
	}
}
/// Dividing a InverseInductance by a InverseMagneticFlux returns a value of type Current
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H.clone() / rhs.per_Wb}
	}
}
/// Dividing a InverseInductance by a InverseMagneticFlux returns a value of type Current
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H / rhs.per_Wb.clone()}
	}
}
/// Dividing a InverseInductance by a InverseMagneticFlux returns a value of type Current
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H.clone() / rhs.per_Wb.clone()}
	}
}

// InverseInductance * MagneticFlux -> Current
/// Multiplying a InverseInductance by a MagneticFlux returns a value of type Current
impl<T> core::ops::Mul<MagneticFlux<T>> for InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H * rhs.Wb}
	}
}
/// Multiplying a InverseInductance by a MagneticFlux returns a value of type Current
impl<T> core::ops::Mul<MagneticFlux<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H.clone() * rhs.Wb}
	}
}
/// Multiplying a InverseInductance by a MagneticFlux returns a value of type Current
impl<T> core::ops::Mul<&MagneticFlux<T>> for InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H * rhs.Wb.clone()}
	}
}
/// Multiplying a InverseInductance by a MagneticFlux returns a value of type Current
impl<T> core::ops::Mul<&MagneticFlux<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Current{A: self.per_H.clone() * rhs.Wb.clone()}
	}
}

// InverseInductance * Resistance -> Frequency
/// Multiplying a InverseInductance by a Resistance returns a value of type Frequency
impl<T> core::ops::Mul<Resistance<T>> for InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_H * rhs.Ohm}
	}
}
/// Multiplying a InverseInductance by a Resistance returns a value of type Frequency
impl<T> core::ops::Mul<Resistance<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_H.clone() * rhs.Ohm}
	}
}
/// Multiplying a InverseInductance by a Resistance returns a value of type Frequency
impl<T> core::ops::Mul<&Resistance<T>> for InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_H * rhs.Ohm.clone()}
	}
}
/// Multiplying a InverseInductance by a Resistance returns a value of type Frequency
impl<T> core::ops::Mul<&Resistance<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Frequency{Hz: self.per_H.clone() * rhs.Ohm.clone()}
	}
}

// InverseInductance / Frequency -> Conductance
/// Dividing a InverseInductance by a Frequency returns a value of type Conductance
impl<T> core::ops::Div<Frequency<T>> for InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Conductance{S: self.per_H / rhs.Hz}
	}
}
/// Dividing a InverseInductance by a Frequency returns a value of type Conductance
impl<T> core::ops::Div<Frequency<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Conductance{S: self.per_H.clone() / rhs.Hz}
	}
}
/// Dividing a InverseInductance by a Frequency returns a value of type Conductance
impl<T> core::ops::Div<&Frequency<T>> for InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Conductance{S: self.per_H / rhs.Hz.clone()}
	}
}
/// Dividing a InverseInductance by a Frequency returns a value of type Conductance
impl<T> core::ops::Div<&Frequency<T>> for &InverseInductance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Conductance{S: self.per_H.clone() / rhs.Hz.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<InverseInductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
impl<T> core::ops::Div<&InverseInductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseInductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseInductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseInductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseInductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseInductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseInductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseInductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseInductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

// 1/InverseInductance -> Inductance
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseInductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseInductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Inductance<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseInductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self) / rhs.per_H.clone()}
	}
}
/// Dividing a scalar value by a InverseInductance unit value returns a value of type Inductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseInductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Inductance<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		Inductance{H: T::from(self.clone()) / rhs.per_H.clone()}
	}
}

/// The inverse of luminous flux unit type, defined as inverse lumens in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseLuminousFlux<T: NumLike>{
	/// The value of this Inverse luminous flux in inverse lumens
	pub per_lm: T
}

impl<T> InverseLuminousFlux<T> where T: NumLike {

	/// Returns the standard unit name of inverse luminous flux: "inverse lumens"
	pub fn unit_name() -> &'static str { "inverse lumens" }
	
	/// Returns the abbreviated name or symbol of inverse luminous flux: "1/lm" for inverse lumens
	pub fn unit_symbol() -> &'static str { "1/lm" }
	
	/// Returns a new inverse luminous flux value from the given number of inverse lumens
	///
	/// # Arguments
	/// * `per_lm` - Any number-like type, representing a quantity of inverse lumens
	pub fn from_per_lm(per_lm: T) -> Self { InverseLuminousFlux{per_lm: per_lm} }
	
	/// Returns a copy of this inverse luminous flux value in inverse lumens
	pub fn to_per_lm(&self) -> T { self.per_lm.clone() }

	/// Returns a new inverse luminous flux value from the given number of inverse lumens
	///
	/// # Arguments
	/// * `per_lumens` - Any number-like type, representing a quantity of inverse lumens
	pub fn from_per_lumens(per_lumens: T) -> Self { InverseLuminousFlux{per_lm: per_lumens} }
	
	/// Returns a copy of this inverse luminous flux value in inverse lumens
	pub fn to_per_lumens(&self) -> T { self.per_lm.clone() }

}

impl<T> fmt::Display for InverseLuminousFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_lm, Self::unit_symbol())
	}
}

impl<T> InverseLuminousFlux<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse luminous flux value in inverse millilumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mlm(&self) -> T {
		return self.per_lm.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse luminous flux value from the given number of inverse millilumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mlm` - Any number-like type, representing a quantity of inverse millilumens
	pub fn from_per_mlm(per_mlm: T) -> Self {
		InverseLuminousFlux{per_lm: per_mlm * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse luminous flux value in inverse microlumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_ulm(&self) -> T {
		return self.per_lm.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse luminous flux value from the given number of inverse microlumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_ulm` - Any number-like type, representing a quantity of inverse microlumens
	pub fn from_per_ulm(per_ulm: T) -> Self {
		InverseLuminousFlux{per_lm: per_ulm * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse luminous flux value in inverse nanolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nlm(&self) -> T {
		return self.per_lm.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse luminous flux value from the given number of inverse nanolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nlm` - Any number-like type, representing a quantity of inverse nanolumens
	pub fn from_per_nlm(per_nlm: T) -> Self {
		InverseLuminousFlux{per_lm: per_nlm * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse luminous flux value in inverse kilolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_klm(&self) -> T {
		return self.per_lm.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse luminous flux value from the given number of inverse kilolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_klm` - Any number-like type, representing a quantity of inverse kilolumens
	pub fn from_per_klm(per_klm: T) -> Self {
		InverseLuminousFlux{per_lm: per_klm * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse luminous flux value in inverse megalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_Mlm(&self) -> T {
		return self.per_lm.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse luminous flux value from the given number of inverse megalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_Mlm` - Any number-like type, representing a quantity of inverse megalumens
	pub fn from_per_Mlm(per_Mlm: T) -> Self {
		InverseLuminousFlux{per_lm: per_Mlm * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse luminous flux value in inverse gigalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_Glm(&self) -> T {
		return self.per_lm.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse luminous flux value from the given number of inverse gigalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_Glm` - Any number-like type, representing a quantity of inverse gigalumens
	pub fn from_per_Glm(per_Glm: T) -> Self {
		InverseLuminousFlux{per_lm: per_Glm * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseLuminousFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseLuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseLuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminousFlux{per_lm: self * rhs.per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseLuminousFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseLuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseLuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.clone() * rhs.per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseLuminousFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseLuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseLuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminousFlux{per_lm: self * rhs.per_lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseLuminousFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseLuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseLuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.clone() * rhs.per_lm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminousFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseLuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: InverseLuminousFlux<num_complex::Complex32>) -> Self::Output {
		InverseLuminousFlux{per_lm: self * rhs.per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminousFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseLuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: InverseLuminousFlux<num_complex::Complex32>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.clone() * rhs.per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminousFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseLuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: &InverseLuminousFlux<num_complex::Complex32>) -> Self::Output {
		InverseLuminousFlux{per_lm: self * rhs.per_lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminousFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseLuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: &InverseLuminousFlux<num_complex::Complex32>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.clone() * rhs.per_lm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminousFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseLuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: InverseLuminousFlux<num_complex::Complex64>) -> Self::Output {
		InverseLuminousFlux{per_lm: self * rhs.per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseLuminousFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseLuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: InverseLuminousFlux<num_complex::Complex64>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.clone() * rhs.per_lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminousFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseLuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: &InverseLuminousFlux<num_complex::Complex64>) -> Self::Output {
		InverseLuminousFlux{per_lm: self * rhs.per_lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseLuminousFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseLuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: &InverseLuminousFlux<num_complex::Complex64>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.clone() * rhs.per_lm.clone()}
	}
}




// InverseLuminousFlux / InverseLuminosity -> InverseSolidAngle
/// Dividing a InverseLuminousFlux by a InverseLuminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Div<InverseLuminosity<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm / rhs.per_cd}
	}
}
/// Dividing a InverseLuminousFlux by a InverseLuminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Div<InverseLuminosity<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm.clone() / rhs.per_cd}
	}
}
/// Dividing a InverseLuminousFlux by a InverseLuminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&InverseLuminosity<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm / rhs.per_cd.clone()}
	}
}
/// Dividing a InverseLuminousFlux by a InverseLuminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&InverseLuminosity<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm.clone() / rhs.per_cd.clone()}
	}
}

// InverseLuminousFlux * Luminosity -> InverseSolidAngle
/// Multiplying a InverseLuminousFlux by a Luminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<Luminosity<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: Luminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm * rhs.cd}
	}
}
/// Multiplying a InverseLuminousFlux by a Luminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<Luminosity<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: Luminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm.clone() * rhs.cd}
	}
}
/// Multiplying a InverseLuminousFlux by a Luminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<&Luminosity<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm * rhs.cd.clone()}
	}
}
/// Multiplying a InverseLuminousFlux by a Luminosity returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<&Luminosity<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_lm.clone() * rhs.cd.clone()}
	}
}

// InverseLuminousFlux / AreaPerLumen -> InverseArea
/// Dividing a InverseLuminousFlux by a AreaPerLumen returns a value of type InverseArea
impl<T> core::ops::Div<AreaPerLumen<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm / rhs.m2_per_lm}
	}
}
/// Dividing a InverseLuminousFlux by a AreaPerLumen returns a value of type InverseArea
impl<T> core::ops::Div<AreaPerLumen<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm.clone() / rhs.m2_per_lm}
	}
}
/// Dividing a InverseLuminousFlux by a AreaPerLumen returns a value of type InverseArea
impl<T> core::ops::Div<&AreaPerLumen<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a InverseLuminousFlux by a AreaPerLumen returns a value of type InverseArea
impl<T> core::ops::Div<&AreaPerLumen<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm.clone() / rhs.m2_per_lm.clone()}
	}
}

// InverseLuminousFlux * Illuminance -> InverseArea
/// Multiplying a InverseLuminousFlux by a Illuminance returns a value of type InverseArea
impl<T> core::ops::Mul<Illuminance<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: Illuminance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm * rhs.lux}
	}
}
/// Multiplying a InverseLuminousFlux by a Illuminance returns a value of type InverseArea
impl<T> core::ops::Mul<Illuminance<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: Illuminance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm.clone() * rhs.lux}
	}
}
/// Multiplying a InverseLuminousFlux by a Illuminance returns a value of type InverseArea
impl<T> core::ops::Mul<&Illuminance<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &Illuminance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm * rhs.lux.clone()}
	}
}
/// Multiplying a InverseLuminousFlux by a Illuminance returns a value of type InverseArea
impl<T> core::ops::Mul<&Illuminance<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &Illuminance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_lm.clone() * rhs.lux.clone()}
	}
}

// InverseLuminousFlux * Area -> AreaPerLumen
/// Multiplying a InverseLuminousFlux by a Area returns a value of type AreaPerLumen
impl<T> core::ops::Mul<Area<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm * rhs.m2}
	}
}
/// Multiplying a InverseLuminousFlux by a Area returns a value of type AreaPerLumen
impl<T> core::ops::Mul<Area<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm.clone() * rhs.m2}
	}
}
/// Multiplying a InverseLuminousFlux by a Area returns a value of type AreaPerLumen
impl<T> core::ops::Mul<&Area<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm * rhs.m2.clone()}
	}
}
/// Multiplying a InverseLuminousFlux by a Area returns a value of type AreaPerLumen
impl<T> core::ops::Mul<&Area<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm.clone() * rhs.m2.clone()}
	}
}

// InverseLuminousFlux / InverseArea -> AreaPerLumen
/// Dividing a InverseLuminousFlux by a InverseArea returns a value of type AreaPerLumen
impl<T> core::ops::Div<InverseArea<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm / rhs.per_m2}
	}
}
/// Dividing a InverseLuminousFlux by a InverseArea returns a value of type AreaPerLumen
impl<T> core::ops::Div<InverseArea<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm.clone() / rhs.per_m2}
	}
}
/// Dividing a InverseLuminousFlux by a InverseArea returns a value of type AreaPerLumen
impl<T> core::ops::Div<&InverseArea<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm / rhs.per_m2.clone()}
	}
}
/// Dividing a InverseLuminousFlux by a InverseArea returns a value of type AreaPerLumen
impl<T> core::ops::Div<&InverseArea<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.per_lm.clone() / rhs.per_m2.clone()}
	}
}

// InverseLuminousFlux / InverseSolidAngle -> InverseLuminosity
/// Dividing a InverseLuminousFlux by a InverseSolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Div<InverseSolidAngle<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm / rhs.per_sr}
	}
}
/// Dividing a InverseLuminousFlux by a InverseSolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Div<InverseSolidAngle<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm.clone() / rhs.per_sr}
	}
}
/// Dividing a InverseLuminousFlux by a InverseSolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Div<&InverseSolidAngle<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm / rhs.per_sr.clone()}
	}
}
/// Dividing a InverseLuminousFlux by a InverseSolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm.clone() / rhs.per_sr.clone()}
	}
}

// InverseLuminousFlux * SolidAngle -> InverseLuminosity
/// Multiplying a InverseLuminousFlux by a SolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Mul<SolidAngle<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm * rhs.sr}
	}
}
/// Multiplying a InverseLuminousFlux by a SolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Mul<SolidAngle<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm.clone() * rhs.sr}
	}
}
/// Multiplying a InverseLuminousFlux by a SolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Mul<&SolidAngle<T>> for InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm * rhs.sr.clone()}
	}
}
/// Multiplying a InverseLuminousFlux by a SolidAngle returns a value of type InverseLuminosity
impl<T> core::ops::Mul<&SolidAngle<T>> for &InverseLuminousFlux<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.per_lm.clone() * rhs.sr.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseLuminousFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminousFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

// 1/InverseLuminousFlux -> LuminousFlux
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminousFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self) / rhs.per_lm.clone()}
	}
}
/// Dividing a scalar value by a InverseLuminousFlux unit value returns a value of type LuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		LuminousFlux{lm: T::from(self.clone()) / rhs.per_lm.clone()}
	}
}

/// The inverse of magnetic flux unit type, defined as inverse webers in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseMagneticFlux<T: NumLike>{
	/// The value of this Inverse magnetic flux in inverse webers
	pub per_Wb: T
}

impl<T> InverseMagneticFlux<T> where T: NumLike {

	/// Returns the standard unit name of inverse magnetic flux: "inverse webers"
	pub fn unit_name() -> &'static str { "inverse webers" }
	
	/// Returns the abbreviated name or symbol of inverse magnetic flux: "1/Wb" for inverse webers
	pub fn unit_symbol() -> &'static str { "1/Wb" }
	
	/// Returns a new inverse magnetic flux value from the given number of inverse webers
	///
	/// # Arguments
	/// * `per_Wb` - Any number-like type, representing a quantity of inverse webers
	pub fn from_per_Wb(per_Wb: T) -> Self { InverseMagneticFlux{per_Wb: per_Wb} }
	
	/// Returns a copy of this inverse magnetic flux value in inverse webers
	pub fn to_per_Wb(&self) -> T { self.per_Wb.clone() }

	/// Returns a new inverse magnetic flux value from the given number of inverse webers
	///
	/// # Arguments
	/// * `per_weber` - Any number-like type, representing a quantity of inverse webers
	pub fn from_per_weber(per_weber: T) -> Self { InverseMagneticFlux{per_Wb: per_weber} }
	
	/// Returns a copy of this inverse magnetic flux value in inverse webers
	pub fn to_per_weber(&self) -> T { self.per_Wb.clone() }

}

impl<T> fmt::Display for InverseMagneticFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_Wb, Self::unit_symbol())
	}
}

impl<T> InverseMagneticFlux<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse magnetic flux value in inverse milliwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mWb(&self) -> T {
		return self.per_Wb.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse magnetic flux value from the given number of inverse milliwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mWb` - Any number-like type, representing a quantity of inverse milliwebers
	pub fn from_per_mWb(per_mWb: T) -> Self {
		InverseMagneticFlux{per_Wb: per_mWb * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse magnetic flux value in inverse microwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uWb(&self) -> T {
		return self.per_Wb.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse magnetic flux value from the given number of inverse microwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uWb` - Any number-like type, representing a quantity of inverse microwebers
	pub fn from_per_uWb(per_uWb: T) -> Self {
		InverseMagneticFlux{per_Wb: per_uWb * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse magnetic flux value in inverse nanowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nWb(&self) -> T {
		return self.per_Wb.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse magnetic flux value from the given number of inverse nanowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nWb` - Any number-like type, representing a quantity of inverse nanowebers
	pub fn from_per_nWb(per_nWb: T) -> Self {
		InverseMagneticFlux{per_Wb: per_nWb * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse magnetic flux value in inverse kilowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kWb(&self) -> T {
		return self.per_Wb.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse magnetic flux value from the given number of inverse kilowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kWb` - Any number-like type, representing a quantity of inverse kilowebers
	pub fn from_per_kWb(per_kWb: T) -> Self {
		InverseMagneticFlux{per_Wb: per_kWb * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse magnetic flux value in inverse megawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_MWb(&self) -> T {
		return self.per_Wb.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse magnetic flux value from the given number of inverse megawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_MWb` - Any number-like type, representing a quantity of inverse megawebers
	pub fn from_per_MWb(per_MWb: T) -> Self {
		InverseMagneticFlux{per_Wb: per_MWb * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse magnetic flux value in inverse gigawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GWb(&self) -> T {
		return self.per_Wb.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse magnetic flux value from the given number of inverse gigawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GWb` - Any number-like type, representing a quantity of inverse gigawebers
	pub fn from_per_GWb(per_GWb: T) -> Self {
		InverseMagneticFlux{per_Wb: per_GWb * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseMagneticFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseMagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseMagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self * rhs.per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseMagneticFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseMagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseMagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.clone() * rhs.per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseMagneticFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseMagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseMagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self * rhs.per_Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseMagneticFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseMagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseMagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.clone() * rhs.per_Wb.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseMagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: InverseMagneticFlux<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self * rhs.per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseMagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: InverseMagneticFlux<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.clone() * rhs.per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseMagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: &InverseMagneticFlux<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self * rhs.per_Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseMagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: &InverseMagneticFlux<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.clone() * rhs.per_Wb.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseMagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: InverseMagneticFlux<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self * rhs.per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseMagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: InverseMagneticFlux<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.clone() * rhs.per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseMagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: &InverseMagneticFlux<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self * rhs.per_Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseMagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: &InverseMagneticFlux<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.clone() * rhs.per_Wb.clone()}
	}
}




// InverseMagneticFlux * Current -> InverseInductance
/// Multiplying a InverseMagneticFlux by a Current returns a value of type InverseInductance
impl<T> core::ops::Mul<Current<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb * rhs.A}
	}
}
/// Multiplying a InverseMagneticFlux by a Current returns a value of type InverseInductance
impl<T> core::ops::Mul<Current<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb.clone() * rhs.A}
	}
}
/// Multiplying a InverseMagneticFlux by a Current returns a value of type InverseInductance
impl<T> core::ops::Mul<&Current<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb * rhs.A.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Current returns a value of type InverseInductance
impl<T> core::ops::Mul<&Current<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb.clone() * rhs.A.clone()}
	}
}

// InverseMagneticFlux / Current -> InverseEnergy
/// Dividing a InverseMagneticFlux by a Current returns a value of type InverseEnergy
impl<T> core::ops::Div<Current<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb / rhs.A}
	}
}
/// Dividing a InverseMagneticFlux by a Current returns a value of type InverseEnergy
impl<T> core::ops::Div<Current<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb.clone() / rhs.A}
	}
}
/// Dividing a InverseMagneticFlux by a Current returns a value of type InverseEnergy
impl<T> core::ops::Div<&Current<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb / rhs.A.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a Current returns a value of type InverseEnergy
impl<T> core::ops::Div<&Current<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb.clone() / rhs.A.clone()}
	}
}

// InverseMagneticFlux * InverseCurrent -> InverseEnergy
/// Multiplying a InverseMagneticFlux by a InverseCurrent returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseCurrent<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb * rhs.per_A}
	}
}
/// Multiplying a InverseMagneticFlux by a InverseCurrent returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseCurrent<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb.clone() * rhs.per_A}
	}
}
/// Multiplying a InverseMagneticFlux by a InverseCurrent returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseCurrent<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb * rhs.per_A.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a InverseCurrent returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseCurrent<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_Wb.clone() * rhs.per_A.clone()}
	}
}

// InverseMagneticFlux / InverseCurrent -> InverseInductance
/// Dividing a InverseMagneticFlux by a InverseCurrent returns a value of type InverseInductance
impl<T> core::ops::Div<InverseCurrent<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb / rhs.per_A}
	}
}
/// Dividing a InverseMagneticFlux by a InverseCurrent returns a value of type InverseInductance
impl<T> core::ops::Div<InverseCurrent<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb.clone() / rhs.per_A}
	}
}
/// Dividing a InverseMagneticFlux by a InverseCurrent returns a value of type InverseInductance
impl<T> core::ops::Div<&InverseCurrent<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb / rhs.per_A.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseCurrent returns a value of type InverseInductance
impl<T> core::ops::Div<&InverseCurrent<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseInductance<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InverseInductance{per_H: self.per_Wb.clone() / rhs.per_A.clone()}
	}
}

// InverseMagneticFlux * Time -> InverseVoltage
/// Multiplying a InverseMagneticFlux by a Time returns a value of type InverseVoltage
impl<T> core::ops::Mul<Time<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb * rhs.s}
	}
}
/// Multiplying a InverseMagneticFlux by a Time returns a value of type InverseVoltage
impl<T> core::ops::Mul<Time<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb.clone() * rhs.s}
	}
}
/// Multiplying a InverseMagneticFlux by a Time returns a value of type InverseVoltage
impl<T> core::ops::Mul<&Time<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb * rhs.s.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Time returns a value of type InverseVoltage
impl<T> core::ops::Mul<&Time<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb.clone() * rhs.s.clone()}
	}
}

// InverseMagneticFlux * Charge -> Conductance
/// Multiplying a InverseMagneticFlux by a Charge returns a value of type Conductance
impl<T> core::ops::Mul<Charge<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Conductance{S: self.per_Wb * rhs.C}
	}
}
/// Multiplying a InverseMagneticFlux by a Charge returns a value of type Conductance
impl<T> core::ops::Mul<Charge<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Conductance{S: self.per_Wb.clone() * rhs.C}
	}
}
/// Multiplying a InverseMagneticFlux by a Charge returns a value of type Conductance
impl<T> core::ops::Mul<&Charge<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Conductance{S: self.per_Wb * rhs.C.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Charge returns a value of type Conductance
impl<T> core::ops::Mul<&Charge<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Conductance{S: self.per_Wb.clone() * rhs.C.clone()}
	}
}

// InverseMagneticFlux / Conductance -> InverseCharge
/// Dividing a InverseMagneticFlux by a Conductance returns a value of type InverseCharge
impl<T> core::ops::Div<Conductance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb / rhs.S}
	}
}
/// Dividing a InverseMagneticFlux by a Conductance returns a value of type InverseCharge
impl<T> core::ops::Div<Conductance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb.clone() / rhs.S}
	}
}
/// Dividing a InverseMagneticFlux by a Conductance returns a value of type InverseCharge
impl<T> core::ops::Div<&Conductance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb / rhs.S.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a Conductance returns a value of type InverseCharge
impl<T> core::ops::Div<&Conductance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb.clone() / rhs.S.clone()}
	}
}

// InverseMagneticFlux * Inductance -> InverseCurrent
/// Multiplying a InverseMagneticFlux by a Inductance returns a value of type InverseCurrent
impl<T> core::ops::Mul<Inductance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb * rhs.H}
	}
}
/// Multiplying a InverseMagneticFlux by a Inductance returns a value of type InverseCurrent
impl<T> core::ops::Mul<Inductance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb.clone() * rhs.H}
	}
}
/// Multiplying a InverseMagneticFlux by a Inductance returns a value of type InverseCurrent
impl<T> core::ops::Mul<&Inductance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb * rhs.H.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Inductance returns a value of type InverseCurrent
impl<T> core::ops::Mul<&Inductance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb.clone() * rhs.H.clone()}
	}
}

// InverseMagneticFlux / InverseCharge -> Conductance
/// Dividing a InverseMagneticFlux by a InverseCharge returns a value of type Conductance
impl<T> core::ops::Div<InverseCharge<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Conductance{S: self.per_Wb / rhs.per_C}
	}
}
/// Dividing a InverseMagneticFlux by a InverseCharge returns a value of type Conductance
impl<T> core::ops::Div<InverseCharge<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Conductance{S: self.per_Wb.clone() / rhs.per_C}
	}
}
/// Dividing a InverseMagneticFlux by a InverseCharge returns a value of type Conductance
impl<T> core::ops::Div<&InverseCharge<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Conductance{S: self.per_Wb / rhs.per_C.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseCharge returns a value of type Conductance
impl<T> core::ops::Div<&InverseCharge<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Conductance{S: self.per_Wb.clone() / rhs.per_C.clone()}
	}
}

// InverseMagneticFlux / InverseInductance -> InverseCurrent
/// Dividing a InverseMagneticFlux by a InverseInductance returns a value of type InverseCurrent
impl<T> core::ops::Div<InverseInductance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb / rhs.per_H}
	}
}
/// Dividing a InverseMagneticFlux by a InverseInductance returns a value of type InverseCurrent
impl<T> core::ops::Div<InverseInductance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: InverseInductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb.clone() / rhs.per_H}
	}
}
/// Dividing a InverseMagneticFlux by a InverseInductance returns a value of type InverseCurrent
impl<T> core::ops::Div<&InverseInductance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb / rhs.per_H.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseInductance returns a value of type InverseCurrent
impl<T> core::ops::Div<&InverseInductance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &InverseInductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_Wb.clone() / rhs.per_H.clone()}
	}
}

// InverseMagneticFlux / InverseMagneticFluxDensity -> InverseArea
/// Dividing a InverseMagneticFlux by a InverseMagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb / rhs.m2_per_Wb}
	}
}
/// Dividing a InverseMagneticFlux by a InverseMagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb.clone() / rhs.m2_per_Wb}
	}
}
/// Dividing a InverseMagneticFlux by a InverseMagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseMagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb.clone() / rhs.m2_per_Wb.clone()}
	}
}

// InverseMagneticFlux / InverseVoltage -> Frequency
/// Dividing a InverseMagneticFlux by a InverseVoltage returns a value of type Frequency
impl<T> core::ops::Div<InverseVoltage<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb / rhs.per_V}
	}
}
/// Dividing a InverseMagneticFlux by a InverseVoltage returns a value of type Frequency
impl<T> core::ops::Div<InverseVoltage<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb.clone() / rhs.per_V}
	}
}
/// Dividing a InverseMagneticFlux by a InverseVoltage returns a value of type Frequency
impl<T> core::ops::Div<&InverseVoltage<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb / rhs.per_V.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseVoltage returns a value of type Frequency
impl<T> core::ops::Div<&InverseVoltage<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb.clone() / rhs.per_V.clone()}
	}
}

// InverseMagneticFlux * MagneticFluxDensity -> InverseArea
/// Multiplying a InverseMagneticFlux by a MagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Mul<MagneticFluxDensity<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb * rhs.T}
	}
}
/// Multiplying a InverseMagneticFlux by a MagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Mul<MagneticFluxDensity<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb.clone() * rhs.T}
	}
}
/// Multiplying a InverseMagneticFlux by a MagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Mul<&MagneticFluxDensity<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb * rhs.T.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a MagneticFluxDensity returns a value of type InverseArea
impl<T> core::ops::Mul<&MagneticFluxDensity<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseArea{per_m2: self.per_Wb.clone() * rhs.T.clone()}
	}
}

// InverseMagneticFlux * Resistance -> InverseCharge
/// Multiplying a InverseMagneticFlux by a Resistance returns a value of type InverseCharge
impl<T> core::ops::Mul<Resistance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb * rhs.Ohm}
	}
}
/// Multiplying a InverseMagneticFlux by a Resistance returns a value of type InverseCharge
impl<T> core::ops::Mul<Resistance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb.clone() * rhs.Ohm}
	}
}
/// Multiplying a InverseMagneticFlux by a Resistance returns a value of type InverseCharge
impl<T> core::ops::Mul<&Resistance<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb * rhs.Ohm.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Resistance returns a value of type InverseCharge
impl<T> core::ops::Mul<&Resistance<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_Wb.clone() * rhs.Ohm.clone()}
	}
}

// InverseMagneticFlux * Voltage -> Frequency
/// Multiplying a InverseMagneticFlux by a Voltage returns a value of type Frequency
impl<T> core::ops::Mul<Voltage<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb * rhs.V}
	}
}
/// Multiplying a InverseMagneticFlux by a Voltage returns a value of type Frequency
impl<T> core::ops::Mul<Voltage<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb.clone() * rhs.V}
	}
}
/// Multiplying a InverseMagneticFlux by a Voltage returns a value of type Frequency
impl<T> core::ops::Mul<&Voltage<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb * rhs.V.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Voltage returns a value of type Frequency
impl<T> core::ops::Mul<&Voltage<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Frequency{Hz: self.per_Wb.clone() * rhs.V.clone()}
	}
}

// InverseMagneticFlux * Area -> InverseMagneticFluxDensity
/// Multiplying a InverseMagneticFlux by a Area returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<Area<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb * rhs.m2}
	}
}
/// Multiplying a InverseMagneticFlux by a Area returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<Area<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb.clone() * rhs.m2}
	}
}
/// Multiplying a InverseMagneticFlux by a Area returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<&Area<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb * rhs.m2.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Area returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<&Area<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb.clone() * rhs.m2.clone()}
	}
}

// InverseMagneticFlux / InverseArea -> InverseMagneticFluxDensity
/// Dividing a InverseMagneticFlux by a InverseArea returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<InverseArea<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb / rhs.per_m2}
	}
}
/// Dividing a InverseMagneticFlux by a InverseArea returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<InverseArea<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb.clone() / rhs.per_m2}
	}
}
/// Dividing a InverseMagneticFlux by a InverseArea returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&InverseArea<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb / rhs.per_m2.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseArea returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&InverseArea<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.per_Wb.clone() / rhs.per_m2.clone()}
	}
}

// InverseMagneticFlux * Energy -> Current
/// Multiplying a InverseMagneticFlux by a Energy returns a value of type Current
impl<T> core::ops::Mul<Energy<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Current{A: self.per_Wb * rhs.J}
	}
}
/// Multiplying a InverseMagneticFlux by a Energy returns a value of type Current
impl<T> core::ops::Mul<Energy<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() * rhs.J}
	}
}
/// Multiplying a InverseMagneticFlux by a Energy returns a value of type Current
impl<T> core::ops::Mul<&Energy<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Current{A: self.per_Wb * rhs.J.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Energy returns a value of type Current
impl<T> core::ops::Mul<&Energy<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() * rhs.J.clone()}
	}
}

// InverseMagneticFlux * Torque -> Current
/// Multiplying a InverseMagneticFlux by a Torque returns a value of type Current
impl<T> core::ops::Mul<Torque<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Current{A: self.per_Wb * rhs.Nm}
	}
}
/// Multiplying a InverseMagneticFlux by a Torque returns a value of type Current
impl<T> core::ops::Mul<Torque<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() * rhs.Nm}
	}
}
/// Multiplying a InverseMagneticFlux by a Torque returns a value of type Current
impl<T> core::ops::Mul<&Torque<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Current{A: self.per_Wb * rhs.Nm.clone()}
	}
}
/// Multiplying a InverseMagneticFlux by a Torque returns a value of type Current
impl<T> core::ops::Mul<&Torque<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() * rhs.Nm.clone()}
	}
}

// InverseMagneticFlux / Frequency -> InverseVoltage
/// Dividing a InverseMagneticFlux by a Frequency returns a value of type InverseVoltage
impl<T> core::ops::Div<Frequency<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb / rhs.Hz}
	}
}
/// Dividing a InverseMagneticFlux by a Frequency returns a value of type InverseVoltage
impl<T> core::ops::Div<Frequency<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb.clone() / rhs.Hz}
	}
}
/// Dividing a InverseMagneticFlux by a Frequency returns a value of type InverseVoltage
impl<T> core::ops::Div<&Frequency<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb / rhs.Hz.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a Frequency returns a value of type InverseVoltage
impl<T> core::ops::Div<&Frequency<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseVoltage{per_V: self.per_Wb.clone() / rhs.Hz.clone()}
	}
}

// InverseMagneticFlux / InverseEnergy -> Current
/// Dividing a InverseMagneticFlux by a InverseEnergy returns a value of type Current
impl<T> core::ops::Div<InverseEnergy<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Current{A: self.per_Wb / rhs.per_J}
	}
}
/// Dividing a InverseMagneticFlux by a InverseEnergy returns a value of type Current
impl<T> core::ops::Div<InverseEnergy<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() / rhs.per_J}
	}
}
/// Dividing a InverseMagneticFlux by a InverseEnergy returns a value of type Current
impl<T> core::ops::Div<&InverseEnergy<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Current{A: self.per_Wb / rhs.per_J.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseEnergy returns a value of type Current
impl<T> core::ops::Div<&InverseEnergy<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() / rhs.per_J.clone()}
	}
}

// InverseMagneticFlux / InverseTorque -> Current
/// Dividing a InverseMagneticFlux by a InverseTorque returns a value of type Current
impl<T> core::ops::Div<InverseTorque<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Current{A: self.per_Wb / rhs.per_Nm}
	}
}
/// Dividing a InverseMagneticFlux by a InverseTorque returns a value of type Current
impl<T> core::ops::Div<InverseTorque<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() / rhs.per_Nm}
	}
}
/// Dividing a InverseMagneticFlux by a InverseTorque returns a value of type Current
impl<T> core::ops::Div<&InverseTorque<T>> for InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Current{A: self.per_Wb / rhs.per_Nm.clone()}
	}
}
/// Dividing a InverseMagneticFlux by a InverseTorque returns a value of type Current
impl<T> core::ops::Div<&InverseTorque<T>> for &InverseMagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Current{A: self.per_Wb.clone() / rhs.per_Nm.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseMagneticFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

// 1/InverseMagneticFlux -> MagneticFlux
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self) / rhs.per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFlux unit value returns a value of type MagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFlux{Wb: T::from(self.clone()) / rhs.per_Wb.clone()}
	}
}

/// The inverse of magnetic flux density unit type, defined as square meters per weber in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseMagneticFluxDensity<T: NumLike>{
	/// The value of this Inverse magnetic flux density in square meters per weber
	pub m2_per_Wb: T
}

impl<T> InverseMagneticFluxDensity<T> where T: NumLike {

	/// Returns the standard unit name of inverse magnetic flux density: "square meters per weber"
	pub fn unit_name() -> &'static str { "square meters per weber" }
	
	/// Returns the abbreviated name or symbol of inverse magnetic flux density: "m²/Wb" for square meters per weber
	pub fn unit_symbol() -> &'static str { "m²/Wb" }
	
	/// Returns a new inverse magnetic flux density value from the given number of square meters per weber
	///
	/// # Arguments
	/// * `m2_per_Wb` - Any number-like type, representing a quantity of square meters per weber
	pub fn from_m2_per_Wb(m2_per_Wb: T) -> Self { InverseMagneticFluxDensity{m2_per_Wb: m2_per_Wb} }
	
	/// Returns a copy of this inverse magnetic flux density value in square meters per weber
	pub fn to_m2_per_Wb(&self) -> T { self.m2_per_Wb.clone() }

	/// Returns a new inverse magnetic flux density value from the given number of square meters per weber
	///
	/// # Arguments
	/// * `square_meters_per_weber` - Any number-like type, representing a quantity of square meters per weber
	pub fn from_square_meters_per_weber(square_meters_per_weber: T) -> Self { InverseMagneticFluxDensity{m2_per_Wb: square_meters_per_weber} }
	
	/// Returns a copy of this inverse magnetic flux density value in square meters per weber
	pub fn to_square_meters_per_weber(&self) -> T { self.m2_per_Wb.clone() }

	/// Returns a new inverse magnetic flux density value from the given number of inverse teslas
	///
	/// # Arguments
	/// * `per_T` - Any number-like type, representing a quantity of square meters per weber
	pub fn from_per_T(per_T: T) -> Self { InverseMagneticFluxDensity{m2_per_Wb: per_T} }
	
	/// Returns a copy of this inverse magnetic flux density value in inverse teslas
	pub fn to_per_T(&self) -> T { self.m2_per_Wb.clone() }

	/// Returns a new inverse magnetic flux density value from the given number of inverse teslas
	///
	/// # Arguments
	/// * `per_tesla` - Any number-like type, representing a quantity of square meters per weber
	pub fn from_per_tesla(per_tesla: T) -> Self { InverseMagneticFluxDensity{m2_per_Wb: per_tesla} }
	
	/// Returns a copy of this inverse magnetic flux density value in inverse teslas
	pub fn to_per_tesla(&self) -> T { self.m2_per_Wb.clone() }

}

impl<T> fmt::Display for InverseMagneticFluxDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m2_per_Wb, Self::unit_symbol())
	}
}

impl<T> InverseMagneticFluxDensity<T> where T: NumLike+From<f64> {
	
}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseMagneticFluxDensity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseMagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseMagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self * rhs.m2_per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseMagneticFluxDensity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseMagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseMagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.clone() * rhs.m2_per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseMagneticFluxDensity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseMagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self * rhs.m2_per_Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseMagneticFluxDensity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseMagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.clone() * rhs.m2_per_Wb.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFluxDensity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: InverseMagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self * rhs.m2_per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFluxDensity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: InverseMagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.clone() * rhs.m2_per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFluxDensity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self * rhs.m2_per_Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFluxDensity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.clone() * rhs.m2_per_Wb.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFluxDensity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: InverseMagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self * rhs.m2_per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseMagneticFluxDensity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: InverseMagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.clone() * rhs.m2_per_Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFluxDensity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self * rhs.m2_per_Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseMagneticFluxDensity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseMagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.clone() * rhs.m2_per_Wb.clone()}
	}
}




// InverseMagneticFluxDensity / InverseMagneticFlux -> Area
/// Dividing a InverseMagneticFluxDensity by a InverseMagneticFlux returns a value of type Area
impl<T> core::ops::Div<InverseMagneticFlux<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb / rhs.per_Wb}
	}
}
/// Dividing a InverseMagneticFluxDensity by a InverseMagneticFlux returns a value of type Area
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb.clone() / rhs.per_Wb}
	}
}
/// Dividing a InverseMagneticFluxDensity by a InverseMagneticFlux returns a value of type Area
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb / rhs.per_Wb.clone()}
	}
}
/// Dividing a InverseMagneticFluxDensity by a InverseMagneticFlux returns a value of type Area
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb.clone() / rhs.per_Wb.clone()}
	}
}

// InverseMagneticFluxDensity * MagneticFlux -> Area
/// Multiplying a InverseMagneticFluxDensity by a MagneticFlux returns a value of type Area
impl<T> core::ops::Mul<MagneticFlux<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb * rhs.Wb}
	}
}
/// Multiplying a InverseMagneticFluxDensity by a MagneticFlux returns a value of type Area
impl<T> core::ops::Mul<MagneticFlux<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb.clone() * rhs.Wb}
	}
}
/// Multiplying a InverseMagneticFluxDensity by a MagneticFlux returns a value of type Area
impl<T> core::ops::Mul<&MagneticFlux<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb * rhs.Wb.clone()}
	}
}
/// Multiplying a InverseMagneticFluxDensity by a MagneticFlux returns a value of type Area
impl<T> core::ops::Mul<&MagneticFlux<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Area{m2: self.m2_per_Wb.clone() * rhs.Wb.clone()}
	}
}

// InverseMagneticFluxDensity / Area -> InverseMagneticFlux
/// Dividing a InverseMagneticFluxDensity by a Area returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Area<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb / rhs.m2}
	}
}
/// Dividing a InverseMagneticFluxDensity by a Area returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Area<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb.clone() / rhs.m2}
	}
}
/// Dividing a InverseMagneticFluxDensity by a Area returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Area<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb / rhs.m2.clone()}
	}
}
/// Dividing a InverseMagneticFluxDensity by a Area returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Area<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb.clone() / rhs.m2.clone()}
	}
}

// InverseMagneticFluxDensity * InverseArea -> InverseMagneticFlux
/// Multiplying a InverseMagneticFluxDensity by a InverseArea returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseArea<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb * rhs.per_m2}
	}
}
/// Multiplying a InverseMagneticFluxDensity by a InverseArea returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseArea<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb.clone() * rhs.per_m2}
	}
}
/// Multiplying a InverseMagneticFluxDensity by a InverseArea returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseArea<T>> for InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb * rhs.per_m2.clone()}
	}
}
/// Multiplying a InverseMagneticFluxDensity by a InverseArea returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseArea<T>> for &InverseMagneticFluxDensity<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.m2_per_Wb.clone() * rhs.per_m2.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for f64 where T: NumLike+From<f64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for f64 where T: NumLike+From<f64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for f32 where T: NumLike+From<f32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for f32 where T: NumLike+From<f32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for i64 where T: NumLike+From<i64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for i64 where T: NumLike+From<i64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for i32 where T: NumLike+From<i32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for i32 where T: NumLike+From<i32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

// 1/InverseMagneticFluxDensity -> MagneticFluxDensity
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self) / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a scalar value by a InverseMagneticFluxDensity unit value returns a value of type MagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFluxDensity{T: T::from(self.clone()) / rhs.m2_per_Wb.clone()}
	}
}

/// The inverse of voltage unit type, defined as inverse volts in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseVoltage<T: NumLike>{
	/// The value of this Inverse voltage in inverse volts
	pub per_V: T
}

impl<T> InverseVoltage<T> where T: NumLike {

	/// Returns the standard unit name of inverse voltage: "inverse volts"
	pub fn unit_name() -> &'static str { "inverse volts" }
	
	/// Returns the abbreviated name or symbol of inverse voltage: "1/V" for inverse volts
	pub fn unit_symbol() -> &'static str { "1/V" }
	
	/// Returns a new inverse voltage value from the given number of inverse volts
	///
	/// # Arguments
	/// * `per_V` - Any number-like type, representing a quantity of inverse volts
	pub fn from_per_V(per_V: T) -> Self { InverseVoltage{per_V: per_V} }
	
	/// Returns a copy of this inverse voltage value in inverse volts
	pub fn to_per_V(&self) -> T { self.per_V.clone() }

	/// Returns a new inverse voltage value from the given number of inverse volts
	///
	/// # Arguments
	/// * `per_volt` - Any number-like type, representing a quantity of inverse volts
	pub fn from_per_volt(per_volt: T) -> Self { InverseVoltage{per_V: per_volt} }
	
	/// Returns a copy of this inverse voltage value in inverse volts
	pub fn to_per_volt(&self) -> T { self.per_V.clone() }

}

impl<T> fmt::Display for InverseVoltage<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_V, Self::unit_symbol())
	}
}

impl<T> InverseVoltage<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse voltage value in inverse millivolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mV(&self) -> T {
		return self.per_V.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse voltage value from the given number of inverse millivolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mV` - Any number-like type, representing a quantity of inverse millivolts
	pub fn from_per_mV(per_mV: T) -> Self {
		InverseVoltage{per_V: per_mV * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse voltage value in inverse microvolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uV(&self) -> T {
		return self.per_V.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse voltage value from the given number of inverse microvolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uV` - Any number-like type, representing a quantity of inverse microvolts
	pub fn from_per_uV(per_uV: T) -> Self {
		InverseVoltage{per_V: per_uV * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse voltage value in inverse nanovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nV(&self) -> T {
		return self.per_V.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse voltage value from the given number of inverse nanovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nV` - Any number-like type, representing a quantity of inverse nanovolts
	pub fn from_per_nV(per_nV: T) -> Self {
		InverseVoltage{per_V: per_nV * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse voltage value in inverse kilovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_kV(&self) -> T {
		return self.per_V.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse voltage value from the given number of inverse kilovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_kV` - Any number-like type, representing a quantity of inverse kilovolts
	pub fn from_per_kV(per_kV: T) -> Self {
		InverseVoltage{per_V: per_kV * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse voltage value in inverse megavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_MV(&self) -> T {
		return self.per_V.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse voltage value from the given number of inverse megavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_MV` - Any number-like type, representing a quantity of inverse megavolts
	pub fn from_per_MV(per_MV: T) -> Self {
		InverseVoltage{per_V: per_MV * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inverse voltage value in inverse gigavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GV(&self) -> T {
		return self.per_V.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inverse voltage value from the given number of inverse gigavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GV` - Any number-like type, representing a quantity of inverse gigavolts
	pub fn from_per_GV(per_GV: T) -> Self {
		InverseVoltage{per_V: per_GV * T::from(1e-09_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseVoltage<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseVoltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseVoltage<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVoltage{per_V: self * rhs.per_V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseVoltage<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseVoltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseVoltage<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVoltage{per_V: self.clone() * rhs.per_V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseVoltage<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseVoltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseVoltage<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVoltage{per_V: self * rhs.per_V.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseVoltage<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseVoltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseVoltage<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVoltage{per_V: self.clone() * rhs.per_V.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVoltage<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseVoltage<num_complex::Complex32>;
	fn mul(self, rhs: InverseVoltage<num_complex::Complex32>) -> Self::Output {
		InverseVoltage{per_V: self * rhs.per_V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVoltage<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseVoltage<num_complex::Complex32>;
	fn mul(self, rhs: InverseVoltage<num_complex::Complex32>) -> Self::Output {
		InverseVoltage{per_V: self.clone() * rhs.per_V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVoltage<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseVoltage<num_complex::Complex32>;
	fn mul(self, rhs: &InverseVoltage<num_complex::Complex32>) -> Self::Output {
		InverseVoltage{per_V: self * rhs.per_V.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVoltage<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseVoltage<num_complex::Complex32>;
	fn mul(self, rhs: &InverseVoltage<num_complex::Complex32>) -> Self::Output {
		InverseVoltage{per_V: self.clone() * rhs.per_V.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVoltage<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseVoltage<num_complex::Complex64>;
	fn mul(self, rhs: InverseVoltage<num_complex::Complex64>) -> Self::Output {
		InverseVoltage{per_V: self * rhs.per_V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVoltage<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseVoltage<num_complex::Complex64>;
	fn mul(self, rhs: InverseVoltage<num_complex::Complex64>) -> Self::Output {
		InverseVoltage{per_V: self.clone() * rhs.per_V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVoltage<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseVoltage<num_complex::Complex64>;
	fn mul(self, rhs: &InverseVoltage<num_complex::Complex64>) -> Self::Output {
		InverseVoltage{per_V: self * rhs.per_V.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVoltage<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseVoltage<num_complex::Complex64>;
	fn mul(self, rhs: &InverseVoltage<num_complex::Complex64>) -> Self::Output {
		InverseVoltage{per_V: self.clone() * rhs.per_V.clone()}
	}
}




// InverseVoltage * Current -> Conductance
/// Multiplying a InverseVoltage by a Current returns a value of type Conductance
impl<T> core::ops::Mul<Current<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Conductance{S: self.per_V * rhs.A}
	}
}
/// Multiplying a InverseVoltage by a Current returns a value of type Conductance
impl<T> core::ops::Mul<Current<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Conductance{S: self.per_V.clone() * rhs.A}
	}
}
/// Multiplying a InverseVoltage by a Current returns a value of type Conductance
impl<T> core::ops::Mul<&Current<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Conductance{S: self.per_V * rhs.A.clone()}
	}
}
/// Multiplying a InverseVoltage by a Current returns a value of type Conductance
impl<T> core::ops::Mul<&Current<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Conductance{S: self.per_V.clone() * rhs.A.clone()}
	}
}

// InverseVoltage / Current -> InversePower
/// Dividing a InverseVoltage by a Current returns a value of type InversePower
impl<T> core::ops::Div<Current<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InversePower{per_W: self.per_V / rhs.A}
	}
}
/// Dividing a InverseVoltage by a Current returns a value of type InversePower
impl<T> core::ops::Div<Current<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		InversePower{per_W: self.per_V.clone() / rhs.A}
	}
}
/// Dividing a InverseVoltage by a Current returns a value of type InversePower
impl<T> core::ops::Div<&Current<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InversePower{per_W: self.per_V / rhs.A.clone()}
	}
}
/// Dividing a InverseVoltage by a Current returns a value of type InversePower
impl<T> core::ops::Div<&Current<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		InversePower{per_W: self.per_V.clone() / rhs.A.clone()}
	}
}

// InverseVoltage * InverseCurrent -> InversePower
/// Multiplying a InverseVoltage by a InverseCurrent returns a value of type InversePower
impl<T> core::ops::Mul<InverseCurrent<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InversePower{per_W: self.per_V * rhs.per_A}
	}
}
/// Multiplying a InverseVoltage by a InverseCurrent returns a value of type InversePower
impl<T> core::ops::Mul<InverseCurrent<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		InversePower{per_W: self.per_V.clone() * rhs.per_A}
	}
}
/// Multiplying a InverseVoltage by a InverseCurrent returns a value of type InversePower
impl<T> core::ops::Mul<&InverseCurrent<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InversePower{per_W: self.per_V * rhs.per_A.clone()}
	}
}
/// Multiplying a InverseVoltage by a InverseCurrent returns a value of type InversePower
impl<T> core::ops::Mul<&InverseCurrent<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InversePower<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		InversePower{per_W: self.per_V.clone() * rhs.per_A.clone()}
	}
}

// InverseVoltage / InverseCurrent -> Conductance
/// Dividing a InverseVoltage by a InverseCurrent returns a value of type Conductance
impl<T> core::ops::Div<InverseCurrent<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Conductance{S: self.per_V / rhs.per_A}
	}
}
/// Dividing a InverseVoltage by a InverseCurrent returns a value of type Conductance
impl<T> core::ops::Div<InverseCurrent<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Conductance{S: self.per_V.clone() / rhs.per_A}
	}
}
/// Dividing a InverseVoltage by a InverseCurrent returns a value of type Conductance
impl<T> core::ops::Div<&InverseCurrent<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Conductance{S: self.per_V / rhs.per_A.clone()}
	}
}
/// Dividing a InverseVoltage by a InverseCurrent returns a value of type Conductance
impl<T> core::ops::Div<&InverseCurrent<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Conductance{S: self.per_V.clone() / rhs.per_A.clone()}
	}
}

// InverseVoltage / Time -> InverseMagneticFlux
/// Dividing a InverseVoltage by a Time returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Time<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V / rhs.s}
	}
}
/// Dividing a InverseVoltage by a Time returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<Time<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V.clone() / rhs.s}
	}
}
/// Dividing a InverseVoltage by a Time returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Time<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V / rhs.s.clone()}
	}
}
/// Dividing a InverseVoltage by a Time returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&Time<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V.clone() / rhs.s.clone()}
	}
}

// InverseVoltage / Capacitance -> InverseCharge
/// Dividing a InverseVoltage by a Capacitance returns a value of type InverseCharge
impl<T> core::ops::Div<Capacitance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V / rhs.F}
	}
}
/// Dividing a InverseVoltage by a Capacitance returns a value of type InverseCharge
impl<T> core::ops::Div<Capacitance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V.clone() / rhs.F}
	}
}
/// Dividing a InverseVoltage by a Capacitance returns a value of type InverseCharge
impl<T> core::ops::Div<&Capacitance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V / rhs.F.clone()}
	}
}
/// Dividing a InverseVoltage by a Capacitance returns a value of type InverseCharge
impl<T> core::ops::Div<&Capacitance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V.clone() / rhs.F.clone()}
	}
}

// InverseVoltage * Charge -> Capacitance
/// Multiplying a InverseVoltage by a Charge returns a value of type Capacitance
impl<T> core::ops::Mul<Charge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Capacitance{F: self.per_V * rhs.C}
	}
}
/// Multiplying a InverseVoltage by a Charge returns a value of type Capacitance
impl<T> core::ops::Mul<Charge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Capacitance{F: self.per_V.clone() * rhs.C}
	}
}
/// Multiplying a InverseVoltage by a Charge returns a value of type Capacitance
impl<T> core::ops::Mul<&Charge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Capacitance{F: self.per_V * rhs.C.clone()}
	}
}
/// Multiplying a InverseVoltage by a Charge returns a value of type Capacitance
impl<T> core::ops::Mul<&Charge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Capacitance{F: self.per_V.clone() * rhs.C.clone()}
	}
}

// InverseVoltage / Charge -> InverseEnergy
/// Dividing a InverseVoltage by a Charge returns a value of type InverseEnergy
impl<T> core::ops::Div<Charge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V / rhs.C}
	}
}
/// Dividing a InverseVoltage by a Charge returns a value of type InverseEnergy
impl<T> core::ops::Div<Charge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V.clone() / rhs.C}
	}
}
/// Dividing a InverseVoltage by a Charge returns a value of type InverseEnergy
impl<T> core::ops::Div<&Charge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V / rhs.C.clone()}
	}
}
/// Dividing a InverseVoltage by a Charge returns a value of type InverseEnergy
impl<T> core::ops::Div<&Charge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V.clone() / rhs.C.clone()}
	}
}

// InverseVoltage / Conductance -> InverseCurrent
/// Dividing a InverseVoltage by a Conductance returns a value of type InverseCurrent
impl<T> core::ops::Div<Conductance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V / rhs.S}
	}
}
/// Dividing a InverseVoltage by a Conductance returns a value of type InverseCurrent
impl<T> core::ops::Div<Conductance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V.clone() / rhs.S}
	}
}
/// Dividing a InverseVoltage by a Conductance returns a value of type InverseCurrent
impl<T> core::ops::Div<&Conductance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V / rhs.S.clone()}
	}
}
/// Dividing a InverseVoltage by a Conductance returns a value of type InverseCurrent
impl<T> core::ops::Div<&Conductance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V.clone() / rhs.S.clone()}
	}
}

// InverseVoltage * Elastance -> InverseCharge
/// Multiplying a InverseVoltage by a Elastance returns a value of type InverseCharge
impl<T> core::ops::Mul<Elastance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V * rhs.per_F}
	}
}
/// Multiplying a InverseVoltage by a Elastance returns a value of type InverseCharge
impl<T> core::ops::Mul<Elastance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: Elastance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V.clone() * rhs.per_F}
	}
}
/// Multiplying a InverseVoltage by a Elastance returns a value of type InverseCharge
impl<T> core::ops::Mul<&Elastance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V * rhs.per_F.clone()}
	}
}
/// Multiplying a InverseVoltage by a Elastance returns a value of type InverseCharge
impl<T> core::ops::Mul<&Elastance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &Elastance<T>) -> Self::Output {
		InverseCharge{per_C: self.per_V.clone() * rhs.per_F.clone()}
	}
}

// InverseVoltage * InverseCharge -> InverseEnergy
/// Multiplying a InverseVoltage by a InverseCharge returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseCharge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V * rhs.per_C}
	}
}
/// Multiplying a InverseVoltage by a InverseCharge returns a value of type InverseEnergy
impl<T> core::ops::Mul<InverseCharge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V.clone() * rhs.per_C}
	}
}
/// Multiplying a InverseVoltage by a InverseCharge returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseCharge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V * rhs.per_C.clone()}
	}
}
/// Multiplying a InverseVoltage by a InverseCharge returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InverseCharge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_V.clone() * rhs.per_C.clone()}
	}
}

// InverseVoltage / InverseCharge -> Capacitance
/// Dividing a InverseVoltage by a InverseCharge returns a value of type Capacitance
impl<T> core::ops::Div<InverseCharge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Capacitance{F: self.per_V / rhs.per_C}
	}
}
/// Dividing a InverseVoltage by a InverseCharge returns a value of type Capacitance
impl<T> core::ops::Div<InverseCharge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Capacitance{F: self.per_V.clone() / rhs.per_C}
	}
}
/// Dividing a InverseVoltage by a InverseCharge returns a value of type Capacitance
impl<T> core::ops::Div<&InverseCharge<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Capacitance{F: self.per_V / rhs.per_C.clone()}
	}
}
/// Dividing a InverseVoltage by a InverseCharge returns a value of type Capacitance
impl<T> core::ops::Div<&InverseCharge<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Capacitance{F: self.per_V.clone() / rhs.per_C.clone()}
	}
}

// InverseVoltage / InverseMagneticFlux -> Time
/// Dividing a InverseVoltage by a InverseMagneticFlux returns a value of type Time
impl<T> core::ops::Div<InverseMagneticFlux<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V / rhs.per_Wb}
	}
}
/// Dividing a InverseVoltage by a InverseMagneticFlux returns a value of type Time
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V.clone() / rhs.per_Wb}
	}
}
/// Dividing a InverseVoltage by a InverseMagneticFlux returns a value of type Time
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V / rhs.per_Wb.clone()}
	}
}
/// Dividing a InverseVoltage by a InverseMagneticFlux returns a value of type Time
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V.clone() / rhs.per_Wb.clone()}
	}
}

// InverseVoltage * MagneticFlux -> Time
/// Multiplying a InverseVoltage by a MagneticFlux returns a value of type Time
impl<T> core::ops::Mul<MagneticFlux<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V * rhs.Wb}
	}
}
/// Multiplying a InverseVoltage by a MagneticFlux returns a value of type Time
impl<T> core::ops::Mul<MagneticFlux<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V.clone() * rhs.Wb}
	}
}
/// Multiplying a InverseVoltage by a MagneticFlux returns a value of type Time
impl<T> core::ops::Mul<&MagneticFlux<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V * rhs.Wb.clone()}
	}
}
/// Multiplying a InverseVoltage by a MagneticFlux returns a value of type Time
impl<T> core::ops::Mul<&MagneticFlux<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Time{s: self.per_V.clone() * rhs.Wb.clone()}
	}
}

// InverseVoltage * Resistance -> InverseCurrent
/// Multiplying a InverseVoltage by a Resistance returns a value of type InverseCurrent
impl<T> core::ops::Mul<Resistance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V * rhs.Ohm}
	}
}
/// Multiplying a InverseVoltage by a Resistance returns a value of type InverseCurrent
impl<T> core::ops::Mul<Resistance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V.clone() * rhs.Ohm}
	}
}
/// Multiplying a InverseVoltage by a Resistance returns a value of type InverseCurrent
impl<T> core::ops::Mul<&Resistance<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V * rhs.Ohm.clone()}
	}
}
/// Multiplying a InverseVoltage by a Resistance returns a value of type InverseCurrent
impl<T> core::ops::Mul<&Resistance<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		InverseCurrent{per_A: self.per_V.clone() * rhs.Ohm.clone()}
	}
}

// InverseVoltage * Energy -> Charge
/// Multiplying a InverseVoltage by a Energy returns a value of type Charge
impl<T> core::ops::Mul<Energy<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Charge{C: self.per_V * rhs.J}
	}
}
/// Multiplying a InverseVoltage by a Energy returns a value of type Charge
impl<T> core::ops::Mul<Energy<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Charge{C: self.per_V.clone() * rhs.J}
	}
}
/// Multiplying a InverseVoltage by a Energy returns a value of type Charge
impl<T> core::ops::Mul<&Energy<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Charge{C: self.per_V * rhs.J.clone()}
	}
}
/// Multiplying a InverseVoltage by a Energy returns a value of type Charge
impl<T> core::ops::Mul<&Energy<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Charge{C: self.per_V.clone() * rhs.J.clone()}
	}
}

// InverseVoltage * Torque -> Charge
/// Multiplying a InverseVoltage by a Torque returns a value of type Charge
impl<T> core::ops::Mul<Torque<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Charge{C: self.per_V * rhs.Nm}
	}
}
/// Multiplying a InverseVoltage by a Torque returns a value of type Charge
impl<T> core::ops::Mul<Torque<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Charge{C: self.per_V.clone() * rhs.Nm}
	}
}
/// Multiplying a InverseVoltage by a Torque returns a value of type Charge
impl<T> core::ops::Mul<&Torque<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Charge{C: self.per_V * rhs.Nm.clone()}
	}
}
/// Multiplying a InverseVoltage by a Torque returns a value of type Charge
impl<T> core::ops::Mul<&Torque<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Charge{C: self.per_V.clone() * rhs.Nm.clone()}
	}
}

// InverseVoltage * Frequency -> InverseMagneticFlux
/// Multiplying a InverseVoltage by a Frequency returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<Frequency<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V * rhs.Hz}
	}
}
/// Multiplying a InverseVoltage by a Frequency returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<Frequency<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V.clone() * rhs.Hz}
	}
}
/// Multiplying a InverseVoltage by a Frequency returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&Frequency<T>> for InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V * rhs.Hz.clone()}
	}
}
/// Multiplying a InverseVoltage by a Frequency returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&Frequency<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_V.clone() * rhs.Hz.clone()}
	}
}

// InverseVoltage / InverseEnergy -> Charge
/// Dividing a InverseVoltage by a InverseEnergy returns a value of type Charge
impl<T> core::ops::Div<InverseEnergy<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Charge{C: self.per_V / rhs.per_J}
	}
}
/// Dividing a InverseVoltage by a InverseEnergy returns a value of type Charge
impl<T> core::ops::Div<InverseEnergy<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Charge{C: self.per_V.clone() / rhs.per_J}
	}
}
/// Dividing a InverseVoltage by a InverseEnergy returns a value of type Charge
impl<T> core::ops::Div<&InverseEnergy<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Charge{C: self.per_V / rhs.per_J.clone()}
	}
}
/// Dividing a InverseVoltage by a InverseEnergy returns a value of type Charge
impl<T> core::ops::Div<&InverseEnergy<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Charge{C: self.per_V.clone() / rhs.per_J.clone()}
	}
}

// InverseVoltage / InverseTorque -> Charge
/// Dividing a InverseVoltage by a InverseTorque returns a value of type Charge
impl<T> core::ops::Div<InverseTorque<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Charge{C: self.per_V / rhs.per_Nm}
	}
}
/// Dividing a InverseVoltage by a InverseTorque returns a value of type Charge
impl<T> core::ops::Div<InverseTorque<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Charge{C: self.per_V.clone() / rhs.per_Nm}
	}
}
/// Dividing a InverseVoltage by a InverseTorque returns a value of type Charge
impl<T> core::ops::Div<&InverseTorque<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Charge{C: self.per_V / rhs.per_Nm.clone()}
	}
}
/// Dividing a InverseVoltage by a InverseTorque returns a value of type Charge
impl<T> core::ops::Div<&InverseTorque<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Charge{C: self.per_V.clone() / rhs.per_Nm.clone()}
	}
}

// InverseVoltage / InversePower -> Current
/// Dividing a InverseVoltage by a InversePower returns a value of type Current
impl<T> core::ops::Div<InversePower<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InversePower<T>) -> Self::Output {
		Current{A: self.per_V / rhs.per_W}
	}
}
/// Dividing a InverseVoltage by a InversePower returns a value of type Current
impl<T> core::ops::Div<InversePower<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: InversePower<T>) -> Self::Output {
		Current{A: self.per_V.clone() / rhs.per_W}
	}
}
/// Dividing a InverseVoltage by a InversePower returns a value of type Current
impl<T> core::ops::Div<&InversePower<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InversePower<T>) -> Self::Output {
		Current{A: self.per_V / rhs.per_W.clone()}
	}
}
/// Dividing a InverseVoltage by a InversePower returns a value of type Current
impl<T> core::ops::Div<&InversePower<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &InversePower<T>) -> Self::Output {
		Current{A: self.per_V.clone() / rhs.per_W.clone()}
	}
}

// InverseVoltage * Power -> Current
/// Multiplying a InverseVoltage by a Power returns a value of type Current
impl<T> core::ops::Mul<Power<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Current{A: self.per_V * rhs.W}
	}
}
/// Multiplying a InverseVoltage by a Power returns a value of type Current
impl<T> core::ops::Mul<Power<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Current{A: self.per_V.clone() * rhs.W}
	}
}
/// Multiplying a InverseVoltage by a Power returns a value of type Current
impl<T> core::ops::Mul<&Power<T>> for InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Current{A: self.per_V * rhs.W.clone()}
	}
}
/// Multiplying a InverseVoltage by a Power returns a value of type Current
impl<T> core::ops::Mul<&Power<T>> for &InverseVoltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Current{A: self.per_V.clone() * rhs.W.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for f64 where T: NumLike+From<f64> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for f64 where T: NumLike+From<f64> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for f32 where T: NumLike+From<f32> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for f32 where T: NumLike+From<f32> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for i64 where T: NumLike+From<i64> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for i64 where T: NumLike+From<i64> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for i32 where T: NumLike+From<i32> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<InverseVoltage<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for i32 where T: NumLike+From<i32> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
impl<T> core::ops::Div<&InverseVoltage<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseVoltage<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseVoltage<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseVoltage<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseVoltage<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVoltage<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVoltage<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVoltage<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVoltage<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

// 1/InverseVoltage -> Voltage
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVoltage<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVoltage<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVoltage<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self) / rhs.per_V.clone()}
	}
}
/// Dividing a scalar value by a InverseVoltage unit value returns a value of type Voltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVoltage<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Voltage{V: T::from(self.clone()) / rhs.per_V.clone()}
	}
}

/// The luminous flux unit type, defined as lumens in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct LuminousFlux<T: NumLike>{
	/// The value of this Luminous flux in lumens
	pub lm: T
}

impl<T> LuminousFlux<T> where T: NumLike {

	/// Returns the standard unit name of luminous flux: "lumens"
	pub fn unit_name() -> &'static str { "lumens" }
	
	/// Returns the abbreviated name or symbol of luminous flux: "lm" for lumens
	pub fn unit_symbol() -> &'static str { "lm" }
	
	/// Returns a new luminous flux value from the given number of lumens
	///
	/// # Arguments
	/// * `lm` - Any number-like type, representing a quantity of lumens
	pub fn from_lm(lm: T) -> Self { LuminousFlux{lm: lm} }
	
	/// Returns a copy of this luminous flux value in lumens
	pub fn to_lm(&self) -> T { self.lm.clone() }

	/// Returns a new luminous flux value from the given number of lumens
	///
	/// # Arguments
	/// * `lumens` - Any number-like type, representing a quantity of lumens
	pub fn from_lumens(lumens: T) -> Self { LuminousFlux{lm: lumens} }
	
	/// Returns a copy of this luminous flux value in lumens
	pub fn to_lumens(&self) -> T { self.lm.clone() }

}

impl<T> fmt::Display for LuminousFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.lm, Self::unit_symbol())
	}
}

impl<T> LuminousFlux<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this luminous flux value in millilumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mlm(&self) -> T {
		return self.lm.clone() * T::from(1000.0_f64);
	}

	/// Returns a new luminous flux value from the given number of millilumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mlm` - Any number-like type, representing a quantity of millilumens
	pub fn from_mlm(mlm: T) -> Self {
		LuminousFlux{lm: mlm * T::from(0.001_f64)}
	}

	/// Returns a copy of this luminous flux value in microlumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ulm(&self) -> T {
		return self.lm.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new luminous flux value from the given number of microlumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ulm` - Any number-like type, representing a quantity of microlumens
	pub fn from_ulm(ulm: T) -> Self {
		LuminousFlux{lm: ulm * T::from(1e-06_f64)}
	}

	/// Returns a copy of this luminous flux value in nanolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nlm(&self) -> T {
		return self.lm.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new luminous flux value from the given number of nanolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nlm` - Any number-like type, representing a quantity of nanolumens
	pub fn from_nlm(nlm: T) -> Self {
		LuminousFlux{lm: nlm * T::from(1e-09_f64)}
	}

	/// Returns a copy of this luminous flux value in kilolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_klm(&self) -> T {
		return self.lm.clone() * T::from(0.001_f64);
	}

	/// Returns a new luminous flux value from the given number of kilolumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `klm` - Any number-like type, representing a quantity of kilolumens
	pub fn from_klm(klm: T) -> Self {
		LuminousFlux{lm: klm * T::from(1000.0_f64)}
	}

	/// Returns a copy of this luminous flux value in megalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Mlm(&self) -> T {
		return self.lm.clone() * T::from(1e-06_f64);
	}

	/// Returns a new luminous flux value from the given number of megalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Mlm` - Any number-like type, representing a quantity of megalumens
	pub fn from_Mlm(Mlm: T) -> Self {
		LuminousFlux{lm: Mlm * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this luminous flux value in gigalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_Glm(&self) -> T {
		return self.lm.clone() * T::from(1e-09_f64);
	}

	/// Returns a new luminous flux value from the given number of gigalumens
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `Glm` - Any number-like type, representing a quantity of gigalumens
	pub fn from_Glm(Glm: T) -> Self {
		LuminousFlux{lm: Glm * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<LuminousFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = LuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: LuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		LuminousFlux{lm: self * rhs.lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<LuminousFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = LuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: LuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		LuminousFlux{lm: self.clone() * rhs.lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&LuminousFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = LuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &LuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		LuminousFlux{lm: self * rhs.lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&LuminousFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = LuminousFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &LuminousFlux<num_bigfloat::BigFloat>) -> Self::Output {
		LuminousFlux{lm: self.clone() * rhs.lm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<LuminousFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = LuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: LuminousFlux<num_complex::Complex32>) -> Self::Output {
		LuminousFlux{lm: self * rhs.lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<LuminousFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = LuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: LuminousFlux<num_complex::Complex32>) -> Self::Output {
		LuminousFlux{lm: self.clone() * rhs.lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&LuminousFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = LuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: &LuminousFlux<num_complex::Complex32>) -> Self::Output {
		LuminousFlux{lm: self * rhs.lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&LuminousFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = LuminousFlux<num_complex::Complex32>;
	fn mul(self, rhs: &LuminousFlux<num_complex::Complex32>) -> Self::Output {
		LuminousFlux{lm: self.clone() * rhs.lm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<LuminousFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = LuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: LuminousFlux<num_complex::Complex64>) -> Self::Output {
		LuminousFlux{lm: self * rhs.lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<LuminousFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = LuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: LuminousFlux<num_complex::Complex64>) -> Self::Output {
		LuminousFlux{lm: self.clone() * rhs.lm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&LuminousFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = LuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: &LuminousFlux<num_complex::Complex64>) -> Self::Output {
		LuminousFlux{lm: self * rhs.lm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&LuminousFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = LuminousFlux<num_complex::Complex64>;
	fn mul(self, rhs: &LuminousFlux<num_complex::Complex64>) -> Self::Output {
		LuminousFlux{lm: self.clone() * rhs.lm.clone()}
	}
}




// LuminousFlux * InverseLuminosity -> SolidAngle
/// Multiplying a LuminousFlux by a InverseLuminosity returns a value of type SolidAngle
impl<T> core::ops::Mul<InverseLuminosity<T>> for LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: InverseLuminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm * rhs.per_cd}
	}
}
/// Multiplying a LuminousFlux by a InverseLuminosity returns a value of type SolidAngle
impl<T> core::ops::Mul<InverseLuminosity<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: InverseLuminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm.clone() * rhs.per_cd}
	}
}
/// Multiplying a LuminousFlux by a InverseLuminosity returns a value of type SolidAngle
impl<T> core::ops::Mul<&InverseLuminosity<T>> for LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm * rhs.per_cd.clone()}
	}
}
/// Multiplying a LuminousFlux by a InverseLuminosity returns a value of type SolidAngle
impl<T> core::ops::Mul<&InverseLuminosity<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm.clone() * rhs.per_cd.clone()}
	}
}

// LuminousFlux / Luminosity -> SolidAngle
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> core::ops::Div<Luminosity<T>> for LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm / rhs.cd}
	}
}
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> core::ops::Div<Luminosity<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm.clone() / rhs.cd}
	}
}
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> core::ops::Div<&Luminosity<T>> for LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm / rhs.cd.clone()}
	}
}
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> core::ops::Div<&Luminosity<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm.clone() / rhs.cd.clone()}
	}
}

// LuminousFlux * AreaPerLumen -> Area
/// Multiplying a LuminousFlux by a AreaPerLumen returns a value of type Area
impl<T> core::ops::Mul<AreaPerLumen<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Area{m2: self.lm * rhs.m2_per_lm}
	}
}
/// Multiplying a LuminousFlux by a AreaPerLumen returns a value of type Area
impl<T> core::ops::Mul<AreaPerLumen<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: AreaPerLumen<T>) -> Self::Output {
		Area{m2: self.lm.clone() * rhs.m2_per_lm}
	}
}
/// Multiplying a LuminousFlux by a AreaPerLumen returns a value of type Area
impl<T> core::ops::Mul<&AreaPerLumen<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Area{m2: self.lm * rhs.m2_per_lm.clone()}
	}
}
/// Multiplying a LuminousFlux by a AreaPerLumen returns a value of type Area
impl<T> core::ops::Mul<&AreaPerLumen<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		Area{m2: self.lm.clone() * rhs.m2_per_lm.clone()}
	}
}

// LuminousFlux / Illuminance -> Area
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> core::ops::Div<Illuminance<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		Area{m2: self.lm / rhs.lux}
	}
}
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> core::ops::Div<Illuminance<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		Area{m2: self.lm.clone() / rhs.lux}
	}
}
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> core::ops::Div<&Illuminance<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		Area{m2: self.lm / rhs.lux.clone()}
	}
}
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> core::ops::Div<&Illuminance<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		Area{m2: self.lm.clone() / rhs.lux.clone()}
	}
}

// LuminousFlux / Area -> Illuminance
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> core::ops::Div<Area<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Illuminance{lux: self.lm / rhs.m2}
	}
}
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> core::ops::Div<Area<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Illuminance{lux: self.lm.clone() / rhs.m2}
	}
}
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> core::ops::Div<&Area<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Illuminance{lux: self.lm / rhs.m2.clone()}
	}
}
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> core::ops::Div<&Area<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Illuminance{lux: self.lm.clone() / rhs.m2.clone()}
	}
}

// LuminousFlux * InverseArea -> Illuminance
/// Multiplying a LuminousFlux by a InverseArea returns a value of type Illuminance
impl<T> core::ops::Mul<InverseArea<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		Illuminance{lux: self.lm * rhs.per_m2}
	}
}
/// Multiplying a LuminousFlux by a InverseArea returns a value of type Illuminance
impl<T> core::ops::Mul<InverseArea<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		Illuminance{lux: self.lm.clone() * rhs.per_m2}
	}
}
/// Multiplying a LuminousFlux by a InverseArea returns a value of type Illuminance
impl<T> core::ops::Mul<&InverseArea<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		Illuminance{lux: self.lm * rhs.per_m2.clone()}
	}
}
/// Multiplying a LuminousFlux by a InverseArea returns a value of type Illuminance
impl<T> core::ops::Mul<&InverseArea<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		Illuminance{lux: self.lm.clone() * rhs.per_m2.clone()}
	}
}

// LuminousFlux * InverseSolidAngle -> Luminosity
/// Multiplying a LuminousFlux by a InverseSolidAngle returns a value of type Luminosity
impl<T> core::ops::Mul<InverseSolidAngle<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm * rhs.per_sr}
	}
}
/// Multiplying a LuminousFlux by a InverseSolidAngle returns a value of type Luminosity
impl<T> core::ops::Mul<InverseSolidAngle<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm.clone() * rhs.per_sr}
	}
}
/// Multiplying a LuminousFlux by a InverseSolidAngle returns a value of type Luminosity
impl<T> core::ops::Mul<&InverseSolidAngle<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm * rhs.per_sr.clone()}
	}
}
/// Multiplying a LuminousFlux by a InverseSolidAngle returns a value of type Luminosity
impl<T> core::ops::Mul<&InverseSolidAngle<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm.clone() * rhs.per_sr.clone()}
	}
}

// LuminousFlux / SolidAngle -> Luminosity
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> core::ops::Div<SolidAngle<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm / rhs.sr}
	}
}
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> core::ops::Div<SolidAngle<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm.clone() / rhs.sr}
	}
}
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> core::ops::Div<&SolidAngle<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm / rhs.sr.clone()}
	}
}
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> core::ops::Div<&SolidAngle<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm.clone() / rhs.sr.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<LuminousFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&LuminousFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<LuminousFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<LuminousFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&LuminousFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&LuminousFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<LuminousFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<LuminousFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&LuminousFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&LuminousFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

// 1/LuminousFlux -> InverseLuminousFlux
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<LuminousFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<LuminousFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&LuminousFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self) / rhs.lm.clone()}
	}
}
/// Dividing a scalar value by a LuminousFlux unit value returns a value of type InverseLuminousFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&LuminousFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: T::from(self.clone()) / rhs.lm.clone()}
	}
}

/// The magnetic flux unit type, defined as webers in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MagneticFlux<T: NumLike>{
	/// The value of this Magnetic flux in webers
	pub Wb: T
}

impl<T> MagneticFlux<T> where T: NumLike {

	/// Returns the standard unit name of magnetic flux: "webers"
	pub fn unit_name() -> &'static str { "webers" }
	
	/// Returns the abbreviated name or symbol of magnetic flux: "Wb" for webers
	pub fn unit_symbol() -> &'static str { "Wb" }
	
	/// Returns a new magnetic flux value from the given number of webers
	///
	/// # Arguments
	/// * `Wb` - Any number-like type, representing a quantity of webers
	pub fn from_Wb(Wb: T) -> Self { MagneticFlux{Wb: Wb} }
	
	/// Returns a copy of this magnetic flux value in webers
	pub fn to_Wb(&self) -> T { self.Wb.clone() }

	/// Returns a new magnetic flux value from the given number of webers
	///
	/// # Arguments
	/// * `webers` - Any number-like type, representing a quantity of webers
	pub fn from_webers(webers: T) -> Self { MagneticFlux{Wb: webers} }
	
	/// Returns a copy of this magnetic flux value in webers
	pub fn to_webers(&self) -> T { self.Wb.clone() }

}

impl<T> fmt::Display for MagneticFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Wb, Self::unit_symbol())
	}
}

impl<T> MagneticFlux<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this magnetic flux value in milliwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mWb(&self) -> T {
		return self.Wb.clone() * T::from(1000.0_f64);
	}

	/// Returns a new magnetic flux value from the given number of milliwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mWb` - Any number-like type, representing a quantity of milliwebers
	pub fn from_mWb(mWb: T) -> Self {
		MagneticFlux{Wb: mWb * T::from(0.001_f64)}
	}

	/// Returns a copy of this magnetic flux value in microwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uWb(&self) -> T {
		return self.Wb.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new magnetic flux value from the given number of microwebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uWb` - Any number-like type, representing a quantity of microwebers
	pub fn from_uWb(uWb: T) -> Self {
		MagneticFlux{Wb: uWb * T::from(1e-06_f64)}
	}

	/// Returns a copy of this magnetic flux value in nanowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nWb(&self) -> T {
		return self.Wb.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new magnetic flux value from the given number of nanowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nWb` - Any number-like type, representing a quantity of nanowebers
	pub fn from_nWb(nWb: T) -> Self {
		MagneticFlux{Wb: nWb * T::from(1e-09_f64)}
	}

	/// Returns a copy of this magnetic flux value in kilowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kWb(&self) -> T {
		return self.Wb.clone() * T::from(0.001_f64);
	}

	/// Returns a new magnetic flux value from the given number of kilowebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kWb` - Any number-like type, representing a quantity of kilowebers
	pub fn from_kWb(kWb: T) -> Self {
		MagneticFlux{Wb: kWb * T::from(1000.0_f64)}
	}

	/// Returns a copy of this magnetic flux value in megawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MWb(&self) -> T {
		return self.Wb.clone() * T::from(1e-06_f64);
	}

	/// Returns a new magnetic flux value from the given number of megawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MWb` - Any number-like type, representing a quantity of megawebers
	pub fn from_MWb(MWb: T) -> Self {
		MagneticFlux{Wb: MWb * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this magnetic flux value in gigawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GWb(&self) -> T {
		return self.Wb.clone() * T::from(1e-09_f64);
	}

	/// Returns a new magnetic flux value from the given number of gigawebers
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GWb` - Any number-like type, representing a quantity of gigawebers
	pub fn from_GWb(GWb: T) -> Self {
		MagneticFlux{Wb: GWb * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<MagneticFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = MagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: MagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFlux{Wb: self * rhs.Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<MagneticFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = MagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: MagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFlux{Wb: self.clone() * rhs.Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&MagneticFlux<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = MagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &MagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFlux{Wb: self * rhs.Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&MagneticFlux<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = MagneticFlux<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &MagneticFlux<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFlux{Wb: self.clone() * rhs.Wb.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = MagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: MagneticFlux<num_complex::Complex32>) -> Self::Output {
		MagneticFlux{Wb: self * rhs.Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = MagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: MagneticFlux<num_complex::Complex32>) -> Self::Output {
		MagneticFlux{Wb: self.clone() * rhs.Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFlux<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = MagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: &MagneticFlux<num_complex::Complex32>) -> Self::Output {
		MagneticFlux{Wb: self * rhs.Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFlux<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = MagneticFlux<num_complex::Complex32>;
	fn mul(self, rhs: &MagneticFlux<num_complex::Complex32>) -> Self::Output {
		MagneticFlux{Wb: self.clone() * rhs.Wb.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = MagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: MagneticFlux<num_complex::Complex64>) -> Self::Output {
		MagneticFlux{Wb: self * rhs.Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = MagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: MagneticFlux<num_complex::Complex64>) -> Self::Output {
		MagneticFlux{Wb: self.clone() * rhs.Wb}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFlux<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = MagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: &MagneticFlux<num_complex::Complex64>) -> Self::Output {
		MagneticFlux{Wb: self * rhs.Wb.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFlux<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = MagneticFlux<num_complex::Complex64>;
	fn mul(self, rhs: &MagneticFlux<num_complex::Complex64>) -> Self::Output {
		MagneticFlux{Wb: self.clone() * rhs.Wb.clone()}
	}
}



/// Converts a MagneticFlux into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::MagneticFlux> for MagneticFlux<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::MagneticFlux {
		uom::si::f32::MagneticFlux::new::<uom::si::magnetic_flux::weber>(self.Wb.into())
	}
}

/// Creates a MagneticFlux from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::MagneticFlux> for MagneticFlux<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::MagneticFlux) -> Self {
		MagneticFlux{Wb: T::from(src.value)}
	}
}

/// Converts a MagneticFlux into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::MagneticFlux> for MagneticFlux<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::MagneticFlux {
		uom::si::f64::MagneticFlux::new::<uom::si::magnetic_flux::weber>(self.Wb.into())
	}
}

/// Creates a MagneticFlux from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::MagneticFlux> for MagneticFlux<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::MagneticFlux) -> Self {
		MagneticFlux{Wb: T::from(src.value)}
	}
}


// MagneticFlux * Current -> Energy
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> core::ops::Mul<Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Energy{J: self.Wb * rhs.A}
	}
}
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> core::ops::Mul<Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Energy{J: self.Wb.clone() * rhs.A}
	}
}
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> core::ops::Mul<&Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Energy{J: self.Wb * rhs.A.clone()}
	}
}
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> core::ops::Mul<&Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Energy{J: self.Wb.clone() * rhs.A.clone()}
	}
}

// MagneticFlux / Current -> Inductance
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> core::ops::Div<Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Inductance{H: self.Wb / rhs.A}
	}
}
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> core::ops::Div<Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Inductance{H: self.Wb.clone() / rhs.A}
	}
}
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> core::ops::Div<&Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Inductance{H: self.Wb / rhs.A.clone()}
	}
}
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> core::ops::Div<&Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Inductance{H: self.Wb.clone() / rhs.A.clone()}
	}
}

// MagneticFlux * InverseCurrent -> Inductance
/// Multiplying a MagneticFlux by a InverseCurrent returns a value of type Inductance
impl<T> core::ops::Mul<InverseCurrent<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		Inductance{H: self.Wb * rhs.per_A}
	}
}
/// Multiplying a MagneticFlux by a InverseCurrent returns a value of type Inductance
impl<T> core::ops::Mul<InverseCurrent<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		Inductance{H: self.Wb.clone() * rhs.per_A}
	}
}
/// Multiplying a MagneticFlux by a InverseCurrent returns a value of type Inductance
impl<T> core::ops::Mul<&InverseCurrent<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Inductance{H: self.Wb * rhs.per_A.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseCurrent returns a value of type Inductance
impl<T> core::ops::Mul<&InverseCurrent<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Inductance{H: self.Wb.clone() * rhs.per_A.clone()}
	}
}

// MagneticFlux / InverseCurrent -> Energy
/// Dividing a MagneticFlux by a InverseCurrent returns a value of type Energy
impl<T> core::ops::Div<InverseCurrent<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Energy{J: self.Wb / rhs.per_A}
	}
}
/// Dividing a MagneticFlux by a InverseCurrent returns a value of type Energy
impl<T> core::ops::Div<InverseCurrent<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Energy{J: self.Wb.clone() / rhs.per_A}
	}
}
/// Dividing a MagneticFlux by a InverseCurrent returns a value of type Energy
impl<T> core::ops::Div<&InverseCurrent<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Energy{J: self.Wb / rhs.per_A.clone()}
	}
}
/// Dividing a MagneticFlux by a InverseCurrent returns a value of type Energy
impl<T> core::ops::Div<&InverseCurrent<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Energy{J: self.Wb.clone() / rhs.per_A.clone()}
	}
}

// MagneticFlux / Time -> Voltage
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> core::ops::Div<Time<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Voltage{V: self.Wb / rhs.s}
	}
}
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> core::ops::Div<Time<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() / rhs.s}
	}
}
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> core::ops::Div<&Time<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Voltage{V: self.Wb / rhs.s.clone()}
	}
}
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> core::ops::Div<&Time<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() / rhs.s.clone()}
	}
}

// MagneticFlux / Charge -> Resistance
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> core::ops::Div<Charge<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb / rhs.C}
	}
}
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> core::ops::Div<Charge<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb.clone() / rhs.C}
	}
}
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> core::ops::Div<&Charge<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb / rhs.C.clone()}
	}
}
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> core::ops::Div<&Charge<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb.clone() / rhs.C.clone()}
	}
}

// MagneticFlux * Conductance -> Charge
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> core::ops::Mul<Conductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Charge{C: self.Wb * rhs.S}
	}
}
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> core::ops::Mul<Conductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() * rhs.S}
	}
}
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> core::ops::Mul<&Conductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Charge{C: self.Wb * rhs.S.clone()}
	}
}
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> core::ops::Mul<&Conductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() * rhs.S.clone()}
	}
}

// MagneticFlux / Inductance -> Current
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> core::ops::Div<Inductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Current{A: self.Wb / rhs.H}
	}
}
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> core::ops::Div<Inductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Current{A: self.Wb.clone() / rhs.H}
	}
}
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> core::ops::Div<&Inductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Current{A: self.Wb / rhs.H.clone()}
	}
}
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> core::ops::Div<&Inductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Current{A: self.Wb.clone() / rhs.H.clone()}
	}
}

// MagneticFlux * InverseCharge -> Resistance
/// Multiplying a MagneticFlux by a InverseCharge returns a value of type Resistance
impl<T> core::ops::Mul<InverseCharge<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb * rhs.per_C}
	}
}
/// Multiplying a MagneticFlux by a InverseCharge returns a value of type Resistance
impl<T> core::ops::Mul<InverseCharge<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb.clone() * rhs.per_C}
	}
}
/// Multiplying a MagneticFlux by a InverseCharge returns a value of type Resistance
impl<T> core::ops::Mul<&InverseCharge<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb * rhs.per_C.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseCharge returns a value of type Resistance
impl<T> core::ops::Mul<&InverseCharge<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb.clone() * rhs.per_C.clone()}
	}
}

// MagneticFlux * InverseInductance -> Current
/// Multiplying a MagneticFlux by a InverseInductance returns a value of type Current
impl<T> core::ops::Mul<InverseInductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		Current{A: self.Wb * rhs.per_H}
	}
}
/// Multiplying a MagneticFlux by a InverseInductance returns a value of type Current
impl<T> core::ops::Mul<InverseInductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		Current{A: self.Wb.clone() * rhs.per_H}
	}
}
/// Multiplying a MagneticFlux by a InverseInductance returns a value of type Current
impl<T> core::ops::Mul<&InverseInductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		Current{A: self.Wb * rhs.per_H.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseInductance returns a value of type Current
impl<T> core::ops::Mul<&InverseInductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		Current{A: self.Wb.clone() * rhs.per_H.clone()}
	}
}

// MagneticFlux * InverseMagneticFluxDensity -> Area
/// Multiplying a MagneticFlux by a InverseMagneticFluxDensity returns a value of type Area
impl<T> core::ops::Mul<InverseMagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb * rhs.m2_per_Wb}
	}
}
/// Multiplying a MagneticFlux by a InverseMagneticFluxDensity returns a value of type Area
impl<T> core::ops::Mul<InverseMagneticFluxDensity<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb.clone() * rhs.m2_per_Wb}
	}
}
/// Multiplying a MagneticFlux by a InverseMagneticFluxDensity returns a value of type Area
impl<T> core::ops::Mul<&InverseMagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb * rhs.m2_per_Wb.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseMagneticFluxDensity returns a value of type Area
impl<T> core::ops::Mul<&InverseMagneticFluxDensity<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb.clone() * rhs.m2_per_Wb.clone()}
	}
}

// MagneticFlux * InverseVoltage -> Time
/// Multiplying a MagneticFlux by a InverseVoltage returns a value of type Time
impl<T> core::ops::Mul<InverseVoltage<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		Time{s: self.Wb * rhs.per_V}
	}
}
/// Multiplying a MagneticFlux by a InverseVoltage returns a value of type Time
impl<T> core::ops::Mul<InverseVoltage<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		Time{s: self.Wb.clone() * rhs.per_V}
	}
}
/// Multiplying a MagneticFlux by a InverseVoltage returns a value of type Time
impl<T> core::ops::Mul<&InverseVoltage<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Time{s: self.Wb * rhs.per_V.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseVoltage returns a value of type Time
impl<T> core::ops::Mul<&InverseVoltage<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		Time{s: self.Wb.clone() * rhs.per_V.clone()}
	}
}

// MagneticFlux / MagneticFluxDensity -> Area
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> core::ops::Div<MagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb / rhs.T}
	}
}
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb.clone() / rhs.T}
	}
}
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb / rhs.T.clone()}
	}
}
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb.clone() / rhs.T.clone()}
	}
}

// MagneticFlux / Resistance -> Charge
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> core::ops::Div<Resistance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Charge{C: self.Wb / rhs.Ohm}
	}
}
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> core::ops::Div<Resistance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() / rhs.Ohm}
	}
}
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> core::ops::Div<&Resistance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Charge{C: self.Wb / rhs.Ohm.clone()}
	}
}
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> core::ops::Div<&Resistance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() / rhs.Ohm.clone()}
	}
}

// MagneticFlux / Voltage -> Time
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> core::ops::Div<Voltage<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Time{s: self.Wb / rhs.V}
	}
}
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> core::ops::Div<Voltage<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Time{s: self.Wb.clone() / rhs.V}
	}
}
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> core::ops::Div<&Voltage<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Time{s: self.Wb / rhs.V.clone()}
	}
}
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> core::ops::Div<&Voltage<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Time{s: self.Wb.clone() / rhs.V.clone()}
	}
}

// MagneticFlux / Area -> MagneticFluxDensity
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<Area<T>> for MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb / rhs.m2}
	}
}
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<Area<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb.clone() / rhs.m2}
	}
}
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&Area<T>> for MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb / rhs.m2.clone()}
	}
}
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&Area<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb.clone() / rhs.m2.clone()}
	}
}

// MagneticFlux * InverseArea -> MagneticFluxDensity
/// Multiplying a MagneticFlux by a InverseArea returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<InverseArea<T>> for MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb * rhs.per_m2}
	}
}
/// Multiplying a MagneticFlux by a InverseArea returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<InverseArea<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb.clone() * rhs.per_m2}
	}
}
/// Multiplying a MagneticFlux by a InverseArea returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<&InverseArea<T>> for MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb * rhs.per_m2.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseArea returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<&InverseArea<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb.clone() * rhs.per_m2.clone()}
	}
}

// MagneticFlux / Energy -> InverseCurrent
/// Dividing a MagneticFlux by a Energy returns a value of type InverseCurrent
impl<T> core::ops::Div<Energy<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb / rhs.J}
	}
}
/// Dividing a MagneticFlux by a Energy returns a value of type InverseCurrent
impl<T> core::ops::Div<Energy<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() / rhs.J}
	}
}
/// Dividing a MagneticFlux by a Energy returns a value of type InverseCurrent
impl<T> core::ops::Div<&Energy<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb / rhs.J.clone()}
	}
}
/// Dividing a MagneticFlux by a Energy returns a value of type InverseCurrent
impl<T> core::ops::Div<&Energy<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() / rhs.J.clone()}
	}
}

// MagneticFlux / Torque -> InverseCurrent
/// Dividing a MagneticFlux by a Torque returns a value of type InverseCurrent
impl<T> core::ops::Div<Torque<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb / rhs.Nm}
	}
}
/// Dividing a MagneticFlux by a Torque returns a value of type InverseCurrent
impl<T> core::ops::Div<Torque<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() / rhs.Nm}
	}
}
/// Dividing a MagneticFlux by a Torque returns a value of type InverseCurrent
impl<T> core::ops::Div<&Torque<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb / rhs.Nm.clone()}
	}
}
/// Dividing a MagneticFlux by a Torque returns a value of type InverseCurrent
impl<T> core::ops::Div<&Torque<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() / rhs.Nm.clone()}
	}
}

// MagneticFlux * Frequency -> Voltage
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> core::ops::Mul<Frequency<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb * rhs.Hz}
	}
}
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> core::ops::Mul<Frequency<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() * rhs.Hz}
	}
}
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> core::ops::Mul<&Frequency<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb * rhs.Hz.clone()}
	}
}
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> core::ops::Mul<&Frequency<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() * rhs.Hz.clone()}
	}
}

// MagneticFlux * InverseEnergy -> InverseCurrent
/// Multiplying a MagneticFlux by a InverseEnergy returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseEnergy<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb * rhs.per_J}
	}
}
/// Multiplying a MagneticFlux by a InverseEnergy returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseEnergy<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() * rhs.per_J}
	}
}
/// Multiplying a MagneticFlux by a InverseEnergy returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseEnergy<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb * rhs.per_J.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseEnergy returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseEnergy<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() * rhs.per_J.clone()}
	}
}

// MagneticFlux * InverseTorque -> InverseCurrent
/// Multiplying a MagneticFlux by a InverseTorque returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseTorque<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb * rhs.per_Nm}
	}
}
/// Multiplying a MagneticFlux by a InverseTorque returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseTorque<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() * rhs.per_Nm}
	}
}
/// Multiplying a MagneticFlux by a InverseTorque returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseTorque<T>> for MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb * rhs.per_Nm.clone()}
	}
}
/// Multiplying a MagneticFlux by a InverseTorque returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseTorque<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseCurrent{per_A: self.Wb.clone() * rhs.per_Nm.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFlux<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<MagneticFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<MagneticFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&MagneticFlux<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&MagneticFlux<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFlux<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFlux<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

// 1/MagneticFlux -> InverseMagneticFlux
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFlux<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self) / rhs.Wb.clone()}
	}
}
/// Dividing a scalar value by a MagneticFlux unit value returns a value of type InverseMagneticFlux
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFlux<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: T::from(self.clone()) / rhs.Wb.clone()}
	}
}

/// The magnetic flux density unit type, defined as teslas in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MagneticFluxDensity<T: NumLike>{
	/// The value of this Magnetic flux density in teslas
	pub T: T
}

impl<T> MagneticFluxDensity<T> where T: NumLike {

	/// Returns the standard unit name of magnetic flux density: "teslas"
	pub fn unit_name() -> &'static str { "teslas" }
	
	/// Returns the abbreviated name or symbol of magnetic flux density: "T" for teslas
	pub fn unit_symbol() -> &'static str { "T" }
	
	/// Returns a new magnetic flux density value from the given number of teslas
	///
	/// # Arguments
	/// * `T` - Any number-like type, representing a quantity of teslas
	pub fn from_T(T: T) -> Self { MagneticFluxDensity{T: T} }
	
	/// Returns a copy of this magnetic flux density value in teslas
	pub fn to_T(&self) -> T { self.T.clone() }

	/// Returns a new magnetic flux density value from the given number of teslas
	///
	/// # Arguments
	/// * `teslas` - Any number-like type, representing a quantity of teslas
	pub fn from_teslas(teslas: T) -> Self { MagneticFluxDensity{T: teslas} }
	
	/// Returns a copy of this magnetic flux density value in teslas
	pub fn to_teslas(&self) -> T { self.T.clone() }

}

impl<T> fmt::Display for MagneticFluxDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.T, Self::unit_symbol())
	}
}

impl<T> MagneticFluxDensity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this magnetic flux density value in milliteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mT(&self) -> T {
		return self.T.clone() * T::from(1000.0_f64);
	}

	/// Returns a new magnetic flux density value from the given number of milliteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mT` - Any number-like type, representing a quantity of milliteslas
	pub fn from_mT(mT: T) -> Self {
		MagneticFluxDensity{T: mT * T::from(0.001_f64)}
	}

	/// Returns a copy of this magnetic flux density value in microteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uT(&self) -> T {
		return self.T.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new magnetic flux density value from the given number of microteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uT` - Any number-like type, representing a quantity of microteslas
	pub fn from_uT(uT: T) -> Self {
		MagneticFluxDensity{T: uT * T::from(1e-06_f64)}
	}

	/// Returns a copy of this magnetic flux density value in nanoteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nT(&self) -> T {
		return self.T.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new magnetic flux density value from the given number of nanoteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nT` - Any number-like type, representing a quantity of nanoteslas
	pub fn from_nT(nT: T) -> Self {
		MagneticFluxDensity{T: nT * T::from(1e-09_f64)}
	}

	/// Returns a copy of this magnetic flux density value in kiloteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kT(&self) -> T {
		return self.T.clone() * T::from(0.001_f64);
	}

	/// Returns a new magnetic flux density value from the given number of kiloteslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kT` - Any number-like type, representing a quantity of kiloteslas
	pub fn from_kT(kT: T) -> Self {
		MagneticFluxDensity{T: kT * T::from(1000.0_f64)}
	}

	/// Returns a copy of this magnetic flux density value in megateslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MT(&self) -> T {
		return self.T.clone() * T::from(1e-06_f64);
	}

	/// Returns a new magnetic flux density value from the given number of megateslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MT` - Any number-like type, representing a quantity of megateslas
	pub fn from_MT(MT: T) -> Self {
		MagneticFluxDensity{T: MT * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this magnetic flux density value in gigateslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GT(&self) -> T {
		return self.T.clone() * T::from(1e-09_f64);
	}

	/// Returns a new magnetic flux density value from the given number of gigateslas
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GT` - Any number-like type, representing a quantity of gigateslas
	pub fn from_GT(GT: T) -> Self {
		MagneticFluxDensity{T: GT * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<MagneticFluxDensity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = MagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: MagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFluxDensity{T: self * rhs.T}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<MagneticFluxDensity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = MagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: MagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFluxDensity{T: self.clone() * rhs.T}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&MagneticFluxDensity<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = MagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &MagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFluxDensity{T: self * rhs.T.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&MagneticFluxDensity<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = MagneticFluxDensity<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &MagneticFluxDensity<num_bigfloat::BigFloat>) -> Self::Output {
		MagneticFluxDensity{T: self.clone() * rhs.T.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFluxDensity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = MagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: MagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		MagneticFluxDensity{T: self * rhs.T}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFluxDensity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = MagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: MagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		MagneticFluxDensity{T: self.clone() * rhs.T}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFluxDensity<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = MagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: &MagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		MagneticFluxDensity{T: self * rhs.T.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFluxDensity<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = MagneticFluxDensity<num_complex::Complex32>;
	fn mul(self, rhs: &MagneticFluxDensity<num_complex::Complex32>) -> Self::Output {
		MagneticFluxDensity{T: self.clone() * rhs.T.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFluxDensity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = MagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: MagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		MagneticFluxDensity{T: self * rhs.T}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<MagneticFluxDensity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = MagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: MagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		MagneticFluxDensity{T: self.clone() * rhs.T}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFluxDensity<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = MagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: &MagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		MagneticFluxDensity{T: self * rhs.T.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&MagneticFluxDensity<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = MagneticFluxDensity<num_complex::Complex64>;
	fn mul(self, rhs: &MagneticFluxDensity<num_complex::Complex64>) -> Self::Output {
		MagneticFluxDensity{T: self.clone() * rhs.T.clone()}
	}
}



/// Converts a MagneticFluxDensity into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::MagneticFluxDensity {
		uom::si::f32::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(self.T.into())
	}
}

/// Creates a MagneticFluxDensity from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::MagneticFluxDensity) -> Self {
		MagneticFluxDensity{T: T::from(src.value)}
	}
}

/// Converts a MagneticFluxDensity into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::MagneticFluxDensity {
		uom::si::f64::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(self.T.into())
	}
}

/// Creates a MagneticFluxDensity from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::MagneticFluxDensity) -> Self {
		MagneticFluxDensity{T: T::from(src.value)}
	}
}


// MagneticFluxDensity * InverseMagneticFlux -> InverseArea
/// Multiplying a MagneticFluxDensity by a InverseMagneticFlux returns a value of type InverseArea
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T * rhs.per_Wb}
	}
}
/// Multiplying a MagneticFluxDensity by a InverseMagneticFlux returns a value of type InverseArea
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T.clone() * rhs.per_Wb}
	}
}
/// Multiplying a MagneticFluxDensity by a InverseMagneticFlux returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T * rhs.per_Wb.clone()}
	}
}
/// Multiplying a MagneticFluxDensity by a InverseMagneticFlux returns a value of type InverseArea
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T.clone() * rhs.per_Wb.clone()}
	}
}

// MagneticFluxDensity / MagneticFlux -> InverseArea
/// Dividing a MagneticFluxDensity by a MagneticFlux returns a value of type InverseArea
impl<T> core::ops::Div<MagneticFlux<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T / rhs.Wb}
	}
}
/// Dividing a MagneticFluxDensity by a MagneticFlux returns a value of type InverseArea
impl<T> core::ops::Div<MagneticFlux<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T.clone() / rhs.Wb}
	}
}
/// Dividing a MagneticFluxDensity by a MagneticFlux returns a value of type InverseArea
impl<T> core::ops::Div<&MagneticFlux<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T / rhs.Wb.clone()}
	}
}
/// Dividing a MagneticFluxDensity by a MagneticFlux returns a value of type InverseArea
impl<T> core::ops::Div<&MagneticFlux<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseArea{per_m2: self.T.clone() / rhs.Wb.clone()}
	}
}

// MagneticFluxDensity * Area -> MagneticFlux
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> core::ops::Mul<Area<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T * rhs.m2}
	}
}
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> core::ops::Mul<Area<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T.clone() * rhs.m2}
	}
}
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Area<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T * rhs.m2.clone()}
	}
}
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Area<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T.clone() * rhs.m2.clone()}
	}
}

// MagneticFluxDensity / InverseArea -> MagneticFlux
/// Dividing a MagneticFluxDensity by a InverseArea returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseArea<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		MagneticFlux{Wb: self.T / rhs.per_m2}
	}
}
/// Dividing a MagneticFluxDensity by a InverseArea returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseArea<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		MagneticFlux{Wb: self.T.clone() / rhs.per_m2}
	}
}
/// Dividing a MagneticFluxDensity by a InverseArea returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseArea<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		MagneticFlux{Wb: self.T / rhs.per_m2.clone()}
	}
}
/// Dividing a MagneticFluxDensity by a InverseArea returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseArea<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		MagneticFlux{Wb: self.T.clone() / rhs.per_m2.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<MagneticFluxDensity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFluxDensity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

// 1/MagneticFluxDensity -> InverseMagneticFluxDensity
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFluxDensity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self) / rhs.T.clone()}
	}
}
/// Dividing a scalar value by a MagneticFluxDensity unit value returns a value of type InverseMagneticFluxDensity
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: T::from(self.clone()) / rhs.T.clone()}
	}
}

/// The electrical resistance unit type, defined as ohms in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Resistance<T: NumLike>{
	/// The value of this Electrical resistance in ohms
	pub Ohm: T
}

impl<T> Resistance<T> where T: NumLike {

	/// Returns the standard unit name of electrical resistance: "ohms"
	pub fn unit_name() -> &'static str { "ohms" }
	
	/// Returns the abbreviated name or symbol of electrical resistance: "Ohm" for ohms
	pub fn unit_symbol() -> &'static str { "Ohm" }
	
	/// Returns a new electrical resistance value from the given number of ohms
	///
	/// # Arguments
	/// * `Ohm` - Any number-like type, representing a quantity of ohms
	pub fn from_Ohm(Ohm: T) -> Self { Resistance{Ohm: Ohm} }
	
	/// Returns a copy of this electrical resistance value in ohms
	pub fn to_Ohm(&self) -> T { self.Ohm.clone() }

	/// Returns a new electrical resistance value from the given number of ohms
	///
	/// # Arguments
	/// * `ohms` - Any number-like type, representing a quantity of ohms
	pub fn from_ohms(ohms: T) -> Self { Resistance{Ohm: ohms} }
	
	/// Returns a copy of this electrical resistance value in ohms
	pub fn to_ohms(&self) -> T { self.Ohm.clone() }

}

impl<T> fmt::Display for Resistance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Ohm, Self::unit_symbol())
	}
}

impl<T> Resistance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical resistance value in milliohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical resistance value from the given number of milliohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mOhm` - Any number-like type, representing a quantity of milliohms
	pub fn from_mOhm(mOhm: T) -> Self {
		Resistance{Ohm: mOhm * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical resistance value in microohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical resistance value from the given number of microohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uOhm` - Any number-like type, representing a quantity of microohms
	pub fn from_uOhm(uOhm: T) -> Self {
		Resistance{Ohm: uOhm * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical resistance value in nanoohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical resistance value from the given number of nanoohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nOhm` - Any number-like type, representing a quantity of nanoohms
	pub fn from_nOhm(nOhm: T) -> Self {
		Resistance{Ohm: nOhm * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical resistance value in kiloohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kOhm(&self) -> T {
		return self.Ohm.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical resistance value from the given number of kiloohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kOhm` - Any number-like type, representing a quantity of kiloohms
	pub fn from_kOhm(kOhm: T) -> Self {
		Resistance{Ohm: kOhm * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical resistance value in megaohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical resistance value from the given number of megaohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MOhm` - Any number-like type, representing a quantity of megaohms
	pub fn from_MOhm(MOhm: T) -> Self {
		Resistance{Ohm: MOhm * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical resistance value in gigaohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical resistance value from the given number of gigaohms
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GOhm` - Any number-like type, representing a quantity of gigaohms
	pub fn from_GOhm(GOhm: T) -> Self {
		Resistance{Ohm: GOhm * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Resistance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Resistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Resistance<num_bigfloat::BigFloat>) -> Self::Output {
		Resistance{Ohm: self * rhs.Ohm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Resistance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Resistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Resistance<num_bigfloat::BigFloat>) -> Self::Output {
		Resistance{Ohm: self.clone() * rhs.Ohm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Resistance<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Resistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Resistance<num_bigfloat::BigFloat>) -> Self::Output {
		Resistance{Ohm: self * rhs.Ohm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Resistance<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Resistance<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Resistance<num_bigfloat::BigFloat>) -> Self::Output {
		Resistance{Ohm: self.clone() * rhs.Ohm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Resistance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Resistance<num_complex::Complex32>;
	fn mul(self, rhs: Resistance<num_complex::Complex32>) -> Self::Output {
		Resistance{Ohm: self * rhs.Ohm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Resistance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Resistance<num_complex::Complex32>;
	fn mul(self, rhs: Resistance<num_complex::Complex32>) -> Self::Output {
		Resistance{Ohm: self.clone() * rhs.Ohm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Resistance<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Resistance<num_complex::Complex32>;
	fn mul(self, rhs: &Resistance<num_complex::Complex32>) -> Self::Output {
		Resistance{Ohm: self * rhs.Ohm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Resistance<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Resistance<num_complex::Complex32>;
	fn mul(self, rhs: &Resistance<num_complex::Complex32>) -> Self::Output {
		Resistance{Ohm: self.clone() * rhs.Ohm.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Resistance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Resistance<num_complex::Complex64>;
	fn mul(self, rhs: Resistance<num_complex::Complex64>) -> Self::Output {
		Resistance{Ohm: self * rhs.Ohm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Resistance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Resistance<num_complex::Complex64>;
	fn mul(self, rhs: Resistance<num_complex::Complex64>) -> Self::Output {
		Resistance{Ohm: self.clone() * rhs.Ohm}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Resistance<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Resistance<num_complex::Complex64>;
	fn mul(self, rhs: &Resistance<num_complex::Complex64>) -> Self::Output {
		Resistance{Ohm: self * rhs.Ohm.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Resistance<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Resistance<num_complex::Complex64>;
	fn mul(self, rhs: &Resistance<num_complex::Complex64>) -> Self::Output {
		Resistance{Ohm: self.clone() * rhs.Ohm.clone()}
	}
}



/// Converts a Resistance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricalResistance> for Resistance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricalResistance {
		uom::si::f32::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(self.Ohm.into())
	}
}

/// Creates a Resistance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricalResistance> for Resistance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricalResistance) -> Self {
		Resistance{Ohm: T::from(src.value)}
	}
}

/// Converts a Resistance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricalResistance> for Resistance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricalResistance {
		uom::si::f64::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(self.Ohm.into())
	}
}

/// Creates a Resistance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricalResistance> for Resistance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricalResistance) -> Self {
		Resistance{Ohm: T::from(src.value)}
	}
}


// Resistance * Current -> Voltage
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> core::ops::Mul<Current<T>> for Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Voltage{V: self.Ohm * rhs.A}
	}
}
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> core::ops::Mul<Current<T>> for &Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Voltage{V: self.Ohm.clone() * rhs.A}
	}
}
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> core::ops::Mul<&Current<T>> for Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Voltage{V: self.Ohm * rhs.A.clone()}
	}
}
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> core::ops::Mul<&Current<T>> for &Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Voltage{V: self.Ohm.clone() * rhs.A.clone()}
	}
}

// Resistance / InverseCurrent -> Voltage
/// Dividing a Resistance by a InverseCurrent returns a value of type Voltage
impl<T> core::ops::Div<InverseCurrent<T>> for Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Voltage{V: self.Ohm / rhs.per_A}
	}
}
/// Dividing a Resistance by a InverseCurrent returns a value of type Voltage
impl<T> core::ops::Div<InverseCurrent<T>> for &Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Voltage{V: self.Ohm.clone() / rhs.per_A}
	}
}
/// Dividing a Resistance by a InverseCurrent returns a value of type Voltage
impl<T> core::ops::Div<&InverseCurrent<T>> for Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Voltage{V: self.Ohm / rhs.per_A.clone()}
	}
}
/// Dividing a Resistance by a InverseCurrent returns a value of type Voltage
impl<T> core::ops::Div<&InverseCurrent<T>> for &Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Voltage{V: self.Ohm.clone() / rhs.per_A.clone()}
	}
}

// Resistance * Time -> Inductance
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> core::ops::Mul<Time<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Inductance{H: self.Ohm * rhs.s}
	}
}
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> core::ops::Mul<Time<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() * rhs.s}
	}
}
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> core::ops::Mul<&Time<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Inductance{H: self.Ohm * rhs.s.clone()}
	}
}
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> core::ops::Mul<&Time<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() * rhs.s.clone()}
	}
}

// Resistance / Time -> Elastance
/// Dividing a Resistance by a Time returns a value of type Elastance
impl<T> core::ops::Div<Time<T>> for Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Elastance{per_F: self.Ohm / rhs.s}
	}
}
/// Dividing a Resistance by a Time returns a value of type Elastance
impl<T> core::ops::Div<Time<T>> for &Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Elastance{per_F: self.Ohm.clone() / rhs.s}
	}
}
/// Dividing a Resistance by a Time returns a value of type Elastance
impl<T> core::ops::Div<&Time<T>> for Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Elastance{per_F: self.Ohm / rhs.s.clone()}
	}
}
/// Dividing a Resistance by a Time returns a value of type Elastance
impl<T> core::ops::Div<&Time<T>> for &Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Elastance{per_F: self.Ohm.clone() / rhs.s.clone()}
	}
}

// Resistance * Capacitance -> Time
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> core::ops::Mul<Capacitance<T>> for Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm * rhs.F}
	}
}
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> core::ops::Mul<Capacitance<T>> for &Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm.clone() * rhs.F}
	}
}
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> core::ops::Mul<&Capacitance<T>> for Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm * rhs.F.clone()}
	}
}
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> core::ops::Mul<&Capacitance<T>> for &Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm.clone() * rhs.F.clone()}
	}
}

// Resistance * Charge -> MagneticFlux
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> core::ops::Mul<Charge<T>> for Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm * rhs.C}
	}
}
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> core::ops::Mul<Charge<T>> for &Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm.clone() * rhs.C}
	}
}
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Charge<T>> for Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm * rhs.C.clone()}
	}
}
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Charge<T>> for &Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm.clone() * rhs.C.clone()}
	}
}

// Resistance / Elastance -> Time
/// Dividing a Resistance by a Elastance returns a value of type Time
impl<T> core::ops::Div<Elastance<T>> for Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Time{s: self.Ohm / rhs.per_F}
	}
}
/// Dividing a Resistance by a Elastance returns a value of type Time
impl<T> core::ops::Div<Elastance<T>> for &Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Time{s: self.Ohm.clone() / rhs.per_F}
	}
}
/// Dividing a Resistance by a Elastance returns a value of type Time
impl<T> core::ops::Div<&Elastance<T>> for Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Time{s: self.Ohm / rhs.per_F.clone()}
	}
}
/// Dividing a Resistance by a Elastance returns a value of type Time
impl<T> core::ops::Div<&Elastance<T>> for &Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Time{s: self.Ohm.clone() / rhs.per_F.clone()}
	}
}

// Resistance / Inductance -> Frequency
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> core::ops::Div<Inductance<T>> for Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm / rhs.H}
	}
}
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> core::ops::Div<Inductance<T>> for &Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm.clone() / rhs.H}
	}
}
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> core::ops::Div<&Inductance<T>> for Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm / rhs.H.clone()}
	}
}
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> core::ops::Div<&Inductance<T>> for &Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm.clone() / rhs.H.clone()}
	}
}

// Resistance / InverseCharge -> MagneticFlux
/// Dividing a Resistance by a InverseCharge returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseCharge<T>> for Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm / rhs.per_C}
	}
}
/// Dividing a Resistance by a InverseCharge returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseCharge<T>> for &Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm.clone() / rhs.per_C}
	}
}
/// Dividing a Resistance by a InverseCharge returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseCharge<T>> for Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm / rhs.per_C.clone()}
	}
}
/// Dividing a Resistance by a InverseCharge returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseCharge<T>> for &Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm.clone() / rhs.per_C.clone()}
	}
}

// Resistance * InverseInductance -> Frequency
/// Multiplying a Resistance by a InverseInductance returns a value of type Frequency
impl<T> core::ops::Mul<InverseInductance<T>> for Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm * rhs.per_H}
	}
}
/// Multiplying a Resistance by a InverseInductance returns a value of type Frequency
impl<T> core::ops::Mul<InverseInductance<T>> for &Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: InverseInductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm.clone() * rhs.per_H}
	}
}
/// Multiplying a Resistance by a InverseInductance returns a value of type Frequency
impl<T> core::ops::Mul<&InverseInductance<T>> for Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm * rhs.per_H.clone()}
	}
}
/// Multiplying a Resistance by a InverseInductance returns a value of type Frequency
impl<T> core::ops::Mul<&InverseInductance<T>> for &Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &InverseInductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm.clone() * rhs.per_H.clone()}
	}
}

// Resistance * InverseMagneticFlux -> InverseCharge
/// Multiplying a Resistance by a InverseMagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm * rhs.per_Wb}
	}
}
/// Multiplying a Resistance by a InverseMagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Resistance by a InverseMagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Resistance by a InverseMagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm.clone() * rhs.per_Wb.clone()}
	}
}

// Resistance * InverseVoltage -> InverseCurrent
/// Multiplying a Resistance by a InverseVoltage returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseVoltage<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm * rhs.per_V}
	}
}
/// Multiplying a Resistance by a InverseVoltage returns a value of type InverseCurrent
impl<T> core::ops::Mul<InverseVoltage<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InverseVoltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm.clone() * rhs.per_V}
	}
}
/// Multiplying a Resistance by a InverseVoltage returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseVoltage<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm * rhs.per_V.clone()}
	}
}
/// Multiplying a Resistance by a InverseVoltage returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InverseVoltage<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InverseVoltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm.clone() * rhs.per_V.clone()}
	}
}

// Resistance / MagneticFlux -> InverseCharge
/// Dividing a Resistance by a MagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Div<MagneticFlux<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm / rhs.Wb}
	}
}
/// Dividing a Resistance by a MagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Div<MagneticFlux<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm.clone() / rhs.Wb}
	}
}
/// Dividing a Resistance by a MagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Div<&MagneticFlux<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm / rhs.Wb.clone()}
	}
}
/// Dividing a Resistance by a MagneticFlux returns a value of type InverseCharge
impl<T> core::ops::Div<&MagneticFlux<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseCharge{per_C: self.Ohm.clone() / rhs.Wb.clone()}
	}
}

// Resistance / Voltage -> InverseCurrent
/// Dividing a Resistance by a Voltage returns a value of type InverseCurrent
impl<T> core::ops::Div<Voltage<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm / rhs.V}
	}
}
/// Dividing a Resistance by a Voltage returns a value of type InverseCurrent
impl<T> core::ops::Div<Voltage<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm.clone() / rhs.V}
	}
}
/// Dividing a Resistance by a Voltage returns a value of type InverseCurrent
impl<T> core::ops::Div<&Voltage<T>> for Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm / rhs.V.clone()}
	}
}
/// Dividing a Resistance by a Voltage returns a value of type InverseCurrent
impl<T> core::ops::Div<&Voltage<T>> for &Resistance<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseCurrent{per_A: self.Ohm.clone() / rhs.V.clone()}
	}
}

// Resistance * Frequency -> Elastance
/// Multiplying a Resistance by a Frequency returns a value of type Elastance
impl<T> core::ops::Mul<Frequency<T>> for Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Elastance{per_F: self.Ohm * rhs.Hz}
	}
}
/// Multiplying a Resistance by a Frequency returns a value of type Elastance
impl<T> core::ops::Mul<Frequency<T>> for &Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Elastance{per_F: self.Ohm.clone() * rhs.Hz}
	}
}
/// Multiplying a Resistance by a Frequency returns a value of type Elastance
impl<T> core::ops::Mul<&Frequency<T>> for Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Elastance{per_F: self.Ohm * rhs.Hz.clone()}
	}
}
/// Multiplying a Resistance by a Frequency returns a value of type Elastance
impl<T> core::ops::Mul<&Frequency<T>> for &Resistance<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Elastance{per_F: self.Ohm.clone() * rhs.Hz.clone()}
	}
}

// Resistance / Frequency -> Inductance
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> core::ops::Div<Frequency<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm / rhs.Hz}
	}
}
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> core::ops::Div<Frequency<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() / rhs.Hz}
	}
}
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> core::ops::Div<&Frequency<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm / rhs.Hz.clone()}
	}
}
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> core::ops::Div<&Frequency<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() / rhs.Hz.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<Resistance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> core::ops::Div<&Resistance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Resistance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Resistance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Resistance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Resistance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Resistance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Resistance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Resistance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Resistance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Resistance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Resistance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Resistance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Resistance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

/// The voltage unit type, defined as volts in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Voltage<T: NumLike>{
	/// The value of this Voltage in volts
	pub V: T
}

impl<T> Voltage<T> where T: NumLike {

	/// Returns the standard unit name of voltage: "volts"
	pub fn unit_name() -> &'static str { "volts" }
	
	/// Returns the abbreviated name or symbol of voltage: "V" for volts
	pub fn unit_symbol() -> &'static str { "V" }
	
	/// Returns a new voltage value from the given number of volts
	///
	/// # Arguments
	/// * `V` - Any number-like type, representing a quantity of volts
	pub fn from_V(V: T) -> Self { Voltage{V: V} }
	
	/// Returns a copy of this voltage value in volts
	pub fn to_V(&self) -> T { self.V.clone() }

	/// Returns a new voltage value from the given number of volts
	///
	/// # Arguments
	/// * `volts` - Any number-like type, representing a quantity of volts
	pub fn from_volts(volts: T) -> Self { Voltage{V: volts} }
	
	/// Returns a copy of this voltage value in volts
	pub fn to_volts(&self) -> T { self.V.clone() }

}

impl<T> fmt::Display for Voltage<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.V, Self::unit_symbol())
	}
}

impl<T> Voltage<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this voltage value in millivolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mV(&self) -> T {
		return self.V.clone() * T::from(1000.0_f64);
	}

	/// Returns a new voltage value from the given number of millivolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mV` - Any number-like type, representing a quantity of millivolts
	pub fn from_mV(mV: T) -> Self {
		Voltage{V: mV * T::from(0.001_f64)}
	}

	/// Returns a copy of this voltage value in microvolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uV(&self) -> T {
		return self.V.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new voltage value from the given number of microvolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uV` - Any number-like type, representing a quantity of microvolts
	pub fn from_uV(uV: T) -> Self {
		Voltage{V: uV * T::from(1e-06_f64)}
	}

	/// Returns a copy of this voltage value in nanovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nV(&self) -> T {
		return self.V.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new voltage value from the given number of nanovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nV` - Any number-like type, representing a quantity of nanovolts
	pub fn from_nV(nV: T) -> Self {
		Voltage{V: nV * T::from(1e-09_f64)}
	}

	/// Returns a copy of this voltage value in kilovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_kV(&self) -> T {
		return self.V.clone() * T::from(0.001_f64);
	}

	/// Returns a new voltage value from the given number of kilovolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `kV` - Any number-like type, representing a quantity of kilovolts
	pub fn from_kV(kV: T) -> Self {
		Voltage{V: kV * T::from(1000.0_f64)}
	}

	/// Returns a copy of this voltage value in megavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_MV(&self) -> T {
		return self.V.clone() * T::from(1e-06_f64);
	}

	/// Returns a new voltage value from the given number of megavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `MV` - Any number-like type, representing a quantity of megavolts
	pub fn from_MV(MV: T) -> Self {
		Voltage{V: MV * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this voltage value in gigavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GV(&self) -> T {
		return self.V.clone() * T::from(1e-09_f64);
	}

	/// Returns a new voltage value from the given number of gigavolts
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GV` - Any number-like type, representing a quantity of gigavolts
	pub fn from_GV(GV: T) -> Self {
		Voltage{V: GV * T::from(1000000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Voltage<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Voltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Voltage<num_bigfloat::BigFloat>) -> Self::Output {
		Voltage{V: self * rhs.V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Voltage<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Voltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Voltage<num_bigfloat::BigFloat>) -> Self::Output {
		Voltage{V: self.clone() * rhs.V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Voltage<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Voltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Voltage<num_bigfloat::BigFloat>) -> Self::Output {
		Voltage{V: self * rhs.V.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Voltage<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Voltage<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Voltage<num_bigfloat::BigFloat>) -> Self::Output {
		Voltage{V: self.clone() * rhs.V.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Voltage<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Voltage<num_complex::Complex32>;
	fn mul(self, rhs: Voltage<num_complex::Complex32>) -> Self::Output {
		Voltage{V: self * rhs.V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Voltage<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Voltage<num_complex::Complex32>;
	fn mul(self, rhs: Voltage<num_complex::Complex32>) -> Self::Output {
		Voltage{V: self.clone() * rhs.V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Voltage<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Voltage<num_complex::Complex32>;
	fn mul(self, rhs: &Voltage<num_complex::Complex32>) -> Self::Output {
		Voltage{V: self * rhs.V.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Voltage<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Voltage<num_complex::Complex32>;
	fn mul(self, rhs: &Voltage<num_complex::Complex32>) -> Self::Output {
		Voltage{V: self.clone() * rhs.V.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Voltage<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Voltage<num_complex::Complex64>;
	fn mul(self, rhs: Voltage<num_complex::Complex64>) -> Self::Output {
		Voltage{V: self * rhs.V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Voltage<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Voltage<num_complex::Complex64>;
	fn mul(self, rhs: Voltage<num_complex::Complex64>) -> Self::Output {
		Voltage{V: self.clone() * rhs.V}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Voltage<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Voltage<num_complex::Complex64>;
	fn mul(self, rhs: &Voltage<num_complex::Complex64>) -> Self::Output {
		Voltage{V: self * rhs.V.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Voltage<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Voltage<num_complex::Complex64>;
	fn mul(self, rhs: &Voltage<num_complex::Complex64>) -> Self::Output {
		Voltage{V: self.clone() * rhs.V.clone()}
	}
}



/// Converts a Voltage into the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricPotential> for Voltage<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricPotential {
		uom::si::f32::ElectricPotential::new::<uom::si::electric_potential::volt>(self.V.into())
	}
}

/// Creates a Voltage from the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricPotential> for Voltage<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricPotential) -> Self {
		Voltage{V: T::from(src.value)}
	}
}

/// Converts a Voltage into the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricPotential> for Voltage<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricPotential {
		uom::si::f64::ElectricPotential::new::<uom::si::electric_potential::volt>(self.V.into())
	}
}

/// Creates a Voltage from the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricPotential> for Voltage<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricPotential) -> Self {
		Voltage{V: T::from(src.value)}
	}
}


// Voltage * Current -> Power
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> core::ops::Mul<Current<T>> for Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Power{W: self.V * rhs.A}
	}
}
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> core::ops::Mul<Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Power{W: self.V.clone() * rhs.A}
	}
}
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> core::ops::Mul<&Current<T>> for Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Power{W: self.V * rhs.A.clone()}
	}
}
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> core::ops::Mul<&Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Power{W: self.V.clone() * rhs.A.clone()}
	}
}

// Voltage / Current -> Resistance
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> core::ops::Div<Current<T>> for Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Resistance{Ohm: self.V / rhs.A}
	}
}
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> core::ops::Div<Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Resistance{Ohm: self.V.clone() / rhs.A}
	}
}
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> core::ops::Div<&Current<T>> for Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Resistance{Ohm: self.V / rhs.A.clone()}
	}
}
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> core::ops::Div<&Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Resistance{Ohm: self.V.clone() / rhs.A.clone()}
	}
}

// Voltage * InverseCurrent -> Resistance
/// Multiplying a Voltage by a InverseCurrent returns a value of type Resistance
impl<T> core::ops::Mul<InverseCurrent<T>> for Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		Resistance{Ohm: self.V * rhs.per_A}
	}
}
/// Multiplying a Voltage by a InverseCurrent returns a value of type Resistance
impl<T> core::ops::Mul<InverseCurrent<T>> for &Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: InverseCurrent<T>) -> Self::Output {
		Resistance{Ohm: self.V.clone() * rhs.per_A}
	}
}
/// Multiplying a Voltage by a InverseCurrent returns a value of type Resistance
impl<T> core::ops::Mul<&InverseCurrent<T>> for Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Resistance{Ohm: self.V * rhs.per_A.clone()}
	}
}
/// Multiplying a Voltage by a InverseCurrent returns a value of type Resistance
impl<T> core::ops::Mul<&InverseCurrent<T>> for &Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Resistance{Ohm: self.V.clone() * rhs.per_A.clone()}
	}
}

// Voltage / InverseCurrent -> Power
/// Dividing a Voltage by a InverseCurrent returns a value of type Power
impl<T> core::ops::Div<InverseCurrent<T>> for Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Power{W: self.V / rhs.per_A}
	}
}
/// Dividing a Voltage by a InverseCurrent returns a value of type Power
impl<T> core::ops::Div<InverseCurrent<T>> for &Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: InverseCurrent<T>) -> Self::Output {
		Power{W: self.V.clone() / rhs.per_A}
	}
}
/// Dividing a Voltage by a InverseCurrent returns a value of type Power
impl<T> core::ops::Div<&InverseCurrent<T>> for Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Power{W: self.V / rhs.per_A.clone()}
	}
}
/// Dividing a Voltage by a InverseCurrent returns a value of type Power
impl<T> core::ops::Div<&InverseCurrent<T>> for &Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &InverseCurrent<T>) -> Self::Output {
		Power{W: self.V.clone() / rhs.per_A.clone()}
	}
}

// Voltage * Time -> MagneticFlux
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> core::ops::Mul<Time<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V * rhs.s}
	}
}
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> core::ops::Mul<Time<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() * rhs.s}
	}
}
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Time<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V * rhs.s.clone()}
	}
}
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> core::ops::Mul<&Time<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() * rhs.s.clone()}
	}
}

// Voltage * Capacitance -> Charge
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> core::ops::Mul<Capacitance<T>> for Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Charge{C: self.V * rhs.F}
	}
}
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> core::ops::Mul<Capacitance<T>> for &Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Charge{C: self.V.clone() * rhs.F}
	}
}
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> core::ops::Mul<&Capacitance<T>> for Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Charge{C: self.V * rhs.F.clone()}
	}
}
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> core::ops::Mul<&Capacitance<T>> for &Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Charge{C: self.V.clone() * rhs.F.clone()}
	}
}

// Voltage * Charge -> Energy
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> core::ops::Mul<Charge<T>> for Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Energy{J: self.V * rhs.C}
	}
}
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> core::ops::Mul<Charge<T>> for &Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Energy{J: self.V.clone() * rhs.C}
	}
}
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> core::ops::Mul<&Charge<T>> for Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Energy{J: self.V * rhs.C.clone()}
	}
}
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> core::ops::Mul<&Charge<T>> for &Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Energy{J: self.V.clone() * rhs.C.clone()}
	}
}

// Voltage / Charge -> Elastance
/// Dividing a Voltage by a Charge returns a value of type Elastance
impl<T> core::ops::Div<Charge<T>> for Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Elastance{per_F: self.V / rhs.C}
	}
}
/// Dividing a Voltage by a Charge returns a value of type Elastance
impl<T> core::ops::Div<Charge<T>> for &Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Elastance{per_F: self.V.clone() / rhs.C}
	}
}
/// Dividing a Voltage by a Charge returns a value of type Elastance
impl<T> core::ops::Div<&Charge<T>> for Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Elastance{per_F: self.V / rhs.C.clone()}
	}
}
/// Dividing a Voltage by a Charge returns a value of type Elastance
impl<T> core::ops::Div<&Charge<T>> for &Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Elastance{per_F: self.V.clone() / rhs.C.clone()}
	}
}

// Voltage * Conductance -> Current
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> core::ops::Mul<Conductance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Current{A: self.V * rhs.S}
	}
}
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> core::ops::Mul<Conductance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Current{A: self.V.clone() * rhs.S}
	}
}
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> core::ops::Mul<&Conductance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Current{A: self.V * rhs.S.clone()}
	}
}
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> core::ops::Mul<&Conductance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Current{A: self.V.clone() * rhs.S.clone()}
	}
}

// Voltage / Elastance -> Charge
/// Dividing a Voltage by a Elastance returns a value of type Charge
impl<T> core::ops::Div<Elastance<T>> for Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Charge{C: self.V / rhs.per_F}
	}
}
/// Dividing a Voltage by a Elastance returns a value of type Charge
impl<T> core::ops::Div<Elastance<T>> for &Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Elastance<T>) -> Self::Output {
		Charge{C: self.V.clone() / rhs.per_F}
	}
}
/// Dividing a Voltage by a Elastance returns a value of type Charge
impl<T> core::ops::Div<&Elastance<T>> for Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Charge{C: self.V / rhs.per_F.clone()}
	}
}
/// Dividing a Voltage by a Elastance returns a value of type Charge
impl<T> core::ops::Div<&Elastance<T>> for &Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Elastance<T>) -> Self::Output {
		Charge{C: self.V.clone() / rhs.per_F.clone()}
	}
}

// Voltage * InverseCharge -> Elastance
/// Multiplying a Voltage by a InverseCharge returns a value of type Elastance
impl<T> core::ops::Mul<InverseCharge<T>> for Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		Elastance{per_F: self.V * rhs.per_C}
	}
}
/// Multiplying a Voltage by a InverseCharge returns a value of type Elastance
impl<T> core::ops::Mul<InverseCharge<T>> for &Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: InverseCharge<T>) -> Self::Output {
		Elastance{per_F: self.V.clone() * rhs.per_C}
	}
}
/// Multiplying a Voltage by a InverseCharge returns a value of type Elastance
impl<T> core::ops::Mul<&InverseCharge<T>> for Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		Elastance{per_F: self.V * rhs.per_C.clone()}
	}
}
/// Multiplying a Voltage by a InverseCharge returns a value of type Elastance
impl<T> core::ops::Mul<&InverseCharge<T>> for &Voltage<T> where T: NumLike {
	type Output = Elastance<T>;
	fn mul(self, rhs: &InverseCharge<T>) -> Self::Output {
		Elastance{per_F: self.V.clone() * rhs.per_C.clone()}
	}
}

// Voltage / InverseCharge -> Energy
/// Dividing a Voltage by a InverseCharge returns a value of type Energy
impl<T> core::ops::Div<InverseCharge<T>> for Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Energy{J: self.V / rhs.per_C}
	}
}
/// Dividing a Voltage by a InverseCharge returns a value of type Energy
impl<T> core::ops::Div<InverseCharge<T>> for &Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InverseCharge<T>) -> Self::Output {
		Energy{J: self.V.clone() / rhs.per_C}
	}
}
/// Dividing a Voltage by a InverseCharge returns a value of type Energy
impl<T> core::ops::Div<&InverseCharge<T>> for Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Energy{J: self.V / rhs.per_C.clone()}
	}
}
/// Dividing a Voltage by a InverseCharge returns a value of type Energy
impl<T> core::ops::Div<&InverseCharge<T>> for &Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InverseCharge<T>) -> Self::Output {
		Energy{J: self.V.clone() / rhs.per_C.clone()}
	}
}

// Voltage * InverseMagneticFlux -> Frequency
/// Multiplying a Voltage by a InverseMagneticFlux returns a value of type Frequency
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V * rhs.per_Wb}
	}
}
/// Multiplying a Voltage by a InverseMagneticFlux returns a value of type Frequency
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Voltage by a InverseMagneticFlux returns a value of type Frequency
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Voltage by a InverseMagneticFlux returns a value of type Frequency
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V.clone() * rhs.per_Wb.clone()}
	}
}

// Voltage / MagneticFlux -> Frequency
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> core::ops::Div<MagneticFlux<T>> for Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V / rhs.Wb}
	}
}
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> core::ops::Div<MagneticFlux<T>> for &Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V.clone() / rhs.Wb}
	}
}
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> core::ops::Div<&MagneticFlux<T>> for Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V / rhs.Wb.clone()}
	}
}
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> core::ops::Div<&MagneticFlux<T>> for &Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V.clone() / rhs.Wb.clone()}
	}
}

// Voltage / Resistance -> Current
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> core::ops::Div<Resistance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Current{A: self.V / rhs.Ohm}
	}
}
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> core::ops::Div<Resistance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Current{A: self.V.clone() / rhs.Ohm}
	}
}
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> core::ops::Div<&Resistance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Current{A: self.V / rhs.Ohm.clone()}
	}
}
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> core::ops::Div<&Resistance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Current{A: self.V.clone() / rhs.Ohm.clone()}
	}
}

// Voltage / Energy -> InverseCharge
/// Dividing a Voltage by a Energy returns a value of type InverseCharge
impl<T> core::ops::Div<Energy<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseCharge{per_C: self.V / rhs.J}
	}
}
/// Dividing a Voltage by a Energy returns a value of type InverseCharge
impl<T> core::ops::Div<Energy<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() / rhs.J}
	}
}
/// Dividing a Voltage by a Energy returns a value of type InverseCharge
impl<T> core::ops::Div<&Energy<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseCharge{per_C: self.V / rhs.J.clone()}
	}
}
/// Dividing a Voltage by a Energy returns a value of type InverseCharge
impl<T> core::ops::Div<&Energy<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() / rhs.J.clone()}
	}
}

// Voltage / Torque -> InverseCharge
/// Dividing a Voltage by a Torque returns a value of type InverseCharge
impl<T> core::ops::Div<Torque<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseCharge{per_C: self.V / rhs.Nm}
	}
}
/// Dividing a Voltage by a Torque returns a value of type InverseCharge
impl<T> core::ops::Div<Torque<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() / rhs.Nm}
	}
}
/// Dividing a Voltage by a Torque returns a value of type InverseCharge
impl<T> core::ops::Div<&Torque<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseCharge{per_C: self.V / rhs.Nm.clone()}
	}
}
/// Dividing a Voltage by a Torque returns a value of type InverseCharge
impl<T> core::ops::Div<&Torque<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() / rhs.Nm.clone()}
	}
}

// Voltage / Frequency -> MagneticFlux
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> core::ops::Div<Frequency<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V / rhs.Hz}
	}
}
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> core::ops::Div<Frequency<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() / rhs.Hz}
	}
}
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> core::ops::Div<&Frequency<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V / rhs.Hz.clone()}
	}
}
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> core::ops::Div<&Frequency<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() / rhs.Hz.clone()}
	}
}

// Voltage * InverseEnergy -> InverseCharge
/// Multiplying a Voltage by a InverseEnergy returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseEnergy<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseCharge{per_C: self.V * rhs.per_J}
	}
}
/// Multiplying a Voltage by a InverseEnergy returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseEnergy<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() * rhs.per_J}
	}
}
/// Multiplying a Voltage by a InverseEnergy returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseEnergy<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseCharge{per_C: self.V * rhs.per_J.clone()}
	}
}
/// Multiplying a Voltage by a InverseEnergy returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseEnergy<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() * rhs.per_J.clone()}
	}
}

// Voltage * InverseTorque -> InverseCharge
/// Multiplying a Voltage by a InverseTorque returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseTorque<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseCharge{per_C: self.V * rhs.per_Nm}
	}
}
/// Multiplying a Voltage by a InverseTorque returns a value of type InverseCharge
impl<T> core::ops::Mul<InverseTorque<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() * rhs.per_Nm}
	}
}
/// Multiplying a Voltage by a InverseTorque returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseTorque<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseCharge{per_C: self.V * rhs.per_Nm.clone()}
	}
}
/// Multiplying a Voltage by a InverseTorque returns a value of type InverseCharge
impl<T> core::ops::Mul<&InverseTorque<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCharge<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InverseCharge{per_C: self.V.clone() * rhs.per_Nm.clone()}
	}
}

// Voltage * InversePower -> InverseCurrent
/// Multiplying a Voltage by a InversePower returns a value of type InverseCurrent
impl<T> core::ops::Mul<InversePower<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InversePower<T>) -> Self::Output {
		InverseCurrent{per_A: self.V * rhs.per_W}
	}
}
/// Multiplying a Voltage by a InversePower returns a value of type InverseCurrent
impl<T> core::ops::Mul<InversePower<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: InversePower<T>) -> Self::Output {
		InverseCurrent{per_A: self.V.clone() * rhs.per_W}
	}
}
/// Multiplying a Voltage by a InversePower returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InversePower<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InversePower<T>) -> Self::Output {
		InverseCurrent{per_A: self.V * rhs.per_W.clone()}
	}
}
/// Multiplying a Voltage by a InversePower returns a value of type InverseCurrent
impl<T> core::ops::Mul<&InversePower<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn mul(self, rhs: &InversePower<T>) -> Self::Output {
		InverseCurrent{per_A: self.V.clone() * rhs.per_W.clone()}
	}
}

// Voltage / Power -> InverseCurrent
/// Dividing a Voltage by a Power returns a value of type InverseCurrent
impl<T> core::ops::Div<Power<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		InverseCurrent{per_A: self.V / rhs.W}
	}
}
/// Dividing a Voltage by a Power returns a value of type InverseCurrent
impl<T> core::ops::Div<Power<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		InverseCurrent{per_A: self.V.clone() / rhs.W}
	}
}
/// Dividing a Voltage by a Power returns a value of type InverseCurrent
impl<T> core::ops::Div<&Power<T>> for Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		InverseCurrent{per_A: self.V / rhs.W.clone()}
	}
}
/// Dividing a Voltage by a Power returns a value of type InverseCurrent
impl<T> core::ops::Div<&Power<T>> for &Voltage<T> where T: NumLike {
	type Output = InverseCurrent<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		InverseCurrent{per_A: self.V.clone() / rhs.W.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<Voltage<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
impl<T> core::ops::Div<&Voltage<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Voltage<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Voltage<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Voltage<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Voltage<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Voltage<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Voltage<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Voltage<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Voltage<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}

// 1/Voltage -> InverseVoltage
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Voltage<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Voltage<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Voltage<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self) / rhs.V.clone()}
	}
}
/// Dividing a scalar value by a Voltage unit value returns a value of type InverseVoltage
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Voltage<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVoltage<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		InverseVoltage{per_V: T::from(self.clone()) / rhs.V.clone()}
	}
}



