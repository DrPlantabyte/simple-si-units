
//! This module provides geometry SI units, such as angle 
//! and inverse of volume.
use core::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::mechanical::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num-bigfloat")]
use num_bigfloat;
#[cfg(feature="num-complex")]
use num_complex;



/// The angle unit type, defined as radians in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Angle<T: NumLike>{
	/// The value of this Angle in radians
	pub rad: T
}

impl<T> Angle<T> where T: NumLike {

	/// Returns the standard unit name of angle: "radians"
	pub fn unit_name() -> &'static str { "radians" }
	
	/// Returns the abbreviated name or symbol of angle: "rad" for radians
	pub fn unit_symbol() -> &'static str { "rad" }
	
	/// Returns a new angle value from the given number of radians
	///
	/// # Arguments
	/// * `rad` - Any number-like type, representing a quantity of radians
	pub fn from_rad(rad: T) -> Self { Angle{rad: rad} }
	
	/// Returns a copy of this angle value in radians
	pub fn to_rad(&self) -> T { self.rad.clone() }

	/// Returns a new angle value from the given number of radians
	///
	/// # Arguments
	/// * `radians` - Any number-like type, representing a quantity of radians
	pub fn from_radians(radians: T) -> Self { Angle{rad: radians} }
	
	/// Returns a copy of this angle value in radians
	pub fn to_radians(&self) -> T { self.rad.clone() }

}

impl<T> fmt::Display for Angle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.rad, Self::unit_symbol())
	}
}

impl<T> Angle<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this angle value in degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_degrees(&self) -> T {
		return self.rad.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angle value from the given number of degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `degrees` - Any number-like type, representing a quantity of degrees
	pub fn from_degrees(degrees: T) -> Self {
		Angle{rad: degrees * T::from(0.0174532925199433_f64)}
	}

	/// Returns a copy of this angle value in degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_deg(&self) -> T {
		return self.rad.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angle value from the given number of degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `deg` - Any number-like type, representing a quantity of degrees
	pub fn from_deg(deg: T) -> Self {
		Angle{rad: deg * T::from(0.0174532925199433_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Angle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Angle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Angle<num_bigfloat::BigFloat>) -> Self::Output {
		Angle{rad: self * rhs.rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Angle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Angle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Angle<num_bigfloat::BigFloat>) -> Self::Output {
		Angle{rad: self.clone() * rhs.rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Angle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Angle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Angle<num_bigfloat::BigFloat>) -> Self::Output {
		Angle{rad: self * rhs.rad.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Angle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Angle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Angle<num_bigfloat::BigFloat>) -> Self::Output {
		Angle{rad: self.clone() * rhs.rad.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Angle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Angle<num_complex::Complex32>;
	fn mul(self, rhs: Angle<num_complex::Complex32>) -> Self::Output {
		Angle{rad: self * rhs.rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Angle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Angle<num_complex::Complex32>;
	fn mul(self, rhs: Angle<num_complex::Complex32>) -> Self::Output {
		Angle{rad: self.clone() * rhs.rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Angle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Angle<num_complex::Complex32>;
	fn mul(self, rhs: &Angle<num_complex::Complex32>) -> Self::Output {
		Angle{rad: self * rhs.rad.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Angle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Angle<num_complex::Complex32>;
	fn mul(self, rhs: &Angle<num_complex::Complex32>) -> Self::Output {
		Angle{rad: self.clone() * rhs.rad.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Angle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Angle<num_complex::Complex64>;
	fn mul(self, rhs: Angle<num_complex::Complex64>) -> Self::Output {
		Angle{rad: self * rhs.rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Angle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Angle<num_complex::Complex64>;
	fn mul(self, rhs: Angle<num_complex::Complex64>) -> Self::Output {
		Angle{rad: self.clone() * rhs.rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Angle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Angle<num_complex::Complex64>;
	fn mul(self, rhs: &Angle<num_complex::Complex64>) -> Self::Output {
		Angle{rad: self * rhs.rad.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Angle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Angle<num_complex::Complex64>;
	fn mul(self, rhs: &Angle<num_complex::Complex64>) -> Self::Output {
		Angle{rad: self.clone() * rhs.rad.clone()}
	}
}



/// Converts a Angle into the equivalent [uom](https://crates.io/crates/uom) type [Angle](https://docs.rs/uom/0.34.0/uom/si/f32/type.Angle.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Angle> for Angle<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Angle {
		uom::si::f32::Angle::new::<uom::si::angle::radian>(self.rad.into())
	}
}

/// Creates a Angle from the equivalent [uom](https://crates.io/crates/uom) type [Angle](https://docs.rs/uom/0.34.0/uom/si/f32/type.Angle.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Angle> for Angle<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Angle) -> Self {
		Angle{rad: T::from(src.value)}
	}
}

/// Converts a Angle into the equivalent [uom](https://crates.io/crates/uom) type [Angle](https://docs.rs/uom/0.34.0/uom/si/f64/type.Angle.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Angle> for Angle<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Angle {
		uom::si::f64::Angle::new::<uom::si::angle::radian>(self.rad.into())
	}
}

/// Creates a Angle from the equivalent [uom](https://crates.io/crates/uom) type [Angle](https://docs.rs/uom/0.34.0/uom/si/f64/type.Angle.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Angle> for Angle<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Angle) -> Self {
		Angle{rad: T::from(src.value)}
	}
}


// Angle / Time -> AngularVelocity
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> core::ops::Div<Time<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad / rhs.s}
	}
}
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> core::ops::Div<Time<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() / rhs.s}
	}
}
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> core::ops::Div<&Time<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad / rhs.s.clone()}
	}
}
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> core::ops::Div<&Time<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() / rhs.s.clone()}
	}
}

// Angle * Angle -> SolidAngle
/// Multiplying a Angle by a Angle returns a value of type SolidAngle
impl<T> core::ops::Mul<Angle<T>> for Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: Angle<T>) -> Self::Output {
		SolidAngle{sr: self.rad * rhs.rad}
	}
}
/// Multiplying a Angle by a Angle returns a value of type SolidAngle
impl<T> core::ops::Mul<Angle<T>> for &Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: Angle<T>) -> Self::Output {
		SolidAngle{sr: self.rad.clone() * rhs.rad}
	}
}
/// Multiplying a Angle by a Angle returns a value of type SolidAngle
impl<T> core::ops::Mul<&Angle<T>> for Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: &Angle<T>) -> Self::Output {
		SolidAngle{sr: self.rad * rhs.rad.clone()}
	}
}
/// Multiplying a Angle by a Angle returns a value of type SolidAngle
impl<T> core::ops::Mul<&Angle<T>> for &Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn mul(self, rhs: &Angle<T>) -> Self::Output {
		SolidAngle{sr: self.rad.clone() * rhs.rad.clone()}
	}
}

// Angle / InverseAngle -> SolidAngle
/// Dividing a Angle by a InverseAngle returns a value of type SolidAngle
impl<T> core::ops::Div<InverseAngle<T>> for Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		SolidAngle{sr: self.rad / rhs.per_rad}
	}
}
/// Dividing a Angle by a InverseAngle returns a value of type SolidAngle
impl<T> core::ops::Div<InverseAngle<T>> for &Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		SolidAngle{sr: self.rad.clone() / rhs.per_rad}
	}
}
/// Dividing a Angle by a InverseAngle returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseAngle<T>> for Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		SolidAngle{sr: self.rad / rhs.per_rad.clone()}
	}
}
/// Dividing a Angle by a InverseAngle returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseAngle<T>> for &Angle<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		SolidAngle{sr: self.rad.clone() / rhs.per_rad.clone()}
	}
}

// Angle * InverseSolidAngle -> InverseAngle
/// Multiplying a Angle by a InverseSolidAngle returns a value of type InverseAngle
impl<T> core::ops::Mul<InverseSolidAngle<T>> for Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad * rhs.per_sr}
	}
}
/// Multiplying a Angle by a InverseSolidAngle returns a value of type InverseAngle
impl<T> core::ops::Mul<InverseSolidAngle<T>> for &Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad.clone() * rhs.per_sr}
	}
}
/// Multiplying a Angle by a InverseSolidAngle returns a value of type InverseAngle
impl<T> core::ops::Mul<&InverseSolidAngle<T>> for Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad * rhs.per_sr.clone()}
	}
}
/// Multiplying a Angle by a InverseSolidAngle returns a value of type InverseAngle
impl<T> core::ops::Mul<&InverseSolidAngle<T>> for &Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad.clone() * rhs.per_sr.clone()}
	}
}

// Angle / SolidAngle -> InverseAngle
/// Dividing a Angle by a SolidAngle returns a value of type InverseAngle
impl<T> core::ops::Div<SolidAngle<T>> for Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad / rhs.sr}
	}
}
/// Dividing a Angle by a SolidAngle returns a value of type InverseAngle
impl<T> core::ops::Div<SolidAngle<T>> for &Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad.clone() / rhs.sr}
	}
}
/// Dividing a Angle by a SolidAngle returns a value of type InverseAngle
impl<T> core::ops::Div<&SolidAngle<T>> for Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad / rhs.sr.clone()}
	}
}
/// Dividing a Angle by a SolidAngle returns a value of type InverseAngle
impl<T> core::ops::Div<&SolidAngle<T>> for &Angle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.rad.clone() / rhs.sr.clone()}
	}
}

// Angle / AngularVelocity -> Time
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> core::ops::Div<AngularVelocity<T>> for Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad / rhs.radps}
	}
}
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> core::ops::Div<AngularVelocity<T>> for &Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad.clone() / rhs.radps}
	}
}
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> core::ops::Div<&AngularVelocity<T>> for Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad / rhs.radps.clone()}
	}
}
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> core::ops::Div<&AngularVelocity<T>> for &Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad.clone() / rhs.radps.clone()}
	}
}

// Angle * Frequency -> AngularVelocity
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> core::ops::Mul<Frequency<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad * rhs.Hz}
	}
}
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> core::ops::Mul<Frequency<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() * rhs.Hz}
	}
}
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> core::ops::Mul<&Frequency<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad * rhs.Hz.clone()}
	}
}
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> core::ops::Mul<&Frequency<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() * rhs.Hz.clone()}
	}
}

// Angle * InverseAngularVelocity -> Time
/// Multiplying a Angle by a InverseAngularVelocity returns a value of type Time
impl<T> core::ops::Mul<InverseAngularVelocity<T>> for Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		Time{s: self.rad * rhs.s_per_rad}
	}
}
/// Multiplying a Angle by a InverseAngularVelocity returns a value of type Time
impl<T> core::ops::Mul<InverseAngularVelocity<T>> for &Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		Time{s: self.rad.clone() * rhs.s_per_rad}
	}
}
/// Multiplying a Angle by a InverseAngularVelocity returns a value of type Time
impl<T> core::ops::Mul<&InverseAngularVelocity<T>> for Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		Time{s: self.rad * rhs.s_per_rad.clone()}
	}
}
/// Multiplying a Angle by a InverseAngularVelocity returns a value of type Time
impl<T> core::ops::Mul<&InverseAngularVelocity<T>> for &Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		Time{s: self.rad.clone() * rhs.s_per_rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<Angle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
impl<T> core::ops::Div<&Angle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Angle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Angle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Angle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Angle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Angle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Angle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Angle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Angle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

// 1/Angle -> InverseAngle
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Angle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Angle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Angle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self) / rhs.rad.clone()}
	}
}
/// Dividing a scalar value by a Angle unit value returns a value of type InverseAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Angle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: T::from(self.clone()) / rhs.rad.clone()}
	}
}

/// The area unit type, defined as square meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Area<T: NumLike>{
	/// The value of this Area in square meters
	pub m2: T
}

impl<T> Area<T> where T: NumLike {

	/// Returns the standard unit name of area: "square meters"
	pub fn unit_name() -> &'static str { "square meters" }
	
	/// Returns the abbreviated name or symbol of area: "m²" for square meters
	pub fn unit_symbol() -> &'static str { "m²" }
	
	/// Returns a new area value from the given number of square meters
	///
	/// # Arguments
	/// * `m2` - Any number-like type, representing a quantity of square meters
	pub fn from_m2(m2: T) -> Self { Area{m2: m2} }
	
	/// Returns a copy of this area value in square meters
	pub fn to_m2(&self) -> T { self.m2.clone() }

	/// Returns a new area value from the given number of square meters
	///
	/// # Arguments
	/// * `square_meters` - Any number-like type, representing a quantity of square meters
	pub fn from_square_meters(square_meters: T) -> Self { Area{m2: square_meters} }
	
	/// Returns a copy of this area value in square meters
	pub fn to_square_meters(&self) -> T { self.m2.clone() }

}

impl<T> fmt::Display for Area<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m2, Self::unit_symbol())
	}
}

impl<T> Area<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this area value in square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_cm2(&self) -> T {
		return self.m2.clone() * T::from(10000.0_f64);
	}

	/// Returns a new area value from the given number of square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `cm2` - Any number-like type, representing a quantity of square cm
	pub fn from_cm2(cm2: T) -> Self {
		Area{m2: cm2 * T::from(0.0001_f64)}
	}

	/// Returns a copy of this area value in square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_square_cm(&self) -> T {
		return self.m2.clone() * T::from(10000.0_f64);
	}

	/// Returns a new area value from the given number of square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `square_cm` - Any number-like type, representing a quantity of square cm
	pub fn from_square_cm(square_cm: T) -> Self {
		Area{m2: square_cm * T::from(0.0001_f64)}
	}

	/// Returns a copy of this area value in square mm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mm2(&self) -> T {
		return self.m2.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new area value from the given number of square mm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mm2` - Any number-like type, representing a quantity of square mm
	pub fn from_mm2(mm2: T) -> Self {
		Area{m2: mm2 * T::from(1e-06_f64)}
	}

	/// Returns a copy of this area value in square um
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_um2(&self) -> T {
		return self.m2.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new area value from the given number of square um
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `um2` - Any number-like type, representing a quantity of square um
	pub fn from_um2(um2: T) -> Self {
		Area{m2: um2 * T::from(1e-12_f64)}
	}

	/// Returns a copy of this area value in square nm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nm2(&self) -> T {
		return self.m2.clone() * T::from(1e+18_f64);
	}

	/// Returns a new area value from the given number of square nm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nm2` - Any number-like type, representing a quantity of square nm
	pub fn from_nm2(nm2: T) -> Self {
		Area{m2: nm2 * T::from(1e-18_f64)}
	}

	/// Returns a copy of this area value in square km
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_km2(&self) -> T {
		return self.m2.clone() * T::from(1e-06_f64);
	}

	/// Returns a new area value from the given number of square km
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `km2` - Any number-like type, representing a quantity of square km
	pub fn from_km2(km2: T) -> Self {
		Area{m2: km2 * T::from(1000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Area<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Area<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Area<num_bigfloat::BigFloat>) -> Self::Output {
		Area{m2: self * rhs.m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Area<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Area<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Area<num_bigfloat::BigFloat>) -> Self::Output {
		Area{m2: self.clone() * rhs.m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Area<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Area<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Area<num_bigfloat::BigFloat>) -> Self::Output {
		Area{m2: self * rhs.m2.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Area<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Area<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Area<num_bigfloat::BigFloat>) -> Self::Output {
		Area{m2: self.clone() * rhs.m2.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Area<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Area<num_complex::Complex32>;
	fn mul(self, rhs: Area<num_complex::Complex32>) -> Self::Output {
		Area{m2: self * rhs.m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Area<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Area<num_complex::Complex32>;
	fn mul(self, rhs: Area<num_complex::Complex32>) -> Self::Output {
		Area{m2: self.clone() * rhs.m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Area<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Area<num_complex::Complex32>;
	fn mul(self, rhs: &Area<num_complex::Complex32>) -> Self::Output {
		Area{m2: self * rhs.m2.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Area<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Area<num_complex::Complex32>;
	fn mul(self, rhs: &Area<num_complex::Complex32>) -> Self::Output {
		Area{m2: self.clone() * rhs.m2.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Area<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Area<num_complex::Complex64>;
	fn mul(self, rhs: Area<num_complex::Complex64>) -> Self::Output {
		Area{m2: self * rhs.m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Area<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Area<num_complex::Complex64>;
	fn mul(self, rhs: Area<num_complex::Complex64>) -> Self::Output {
		Area{m2: self.clone() * rhs.m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Area<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Area<num_complex::Complex64>;
	fn mul(self, rhs: &Area<num_complex::Complex64>) -> Self::Output {
		Area{m2: self * rhs.m2.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Area<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Area<num_complex::Complex64>;
	fn mul(self, rhs: &Area<num_complex::Complex64>) -> Self::Output {
		Area{m2: self.clone() * rhs.m2.clone()}
	}
}



/// Converts a Area into the equivalent [uom](https://crates.io/crates/uom) type [Area](https://docs.rs/uom/0.34.0/uom/si/f32/type.Area.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Area> for Area<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Area {
		uom::si::f32::Area::new::<uom::si::area::square_meter>(self.m2.into())
	}
}

/// Creates a Area from the equivalent [uom](https://crates.io/crates/uom) type [Area](https://docs.rs/uom/0.34.0/uom/si/f32/type.Area.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Area> for Area<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Area) -> Self {
		Area{m2: T::from(src.value)}
	}
}

/// Converts a Area into the equivalent [uom](https://crates.io/crates/uom) type [Area](https://docs.rs/uom/0.34.0/uom/si/f64/type.Area.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Area> for Area<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Area {
		uom::si::f64::Area::new::<uom::si::area::square_meter>(self.m2.into())
	}
}

/// Creates a Area from the equivalent [uom](https://crates.io/crates/uom) type [Area](https://docs.rs/uom/0.34.0/uom/si/f64/type.Area.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Area> for Area<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Area) -> Self {
		Area{m2: T::from(src.value)}
	}
}


// Area * Distance -> Volume
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> core::ops::Mul<Distance<T>> for Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Volume{m3: self.m2 * rhs.m}
	}
}
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> core::ops::Mul<Distance<T>> for &Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Volume{m3: self.m2.clone() * rhs.m}
	}
}
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> core::ops::Mul<&Distance<T>> for Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Volume{m3: self.m2 * rhs.m.clone()}
	}
}
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> core::ops::Mul<&Distance<T>> for &Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Volume{m3: self.m2.clone() * rhs.m.clone()}
	}
}

// Area / Distance -> Distance
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> core::ops::Div<Distance<T>> for Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Distance{m: self.m2 / rhs.m}
	}
}
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> core::ops::Div<Distance<T>> for &Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Distance{m: self.m2.clone() / rhs.m}
	}
}
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> core::ops::Div<&Distance<T>> for Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Distance{m: self.m2 / rhs.m.clone()}
	}
}
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> core::ops::Div<&Distance<T>> for &Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Distance{m: self.m2.clone() / rhs.m.clone()}
	}
}

// Area * InverseDistance -> Distance
/// Multiplying a Area by a InverseDistance returns a value of type Distance
impl<T> core::ops::Mul<InverseDistance<T>> for Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: self.m2 * rhs.per_m}
	}
}
/// Multiplying a Area by a InverseDistance returns a value of type Distance
impl<T> core::ops::Mul<InverseDistance<T>> for &Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		Distance{m: self.m2.clone() * rhs.per_m}
	}
}
/// Multiplying a Area by a InverseDistance returns a value of type Distance
impl<T> core::ops::Mul<&InverseDistance<T>> for Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: self.m2 * rhs.per_m.clone()}
	}
}
/// Multiplying a Area by a InverseDistance returns a value of type Distance
impl<T> core::ops::Mul<&InverseDistance<T>> for &Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		Distance{m: self.m2.clone() * rhs.per_m.clone()}
	}
}

// Area / InverseDistance -> Volume
/// Dividing a Area by a InverseDistance returns a value of type Volume
impl<T> core::ops::Div<InverseDistance<T>> for Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Volume{m3: self.m2 / rhs.per_m}
	}
}
/// Dividing a Area by a InverseDistance returns a value of type Volume
impl<T> core::ops::Div<InverseDistance<T>> for &Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		Volume{m3: self.m2.clone() / rhs.per_m}
	}
}
/// Dividing a Area by a InverseDistance returns a value of type Volume
impl<T> core::ops::Div<&InverseDistance<T>> for Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Volume{m3: self.m2 / rhs.per_m.clone()}
	}
}
/// Dividing a Area by a InverseDistance returns a value of type Volume
impl<T> core::ops::Div<&InverseDistance<T>> for &Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		Volume{m3: self.m2.clone() / rhs.per_m.clone()}
	}
}

// Area * InverseMass -> AreaPerMass
/// Multiplying a Area by a InverseMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<InverseMass<T>> for Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: InverseMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2 * rhs.per_kg}
	}
}
/// Multiplying a Area by a InverseMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<InverseMass<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: InverseMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2.clone() * rhs.per_kg}
	}
}
/// Multiplying a Area by a InverseMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<&InverseMass<T>> for Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: &InverseMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2 * rhs.per_kg.clone()}
	}
}
/// Multiplying a Area by a InverseMass returns a value of type AreaPerMass
impl<T> core::ops::Mul<&InverseMass<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn mul(self, rhs: &InverseMass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2.clone() * rhs.per_kg.clone()}
	}
}

// Area / Mass -> AreaPerMass
/// Dividing a Area by a Mass returns a value of type AreaPerMass
impl<T> core::ops::Div<Mass<T>> for Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2 / rhs.kg}
	}
}
/// Dividing a Area by a Mass returns a value of type AreaPerMass
impl<T> core::ops::Div<Mass<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2.clone() / rhs.kg}
	}
}
/// Dividing a Area by a Mass returns a value of type AreaPerMass
impl<T> core::ops::Div<&Mass<T>> for Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2 / rhs.kg.clone()}
	}
}
/// Dividing a Area by a Mass returns a value of type AreaPerMass
impl<T> core::ops::Div<&Mass<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		AreaPerMass{m2_per_kg: self.m2.clone() / rhs.kg.clone()}
	}
}

// Area / AreaPerLumen -> LuminousFlux
/// Dividing a Area by a AreaPerLumen returns a value of type LuminousFlux
impl<T> core::ops::Div<AreaPerLumen<T>> for Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		LuminousFlux{lm: self.m2 / rhs.m2_per_lm}
	}
}
/// Dividing a Area by a AreaPerLumen returns a value of type LuminousFlux
impl<T> core::ops::Div<AreaPerLumen<T>> for &Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: AreaPerLumen<T>) -> Self::Output {
		LuminousFlux{lm: self.m2.clone() / rhs.m2_per_lm}
	}
}
/// Dividing a Area by a AreaPerLumen returns a value of type LuminousFlux
impl<T> core::ops::Div<&AreaPerLumen<T>> for Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		LuminousFlux{lm: self.m2 / rhs.m2_per_lm.clone()}
	}
}
/// Dividing a Area by a AreaPerLumen returns a value of type LuminousFlux
impl<T> core::ops::Div<&AreaPerLumen<T>> for &Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		LuminousFlux{lm: self.m2.clone() / rhs.m2_per_lm.clone()}
	}
}

// Area * Illuminance -> LuminousFlux
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> core::ops::Mul<Illuminance<T>> for Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2 * rhs.lux}
	}
}
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> core::ops::Mul<Illuminance<T>> for &Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2.clone() * rhs.lux}
	}
}
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> core::ops::Mul<&Illuminance<T>> for Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2 * rhs.lux.clone()}
	}
}
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> core::ops::Mul<&Illuminance<T>> for &Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2.clone() * rhs.lux.clone()}
	}
}

// Area * InverseLuminousFlux -> AreaPerLumen
/// Multiplying a Area by a InverseLuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2 * rhs.per_lm}
	}
}
/// Multiplying a Area by a InverseLuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2.clone() * rhs.per_lm}
	}
}
/// Multiplying a Area by a InverseLuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2 * rhs.per_lm.clone()}
	}
}
/// Multiplying a Area by a InverseLuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2.clone() * rhs.per_lm.clone()}
	}
}

// Area * InverseMagneticFlux -> InverseMagneticFluxDensity
/// Multiplying a Area by a InverseMagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2 * rhs.per_Wb}
	}
}
/// Multiplying a Area by a InverseMagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<InverseMagneticFlux<T>> for &Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2.clone() * rhs.per_Wb}
	}
}
/// Multiplying a Area by a InverseMagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2 * rhs.per_Wb.clone()}
	}
}
/// Multiplying a Area by a InverseMagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Mul<&InverseMagneticFlux<T>> for &Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn mul(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2.clone() * rhs.per_Wb.clone()}
	}
}

// Area / InverseMagneticFluxDensity -> MagneticFlux
/// Dividing a Area by a InverseMagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2 / rhs.m2_per_Wb}
	}
}
/// Dividing a Area by a InverseMagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Div<InverseMagneticFluxDensity<T>> for &Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2.clone() / rhs.m2_per_Wb}
	}
}
/// Dividing a Area by a InverseMagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2 / rhs.m2_per_Wb.clone()}
	}
}
/// Dividing a Area by a InverseMagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Div<&InverseMagneticFluxDensity<T>> for &Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2.clone() / rhs.m2_per_Wb.clone()}
	}
}

// Area / LuminousFlux -> AreaPerLumen
/// Dividing a Area by a LuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Div<LuminousFlux<T>> for Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2 / rhs.lm}
	}
}
/// Dividing a Area by a LuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Div<LuminousFlux<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2.clone() / rhs.lm}
	}
}
/// Dividing a Area by a LuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Div<&LuminousFlux<T>> for Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2 / rhs.lm.clone()}
	}
}
/// Dividing a Area by a LuminousFlux returns a value of type AreaPerLumen
impl<T> core::ops::Div<&LuminousFlux<T>> for &Area<T> where T: NumLike {
	type Output = AreaPerLumen<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		AreaPerLumen{m2_per_lm: self.m2.clone() / rhs.lm.clone()}
	}
}

// Area / MagneticFlux -> InverseMagneticFluxDensity
/// Dividing a Area by a MagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFlux<T>> for Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2 / rhs.Wb}
	}
}
/// Dividing a Area by a MagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<MagneticFlux<T>> for &Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2.clone() / rhs.Wb}
	}
}
/// Dividing a Area by a MagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFlux<T>> for Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2 / rhs.Wb.clone()}
	}
}
/// Dividing a Area by a MagneticFlux returns a value of type InverseMagneticFluxDensity
impl<T> core::ops::Div<&MagneticFlux<T>> for &Area<T> where T: NumLike {
	type Output = InverseMagneticFluxDensity<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		InverseMagneticFluxDensity{m2_per_Wb: self.m2.clone() / rhs.Wb.clone()}
	}
}

// Area * MagneticFluxDensity -> MagneticFlux
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Mul<MagneticFluxDensity<T>> for Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2 * rhs.T}
	}
}
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Mul<MagneticFluxDensity<T>> for &Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2.clone() * rhs.T}
	}
}
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Mul<&MagneticFluxDensity<T>> for Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2 * rhs.T.clone()}
	}
}
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> core::ops::Mul<&MagneticFluxDensity<T>> for &Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2.clone() * rhs.T.clone()}
	}
}

// Area * InverseVolume -> InverseDistance
/// Multiplying a Area by a InverseVolume returns a value of type InverseDistance
impl<T> core::ops::Mul<InverseVolume<T>> for Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2 * rhs.per_m3}
	}
}
/// Multiplying a Area by a InverseVolume returns a value of type InverseDistance
impl<T> core::ops::Mul<InverseVolume<T>> for &Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: InverseVolume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2.clone() * rhs.per_m3}
	}
}
/// Multiplying a Area by a InverseVolume returns a value of type InverseDistance
impl<T> core::ops::Mul<&InverseVolume<T>> for Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2 * rhs.per_m3.clone()}
	}
}
/// Multiplying a Area by a InverseVolume returns a value of type InverseDistance
impl<T> core::ops::Mul<&InverseVolume<T>> for &Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &InverseVolume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2.clone() * rhs.per_m3.clone()}
	}
}

// Area / Volume -> InverseDistance
/// Dividing a Area by a Volume returns a value of type InverseDistance
impl<T> core::ops::Div<Volume<T>> for Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2 / rhs.m3}
	}
}
/// Dividing a Area by a Volume returns a value of type InverseDistance
impl<T> core::ops::Div<Volume<T>> for &Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2.clone() / rhs.m3}
	}
}
/// Dividing a Area by a Volume returns a value of type InverseDistance
impl<T> core::ops::Div<&Volume<T>> for Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2 / rhs.m3.clone()}
	}
}
/// Dividing a Area by a Volume returns a value of type InverseDistance
impl<T> core::ops::Div<&Volume<T>> for &Area<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseDistance{per_m: self.m2.clone() / rhs.m3.clone()}
	}
}

// Area * AreaDensity -> Mass
/// Multiplying a Area by a AreaDensity returns a value of type Mass
impl<T> core::ops::Mul<AreaDensity<T>> for Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		Mass{kg: self.m2 * rhs.kgpm2}
	}
}
/// Multiplying a Area by a AreaDensity returns a value of type Mass
impl<T> core::ops::Mul<AreaDensity<T>> for &Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		Mass{kg: self.m2.clone() * rhs.kgpm2}
	}
}
/// Multiplying a Area by a AreaDensity returns a value of type Mass
impl<T> core::ops::Mul<&AreaDensity<T>> for Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		Mass{kg: self.m2 * rhs.kgpm2.clone()}
	}
}
/// Multiplying a Area by a AreaDensity returns a value of type Mass
impl<T> core::ops::Mul<&AreaDensity<T>> for &Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		Mass{kg: self.m2.clone() * rhs.kgpm2.clone()}
	}
}

// Area / AreaPerMass -> Mass
/// Dividing a Area by a AreaPerMass returns a value of type Mass
impl<T> core::ops::Div<AreaPerMass<T>> for Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: AreaPerMass<T>) -> Self::Output {
		Mass{kg: self.m2 / rhs.m2_per_kg}
	}
}
/// Dividing a Area by a AreaPerMass returns a value of type Mass
impl<T> core::ops::Div<AreaPerMass<T>> for &Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: AreaPerMass<T>) -> Self::Output {
		Mass{kg: self.m2.clone() / rhs.m2_per_kg}
	}
}
/// Dividing a Area by a AreaPerMass returns a value of type Mass
impl<T> core::ops::Div<&AreaPerMass<T>> for Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &AreaPerMass<T>) -> Self::Output {
		Mass{kg: self.m2 / rhs.m2_per_kg.clone()}
	}
}
/// Dividing a Area by a AreaPerMass returns a value of type Mass
impl<T> core::ops::Div<&AreaPerMass<T>> for &Area<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &AreaPerMass<T>) -> Self::Output {
		Mass{kg: self.m2.clone() / rhs.m2_per_kg.clone()}
	}
}

// Area / Force -> InversePressure
/// Dividing a Area by a Force returns a value of type InversePressure
impl<T> core::ops::Div<Force<T>> for Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2 / rhs.N}
	}
}
/// Dividing a Area by a Force returns a value of type InversePressure
impl<T> core::ops::Div<Force<T>> for &Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2.clone() / rhs.N}
	}
}
/// Dividing a Area by a Force returns a value of type InversePressure
impl<T> core::ops::Div<&Force<T>> for Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2 / rhs.N.clone()}
	}
}
/// Dividing a Area by a Force returns a value of type InversePressure
impl<T> core::ops::Div<&Force<T>> for &Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2.clone() / rhs.N.clone()}
	}
}

// Area * InverseForce -> InversePressure
/// Multiplying a Area by a InverseForce returns a value of type InversePressure
impl<T> core::ops::Mul<InverseForce<T>> for Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: InverseForce<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2 * rhs.per_N}
	}
}
/// Multiplying a Area by a InverseForce returns a value of type InversePressure
impl<T> core::ops::Mul<InverseForce<T>> for &Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: InverseForce<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2.clone() * rhs.per_N}
	}
}
/// Multiplying a Area by a InverseForce returns a value of type InversePressure
impl<T> core::ops::Mul<&InverseForce<T>> for Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: &InverseForce<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2 * rhs.per_N.clone()}
	}
}
/// Multiplying a Area by a InverseForce returns a value of type InversePressure
impl<T> core::ops::Mul<&InverseForce<T>> for &Area<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: &InverseForce<T>) -> Self::Output {
		InversePressure{per_Pa: self.m2.clone() * rhs.per_N.clone()}
	}
}

// Area * InverseMomentOfInertia -> InverseMass
/// Multiplying a Area by a InverseMomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Mul<InverseMomentOfInertia<T>> for Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2 * rhs.per_kgm2}
	}
}
/// Multiplying a Area by a InverseMomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Mul<InverseMomentOfInertia<T>> for &Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2.clone() * rhs.per_kgm2}
	}
}
/// Multiplying a Area by a InverseMomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Mul<&InverseMomentOfInertia<T>> for Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2 * rhs.per_kgm2.clone()}
	}
}
/// Multiplying a Area by a InverseMomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Mul<&InverseMomentOfInertia<T>> for &Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2.clone() * rhs.per_kgm2.clone()}
	}
}

// Area / InversePressure -> Force
/// Dividing a Area by a InversePressure returns a value of type Force
impl<T> core::ops::Div<InversePressure<T>> for Area<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InversePressure<T>) -> Self::Output {
		Force{N: self.m2 / rhs.per_Pa}
	}
}
/// Dividing a Area by a InversePressure returns a value of type Force
impl<T> core::ops::Div<InversePressure<T>> for &Area<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: InversePressure<T>) -> Self::Output {
		Force{N: self.m2.clone() / rhs.per_Pa}
	}
}
/// Dividing a Area by a InversePressure returns a value of type Force
impl<T> core::ops::Div<&InversePressure<T>> for Area<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InversePressure<T>) -> Self::Output {
		Force{N: self.m2 / rhs.per_Pa.clone()}
	}
}
/// Dividing a Area by a InversePressure returns a value of type Force
impl<T> core::ops::Div<&InversePressure<T>> for &Area<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &InversePressure<T>) -> Self::Output {
		Force{N: self.m2.clone() / rhs.per_Pa.clone()}
	}
}

// Area / MomentOfInertia -> InverseMass
/// Dividing a Area by a MomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Div<MomentOfInertia<T>> for Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2 / rhs.kgm2}
	}
}
/// Dividing a Area by a MomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Div<MomentOfInertia<T>> for &Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2.clone() / rhs.kgm2}
	}
}
/// Dividing a Area by a MomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Div<&MomentOfInertia<T>> for Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2 / rhs.kgm2.clone()}
	}
}
/// Dividing a Area by a MomentOfInertia returns a value of type InverseMass
impl<T> core::ops::Div<&MomentOfInertia<T>> for &Area<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		InverseMass{per_kg: self.m2.clone() / rhs.kgm2.clone()}
	}
}

// Area * Pressure -> Force
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> core::ops::Mul<Pressure<T>> for Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Force{N: self.m2 * rhs.Pa}
	}
}
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> core::ops::Mul<Pressure<T>> for &Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Force{N: self.m2.clone() * rhs.Pa}
	}
}
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> core::ops::Mul<&Pressure<T>> for Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Force{N: self.m2 * rhs.Pa.clone()}
	}
}
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> core::ops::Mul<&Pressure<T>> for &Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Force{N: self.m2.clone() * rhs.Pa.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<Area<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
impl<T> core::ops::Div<&Area<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Area<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Area<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Area<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Area<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Area<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Area<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Area<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Area<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

// 1/Area -> InverseArea
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Area<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Area<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Area<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self) / rhs.m2.clone()}
	}
}
/// Dividing a scalar value by a Area unit value returns a value of type InverseArea
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Area<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseArea<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		InverseArea{per_m2: T::from(self.clone()) / rhs.m2.clone()}
	}
}

/// The inverse of angle unit type, defined as inverse radians in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseAngle<T: NumLike>{
	/// The value of this Inverse angle in inverse radians
	pub per_rad: T
}

impl<T> InverseAngle<T> where T: NumLike {

	/// Returns the standard unit name of inverse angle: "inverse radians"
	pub fn unit_name() -> &'static str { "inverse radians" }
	
	/// Returns the abbreviated name or symbol of inverse angle: "1/rad" for inverse radians
	pub fn unit_symbol() -> &'static str { "1/rad" }
	
	/// Returns a new inverse angle value from the given number of inverse radians
	///
	/// # Arguments
	/// * `per_rad` - Any number-like type, representing a quantity of inverse radians
	pub fn from_per_rad(per_rad: T) -> Self { InverseAngle{per_rad: per_rad} }
	
	/// Returns a copy of this inverse angle value in inverse radians
	pub fn to_per_rad(&self) -> T { self.per_rad.clone() }

	/// Returns a new inverse angle value from the given number of inverse radians
	///
	/// # Arguments
	/// * `per_radians` - Any number-like type, representing a quantity of inverse radians
	pub fn from_per_radians(per_radians: T) -> Self { InverseAngle{per_rad: per_radians} }
	
	/// Returns a copy of this inverse angle value in inverse radians
	pub fn to_per_radians(&self) -> T { self.per_rad.clone() }

}

impl<T> fmt::Display for InverseAngle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_rad, Self::unit_symbol())
	}
}

impl<T> InverseAngle<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse angle value in inverse degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_degrees(&self) -> T {
		return self.per_rad.clone() * T::from(0.0174532925199433_f64);
	}

	/// Returns a new inverse angle value from the given number of inverse degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_degrees` - Any number-like type, representing a quantity of inverse degrees
	pub fn from_per_degrees(per_degrees: T) -> Self {
		InverseAngle{per_rad: per_degrees * T::from(57.2957795130823_f64)}
	}

	/// Returns a copy of this inverse angle value in inverse degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_deg(&self) -> T {
		return self.per_rad.clone() * T::from(0.0174532925199433_f64);
	}

	/// Returns a new inverse angle value from the given number of inverse degrees
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_deg` - Any number-like type, representing a quantity of inverse degrees
	pub fn from_per_deg(per_deg: T) -> Self {
		InverseAngle{per_rad: per_deg * T::from(57.2957795130823_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseAngle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAngle{per_rad: self * rhs.per_rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseAngle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAngle{per_rad: self.clone() * rhs.per_rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseAngle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAngle{per_rad: self * rhs.per_rad.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseAngle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseAngle{per_rad: self.clone() * rhs.per_rad.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAngle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseAngle<num_complex::Complex32>;
	fn mul(self, rhs: InverseAngle<num_complex::Complex32>) -> Self::Output {
		InverseAngle{per_rad: self * rhs.per_rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAngle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseAngle<num_complex::Complex32>;
	fn mul(self, rhs: InverseAngle<num_complex::Complex32>) -> Self::Output {
		InverseAngle{per_rad: self.clone() * rhs.per_rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAngle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseAngle<num_complex::Complex32>;
	fn mul(self, rhs: &InverseAngle<num_complex::Complex32>) -> Self::Output {
		InverseAngle{per_rad: self * rhs.per_rad.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAngle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseAngle<num_complex::Complex32>;
	fn mul(self, rhs: &InverseAngle<num_complex::Complex32>) -> Self::Output {
		InverseAngle{per_rad: self.clone() * rhs.per_rad.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAngle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseAngle<num_complex::Complex64>;
	fn mul(self, rhs: InverseAngle<num_complex::Complex64>) -> Self::Output {
		InverseAngle{per_rad: self * rhs.per_rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseAngle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseAngle<num_complex::Complex64>;
	fn mul(self, rhs: InverseAngle<num_complex::Complex64>) -> Self::Output {
		InverseAngle{per_rad: self.clone() * rhs.per_rad}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAngle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseAngle<num_complex::Complex64>;
	fn mul(self, rhs: &InverseAngle<num_complex::Complex64>) -> Self::Output {
		InverseAngle{per_rad: self * rhs.per_rad.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseAngle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseAngle<num_complex::Complex64>;
	fn mul(self, rhs: &InverseAngle<num_complex::Complex64>) -> Self::Output {
		InverseAngle{per_rad: self.clone() * rhs.per_rad.clone()}
	}
}




// InverseAngle * Time -> InverseAngularVelocity
/// Multiplying a InverseAngle by a Time returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<Time<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad * rhs.s}
	}
}
/// Multiplying a InverseAngle by a Time returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<Time<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad.clone() * rhs.s}
	}
}
/// Multiplying a InverseAngle by a Time returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<&Time<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad * rhs.s.clone()}
	}
}
/// Multiplying a InverseAngle by a Time returns a value of type InverseAngularVelocity
impl<T> core::ops::Mul<&Time<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad.clone() * rhs.s.clone()}
	}
}

// InverseAngle / Angle -> InverseSolidAngle
/// Dividing a InverseAngle by a Angle returns a value of type InverseSolidAngle
impl<T> core::ops::Div<Angle<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad / rhs.rad}
	}
}
/// Dividing a InverseAngle by a Angle returns a value of type InverseSolidAngle
impl<T> core::ops::Div<Angle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad.clone() / rhs.rad}
	}
}
/// Dividing a InverseAngle by a Angle returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&Angle<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad / rhs.rad.clone()}
	}
}
/// Dividing a InverseAngle by a Angle returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&Angle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad.clone() / rhs.rad.clone()}
	}
}

// InverseAngle * InverseAngle -> InverseSolidAngle
/// Multiplying a InverseAngle by a InverseAngle returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<InverseAngle<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: InverseAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad * rhs.per_rad}
	}
}
/// Multiplying a InverseAngle by a InverseAngle returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<InverseAngle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: InverseAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad.clone() * rhs.per_rad}
	}
}
/// Multiplying a InverseAngle by a InverseAngle returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<&InverseAngle<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: &InverseAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad * rhs.per_rad.clone()}
	}
}
/// Multiplying a InverseAngle by a InverseAngle returns a value of type InverseSolidAngle
impl<T> core::ops::Mul<&InverseAngle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseSolidAngle<T>;
	fn mul(self, rhs: &InverseAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: self.per_rad.clone() * rhs.per_rad.clone()}
	}
}

// InverseAngle / InverseSolidAngle -> Angle
/// Dividing a InverseAngle by a InverseSolidAngle returns a value of type Angle
impl<T> core::ops::Div<InverseSolidAngle<T>> for InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad / rhs.per_sr}
	}
}
/// Dividing a InverseAngle by a InverseSolidAngle returns a value of type Angle
impl<T> core::ops::Div<InverseSolidAngle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad.clone() / rhs.per_sr}
	}
}
/// Dividing a InverseAngle by a InverseSolidAngle returns a value of type Angle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad / rhs.per_sr.clone()}
	}
}
/// Dividing a InverseAngle by a InverseSolidAngle returns a value of type Angle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad.clone() / rhs.per_sr.clone()}
	}
}

// InverseAngle * SolidAngle -> Angle
/// Multiplying a InverseAngle by a SolidAngle returns a value of type Angle
impl<T> core::ops::Mul<SolidAngle<T>> for InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad * rhs.sr}
	}
}
/// Multiplying a InverseAngle by a SolidAngle returns a value of type Angle
impl<T> core::ops::Mul<SolidAngle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad.clone() * rhs.sr}
	}
}
/// Multiplying a InverseAngle by a SolidAngle returns a value of type Angle
impl<T> core::ops::Mul<&SolidAngle<T>> for InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad * rhs.sr.clone()}
	}
}
/// Multiplying a InverseAngle by a SolidAngle returns a value of type Angle
impl<T> core::ops::Mul<&SolidAngle<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		Angle{rad: self.per_rad.clone() * rhs.sr.clone()}
	}
}

// InverseAngle * AngularVelocity -> Frequency
/// Multiplying a InverseAngle by a AngularVelocity returns a value of type Frequency
impl<T> core::ops::Mul<AngularVelocity<T>> for InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad * rhs.radps}
	}
}
/// Multiplying a InverseAngle by a AngularVelocity returns a value of type Frequency
impl<T> core::ops::Mul<AngularVelocity<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad.clone() * rhs.radps}
	}
}
/// Multiplying a InverseAngle by a AngularVelocity returns a value of type Frequency
impl<T> core::ops::Mul<&AngularVelocity<T>> for InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad * rhs.radps.clone()}
	}
}
/// Multiplying a InverseAngle by a AngularVelocity returns a value of type Frequency
impl<T> core::ops::Mul<&AngularVelocity<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad.clone() * rhs.radps.clone()}
	}
}

// InverseAngle / Frequency -> InverseAngularVelocity
/// Dividing a InverseAngle by a Frequency returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<Frequency<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad / rhs.Hz}
	}
}
/// Dividing a InverseAngle by a Frequency returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<Frequency<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad.clone() / rhs.Hz}
	}
}
/// Dividing a InverseAngle by a Frequency returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<&Frequency<T>> for InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad / rhs.Hz.clone()}
	}
}
/// Dividing a InverseAngle by a Frequency returns a value of type InverseAngularVelocity
impl<T> core::ops::Div<&Frequency<T>> for &InverseAngle<T> where T: NumLike {
	type Output = InverseAngularVelocity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		InverseAngularVelocity{s_per_rad: self.per_rad.clone() / rhs.Hz.clone()}
	}
}

// InverseAngle / InverseAngularVelocity -> Frequency
/// Dividing a InverseAngle by a InverseAngularVelocity returns a value of type Frequency
impl<T> core::ops::Div<InverseAngularVelocity<T>> for InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad / rhs.s_per_rad}
	}
}
/// Dividing a InverseAngle by a InverseAngularVelocity returns a value of type Frequency
impl<T> core::ops::Div<InverseAngularVelocity<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: InverseAngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad.clone() / rhs.s_per_rad}
	}
}
/// Dividing a InverseAngle by a InverseAngularVelocity returns a value of type Frequency
impl<T> core::ops::Div<&InverseAngularVelocity<T>> for InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad / rhs.s_per_rad.clone()}
	}
}
/// Dividing a InverseAngle by a InverseAngularVelocity returns a value of type Frequency
impl<T> core::ops::Div<&InverseAngularVelocity<T>> for &InverseAngle<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &InverseAngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.per_rad.clone() / rhs.s_per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for f64 where T: NumLike+From<f64> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for f64 where T: NumLike+From<f64> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for f32 where T: NumLike+From<f32> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for f32 where T: NumLike+From<f32> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for i64 where T: NumLike+From<i64> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for i64 where T: NumLike+From<i64> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for i32 where T: NumLike+From<i32> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<InverseAngle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for i32 where T: NumLike+From<i32> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
impl<T> core::ops::Div<&InverseAngle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseAngle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseAngle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseAngle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseAngle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAngle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAngle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAngle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAngle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

// 1/InverseAngle -> Angle
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAngle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseAngle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Angle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAngle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self) / rhs.per_rad.clone()}
	}
}
/// Dividing a scalar value by a InverseAngle unit value returns a value of type Angle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseAngle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Angle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: T::from(self.clone()) / rhs.per_rad.clone()}
	}
}

/// The inverse of area unit type, defined as inverse square meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseArea<T: NumLike>{
	/// The value of this Inverse area in inverse square meters
	pub per_m2: T
}

impl<T> InverseArea<T> where T: NumLike {

	/// Returns the standard unit name of inverse area: "inverse square meters"
	pub fn unit_name() -> &'static str { "inverse square meters" }
	
	/// Returns the abbreviated name or symbol of inverse area: "1/m²" for inverse square meters
	pub fn unit_symbol() -> &'static str { "1/m²" }
	
	/// Returns a new inverse area value from the given number of inverse square meters
	///
	/// # Arguments
	/// * `per_m2` - Any number-like type, representing a quantity of inverse square meters
	pub fn from_per_m2(per_m2: T) -> Self { InverseArea{per_m2: per_m2} }
	
	/// Returns a copy of this inverse area value in inverse square meters
	pub fn to_per_m2(&self) -> T { self.per_m2.clone() }

	/// Returns a new inverse area value from the given number of inverse square meters
	///
	/// # Arguments
	/// * `per_square_meter` - Any number-like type, representing a quantity of inverse square meters
	pub fn from_per_square_meter(per_square_meter: T) -> Self { InverseArea{per_m2: per_square_meter} }
	
	/// Returns a copy of this inverse area value in inverse square meters
	pub fn to_per_square_meter(&self) -> T { self.per_m2.clone() }

}

impl<T> fmt::Display for InverseArea<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_m2, Self::unit_symbol())
	}
}

impl<T> InverseArea<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse area value in inverse square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_cm2(&self) -> T {
		return self.per_m2.clone() * T::from(0.0001_f64);
	}

	/// Returns a new inverse area value from the given number of inverse square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_cm2` - Any number-like type, representing a quantity of inverse square cm
	pub fn from_per_cm2(per_cm2: T) -> Self {
		InverseArea{per_m2: per_cm2 * T::from(10000.0_f64)}
	}

	/// Returns a copy of this inverse area value in inverse square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_square_cm(&self) -> T {
		return self.per_m2.clone() * T::from(0.0001_f64);
	}

	/// Returns a new inverse area value from the given number of inverse square cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_square_cm` - Any number-like type, representing a quantity of inverse square cm
	pub fn from_per_square_cm(per_square_cm: T) -> Self {
		InverseArea{per_m2: per_square_cm * T::from(10000.0_f64)}
	}

	/// Returns a copy of this inverse area value in inverse square mm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mm2(&self) -> T {
		return self.per_m2.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse area value from the given number of inverse square mm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mm2` - Any number-like type, representing a quantity of inverse square mm
	pub fn from_per_mm2(per_mm2: T) -> Self {
		InverseArea{per_m2: per_mm2 * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse area value in inverse square um
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_um2(&self) -> T {
		return self.per_m2.clone() * T::from(1e-12_f64);
	}

	/// Returns a new inverse area value from the given number of inverse square um
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_um2` - Any number-like type, representing a quantity of inverse square um
	pub fn from_per_um2(per_um2: T) -> Self {
		InverseArea{per_m2: per_um2 * T::from(1000000000000.0_f64)}
	}

	/// Returns a copy of this inverse area value in inverse square nm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nm2(&self) -> T {
		return self.per_m2.clone() * T::from(1e-18_f64);
	}

	/// Returns a new inverse area value from the given number of inverse square nm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nm2` - Any number-like type, representing a quantity of inverse square nm
	pub fn from_per_nm2(per_nm2: T) -> Self {
		InverseArea{per_m2: per_nm2 * T::from(1e+18_f64)}
	}

	/// Returns a copy of this inverse area value in inverse square km
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_km2(&self) -> T {
		return self.per_m2.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse area value from the given number of inverse square km
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_km2` - Any number-like type, representing a quantity of inverse square km
	pub fn from_per_km2(per_km2: T) -> Self {
		InverseArea{per_m2: per_km2 * T::from(1e-06_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseArea<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseArea<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseArea<num_bigfloat::BigFloat>) -> Self::Output {
		InverseArea{per_m2: self * rhs.per_m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseArea<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseArea<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseArea<num_bigfloat::BigFloat>) -> Self::Output {
		InverseArea{per_m2: self.clone() * rhs.per_m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseArea<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseArea<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseArea<num_bigfloat::BigFloat>) -> Self::Output {
		InverseArea{per_m2: self * rhs.per_m2.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseArea<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseArea<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseArea<num_bigfloat::BigFloat>) -> Self::Output {
		InverseArea{per_m2: self.clone() * rhs.per_m2.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseArea<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseArea<num_complex::Complex32>;
	fn mul(self, rhs: InverseArea<num_complex::Complex32>) -> Self::Output {
		InverseArea{per_m2: self * rhs.per_m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseArea<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseArea<num_complex::Complex32>;
	fn mul(self, rhs: InverseArea<num_complex::Complex32>) -> Self::Output {
		InverseArea{per_m2: self.clone() * rhs.per_m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseArea<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseArea<num_complex::Complex32>;
	fn mul(self, rhs: &InverseArea<num_complex::Complex32>) -> Self::Output {
		InverseArea{per_m2: self * rhs.per_m2.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseArea<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseArea<num_complex::Complex32>;
	fn mul(self, rhs: &InverseArea<num_complex::Complex32>) -> Self::Output {
		InverseArea{per_m2: self.clone() * rhs.per_m2.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseArea<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseArea<num_complex::Complex64>;
	fn mul(self, rhs: InverseArea<num_complex::Complex64>) -> Self::Output {
		InverseArea{per_m2: self * rhs.per_m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseArea<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseArea<num_complex::Complex64>;
	fn mul(self, rhs: InverseArea<num_complex::Complex64>) -> Self::Output {
		InverseArea{per_m2: self.clone() * rhs.per_m2}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseArea<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseArea<num_complex::Complex64>;
	fn mul(self, rhs: &InverseArea<num_complex::Complex64>) -> Self::Output {
		InverseArea{per_m2: self * rhs.per_m2.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseArea<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseArea<num_complex::Complex64>;
	fn mul(self, rhs: &InverseArea<num_complex::Complex64>) -> Self::Output {
		InverseArea{per_m2: self.clone() * rhs.per_m2.clone()}
	}
}



/// Converts a InverseArea into the equivalent [uom](https://crates.io/crates/uom) type [ArealNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.ArealNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ArealNumberDensity> for InverseArea<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ArealNumberDensity {
		uom::si::f32::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(self.per_m2.into())
	}
}

/// Creates a InverseArea from the equivalent [uom](https://crates.io/crates/uom) type [ArealNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.ArealNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ArealNumberDensity> for InverseArea<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ArealNumberDensity) -> Self {
		InverseArea{per_m2: T::from(src.value)}
	}
}

/// Converts a InverseArea into the equivalent [uom](https://crates.io/crates/uom) type [ArealNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.ArealNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ArealNumberDensity> for InverseArea<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ArealNumberDensity {
		uom::si::f64::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(self.per_m2.into())
	}
}

/// Creates a InverseArea from the equivalent [uom](https://crates.io/crates/uom) type [ArealNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.ArealNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ArealNumberDensity> for InverseArea<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ArealNumberDensity) -> Self {
		InverseArea{per_m2: T::from(src.value)}
	}
}


// InverseArea * Distance -> InverseDistance
/// Multiplying a InverseArea by a Distance returns a value of type InverseDistance
impl<T> core::ops::Mul<Distance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2 * rhs.m}
	}
}
/// Multiplying a InverseArea by a Distance returns a value of type InverseDistance
impl<T> core::ops::Mul<Distance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2.clone() * rhs.m}
	}
}
/// Multiplying a InverseArea by a Distance returns a value of type InverseDistance
impl<T> core::ops::Mul<&Distance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2 * rhs.m.clone()}
	}
}
/// Multiplying a InverseArea by a Distance returns a value of type InverseDistance
impl<T> core::ops::Mul<&Distance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2.clone() * rhs.m.clone()}
	}
}

// InverseArea / Distance -> InverseVolume
/// Dividing a InverseArea by a Distance returns a value of type InverseVolume
impl<T> core::ops::Div<Distance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2 / rhs.m}
	}
}
/// Dividing a InverseArea by a Distance returns a value of type InverseVolume
impl<T> core::ops::Div<Distance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2.clone() / rhs.m}
	}
}
/// Dividing a InverseArea by a Distance returns a value of type InverseVolume
impl<T> core::ops::Div<&Distance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2 / rhs.m.clone()}
	}
}
/// Dividing a InverseArea by a Distance returns a value of type InverseVolume
impl<T> core::ops::Div<&Distance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2.clone() / rhs.m.clone()}
	}
}

// InverseArea * InverseDistance -> InverseVolume
/// Multiplying a InverseArea by a InverseDistance returns a value of type InverseVolume
impl<T> core::ops::Mul<InverseDistance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2 * rhs.per_m}
	}
}
/// Multiplying a InverseArea by a InverseDistance returns a value of type InverseVolume
impl<T> core::ops::Mul<InverseDistance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2.clone() * rhs.per_m}
	}
}
/// Multiplying a InverseArea by a InverseDistance returns a value of type InverseVolume
impl<T> core::ops::Mul<&InverseDistance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2 * rhs.per_m.clone()}
	}
}
/// Multiplying a InverseArea by a InverseDistance returns a value of type InverseVolume
impl<T> core::ops::Mul<&InverseDistance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseVolume<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseVolume{per_m3: self.per_m2.clone() * rhs.per_m.clone()}
	}
}

// InverseArea / InverseDistance -> InverseDistance
/// Dividing a InverseArea by a InverseDistance returns a value of type InverseDistance
impl<T> core::ops::Div<InverseDistance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2 / rhs.per_m}
	}
}
/// Dividing a InverseArea by a InverseDistance returns a value of type InverseDistance
impl<T> core::ops::Div<InverseDistance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2.clone() / rhs.per_m}
	}
}
/// Dividing a InverseArea by a InverseDistance returns a value of type InverseDistance
impl<T> core::ops::Div<&InverseDistance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2 / rhs.per_m.clone()}
	}
}
/// Dividing a InverseArea by a InverseDistance returns a value of type InverseDistance
impl<T> core::ops::Div<&InverseDistance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m2.clone() / rhs.per_m.clone()}
	}
}

// InverseArea / InverseMass -> AreaDensity
/// Dividing a InverseArea by a InverseMass returns a value of type AreaDensity
impl<T> core::ops::Div<InverseMass<T>> for InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2 / rhs.per_kg}
	}
}
/// Dividing a InverseArea by a InverseMass returns a value of type AreaDensity
impl<T> core::ops::Div<InverseMass<T>> for &InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2.clone() / rhs.per_kg}
	}
}
/// Dividing a InverseArea by a InverseMass returns a value of type AreaDensity
impl<T> core::ops::Div<&InverseMass<T>> for InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2 / rhs.per_kg.clone()}
	}
}
/// Dividing a InverseArea by a InverseMass returns a value of type AreaDensity
impl<T> core::ops::Div<&InverseMass<T>> for &InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2.clone() / rhs.per_kg.clone()}
	}
}

// InverseArea * Mass -> AreaDensity
/// Multiplying a InverseArea by a Mass returns a value of type AreaDensity
impl<T> core::ops::Mul<Mass<T>> for InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2 * rhs.kg}
	}
}
/// Multiplying a InverseArea by a Mass returns a value of type AreaDensity
impl<T> core::ops::Mul<Mass<T>> for &InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2.clone() * rhs.kg}
	}
}
/// Multiplying a InverseArea by a Mass returns a value of type AreaDensity
impl<T> core::ops::Mul<&Mass<T>> for InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2 * rhs.kg.clone()}
	}
}
/// Multiplying a InverseArea by a Mass returns a value of type AreaDensity
impl<T> core::ops::Mul<&Mass<T>> for &InverseArea<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		AreaDensity{kgpm2: self.per_m2.clone() * rhs.kg.clone()}
	}
}

// InverseArea * AreaPerLumen -> InverseLuminousFlux
/// Multiplying a InverseArea by a AreaPerLumen returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<AreaPerLumen<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: AreaPerLumen<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2 * rhs.m2_per_lm}
	}
}
/// Multiplying a InverseArea by a AreaPerLumen returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<AreaPerLumen<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: AreaPerLumen<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2.clone() * rhs.m2_per_lm}
	}
}
/// Multiplying a InverseArea by a AreaPerLumen returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&AreaPerLumen<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2 * rhs.m2_per_lm.clone()}
	}
}
/// Multiplying a InverseArea by a AreaPerLumen returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&AreaPerLumen<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &AreaPerLumen<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2.clone() * rhs.m2_per_lm.clone()}
	}
}

// InverseArea / Illuminance -> InverseLuminousFlux
/// Dividing a InverseArea by a Illuminance returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<Illuminance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2 / rhs.lux}
	}
}
/// Dividing a InverseArea by a Illuminance returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<Illuminance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2.clone() / rhs.lux}
	}
}
/// Dividing a InverseArea by a Illuminance returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&Illuminance<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2 / rhs.lux.clone()}
	}
}
/// Dividing a InverseArea by a Illuminance returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&Illuminance<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_m2.clone() / rhs.lux.clone()}
	}
}

// InverseArea / InverseLuminousFlux -> Illuminance
/// Dividing a InverseArea by a InverseLuminousFlux returns a value of type Illuminance
impl<T> core::ops::Div<InverseLuminousFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2 / rhs.per_lm}
	}
}
/// Dividing a InverseArea by a InverseLuminousFlux returns a value of type Illuminance
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2.clone() / rhs.per_lm}
	}
}
/// Dividing a InverseArea by a InverseLuminousFlux returns a value of type Illuminance
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2 / rhs.per_lm.clone()}
	}
}
/// Dividing a InverseArea by a InverseLuminousFlux returns a value of type Illuminance
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2.clone() / rhs.per_lm.clone()}
	}
}

// InverseArea / InverseMagneticFlux -> MagneticFluxDensity
/// Dividing a InverseArea by a InverseMagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2 / rhs.per_Wb}
	}
}
/// Dividing a InverseArea by a InverseMagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<InverseMagneticFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: InverseMagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2.clone() / rhs.per_Wb}
	}
}
/// Dividing a InverseArea by a InverseMagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2 / rhs.per_Wb.clone()}
	}
}
/// Dividing a InverseArea by a InverseMagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Div<&InverseMagneticFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &InverseMagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2.clone() / rhs.per_Wb.clone()}
	}
}

// InverseArea * InverseMagneticFluxDensity -> InverseMagneticFlux
/// Multiplying a InverseArea by a InverseMagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseMagneticFluxDensity<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2 * rhs.m2_per_Wb}
	}
}
/// Multiplying a InverseArea by a InverseMagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<InverseMagneticFluxDensity<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2.clone() * rhs.m2_per_Wb}
	}
}
/// Multiplying a InverseArea by a InverseMagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseMagneticFluxDensity<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2 * rhs.m2_per_Wb.clone()}
	}
}
/// Multiplying a InverseArea by a InverseMagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Mul<&InverseMagneticFluxDensity<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn mul(self, rhs: &InverseMagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2.clone() * rhs.m2_per_Wb.clone()}
	}
}

// InverseArea * LuminousFlux -> Illuminance
/// Multiplying a InverseArea by a LuminousFlux returns a value of type Illuminance
impl<T> core::ops::Mul<LuminousFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2 * rhs.lm}
	}
}
/// Multiplying a InverseArea by a LuminousFlux returns a value of type Illuminance
impl<T> core::ops::Mul<LuminousFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2.clone() * rhs.lm}
	}
}
/// Multiplying a InverseArea by a LuminousFlux returns a value of type Illuminance
impl<T> core::ops::Mul<&LuminousFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2 * rhs.lm.clone()}
	}
}
/// Multiplying a InverseArea by a LuminousFlux returns a value of type Illuminance
impl<T> core::ops::Mul<&LuminousFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		Illuminance{lux: self.per_m2.clone() * rhs.lm.clone()}
	}
}

// InverseArea * MagneticFlux -> MagneticFluxDensity
/// Multiplying a InverseArea by a MagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<MagneticFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2 * rhs.Wb}
	}
}
/// Multiplying a InverseArea by a MagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<MagneticFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2.clone() * rhs.Wb}
	}
}
/// Multiplying a InverseArea by a MagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<&MagneticFlux<T>> for InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2 * rhs.Wb.clone()}
	}
}
/// Multiplying a InverseArea by a MagneticFlux returns a value of type MagneticFluxDensity
impl<T> core::ops::Mul<&MagneticFlux<T>> for &InverseArea<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		MagneticFluxDensity{T: self.per_m2.clone() * rhs.Wb.clone()}
	}
}

// InverseArea / MagneticFluxDensity -> InverseMagneticFlux
/// Dividing a InverseArea by a MagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFluxDensity<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2 / rhs.T}
	}
}
/// Dividing a InverseArea by a MagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<MagneticFluxDensity<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2.clone() / rhs.T}
	}
}
/// Dividing a InverseArea by a MagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2 / rhs.T.clone()}
	}
}
/// Dividing a InverseArea by a MagneticFluxDensity returns a value of type InverseMagneticFlux
impl<T> core::ops::Div<&MagneticFluxDensity<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMagneticFlux<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		InverseMagneticFlux{per_Wb: self.per_m2.clone() / rhs.T.clone()}
	}
}

// InverseArea / InverseVolume -> Distance
/// Dividing a InverseArea by a InverseVolume returns a value of type Distance
impl<T> core::ops::Div<InverseVolume<T>> for InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Distance{m: self.per_m2 / rhs.per_m3}
	}
}
/// Dividing a InverseArea by a InverseVolume returns a value of type Distance
impl<T> core::ops::Div<InverseVolume<T>> for &InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Distance{m: self.per_m2.clone() / rhs.per_m3}
	}
}
/// Dividing a InverseArea by a InverseVolume returns a value of type Distance
impl<T> core::ops::Div<&InverseVolume<T>> for InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Distance{m: self.per_m2 / rhs.per_m3.clone()}
	}
}
/// Dividing a InverseArea by a InverseVolume returns a value of type Distance
impl<T> core::ops::Div<&InverseVolume<T>> for &InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Distance{m: self.per_m2.clone() / rhs.per_m3.clone()}
	}
}

// InverseArea * Volume -> Distance
/// Multiplying a InverseArea by a Volume returns a value of type Distance
impl<T> core::ops::Mul<Volume<T>> for InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Distance{m: self.per_m2 * rhs.m3}
	}
}
/// Multiplying a InverseArea by a Volume returns a value of type Distance
impl<T> core::ops::Mul<Volume<T>> for &InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Distance{m: self.per_m2.clone() * rhs.m3}
	}
}
/// Multiplying a InverseArea by a Volume returns a value of type Distance
impl<T> core::ops::Mul<&Volume<T>> for InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Distance{m: self.per_m2 * rhs.m3.clone()}
	}
}
/// Multiplying a InverseArea by a Volume returns a value of type Distance
impl<T> core::ops::Mul<&Volume<T>> for &InverseArea<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Distance{m: self.per_m2.clone() * rhs.m3.clone()}
	}
}

// InverseArea / AreaDensity -> InverseMass
/// Dividing a InverseArea by a AreaDensity returns a value of type InverseMass
impl<T> core::ops::Div<AreaDensity<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2 / rhs.kgpm2}
	}
}
/// Dividing a InverseArea by a AreaDensity returns a value of type InverseMass
impl<T> core::ops::Div<AreaDensity<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2.clone() / rhs.kgpm2}
	}
}
/// Dividing a InverseArea by a AreaDensity returns a value of type InverseMass
impl<T> core::ops::Div<&AreaDensity<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2 / rhs.kgpm2.clone()}
	}
}
/// Dividing a InverseArea by a AreaDensity returns a value of type InverseMass
impl<T> core::ops::Div<&AreaDensity<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2.clone() / rhs.kgpm2.clone()}
	}
}

// InverseArea * AreaPerMass -> InverseMass
/// Multiplying a InverseArea by a AreaPerMass returns a value of type InverseMass
impl<T> core::ops::Mul<AreaPerMass<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: AreaPerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2 * rhs.m2_per_kg}
	}
}
/// Multiplying a InverseArea by a AreaPerMass returns a value of type InverseMass
impl<T> core::ops::Mul<AreaPerMass<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: AreaPerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2.clone() * rhs.m2_per_kg}
	}
}
/// Multiplying a InverseArea by a AreaPerMass returns a value of type InverseMass
impl<T> core::ops::Mul<&AreaPerMass<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &AreaPerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2 * rhs.m2_per_kg.clone()}
	}
}
/// Multiplying a InverseArea by a AreaPerMass returns a value of type InverseMass
impl<T> core::ops::Mul<&AreaPerMass<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &AreaPerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m2.clone() * rhs.m2_per_kg.clone()}
	}
}

// InverseArea * Force -> Pressure
/// Multiplying a InverseArea by a Force returns a value of type Pressure
impl<T> core::ops::Mul<Force<T>> for InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Pressure{Pa: self.per_m2 * rhs.N}
	}
}
/// Multiplying a InverseArea by a Force returns a value of type Pressure
impl<T> core::ops::Mul<Force<T>> for &InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Pressure{Pa: self.per_m2.clone() * rhs.N}
	}
}
/// Multiplying a InverseArea by a Force returns a value of type Pressure
impl<T> core::ops::Mul<&Force<T>> for InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Pressure{Pa: self.per_m2 * rhs.N.clone()}
	}
}
/// Multiplying a InverseArea by a Force returns a value of type Pressure
impl<T> core::ops::Mul<&Force<T>> for &InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Pressure{Pa: self.per_m2.clone() * rhs.N.clone()}
	}
}

// InverseArea / InverseForce -> Pressure
/// Dividing a InverseArea by a InverseForce returns a value of type Pressure
impl<T> core::ops::Div<InverseForce<T>> for InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Pressure{Pa: self.per_m2 / rhs.per_N}
	}
}
/// Dividing a InverseArea by a InverseForce returns a value of type Pressure
impl<T> core::ops::Div<InverseForce<T>> for &InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: InverseForce<T>) -> Self::Output {
		Pressure{Pa: self.per_m2.clone() / rhs.per_N}
	}
}
/// Dividing a InverseArea by a InverseForce returns a value of type Pressure
impl<T> core::ops::Div<&InverseForce<T>> for InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Pressure{Pa: self.per_m2 / rhs.per_N.clone()}
	}
}
/// Dividing a InverseArea by a InverseForce returns a value of type Pressure
impl<T> core::ops::Div<&InverseForce<T>> for &InverseArea<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &InverseForce<T>) -> Self::Output {
		Pressure{Pa: self.per_m2.clone() / rhs.per_N.clone()}
	}
}

// InverseArea / InverseMomentOfInertia -> Mass
/// Dividing a InverseArea by a InverseMomentOfInertia returns a value of type Mass
impl<T> core::ops::Div<InverseMomentOfInertia<T>> for InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2 / rhs.per_kgm2}
	}
}
/// Dividing a InverseArea by a InverseMomentOfInertia returns a value of type Mass
impl<T> core::ops::Div<InverseMomentOfInertia<T>> for &InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: InverseMomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2.clone() / rhs.per_kgm2}
	}
}
/// Dividing a InverseArea by a InverseMomentOfInertia returns a value of type Mass
impl<T> core::ops::Div<&InverseMomentOfInertia<T>> for InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2 / rhs.per_kgm2.clone()}
	}
}
/// Dividing a InverseArea by a InverseMomentOfInertia returns a value of type Mass
impl<T> core::ops::Div<&InverseMomentOfInertia<T>> for &InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &InverseMomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2.clone() / rhs.per_kgm2.clone()}
	}
}

// InverseArea * InversePressure -> InverseForce
/// Multiplying a InverseArea by a InversePressure returns a value of type InverseForce
impl<T> core::ops::Mul<InversePressure<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InversePressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2 * rhs.per_Pa}
	}
}
/// Multiplying a InverseArea by a InversePressure returns a value of type InverseForce
impl<T> core::ops::Mul<InversePressure<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: InversePressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2.clone() * rhs.per_Pa}
	}
}
/// Multiplying a InverseArea by a InversePressure returns a value of type InverseForce
impl<T> core::ops::Mul<&InversePressure<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InversePressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2 * rhs.per_Pa.clone()}
	}
}
/// Multiplying a InverseArea by a InversePressure returns a value of type InverseForce
impl<T> core::ops::Mul<&InversePressure<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn mul(self, rhs: &InversePressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2.clone() * rhs.per_Pa.clone()}
	}
}

// InverseArea * MomentOfInertia -> Mass
/// Multiplying a InverseArea by a MomentOfInertia returns a value of type Mass
impl<T> core::ops::Mul<MomentOfInertia<T>> for InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2 * rhs.kgm2}
	}
}
/// Multiplying a InverseArea by a MomentOfInertia returns a value of type Mass
impl<T> core::ops::Mul<MomentOfInertia<T>> for &InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2.clone() * rhs.kgm2}
	}
}
/// Multiplying a InverseArea by a MomentOfInertia returns a value of type Mass
impl<T> core::ops::Mul<&MomentOfInertia<T>> for InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2 * rhs.kgm2.clone()}
	}
}
/// Multiplying a InverseArea by a MomentOfInertia returns a value of type Mass
impl<T> core::ops::Mul<&MomentOfInertia<T>> for &InverseArea<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		Mass{kg: self.per_m2.clone() * rhs.kgm2.clone()}
	}
}

// InverseArea / Pressure -> InverseForce
/// Dividing a InverseArea by a Pressure returns a value of type InverseForce
impl<T> core::ops::Div<Pressure<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2 / rhs.Pa}
	}
}
/// Dividing a InverseArea by a Pressure returns a value of type InverseForce
impl<T> core::ops::Div<Pressure<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2.clone() / rhs.Pa}
	}
}
/// Dividing a InverseArea by a Pressure returns a value of type InverseForce
impl<T> core::ops::Div<&Pressure<T>> for InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2 / rhs.Pa.clone()}
	}
}
/// Dividing a InverseArea by a Pressure returns a value of type InverseForce
impl<T> core::ops::Div<&Pressure<T>> for &InverseArea<T> where T: NumLike {
	type Output = InverseForce<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		InverseForce{per_N: self.per_m2.clone() / rhs.Pa.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for f64 where T: NumLike+From<f64> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for f64 where T: NumLike+From<f64> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for f32 where T: NumLike+From<f32> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for f32 where T: NumLike+From<f32> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for i64 where T: NumLike+From<i64> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for i64 where T: NumLike+From<i64> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for i32 where T: NumLike+From<i32> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<InverseArea<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for i32 where T: NumLike+From<i32> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
impl<T> core::ops::Div<&InverseArea<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseArea<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseArea<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseArea<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseArea<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseArea<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseArea<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseArea<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseArea<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

// 1/InverseArea -> Area
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseArea<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseArea<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Area<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseArea<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self) / rhs.per_m2.clone()}
	}
}
/// Dividing a scalar value by a InverseArea unit value returns a value of type Area
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseArea<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Area<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		Area{m2: T::from(self.clone()) / rhs.per_m2.clone()}
	}
}

/// The inverse of solid angle unit type, defined as inverse steradian in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseSolidAngle<T: NumLike>{
	/// The value of this Inverse solid angle in inverse steradian
	pub per_sr: T
}

impl<T> InverseSolidAngle<T> where T: NumLike {

	/// Returns the standard unit name of inverse solid angle: "inverse steradian"
	pub fn unit_name() -> &'static str { "inverse steradian" }
	
	/// Returns the abbreviated name or symbol of inverse solid angle: "1/sr" for inverse steradian
	pub fn unit_symbol() -> &'static str { "1/sr" }
	
	/// Returns a new inverse solid angle value from the given number of inverse steradians
	///
	/// # Arguments
	/// * `per_sr` - Any number-like type, representing a quantity of inverse steradian
	pub fn from_per_sr(per_sr: T) -> Self { InverseSolidAngle{per_sr: per_sr} }
	
	/// Returns a copy of this inverse solid angle value in inverse steradians
	pub fn to_per_sr(&self) -> T { self.per_sr.clone() }

	/// Returns a new inverse solid angle value from the given number of inverse steradians
	///
	/// # Arguments
	/// * `per_steradians` - Any number-like type, representing a quantity of inverse steradian
	pub fn from_per_steradians(per_steradians: T) -> Self { InverseSolidAngle{per_sr: per_steradians} }
	
	/// Returns a copy of this inverse solid angle value in inverse steradians
	pub fn to_per_steradians(&self) -> T { self.per_sr.clone() }

}

impl<T> fmt::Display for InverseSolidAngle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_sr, Self::unit_symbol())
	}
}

impl<T> InverseSolidAngle<T> where T: NumLike+From<f64> {
	
}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseSolidAngle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseSolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseSolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseSolidAngle{per_sr: self * rhs.per_sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseSolidAngle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseSolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseSolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseSolidAngle{per_sr: self.clone() * rhs.per_sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseSolidAngle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseSolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseSolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseSolidAngle{per_sr: self * rhs.per_sr.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseSolidAngle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseSolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseSolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		InverseSolidAngle{per_sr: self.clone() * rhs.per_sr.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseSolidAngle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseSolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: InverseSolidAngle<num_complex::Complex32>) -> Self::Output {
		InverseSolidAngle{per_sr: self * rhs.per_sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseSolidAngle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseSolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: InverseSolidAngle<num_complex::Complex32>) -> Self::Output {
		InverseSolidAngle{per_sr: self.clone() * rhs.per_sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseSolidAngle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseSolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: &InverseSolidAngle<num_complex::Complex32>) -> Self::Output {
		InverseSolidAngle{per_sr: self * rhs.per_sr.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseSolidAngle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseSolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: &InverseSolidAngle<num_complex::Complex32>) -> Self::Output {
		InverseSolidAngle{per_sr: self.clone() * rhs.per_sr.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseSolidAngle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseSolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: InverseSolidAngle<num_complex::Complex64>) -> Self::Output {
		InverseSolidAngle{per_sr: self * rhs.per_sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseSolidAngle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseSolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: InverseSolidAngle<num_complex::Complex64>) -> Self::Output {
		InverseSolidAngle{per_sr: self.clone() * rhs.per_sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseSolidAngle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseSolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: &InverseSolidAngle<num_complex::Complex64>) -> Self::Output {
		InverseSolidAngle{per_sr: self * rhs.per_sr.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseSolidAngle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseSolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: &InverseSolidAngle<num_complex::Complex64>) -> Self::Output {
		InverseSolidAngle{per_sr: self.clone() * rhs.per_sr.clone()}
	}
}




// InverseSolidAngle * InverseLuminosity -> InverseLuminousFlux
/// Multiplying a InverseSolidAngle by a InverseLuminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<InverseLuminosity<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: InverseLuminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr * rhs.per_cd}
	}
}
/// Multiplying a InverseSolidAngle by a InverseLuminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<InverseLuminosity<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: InverseLuminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr.clone() * rhs.per_cd}
	}
}
/// Multiplying a InverseSolidAngle by a InverseLuminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&InverseLuminosity<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr * rhs.per_cd.clone()}
	}
}
/// Multiplying a InverseSolidAngle by a InverseLuminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Mul<&InverseLuminosity<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn mul(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr.clone() * rhs.per_cd.clone()}
	}
}

// InverseSolidAngle / Luminosity -> InverseLuminousFlux
/// Dividing a InverseSolidAngle by a Luminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<Luminosity<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr / rhs.cd}
	}
}
/// Dividing a InverseSolidAngle by a Luminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<Luminosity<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr.clone() / rhs.cd}
	}
}
/// Dividing a InverseSolidAngle by a Luminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&Luminosity<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr / rhs.cd.clone()}
	}
}
/// Dividing a InverseSolidAngle by a Luminosity returns a value of type InverseLuminousFlux
impl<T> core::ops::Div<&Luminosity<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseLuminousFlux<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		InverseLuminousFlux{per_lm: self.per_sr.clone() / rhs.cd.clone()}
	}
}

// InverseSolidAngle / InverseLuminousFlux -> Luminosity
/// Dividing a InverseSolidAngle by a InverseLuminousFlux returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminousFlux<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr / rhs.per_lm}
	}
}
/// Dividing a InverseSolidAngle by a InverseLuminousFlux returns a value of type Luminosity
impl<T> core::ops::Div<InverseLuminousFlux<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr.clone() / rhs.per_lm}
	}
}
/// Dividing a InverseSolidAngle by a InverseLuminousFlux returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr / rhs.per_lm.clone()}
	}
}
/// Dividing a InverseSolidAngle by a InverseLuminousFlux returns a value of type Luminosity
impl<T> core::ops::Div<&InverseLuminousFlux<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr.clone() / rhs.per_lm.clone()}
	}
}

// InverseSolidAngle * LuminousFlux -> Luminosity
/// Multiplying a InverseSolidAngle by a LuminousFlux returns a value of type Luminosity
impl<T> core::ops::Mul<LuminousFlux<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr * rhs.lm}
	}
}
/// Multiplying a InverseSolidAngle by a LuminousFlux returns a value of type Luminosity
impl<T> core::ops::Mul<LuminousFlux<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: LuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr.clone() * rhs.lm}
	}
}
/// Multiplying a InverseSolidAngle by a LuminousFlux returns a value of type Luminosity
impl<T> core::ops::Mul<&LuminousFlux<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr * rhs.lm.clone()}
	}
}
/// Multiplying a InverseSolidAngle by a LuminousFlux returns a value of type Luminosity
impl<T> core::ops::Mul<&LuminousFlux<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn mul(self, rhs: &LuminousFlux<T>) -> Self::Output {
		Luminosity{cd: self.per_sr.clone() * rhs.lm.clone()}
	}
}

// InverseSolidAngle * Angle -> InverseAngle
/// Multiplying a InverseSolidAngle by a Angle returns a value of type InverseAngle
impl<T> core::ops::Mul<Angle<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr * rhs.rad}
	}
}
/// Multiplying a InverseSolidAngle by a Angle returns a value of type InverseAngle
impl<T> core::ops::Mul<Angle<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: Angle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr.clone() * rhs.rad}
	}
}
/// Multiplying a InverseSolidAngle by a Angle returns a value of type InverseAngle
impl<T> core::ops::Mul<&Angle<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr * rhs.rad.clone()}
	}
}
/// Multiplying a InverseSolidAngle by a Angle returns a value of type InverseAngle
impl<T> core::ops::Mul<&Angle<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn mul(self, rhs: &Angle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr.clone() * rhs.rad.clone()}
	}
}

// InverseSolidAngle / InverseAngle -> InverseAngle
/// Dividing a InverseSolidAngle by a InverseAngle returns a value of type InverseAngle
impl<T> core::ops::Div<InverseAngle<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr / rhs.per_rad}
	}
}
/// Dividing a InverseSolidAngle by a InverseAngle returns a value of type InverseAngle
impl<T> core::ops::Div<InverseAngle<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: InverseAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr.clone() / rhs.per_rad}
	}
}
/// Dividing a InverseSolidAngle by a InverseAngle returns a value of type InverseAngle
impl<T> core::ops::Div<&InverseAngle<T>> for InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr / rhs.per_rad.clone()}
	}
}
/// Dividing a InverseSolidAngle by a InverseAngle returns a value of type InverseAngle
impl<T> core::ops::Div<&InverseAngle<T>> for &InverseSolidAngle<T> where T: NumLike {
	type Output = InverseAngle<T>;
	fn div(self, rhs: &InverseAngle<T>) -> Self::Output {
		InverseAngle{per_rad: self.per_sr.clone() / rhs.per_rad.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for f64 where T: NumLike+From<f64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for f64 where T: NumLike+From<f64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for f32 where T: NumLike+From<f32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for f32 where T: NumLike+From<f32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for i64 where T: NumLike+From<i64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for i64 where T: NumLike+From<i64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for i32 where T: NumLike+From<i32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<InverseSolidAngle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for i32 where T: NumLike+From<i32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseSolidAngle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseSolidAngle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseSolidAngle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseSolidAngle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseSolidAngle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseSolidAngle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

// 1/InverseSolidAngle -> SolidAngle
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseSolidAngle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseSolidAngle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseSolidAngle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self) / rhs.per_sr.clone()}
	}
}
/// Dividing a scalar value by a InverseSolidAngle unit value returns a value of type SolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseSolidAngle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &InverseSolidAngle<T>) -> Self::Output {
		SolidAngle{sr: T::from(self.clone()) / rhs.per_sr.clone()}
	}
}

/// The inverse of volume unit type, defined as inverse cubic meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct InverseVolume<T: NumLike>{
	/// The value of this Inverse volume in inverse cubic meters
	pub per_m3: T
}

impl<T> InverseVolume<T> where T: NumLike {

	/// Returns the standard unit name of inverse volume: "inverse cubic meters"
	pub fn unit_name() -> &'static str { "inverse cubic meters" }
	
	/// Returns the abbreviated name or symbol of inverse volume: "1/m³" for inverse cubic meters
	pub fn unit_symbol() -> &'static str { "1/m³" }
	
	/// Returns a new inverse volume value from the given number of inverse cubic meters
	///
	/// # Arguments
	/// * `per_m3` - Any number-like type, representing a quantity of inverse cubic meters
	pub fn from_per_m3(per_m3: T) -> Self { InverseVolume{per_m3: per_m3} }
	
	/// Returns a copy of this inverse volume value in inverse cubic meters
	pub fn to_per_m3(&self) -> T { self.per_m3.clone() }

	/// Returns a new inverse volume value from the given number of inverse cubic meters
	///
	/// # Arguments
	/// * `per_cubic_meter` - Any number-like type, representing a quantity of inverse cubic meters
	pub fn from_per_cubic_meter(per_cubic_meter: T) -> Self { InverseVolume{per_m3: per_cubic_meter} }
	
	/// Returns a copy of this inverse volume value in inverse cubic meters
	pub fn to_per_cubic_meter(&self) -> T { self.per_m3.clone() }

	/// Returns a new inverse volume value from the given number of inverse kiloliters
	///
	/// # Arguments
	/// * `per_kL` - Any number-like type, representing a quantity of inverse cubic meters
	pub fn from_per_kL(per_kL: T) -> Self { InverseVolume{per_m3: per_kL} }
	
	/// Returns a copy of this inverse volume value in inverse kiloliters
	pub fn to_per_kL(&self) -> T { self.per_m3.clone() }

}

impl<T> fmt::Display for InverseVolume<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.per_m3, Self::unit_symbol())
	}
}

impl<T> InverseVolume<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inverse volume value in inverse cubic cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_cc(&self) -> T {
		return self.per_m3.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse cubic cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_cc` - Any number-like type, representing a quantity of inverse cubic cm
	pub fn from_per_cc(per_cc: T) -> Self {
		InverseVolume{per_m3: per_cc * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_L(&self) -> T {
		return self.per_m3.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_L` - Any number-like type, representing a quantity of inverse liters
	pub fn from_per_L(per_L: T) -> Self {
		InverseVolume{per_m3: per_L * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_liters(&self) -> T {
		return self.per_m3.clone() * T::from(0.001_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_liters` - Any number-like type, representing a quantity of inverse liters
	pub fn from_per_liters(per_liters: T) -> Self {
		InverseVolume{per_m3: per_liters * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse milliliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_mL(&self) -> T {
		return self.per_m3.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse milliliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_mL` - Any number-like type, representing a quantity of inverse milliliters
	pub fn from_per_mL(per_mL: T) -> Self {
		InverseVolume{per_m3: per_mL * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse microliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_uL(&self) -> T {
		return self.per_m3.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse microliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_uL` - Any number-like type, representing a quantity of inverse microliters
	pub fn from_per_uL(per_uL: T) -> Self {
		InverseVolume{per_m3: per_uL * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse nanoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_nL(&self) -> T {
		return self.per_m3.clone() * T::from(1e-12_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse nanoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_nL` - Any number-like type, representing a quantity of inverse nanoliters
	pub fn from_per_nL(per_nL: T) -> Self {
		InverseVolume{per_m3: per_nL * T::from(1000000000000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse picoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_pL(&self) -> T {
		return self.per_m3.clone() * T::from(1e-15_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse picoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_pL` - Any number-like type, representing a quantity of inverse picoliters
	pub fn from_per_pL(per_pL: T) -> Self {
		InverseVolume{per_m3: per_pL * T::from(1000000000000000.0_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse megaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_ML(&self) -> T {
		return self.per_m3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse megaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_ML` - Any number-like type, representing a quantity of inverse megaliters
	pub fn from_per_ML(per_ML: T) -> Self {
		InverseVolume{per_m3: per_ML * T::from(0.001_f64)}
	}

	/// Returns a copy of this inverse volume value in inverse gigaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_per_GL(&self) -> T {
		return self.per_m3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inverse volume value from the given number of inverse gigaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `per_GL` - Any number-like type, representing a quantity of inverse gigaliters
	pub fn from_per_GL(per_GL: T) -> Self {
		InverseVolume{per_m3: per_GL * T::from(1e-06_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseVolume<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseVolume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseVolume<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVolume{per_m3: self * rhs.per_m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<InverseVolume<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseVolume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: InverseVolume<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVolume{per_m3: self.clone() * rhs.per_m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseVolume<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = InverseVolume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseVolume<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVolume{per_m3: self * rhs.per_m3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&InverseVolume<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = InverseVolume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &InverseVolume<num_bigfloat::BigFloat>) -> Self::Output {
		InverseVolume{per_m3: self.clone() * rhs.per_m3.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVolume<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseVolume<num_complex::Complex32>;
	fn mul(self, rhs: InverseVolume<num_complex::Complex32>) -> Self::Output {
		InverseVolume{per_m3: self * rhs.per_m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVolume<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseVolume<num_complex::Complex32>;
	fn mul(self, rhs: InverseVolume<num_complex::Complex32>) -> Self::Output {
		InverseVolume{per_m3: self.clone() * rhs.per_m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVolume<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = InverseVolume<num_complex::Complex32>;
	fn mul(self, rhs: &InverseVolume<num_complex::Complex32>) -> Self::Output {
		InverseVolume{per_m3: self * rhs.per_m3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVolume<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = InverseVolume<num_complex::Complex32>;
	fn mul(self, rhs: &InverseVolume<num_complex::Complex32>) -> Self::Output {
		InverseVolume{per_m3: self.clone() * rhs.per_m3.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVolume<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseVolume<num_complex::Complex64>;
	fn mul(self, rhs: InverseVolume<num_complex::Complex64>) -> Self::Output {
		InverseVolume{per_m3: self * rhs.per_m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<InverseVolume<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseVolume<num_complex::Complex64>;
	fn mul(self, rhs: InverseVolume<num_complex::Complex64>) -> Self::Output {
		InverseVolume{per_m3: self.clone() * rhs.per_m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVolume<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = InverseVolume<num_complex::Complex64>;
	fn mul(self, rhs: &InverseVolume<num_complex::Complex64>) -> Self::Output {
		InverseVolume{per_m3: self * rhs.per_m3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&InverseVolume<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = InverseVolume<num_complex::Complex64>;
	fn mul(self, rhs: &InverseVolume<num_complex::Complex64>) -> Self::Output {
		InverseVolume{per_m3: self.clone() * rhs.per_m3.clone()}
	}
}



/// Converts a InverseVolume into the equivalent [uom](https://crates.io/crates/uom) type [VolumetricNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.VolumetricNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::VolumetricNumberDensity> for InverseVolume<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::VolumetricNumberDensity {
		uom::si::f32::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(self.per_m3.into())
	}
}

/// Creates a InverseVolume from the equivalent [uom](https://crates.io/crates/uom) type [VolumetricNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.VolumetricNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::VolumetricNumberDensity> for InverseVolume<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::VolumetricNumberDensity) -> Self {
		InverseVolume{per_m3: T::from(src.value)}
	}
}

/// Converts a InverseVolume into the equivalent [uom](https://crates.io/crates/uom) type [VolumetricNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.VolumetricNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::VolumetricNumberDensity> for InverseVolume<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::VolumetricNumberDensity {
		uom::si::f64::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(self.per_m3.into())
	}
}

/// Creates a InverseVolume from the equivalent [uom](https://crates.io/crates/uom) type [VolumetricNumberDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.VolumetricNumberDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::VolumetricNumberDensity> for InverseVolume<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::VolumetricNumberDensity) -> Self {
		InverseVolume{per_m3: T::from(src.value)}
	}
}


// InverseVolume * Amount -> Concentration
/// Multiplying a InverseVolume by a Amount returns a value of type Concentration
impl<T> core::ops::Mul<Amount<T>> for InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3 * rhs.mol}
	}
}
/// Multiplying a InverseVolume by a Amount returns a value of type Concentration
impl<T> core::ops::Mul<Amount<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3.clone() * rhs.mol}
	}
}
/// Multiplying a InverseVolume by a Amount returns a value of type Concentration
impl<T> core::ops::Mul<&Amount<T>> for InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3 * rhs.mol.clone()}
	}
}
/// Multiplying a InverseVolume by a Amount returns a value of type Concentration
impl<T> core::ops::Mul<&Amount<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3.clone() * rhs.mol.clone()}
	}
}

// InverseVolume * Distance -> InverseArea
/// Multiplying a InverseVolume by a Distance returns a value of type InverseArea
impl<T> core::ops::Mul<Distance<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3 * rhs.m}
	}
}
/// Multiplying a InverseVolume by a Distance returns a value of type InverseArea
impl<T> core::ops::Mul<Distance<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3.clone() * rhs.m}
	}
}
/// Multiplying a InverseVolume by a Distance returns a value of type InverseArea
impl<T> core::ops::Mul<&Distance<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3 * rhs.m.clone()}
	}
}
/// Multiplying a InverseVolume by a Distance returns a value of type InverseArea
impl<T> core::ops::Mul<&Distance<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3.clone() * rhs.m.clone()}
	}
}

// InverseVolume / InverseAmount -> Concentration
/// Dividing a InverseVolume by a InverseAmount returns a value of type Concentration
impl<T> core::ops::Div<InverseAmount<T>> for InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3 / rhs.per_mol}
	}
}
/// Dividing a InverseVolume by a InverseAmount returns a value of type Concentration
impl<T> core::ops::Div<InverseAmount<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: InverseAmount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3.clone() / rhs.per_mol}
	}
}
/// Dividing a InverseVolume by a InverseAmount returns a value of type Concentration
impl<T> core::ops::Div<&InverseAmount<T>> for InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3 / rhs.per_mol.clone()}
	}
}
/// Dividing a InverseVolume by a InverseAmount returns a value of type Concentration
impl<T> core::ops::Div<&InverseAmount<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: &InverseAmount<T>) -> Self::Output {
		Concentration{molpm3: self.per_m3.clone() / rhs.per_mol.clone()}
	}
}

// InverseVolume / InverseDistance -> InverseArea
/// Dividing a InverseVolume by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Div<InverseDistance<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3 / rhs.per_m}
	}
}
/// Dividing a InverseVolume by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Div<InverseDistance<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3.clone() / rhs.per_m}
	}
}
/// Dividing a InverseVolume by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Div<&InverseDistance<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3 / rhs.per_m.clone()}
	}
}
/// Dividing a InverseVolume by a InverseDistance returns a value of type InverseArea
impl<T> core::ops::Div<&InverseDistance<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseArea<T>;
	fn div(self, rhs: &InverseDistance<T>) -> Self::Output {
		InverseArea{per_m2: self.per_m3.clone() / rhs.per_m.clone()}
	}
}

// InverseVolume / InverseMass -> Density
/// Dividing a InverseVolume by a InverseMass returns a value of type Density
impl<T> core::ops::Div<InverseMass<T>> for InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3 / rhs.per_kg}
	}
}
/// Dividing a InverseVolume by a InverseMass returns a value of type Density
impl<T> core::ops::Div<InverseMass<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: InverseMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3.clone() / rhs.per_kg}
	}
}
/// Dividing a InverseVolume by a InverseMass returns a value of type Density
impl<T> core::ops::Div<&InverseMass<T>> for InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3 / rhs.per_kg.clone()}
	}
}
/// Dividing a InverseVolume by a InverseMass returns a value of type Density
impl<T> core::ops::Div<&InverseMass<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn div(self, rhs: &InverseMass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3.clone() / rhs.per_kg.clone()}
	}
}

// InverseVolume * Mass -> Density
/// Multiplying a InverseVolume by a Mass returns a value of type Density
impl<T> core::ops::Mul<Mass<T>> for InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3 * rhs.kg}
	}
}
/// Multiplying a InverseVolume by a Mass returns a value of type Density
impl<T> core::ops::Mul<Mass<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3.clone() * rhs.kg}
	}
}
/// Multiplying a InverseVolume by a Mass returns a value of type Density
impl<T> core::ops::Mul<&Mass<T>> for InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3 * rhs.kg.clone()}
	}
}
/// Multiplying a InverseVolume by a Mass returns a value of type Density
impl<T> core::ops::Mul<&Mass<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Density<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Density{kgpm3: self.per_m3.clone() * rhs.kg.clone()}
	}
}

// InverseVolume / Concentration -> InverseAmount
/// Dividing a InverseVolume by a Concentration returns a value of type InverseAmount
impl<T> core::ops::Div<Concentration<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Concentration<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3 / rhs.molpm3}
	}
}
/// Dividing a InverseVolume by a Concentration returns a value of type InverseAmount
impl<T> core::ops::Div<Concentration<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: Concentration<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3.clone() / rhs.molpm3}
	}
}
/// Dividing a InverseVolume by a Concentration returns a value of type InverseAmount
impl<T> core::ops::Div<&Concentration<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Concentration<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3 / rhs.molpm3.clone()}
	}
}
/// Dividing a InverseVolume by a Concentration returns a value of type InverseAmount
impl<T> core::ops::Div<&Concentration<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn div(self, rhs: &Concentration<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3.clone() / rhs.molpm3.clone()}
	}
}

// InverseVolume * MolarVolume -> InverseAmount
/// Multiplying a InverseVolume by a MolarVolume returns a value of type InverseAmount
impl<T> core::ops::Mul<MolarVolume<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: MolarVolume<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3 * rhs.m3_per_mol}
	}
}
/// Multiplying a InverseVolume by a MolarVolume returns a value of type InverseAmount
impl<T> core::ops::Mul<MolarVolume<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: MolarVolume<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3.clone() * rhs.m3_per_mol}
	}
}
/// Multiplying a InverseVolume by a MolarVolume returns a value of type InverseAmount
impl<T> core::ops::Mul<&MolarVolume<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: &MolarVolume<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3 * rhs.m3_per_mol.clone()}
	}
}
/// Multiplying a InverseVolume by a MolarVolume returns a value of type InverseAmount
impl<T> core::ops::Mul<&MolarVolume<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseAmount<T>;
	fn mul(self, rhs: &MolarVolume<T>) -> Self::Output {
		InverseAmount{per_mol: self.per_m3.clone() * rhs.m3_per_mol.clone()}
	}
}

// InverseVolume * Area -> InverseDistance
/// Multiplying a InverseVolume by a Area returns a value of type InverseDistance
impl<T> core::ops::Mul<Area<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3 * rhs.m2}
	}
}
/// Multiplying a InverseVolume by a Area returns a value of type InverseDistance
impl<T> core::ops::Mul<Area<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3.clone() * rhs.m2}
	}
}
/// Multiplying a InverseVolume by a Area returns a value of type InverseDistance
impl<T> core::ops::Mul<&Area<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3 * rhs.m2.clone()}
	}
}
/// Multiplying a InverseVolume by a Area returns a value of type InverseDistance
impl<T> core::ops::Mul<&Area<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3.clone() * rhs.m2.clone()}
	}
}

// InverseVolume / InverseArea -> InverseDistance
/// Dividing a InverseVolume by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Div<InverseArea<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3 / rhs.per_m2}
	}
}
/// Dividing a InverseVolume by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Div<InverseArea<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3.clone() / rhs.per_m2}
	}
}
/// Dividing a InverseVolume by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Div<&InverseArea<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3 / rhs.per_m2.clone()}
	}
}
/// Dividing a InverseVolume by a InverseArea returns a value of type InverseDistance
impl<T> core::ops::Div<&InverseArea<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseDistance<T>;
	fn div(self, rhs: &InverseArea<T>) -> Self::Output {
		InverseDistance{per_m: self.per_m3.clone() / rhs.per_m2.clone()}
	}
}

// InverseVolume / Density -> InverseMass
/// Dividing a InverseVolume by a Density returns a value of type InverseMass
impl<T> core::ops::Div<Density<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3 / rhs.kgpm3}
	}
}
/// Dividing a InverseVolume by a Density returns a value of type InverseMass
impl<T> core::ops::Div<Density<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: Density<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3.clone() / rhs.kgpm3}
	}
}
/// Dividing a InverseVolume by a Density returns a value of type InverseMass
impl<T> core::ops::Div<&Density<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3 / rhs.kgpm3.clone()}
	}
}
/// Dividing a InverseVolume by a Density returns a value of type InverseMass
impl<T> core::ops::Div<&Density<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn div(self, rhs: &Density<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3.clone() / rhs.kgpm3.clone()}
	}
}

// InverseVolume * Energy -> Pressure
/// Multiplying a InverseVolume by a Energy returns a value of type Pressure
impl<T> core::ops::Mul<Energy<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 * rhs.J}
	}
}
/// Multiplying a InverseVolume by a Energy returns a value of type Pressure
impl<T> core::ops::Mul<Energy<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() * rhs.J}
	}
}
/// Multiplying a InverseVolume by a Energy returns a value of type Pressure
impl<T> core::ops::Mul<&Energy<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 * rhs.J.clone()}
	}
}
/// Multiplying a InverseVolume by a Energy returns a value of type Pressure
impl<T> core::ops::Mul<&Energy<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() * rhs.J.clone()}
	}
}

// InverseVolume * Torque -> Pressure
/// Multiplying a InverseVolume by a Torque returns a value of type Pressure
impl<T> core::ops::Mul<Torque<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 * rhs.Nm}
	}
}
/// Multiplying a InverseVolume by a Torque returns a value of type Pressure
impl<T> core::ops::Mul<Torque<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() * rhs.Nm}
	}
}
/// Multiplying a InverseVolume by a Torque returns a value of type Pressure
impl<T> core::ops::Mul<&Torque<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 * rhs.Nm.clone()}
	}
}
/// Multiplying a InverseVolume by a Torque returns a value of type Pressure
impl<T> core::ops::Mul<&Torque<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() * rhs.Nm.clone()}
	}
}

// InverseVolume / InverseEnergy -> Pressure
/// Dividing a InverseVolume by a InverseEnergy returns a value of type Pressure
impl<T> core::ops::Div<InverseEnergy<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 / rhs.per_J}
	}
}
/// Dividing a InverseVolume by a InverseEnergy returns a value of type Pressure
impl<T> core::ops::Div<InverseEnergy<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: InverseEnergy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() / rhs.per_J}
	}
}
/// Dividing a InverseVolume by a InverseEnergy returns a value of type Pressure
impl<T> core::ops::Div<&InverseEnergy<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 / rhs.per_J.clone()}
	}
}
/// Dividing a InverseVolume by a InverseEnergy returns a value of type Pressure
impl<T> core::ops::Div<&InverseEnergy<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &InverseEnergy<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() / rhs.per_J.clone()}
	}
}

// InverseVolume / InverseTorque -> Pressure
/// Dividing a InverseVolume by a InverseTorque returns a value of type Pressure
impl<T> core::ops::Div<InverseTorque<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 / rhs.per_Nm}
	}
}
/// Dividing a InverseVolume by a InverseTorque returns a value of type Pressure
impl<T> core::ops::Div<InverseTorque<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: InverseTorque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() / rhs.per_Nm}
	}
}
/// Dividing a InverseVolume by a InverseTorque returns a value of type Pressure
impl<T> core::ops::Div<&InverseTorque<T>> for InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3 / rhs.per_Nm.clone()}
	}
}
/// Dividing a InverseVolume by a InverseTorque returns a value of type Pressure
impl<T> core::ops::Div<&InverseTorque<T>> for &InverseVolume<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &InverseTorque<T>) -> Self::Output {
		Pressure{Pa: self.per_m3.clone() / rhs.per_Nm.clone()}
	}
}

// InverseVolume * InversePressure -> InverseEnergy
/// Multiplying a InverseVolume by a InversePressure returns a value of type InverseEnergy
impl<T> core::ops::Mul<InversePressure<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InversePressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3 * rhs.per_Pa}
	}
}
/// Multiplying a InverseVolume by a InversePressure returns a value of type InverseEnergy
impl<T> core::ops::Mul<InversePressure<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: InversePressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3.clone() * rhs.per_Pa}
	}
}
/// Multiplying a InverseVolume by a InversePressure returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InversePressure<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InversePressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3 * rhs.per_Pa.clone()}
	}
}
/// Multiplying a InverseVolume by a InversePressure returns a value of type InverseEnergy
impl<T> core::ops::Mul<&InversePressure<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn mul(self, rhs: &InversePressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3.clone() * rhs.per_Pa.clone()}
	}
}

// InverseVolume / Pressure -> InverseEnergy
/// Dividing a InverseVolume by a Pressure returns a value of type InverseEnergy
impl<T> core::ops::Div<Pressure<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3 / rhs.Pa}
	}
}
/// Dividing a InverseVolume by a Pressure returns a value of type InverseEnergy
impl<T> core::ops::Div<Pressure<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3.clone() / rhs.Pa}
	}
}
/// Dividing a InverseVolume by a Pressure returns a value of type InverseEnergy
impl<T> core::ops::Div<&Pressure<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3 / rhs.Pa.clone()}
	}
}
/// Dividing a InverseVolume by a Pressure returns a value of type InverseEnergy
impl<T> core::ops::Div<&Pressure<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseEnergy<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		InverseEnergy{per_J: self.per_m3.clone() / rhs.Pa.clone()}
	}
}

// InverseVolume * VolumePerMass -> InverseMass
/// Multiplying a InverseVolume by a VolumePerMass returns a value of type InverseMass
impl<T> core::ops::Mul<VolumePerMass<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: VolumePerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3 * rhs.m3_per_kg}
	}
}
/// Multiplying a InverseVolume by a VolumePerMass returns a value of type InverseMass
impl<T> core::ops::Mul<VolumePerMass<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: VolumePerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3.clone() * rhs.m3_per_kg}
	}
}
/// Multiplying a InverseVolume by a VolumePerMass returns a value of type InverseMass
impl<T> core::ops::Mul<&VolumePerMass<T>> for InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &VolumePerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3 * rhs.m3_per_kg.clone()}
	}
}
/// Multiplying a InverseVolume by a VolumePerMass returns a value of type InverseMass
impl<T> core::ops::Mul<&VolumePerMass<T>> for &InverseVolume<T> where T: NumLike {
	type Output = InverseMass<T>;
	fn mul(self, rhs: &VolumePerMass<T>) -> Self::Output {
		InverseMass{per_kg: self.per_m3.clone() * rhs.m3_per_kg.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for f64 where T: NumLike+From<f64> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for f64 where T: NumLike+From<f64> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for f32 where T: NumLike+From<f32> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for f32 where T: NumLike+From<f32> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for i64 where T: NumLike+From<i64> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for i64 where T: NumLike+From<i64> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for i32 where T: NumLike+From<i32> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<InverseVolume<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for i32 where T: NumLike+From<i32> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
impl<T> core::ops::Div<&InverseVolume<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseVolume<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<InverseVolume<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseVolume<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&InverseVolume<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVolume<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVolume<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVolume<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVolume<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

// 1/InverseVolume -> Volume
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVolume<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<InverseVolume<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Volume<T>;
	fn div(self, rhs: InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVolume<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self) / rhs.per_m3.clone()}
	}
}
/// Dividing a scalar value by a InverseVolume unit value returns a value of type Volume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&InverseVolume<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Volume<T>;
	fn div(self, rhs: &InverseVolume<T>) -> Self::Output {
		Volume{m3: T::from(self.clone()) / rhs.per_m3.clone()}
	}
}

/// The solid angle unit type, defined as steradian in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct SolidAngle<T: NumLike>{
	/// The value of this Solid angle in steradian
	pub sr: T
}

impl<T> SolidAngle<T> where T: NumLike {

	/// Returns the standard unit name of solid angle: "steradian"
	pub fn unit_name() -> &'static str { "steradian" }
	
	/// Returns the abbreviated name or symbol of solid angle: "sr" for steradian
	pub fn unit_symbol() -> &'static str { "sr" }
	
	/// Returns a new solid angle value from the given number of steradians
	///
	/// # Arguments
	/// * `sr` - Any number-like type, representing a quantity of steradian
	pub fn from_sr(sr: T) -> Self { SolidAngle{sr: sr} }
	
	/// Returns a copy of this solid angle value in steradians
	pub fn to_sr(&self) -> T { self.sr.clone() }

	/// Returns a new solid angle value from the given number of steradians
	///
	/// # Arguments
	/// * `steradians` - Any number-like type, representing a quantity of steradian
	pub fn from_steradians(steradians: T) -> Self { SolidAngle{sr: steradians} }
	
	/// Returns a copy of this solid angle value in steradians
	pub fn to_steradians(&self) -> T { self.sr.clone() }

}

impl<T> fmt::Display for SolidAngle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.sr, Self::unit_symbol())
	}
}

impl<T> SolidAngle<T> where T: NumLike+From<f64> {
	
}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<SolidAngle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = SolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: SolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		SolidAngle{sr: self * rhs.sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<SolidAngle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = SolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: SolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		SolidAngle{sr: self.clone() * rhs.sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&SolidAngle<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = SolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &SolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		SolidAngle{sr: self * rhs.sr.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&SolidAngle<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = SolidAngle<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &SolidAngle<num_bigfloat::BigFloat>) -> Self::Output {
		SolidAngle{sr: self.clone() * rhs.sr.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SolidAngle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = SolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: SolidAngle<num_complex::Complex32>) -> Self::Output {
		SolidAngle{sr: self * rhs.sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SolidAngle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = SolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: SolidAngle<num_complex::Complex32>) -> Self::Output {
		SolidAngle{sr: self.clone() * rhs.sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SolidAngle<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = SolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: &SolidAngle<num_complex::Complex32>) -> Self::Output {
		SolidAngle{sr: self * rhs.sr.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SolidAngle<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = SolidAngle<num_complex::Complex32>;
	fn mul(self, rhs: &SolidAngle<num_complex::Complex32>) -> Self::Output {
		SolidAngle{sr: self.clone() * rhs.sr.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SolidAngle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = SolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: SolidAngle<num_complex::Complex64>) -> Self::Output {
		SolidAngle{sr: self * rhs.sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<SolidAngle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = SolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: SolidAngle<num_complex::Complex64>) -> Self::Output {
		SolidAngle{sr: self.clone() * rhs.sr}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SolidAngle<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = SolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: &SolidAngle<num_complex::Complex64>) -> Self::Output {
		SolidAngle{sr: self * rhs.sr.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&SolidAngle<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = SolidAngle<num_complex::Complex64>;
	fn mul(self, rhs: &SolidAngle<num_complex::Complex64>) -> Self::Output {
		SolidAngle{sr: self.clone() * rhs.sr.clone()}
	}
}



/// Converts a SolidAngle into the equivalent [uom](https://crates.io/crates/uom) type [SolidAngle](https://docs.rs/uom/0.34.0/uom/si/f32/type.SolidAngle.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::SolidAngle> for SolidAngle<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::SolidAngle {
		uom::si::f32::SolidAngle::new::<uom::si::solid_angle::steradian>(self.sr.into())
	}
}

/// Creates a SolidAngle from the equivalent [uom](https://crates.io/crates/uom) type [SolidAngle](https://docs.rs/uom/0.34.0/uom/si/f32/type.SolidAngle.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::SolidAngle> for SolidAngle<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::SolidAngle) -> Self {
		SolidAngle{sr: T::from(src.value)}
	}
}

/// Converts a SolidAngle into the equivalent [uom](https://crates.io/crates/uom) type [SolidAngle](https://docs.rs/uom/0.34.0/uom/si/f64/type.SolidAngle.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::SolidAngle> for SolidAngle<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::SolidAngle {
		uom::si::f64::SolidAngle::new::<uom::si::solid_angle::steradian>(self.sr.into())
	}
}

/// Creates a SolidAngle from the equivalent [uom](https://crates.io/crates/uom) type [SolidAngle](https://docs.rs/uom/0.34.0/uom/si/f64/type.SolidAngle.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::SolidAngle> for SolidAngle<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::SolidAngle) -> Self {
		SolidAngle{sr: T::from(src.value)}
	}
}


// SolidAngle / InverseLuminosity -> LuminousFlux
/// Dividing a SolidAngle by a InverseLuminosity returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminosity<T>> for SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr / rhs.per_cd}
	}
}
/// Dividing a SolidAngle by a InverseLuminosity returns a value of type LuminousFlux
impl<T> core::ops::Div<InverseLuminosity<T>> for &SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: InverseLuminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr.clone() / rhs.per_cd}
	}
}
/// Dividing a SolidAngle by a InverseLuminosity returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminosity<T>> for SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr / rhs.per_cd.clone()}
	}
}
/// Dividing a SolidAngle by a InverseLuminosity returns a value of type LuminousFlux
impl<T> core::ops::Div<&InverseLuminosity<T>> for &SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn div(self, rhs: &InverseLuminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr.clone() / rhs.per_cd.clone()}
	}
}

// SolidAngle * Luminosity -> LuminousFlux
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> core::ops::Mul<Luminosity<T>> for SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr * rhs.cd}
	}
}
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> core::ops::Mul<Luminosity<T>> for &SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr.clone() * rhs.cd}
	}
}
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> core::ops::Mul<&Luminosity<T>> for SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr * rhs.cd.clone()}
	}
}
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> core::ops::Mul<&Luminosity<T>> for &SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr.clone() * rhs.cd.clone()}
	}
}

// SolidAngle * InverseLuminousFlux -> InverseLuminosity
/// Multiplying a SolidAngle by a InverseLuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr * rhs.per_lm}
	}
}
/// Multiplying a SolidAngle by a InverseLuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Mul<InverseLuminousFlux<T>> for &SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: InverseLuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr.clone() * rhs.per_lm}
	}
}
/// Multiplying a SolidAngle by a InverseLuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr * rhs.per_lm.clone()}
	}
}
/// Multiplying a SolidAngle by a InverseLuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Mul<&InverseLuminousFlux<T>> for &SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn mul(self, rhs: &InverseLuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr.clone() * rhs.per_lm.clone()}
	}
}

// SolidAngle / LuminousFlux -> InverseLuminosity
/// Dividing a SolidAngle by a LuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Div<LuminousFlux<T>> for SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr / rhs.lm}
	}
}
/// Dividing a SolidAngle by a LuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Div<LuminousFlux<T>> for &SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: LuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr.clone() / rhs.lm}
	}
}
/// Dividing a SolidAngle by a LuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Div<&LuminousFlux<T>> for SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr / rhs.lm.clone()}
	}
}
/// Dividing a SolidAngle by a LuminousFlux returns a value of type InverseLuminosity
impl<T> core::ops::Div<&LuminousFlux<T>> for &SolidAngle<T> where T: NumLike {
	type Output = InverseLuminosity<T>;
	fn div(self, rhs: &LuminousFlux<T>) -> Self::Output {
		InverseLuminosity{per_cd: self.sr.clone() / rhs.lm.clone()}
	}
}

// SolidAngle / Angle -> Angle
/// Dividing a SolidAngle by a Angle returns a value of type Angle
impl<T> core::ops::Div<Angle<T>> for SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		Angle{rad: self.sr / rhs.rad}
	}
}
/// Dividing a SolidAngle by a Angle returns a value of type Angle
impl<T> core::ops::Div<Angle<T>> for &SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		Angle{rad: self.sr.clone() / rhs.rad}
	}
}
/// Dividing a SolidAngle by a Angle returns a value of type Angle
impl<T> core::ops::Div<&Angle<T>> for SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		Angle{rad: self.sr / rhs.rad.clone()}
	}
}
/// Dividing a SolidAngle by a Angle returns a value of type Angle
impl<T> core::ops::Div<&Angle<T>> for &SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		Angle{rad: self.sr.clone() / rhs.rad.clone()}
	}
}

// SolidAngle * InverseAngle -> Angle
/// Multiplying a SolidAngle by a InverseAngle returns a value of type Angle
impl<T> core::ops::Mul<InverseAngle<T>> for SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: self.sr * rhs.per_rad}
	}
}
/// Multiplying a SolidAngle by a InverseAngle returns a value of type Angle
impl<T> core::ops::Mul<InverseAngle<T>> for &SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: InverseAngle<T>) -> Self::Output {
		Angle{rad: self.sr.clone() * rhs.per_rad}
	}
}
/// Multiplying a SolidAngle by a InverseAngle returns a value of type Angle
impl<T> core::ops::Mul<&InverseAngle<T>> for SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: self.sr * rhs.per_rad.clone()}
	}
}
/// Multiplying a SolidAngle by a InverseAngle returns a value of type Angle
impl<T> core::ops::Mul<&InverseAngle<T>> for &SolidAngle<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &InverseAngle<T>) -> Self::Output {
		Angle{rad: self.sr.clone() * rhs.per_rad.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<SolidAngle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
impl<T> core::ops::Div<&SolidAngle<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<SolidAngle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<SolidAngle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&SolidAngle<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&SolidAngle<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<SolidAngle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<SolidAngle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&SolidAngle<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&SolidAngle<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

// 1/SolidAngle -> InverseSolidAngle
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<SolidAngle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<SolidAngle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&SolidAngle<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self) / rhs.sr.clone()}
	}
}
/// Dividing a scalar value by a SolidAngle unit value returns a value of type InverseSolidAngle
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&SolidAngle<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseSolidAngle<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		InverseSolidAngle{per_sr: T::from(self.clone()) / rhs.sr.clone()}
	}
}

/// The volume unit type, defined as cubic meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Volume<T: NumLike>{
	/// The value of this Volume in cubic meters
	pub m3: T
}

impl<T> Volume<T> where T: NumLike {

	/// Returns the standard unit name of volume: "cubic meters"
	pub fn unit_name() -> &'static str { "cubic meters" }
	
	/// Returns the abbreviated name or symbol of volume: "m³" for cubic meters
	pub fn unit_symbol() -> &'static str { "m³" }
	
	/// Returns a new volume value from the given number of cubic meters
	///
	/// # Arguments
	/// * `m3` - Any number-like type, representing a quantity of cubic meters
	pub fn from_m3(m3: T) -> Self { Volume{m3: m3} }
	
	/// Returns a copy of this volume value in cubic meters
	pub fn to_m3(&self) -> T { self.m3.clone() }

	/// Returns a new volume value from the given number of cubic meters
	///
	/// # Arguments
	/// * `cubic_meters` - Any number-like type, representing a quantity of cubic meters
	pub fn from_cubic_meters(cubic_meters: T) -> Self { Volume{m3: cubic_meters} }
	
	/// Returns a copy of this volume value in cubic meters
	pub fn to_cubic_meters(&self) -> T { self.m3.clone() }

	/// Returns a new volume value from the given number of kiloliters
	///
	/// # Arguments
	/// * `kL` - Any number-like type, representing a quantity of cubic meters
	pub fn from_kL(kL: T) -> Self { Volume{m3: kL} }
	
	/// Returns a copy of this volume value in kiloliters
	pub fn to_kL(&self) -> T { self.m3.clone() }

}

impl<T> fmt::Display for Volume<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m3, Self::unit_symbol())
	}
}

impl<T> Volume<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this volume value in cubic cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_cc(&self) -> T {
		return self.m3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new volume value from the given number of cubic cm
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `cc` - Any number-like type, representing a quantity of cubic cm
	pub fn from_cc(cc: T) -> Self {
		Volume{m3: cc * T::from(1e-06_f64)}
	}

	/// Returns a copy of this volume value in liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_L(&self) -> T {
		return self.m3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new volume value from the given number of liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `L` - Any number-like type, representing a quantity of liters
	pub fn from_L(L: T) -> Self {
		Volume{m3: L * T::from(0.001_f64)}
	}

	/// Returns a copy of this volume value in liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_liters(&self) -> T {
		return self.m3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new volume value from the given number of liters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `liters` - Any number-like type, representing a quantity of liters
	pub fn from_liters(liters: T) -> Self {
		Volume{m3: liters * T::from(0.001_f64)}
	}

	/// Returns a copy of this volume value in milliliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_mL(&self) -> T {
		return self.m3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new volume value from the given number of milliliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `mL` - Any number-like type, representing a quantity of milliliters
	pub fn from_mL(mL: T) -> Self {
		Volume{m3: mL * T::from(1e-06_f64)}
	}

	/// Returns a copy of this volume value in microliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_uL(&self) -> T {
		return self.m3.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new volume value from the given number of microliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `uL` - Any number-like type, representing a quantity of microliters
	pub fn from_uL(uL: T) -> Self {
		Volume{m3: uL * T::from(1e-09_f64)}
	}

	/// Returns a copy of this volume value in nanoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_nL(&self) -> T {
		return self.m3.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new volume value from the given number of nanoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `nL` - Any number-like type, representing a quantity of nanoliters
	pub fn from_nL(nL: T) -> Self {
		Volume{m3: nL * T::from(1e-12_f64)}
	}

	/// Returns a copy of this volume value in picoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_pL(&self) -> T {
		return self.m3.clone() * T::from(1000000000000000.0_f64);
	}

	/// Returns a new volume value from the given number of picoliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `pL` - Any number-like type, representing a quantity of picoliters
	pub fn from_pL(pL: T) -> Self {
		Volume{m3: pL * T::from(1e-15_f64)}
	}

	/// Returns a copy of this volume value in megaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_ML(&self) -> T {
		return self.m3.clone() * T::from(0.001_f64);
	}

	/// Returns a new volume value from the given number of megaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `ML` - Any number-like type, representing a quantity of megaliters
	pub fn from_ML(ML: T) -> Self {
		Volume{m3: ML * T::from(1000.0_f64)}
	}

	/// Returns a copy of this volume value in gigaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	pub fn to_GL(&self) -> T {
		return self.m3.clone() * T::from(1e-06_f64);
	}

	/// Returns a new volume value from the given number of gigaliters
	/// 
	/// *Note: This method is not available for `f32` and other number types lacking the `From<f64>` trait*
	///
	/// # Arguments
	/// * `GL` - Any number-like type, representing a quantity of gigaliters
	pub fn from_GL(GL: T) -> Self {
		Volume{m3: GL * T::from(1000000.0_f64)}
	}

}


/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Volume<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Volume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Volume<num_bigfloat::BigFloat>) -> Self::Output {
		Volume{m3: self * rhs.m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<Volume<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Volume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: Volume<num_bigfloat::BigFloat>) -> Self::Output {
		Volume{m3: self.clone() * rhs.m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Volume<num_bigfloat::BigFloat>> for num_bigfloat::BigFloat {
	type Output = Volume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Volume<num_bigfloat::BigFloat>) -> Self::Output {
		Volume{m3: self * rhs.m3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-bigfloat")]
impl core::ops::Mul<&Volume<num_bigfloat::BigFloat>> for &num_bigfloat::BigFloat {
	type Output = Volume<num_bigfloat::BigFloat>;
	fn mul(self, rhs: &Volume<num_bigfloat::BigFloat>) -> Self::Output {
		Volume{m3: self.clone() * rhs.m3.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Volume<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Volume<num_complex::Complex32>;
	fn mul(self, rhs: Volume<num_complex::Complex32>) -> Self::Output {
		Volume{m3: self * rhs.m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Volume<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Volume<num_complex::Complex32>;
	fn mul(self, rhs: Volume<num_complex::Complex32>) -> Self::Output {
		Volume{m3: self.clone() * rhs.m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Volume<num_complex::Complex32>> for num_complex::Complex32 {
	type Output = Volume<num_complex::Complex32>;
	fn mul(self, rhs: &Volume<num_complex::Complex32>) -> Self::Output {
		Volume{m3: self * rhs.m3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Volume<num_complex::Complex32>> for &num_complex::Complex32 {
	type Output = Volume<num_complex::Complex32>;
	fn mul(self, rhs: &Volume<num_complex::Complex32>) -> Self::Output {
		Volume{m3: self.clone() * rhs.m3.clone()}
	}
}

/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Volume<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Volume<num_complex::Complex64>;
	fn mul(self, rhs: Volume<num_complex::Complex64>) -> Self::Output {
		Volume{m3: self * rhs.m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<Volume<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Volume<num_complex::Complex64>;
	fn mul(self, rhs: Volume<num_complex::Complex64>) -> Self::Output {
		Volume{m3: self.clone() * rhs.m3}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Volume<num_complex::Complex64>> for num_complex::Complex64 {
	type Output = Volume<num_complex::Complex64>;
	fn mul(self, rhs: &Volume<num_complex::Complex64>) -> Self::Output {
		Volume{m3: self * rhs.m3.clone()}
	}
}
/// Multiplying a unit value by a scalar value returns a unit value
#[cfg(feature="num-complex")]
impl core::ops::Mul<&Volume<num_complex::Complex64>> for &num_complex::Complex64 {
	type Output = Volume<num_complex::Complex64>;
	fn mul(self, rhs: &Volume<num_complex::Complex64>) -> Self::Output {
		Volume{m3: self.clone() * rhs.m3.clone()}
	}
}



/// Converts a Volume into the equivalent [uom](https://crates.io/crates/uom) type [Volume](https://docs.rs/uom/0.34.0/uom/si/f32/type.Volume.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Volume> for Volume<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Volume {
		uom::si::f32::Volume::new::<uom::si::volume::cubic_meter>(self.m3.into())
	}
}

/// Creates a Volume from the equivalent [uom](https://crates.io/crates/uom) type [Volume](https://docs.rs/uom/0.34.0/uom/si/f32/type.Volume.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Volume> for Volume<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Volume) -> Self {
		Volume{m3: T::from(src.value)}
	}
}

/// Converts a Volume into the equivalent [uom](https://crates.io/crates/uom) type [Volume](https://docs.rs/uom/0.34.0/uom/si/f64/type.Volume.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Volume> for Volume<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Volume {
		uom::si::f64::Volume::new::<uom::si::volume::cubic_meter>(self.m3.into())
	}
}

/// Creates a Volume from the equivalent [uom](https://crates.io/crates/uom) type [Volume](https://docs.rs/uom/0.34.0/uom/si/f64/type.Volume.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Volume> for Volume<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Volume) -> Self {
		Volume{m3: T::from(src.value)}
	}
}


// Volume / Amount -> MolarVolume
/// Dividing a Volume by a Amount returns a value of type MolarVolume
impl<T> core::ops::Div<Amount<T>> for Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3 / rhs.mol}
	}
}
/// Dividing a Volume by a Amount returns a value of type MolarVolume
impl<T> core::ops::Div<Amount<T>> for &Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: Amount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3.clone() / rhs.mol}
	}
}
/// Dividing a Volume by a Amount returns a value of type MolarVolume
impl<T> core::ops::Div<&Amount<T>> for Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3 / rhs.mol.clone()}
	}
}
/// Dividing a Volume by a Amount returns a value of type MolarVolume
impl<T> core::ops::Div<&Amount<T>> for &Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn div(self, rhs: &Amount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3.clone() / rhs.mol.clone()}
	}
}

// Volume / Distance -> Area
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> core::ops::Div<Distance<T>> for Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m3 / rhs.m}
	}
}
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> core::ops::Div<Distance<T>> for &Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m3.clone() / rhs.m}
	}
}
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> core::ops::Div<&Distance<T>> for Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m3 / rhs.m.clone()}
	}
}
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> core::ops::Div<&Distance<T>> for &Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m3.clone() / rhs.m.clone()}
	}
}

// Volume * InverseAmount -> MolarVolume
/// Multiplying a Volume by a InverseAmount returns a value of type MolarVolume
impl<T> core::ops::Mul<InverseAmount<T>> for Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: InverseAmount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3 * rhs.per_mol}
	}
}
/// Multiplying a Volume by a InverseAmount returns a value of type MolarVolume
impl<T> core::ops::Mul<InverseAmount<T>> for &Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: InverseAmount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3.clone() * rhs.per_mol}
	}
}
/// Multiplying a Volume by a InverseAmount returns a value of type MolarVolume
impl<T> core::ops::Mul<&InverseAmount<T>> for Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: &InverseAmount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3 * rhs.per_mol.clone()}
	}
}
/// Multiplying a Volume by a InverseAmount returns a value of type MolarVolume
impl<T> core::ops::Mul<&InverseAmount<T>> for &Volume<T> where T: NumLike {
	type Output = MolarVolume<T>;
	fn mul(self, rhs: &InverseAmount<T>) -> Self::Output {
		MolarVolume{m3_per_mol: self.m3.clone() * rhs.per_mol.clone()}
	}
}

// Volume * InverseDistance -> Area
/// Multiplying a Volume by a InverseDistance returns a value of type Area
impl<T> core::ops::Mul<InverseDistance<T>> for Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		Area{m2: self.m3 * rhs.per_m}
	}
}
/// Multiplying a Volume by a InverseDistance returns a value of type Area
impl<T> core::ops::Mul<InverseDistance<T>> for &Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: InverseDistance<T>) -> Self::Output {
		Area{m2: self.m3.clone() * rhs.per_m}
	}
}
/// Multiplying a Volume by a InverseDistance returns a value of type Area
impl<T> core::ops::Mul<&InverseDistance<T>> for Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		Area{m2: self.m3 * rhs.per_m.clone()}
	}
}
/// Multiplying a Volume by a InverseDistance returns a value of type Area
impl<T> core::ops::Mul<&InverseDistance<T>> for &Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &InverseDistance<T>) -> Self::Output {
		Area{m2: self.m3.clone() * rhs.per_m.clone()}
	}
}

// Volume * InverseMass -> VolumePerMass
/// Multiplying a Volume by a InverseMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<InverseMass<T>> for Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: InverseMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3 * rhs.per_kg}
	}
}
/// Multiplying a Volume by a InverseMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<InverseMass<T>> for &Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: InverseMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3.clone() * rhs.per_kg}
	}
}
/// Multiplying a Volume by a InverseMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<&InverseMass<T>> for Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: &InverseMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3 * rhs.per_kg.clone()}
	}
}
/// Multiplying a Volume by a InverseMass returns a value of type VolumePerMass
impl<T> core::ops::Mul<&InverseMass<T>> for &Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn mul(self, rhs: &InverseMass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3.clone() * rhs.per_kg.clone()}
	}
}

// Volume / Mass -> VolumePerMass
/// Dividing a Volume by a Mass returns a value of type VolumePerMass
impl<T> core::ops::Div<Mass<T>> for Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3 / rhs.kg}
	}
}
/// Dividing a Volume by a Mass returns a value of type VolumePerMass
impl<T> core::ops::Div<Mass<T>> for &Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3.clone() / rhs.kg}
	}
}
/// Dividing a Volume by a Mass returns a value of type VolumePerMass
impl<T> core::ops::Div<&Mass<T>> for Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3 / rhs.kg.clone()}
	}
}
/// Dividing a Volume by a Mass returns a value of type VolumePerMass
impl<T> core::ops::Div<&Mass<T>> for &Volume<T> where T: NumLike {
	type Output = VolumePerMass<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		VolumePerMass{m3_per_kg: self.m3.clone() / rhs.kg.clone()}
	}
}

// Volume * Concentration -> Amount
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> core::ops::Mul<Concentration<T>> for Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		Amount{mol: self.m3 * rhs.molpm3}
	}
}
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> core::ops::Mul<Concentration<T>> for &Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		Amount{mol: self.m3.clone() * rhs.molpm3}
	}
}
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> core::ops::Mul<&Concentration<T>> for Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		Amount{mol: self.m3 * rhs.molpm3.clone()}
	}
}
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> core::ops::Mul<&Concentration<T>> for &Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		Amount{mol: self.m3.clone() * rhs.molpm3.clone()}
	}
}

// Volume / MolarVolume -> Amount
/// Dividing a Volume by a MolarVolume returns a value of type Amount
impl<T> core::ops::Div<MolarVolume<T>> for Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: MolarVolume<T>) -> Self::Output {
		Amount{mol: self.m3 / rhs.m3_per_mol}
	}
}
/// Dividing a Volume by a MolarVolume returns a value of type Amount
impl<T> core::ops::Div<MolarVolume<T>> for &Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: MolarVolume<T>) -> Self::Output {
		Amount{mol: self.m3.clone() / rhs.m3_per_mol}
	}
}
/// Dividing a Volume by a MolarVolume returns a value of type Amount
impl<T> core::ops::Div<&MolarVolume<T>> for Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &MolarVolume<T>) -> Self::Output {
		Amount{mol: self.m3 / rhs.m3_per_mol.clone()}
	}
}
/// Dividing a Volume by a MolarVolume returns a value of type Amount
impl<T> core::ops::Div<&MolarVolume<T>> for &Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn div(self, rhs: &MolarVolume<T>) -> Self::Output {
		Amount{mol: self.m3.clone() / rhs.m3_per_mol.clone()}
	}
}

// Volume / Area -> Distance
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> core::ops::Div<Area<T>> for Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Distance{m: self.m3 / rhs.m2}
	}
}
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> core::ops::Div<Area<T>> for &Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Distance{m: self.m3.clone() / rhs.m2}
	}
}
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> core::ops::Div<&Area<T>> for Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Distance{m: self.m3 / rhs.m2.clone()}
	}
}
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> core::ops::Div<&Area<T>> for &Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Distance{m: self.m3.clone() / rhs.m2.clone()}
	}
}

// Volume * InverseArea -> Distance
/// Multiplying a Volume by a InverseArea returns a value of type Distance
impl<T> core::ops::Mul<InverseArea<T>> for Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		Distance{m: self.m3 * rhs.per_m2}
	}
}
/// Multiplying a Volume by a InverseArea returns a value of type Distance
impl<T> core::ops::Mul<InverseArea<T>> for &Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: InverseArea<T>) -> Self::Output {
		Distance{m: self.m3.clone() * rhs.per_m2}
	}
}
/// Multiplying a Volume by a InverseArea returns a value of type Distance
impl<T> core::ops::Mul<&InverseArea<T>> for Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		Distance{m: self.m3 * rhs.per_m2.clone()}
	}
}
/// Multiplying a Volume by a InverseArea returns a value of type Distance
impl<T> core::ops::Mul<&InverseArea<T>> for &Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &InverseArea<T>) -> Self::Output {
		Distance{m: self.m3.clone() * rhs.per_m2.clone()}
	}
}

// Volume * Density -> Mass
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> core::ops::Mul<Density<T>> for Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		Mass{kg: self.m3 * rhs.kgpm3}
	}
}
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> core::ops::Mul<Density<T>> for &Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		Mass{kg: self.m3.clone() * rhs.kgpm3}
	}
}
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> core::ops::Mul<&Density<T>> for Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		Mass{kg: self.m3 * rhs.kgpm3.clone()}
	}
}
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> core::ops::Mul<&Density<T>> for &Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		Mass{kg: self.m3.clone() * rhs.kgpm3.clone()}
	}
}

// Volume / Energy -> InversePressure
/// Dividing a Volume by a Energy returns a value of type InversePressure
impl<T> core::ops::Div<Energy<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 / rhs.J}
	}
}
/// Dividing a Volume by a Energy returns a value of type InversePressure
impl<T> core::ops::Div<Energy<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() / rhs.J}
	}
}
/// Dividing a Volume by a Energy returns a value of type InversePressure
impl<T> core::ops::Div<&Energy<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 / rhs.J.clone()}
	}
}
/// Dividing a Volume by a Energy returns a value of type InversePressure
impl<T> core::ops::Div<&Energy<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() / rhs.J.clone()}
	}
}

// Volume / Torque -> InversePressure
/// Dividing a Volume by a Torque returns a value of type InversePressure
impl<T> core::ops::Div<Torque<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 / rhs.Nm}
	}
}
/// Dividing a Volume by a Torque returns a value of type InversePressure
impl<T> core::ops::Div<Torque<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() / rhs.Nm}
	}
}
/// Dividing a Volume by a Torque returns a value of type InversePressure
impl<T> core::ops::Div<&Torque<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 / rhs.Nm.clone()}
	}
}
/// Dividing a Volume by a Torque returns a value of type InversePressure
impl<T> core::ops::Div<&Torque<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() / rhs.Nm.clone()}
	}
}

// Volume * InverseEnergy -> InversePressure
/// Multiplying a Volume by a InverseEnergy returns a value of type InversePressure
impl<T> core::ops::Mul<InverseEnergy<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 * rhs.per_J}
	}
}
/// Multiplying a Volume by a InverseEnergy returns a value of type InversePressure
impl<T> core::ops::Mul<InverseEnergy<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: InverseEnergy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() * rhs.per_J}
	}
}
/// Multiplying a Volume by a InverseEnergy returns a value of type InversePressure
impl<T> core::ops::Mul<&InverseEnergy<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 * rhs.per_J.clone()}
	}
}
/// Multiplying a Volume by a InverseEnergy returns a value of type InversePressure
impl<T> core::ops::Mul<&InverseEnergy<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: &InverseEnergy<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() * rhs.per_J.clone()}
	}
}

// Volume * InverseTorque -> InversePressure
/// Multiplying a Volume by a InverseTorque returns a value of type InversePressure
impl<T> core::ops::Mul<InverseTorque<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 * rhs.per_Nm}
	}
}
/// Multiplying a Volume by a InverseTorque returns a value of type InversePressure
impl<T> core::ops::Mul<InverseTorque<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: InverseTorque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() * rhs.per_Nm}
	}
}
/// Multiplying a Volume by a InverseTorque returns a value of type InversePressure
impl<T> core::ops::Mul<&InverseTorque<T>> for Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3 * rhs.per_Nm.clone()}
	}
}
/// Multiplying a Volume by a InverseTorque returns a value of type InversePressure
impl<T> core::ops::Mul<&InverseTorque<T>> for &Volume<T> where T: NumLike {
	type Output = InversePressure<T>;
	fn mul(self, rhs: &InverseTorque<T>) -> Self::Output {
		InversePressure{per_Pa: self.m3.clone() * rhs.per_Nm.clone()}
	}
}

// Volume / InversePressure -> Energy
/// Dividing a Volume by a InversePressure returns a value of type Energy
impl<T> core::ops::Div<InversePressure<T>> for Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InversePressure<T>) -> Self::Output {
		Energy{J: self.m3 / rhs.per_Pa}
	}
}
/// Dividing a Volume by a InversePressure returns a value of type Energy
impl<T> core::ops::Div<InversePressure<T>> for &Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: InversePressure<T>) -> Self::Output {
		Energy{J: self.m3.clone() / rhs.per_Pa}
	}
}
/// Dividing a Volume by a InversePressure returns a value of type Energy
impl<T> core::ops::Div<&InversePressure<T>> for Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InversePressure<T>) -> Self::Output {
		Energy{J: self.m3 / rhs.per_Pa.clone()}
	}
}
/// Dividing a Volume by a InversePressure returns a value of type Energy
impl<T> core::ops::Div<&InversePressure<T>> for &Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &InversePressure<T>) -> Self::Output {
		Energy{J: self.m3.clone() / rhs.per_Pa.clone()}
	}
}

// Volume * Pressure -> Energy
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> core::ops::Mul<Pressure<T>> for Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Energy{J: self.m3 * rhs.Pa}
	}
}
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> core::ops::Mul<Pressure<T>> for &Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Energy{J: self.m3.clone() * rhs.Pa}
	}
}
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> core::ops::Mul<&Pressure<T>> for Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Energy{J: self.m3 * rhs.Pa.clone()}
	}
}
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> core::ops::Mul<&Pressure<T>> for &Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Energy{J: self.m3.clone() * rhs.Pa.clone()}
	}
}

// Volume / VolumePerMass -> Mass
/// Dividing a Volume by a VolumePerMass returns a value of type Mass
impl<T> core::ops::Div<VolumePerMass<T>> for Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: VolumePerMass<T>) -> Self::Output {
		Mass{kg: self.m3 / rhs.m3_per_kg}
	}
}
/// Dividing a Volume by a VolumePerMass returns a value of type Mass
impl<T> core::ops::Div<VolumePerMass<T>> for &Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: VolumePerMass<T>) -> Self::Output {
		Mass{kg: self.m3.clone() / rhs.m3_per_kg}
	}
}
/// Dividing a Volume by a VolumePerMass returns a value of type Mass
impl<T> core::ops::Div<&VolumePerMass<T>> for Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &VolumePerMass<T>) -> Self::Output {
		Mass{kg: self.m3 / rhs.m3_per_kg.clone()}
	}
}
/// Dividing a Volume by a VolumePerMass returns a value of type Mass
impl<T> core::ops::Div<&VolumePerMass<T>> for &Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &VolumePerMass<T>) -> Self::Output {
		Mass{kg: self.m3.clone() / rhs.m3_per_kg.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for f64 where T: NumLike+From<f64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for &f64 where T: NumLike+From<f64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for f32 where T: NumLike+From<f32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for &f32 where T: NumLike+From<f32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for i64 where T: NumLike+From<i64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for &i64 where T: NumLike+From<i64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<Volume<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for i32 where T: NumLike+From<i32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
impl<T> core::ops::Div<&Volume<T>> for &i32 where T: NumLike+From<i32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Volume<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<Volume<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Volume<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-bigfloat")]
impl<T> core::ops::Div<&Volume<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Volume<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Volume<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Volume<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Volume<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}

// 1/Volume -> InverseVolume
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Volume<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<Volume<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Volume<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self) / rhs.m3.clone()}
	}
}
/// Dividing a scalar value by a Volume unit value returns a value of type InverseVolume
#[cfg(feature="num-complex")]
impl<T> core::ops::Div<&Volume<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = InverseVolume<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		InverseVolume{per_m3: T::from(self.clone()) / rhs.m3.clone()}
	}
}



