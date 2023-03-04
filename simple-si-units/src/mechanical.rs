
//! This module provides mechanical SI units, such as angular velocity 
//! and velocity.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::nuclear::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num_bigfloat")]
use num_bigfloat;
#[cfg(feature="num_complex")]
use num_complex;
#[cfg(feature="astro_float")]
use astro_float;


/// The angular velocity unit type, defined as radians per second in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AngularVelocity<T: NumLike>{
	/// The value of this Angular velocity in radians per second
	pub radps: T
}

impl<T> AngularVelocity<T> where T: NumLike {

	/// Returns the standard unit name of angular velocity: "radians per second"
	pub fn unit_name() -> &'static str {
		return "radians per second";
	}
	
	/// Returns the abbreviated name or symbol of angular velocity: "radps" for radians per second
	pub fn unit_symbol() -> &'static str {
		return "radps";
	}
	
	/// Returns a new angular velocity value from the given number of radians per second
	///
	/// # Arguments
	/// * `radps` - Any number-like type, representing a quantity of radians per second
	pub fn from_radps(radps: T) -> Self {
		AngularVelocity{radps: radps}
	}
	
	/// Returns a copy of this angular velocity value in radians per second
	pub fn to_radps(self) -> T {
		return self.radps.clone();
	}

	/// Returns a new angular velocity value from the given number of radians per second
	///
	/// # Arguments
	/// * `radians_per_second` - Any number-like type, representing a quantity of radians per second
	pub fn from_radians_per_second(radians_per_second: T) -> Self {
		AngularVelocity{radps: radians_per_second}
	}
	
	/// Returns a copy of this angular velocity value in radians per second
	pub fn to_radians_per_second(self) -> T {
		return self.radps.clone();
	}

}

impl<T> fmt::Display for AngularVelocity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.radps, Self::unit_symbol())
	}
}

impl<T> AngularVelocity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this angular velocity value in degrees per second
	pub fn to_degrees_per_second(self) -> T {
		return self.radps.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angular velocity value from the given number of degrees per second
	///
	/// # Arguments
	/// * `degrees_per_second` - Any number-like type, representing a quantity of degrees per second
	pub fn from_degrees_per_second(degrees_per_second: T) -> Self {
		AngularVelocity{radps: degrees_per_second * T::from(0.0174532925199433_f64)}
	}

	/// Returns a copy of this angular velocity value in degrees per second
	pub fn to_degps(self) -> T {
		return self.radps.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angular velocity value from the given number of degrees per second
	///
	/// # Arguments
	/// * `degps` - Any number-like type, representing a quantity of degrees per second
	pub fn from_degps(degps: T) -> Self {
		AngularVelocity{radps: degps * T::from(0.0174532925199433_f64)}
	}

	/// Returns a copy of this angular velocity value in revolutions per second
	pub fn to_rps(self) -> T {
		return self.radps.clone() * T::from(0.159154943091895_f64);
	}

	/// Returns a new angular velocity value from the given number of revolutions per second
	///
	/// # Arguments
	/// * `rps` - Any number-like type, representing a quantity of revolutions per second
	pub fn from_rps(rps: T) -> Self {
		AngularVelocity{radps: rps * T::from(6.28318530717959_f64)}
	}

	/// Returns a copy of this angular velocity value in revolutions per minute
	pub fn to_rpm(self) -> T {
		return self.radps.clone() * T::from(9.54929658551372_f64);
	}

	/// Returns a new angular velocity value from the given number of revolutions per minute
	///
	/// # Arguments
	/// * `rpm` - Any number-like type, representing a quantity of revolutions per minute
	pub fn from_rpm(rpm: T) -> Self {
		AngularVelocity{radps: rpm * T::from(0.10471975511966_f64)}
	}

	/// Returns a copy of this angular velocity value in revolutions per hour
	pub fn to_rph(self) -> T {
		return self.radps.clone() * T::from(572.957795130823_f64);
	}

	/// Returns a new angular velocity value from the given number of revolutions per hour
	///
	/// # Arguments
	/// * `rph` - Any number-like type, representing a quantity of revolutions per hour
	pub fn from_rph(rph: T) -> Self {
		AngularVelocity{radps: rph * T::from(0.0017453292519943_f64)}
	}

}

// AngularVelocity * Time -> Angle
/// Multiplying a AngularVelocity by a Time returns a value of type Angle
impl<T> std::ops::Mul<Time<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Angle{rad: self.radps * rhs.s}
	}
}
/// Multiplying a AngularVelocity by a Time returns a value of type Angle
impl<T> std::ops::Mul<Time<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Angle{rad: self.radps.clone() * rhs.s}
	}
}
/// Multiplying a AngularVelocity by a Time returns a value of type Angle
impl<T> std::ops::Mul<&Time<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Angle{rad: self.radps * rhs.s.clone()}
	}
}
/// Multiplying a AngularVelocity by a Time returns a value of type Angle
impl<T> std::ops::Mul<&Time<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Angle{rad: self.radps.clone() * rhs.s.clone()}
	}
}

// AngularVelocity / Time -> AngularAcceleration
/// Dividing a AngularVelocity by a Time returns a value of type AngularAcceleration
impl<T> std::ops::Div<Time<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps / rhs.s}
	}
}
/// Dividing a AngularVelocity by a Time returns a value of type AngularAcceleration
impl<T> std::ops::Div<Time<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps.clone() / rhs.s}
	}
}
/// Dividing a AngularVelocity by a Time returns a value of type AngularAcceleration
impl<T> std::ops::Div<&Time<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps / rhs.s.clone()}
	}
}
/// Dividing a AngularVelocity by a Time returns a value of type AngularAcceleration
impl<T> std::ops::Div<&Time<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps.clone() / rhs.s.clone()}
	}
}

// AngularVelocity / Angle -> Frequency
/// Dividing a AngularVelocity by a Angle returns a value of type Frequency
impl<T> std::ops::Div<Angle<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		Frequency{Hz: self.radps / rhs.rad}
	}
}
/// Dividing a AngularVelocity by a Angle returns a value of type Frequency
impl<T> std::ops::Div<Angle<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Angle<T>) -> Self::Output {
		Frequency{Hz: self.radps.clone() / rhs.rad}
	}
}
/// Dividing a AngularVelocity by a Angle returns a value of type Frequency
impl<T> std::ops::Div<&Angle<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		Frequency{Hz: self.radps / rhs.rad.clone()}
	}
}
/// Dividing a AngularVelocity by a Angle returns a value of type Frequency
impl<T> std::ops::Div<&Angle<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Angle<T>) -> Self::Output {
		Frequency{Hz: self.radps.clone() / rhs.rad.clone()}
	}
}

// AngularVelocity / AngularAcceleration -> Time
/// Dividing a AngularVelocity by a AngularAcceleration returns a value of type Time
impl<T> std::ops::Div<AngularAcceleration<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: AngularAcceleration<T>) -> Self::Output {
		Time{s: self.radps / rhs.radps2}
	}
}
/// Dividing a AngularVelocity by a AngularAcceleration returns a value of type Time
impl<T> std::ops::Div<AngularAcceleration<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: AngularAcceleration<T>) -> Self::Output {
		Time{s: self.radps.clone() / rhs.radps2}
	}
}
/// Dividing a AngularVelocity by a AngularAcceleration returns a value of type Time
impl<T> std::ops::Div<&AngularAcceleration<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &AngularAcceleration<T>) -> Self::Output {
		Time{s: self.radps / rhs.radps2.clone()}
	}
}
/// Dividing a AngularVelocity by a AngularAcceleration returns a value of type Time
impl<T> std::ops::Div<&AngularAcceleration<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &AngularAcceleration<T>) -> Self::Output {
		Time{s: self.radps.clone() / rhs.radps2.clone()}
	}
}

// AngularVelocity * MomentOfInertia -> AngularMomentum
/// Multiplying a AngularVelocity by a MomentOfInertia returns a value of type AngularMomentum
impl<T> std::ops::Mul<MomentOfInertia<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps * rhs.kgm2}
	}
}
/// Multiplying a AngularVelocity by a MomentOfInertia returns a value of type AngularMomentum
impl<T> std::ops::Mul<MomentOfInertia<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps.clone() * rhs.kgm2}
	}
}
/// Multiplying a AngularVelocity by a MomentOfInertia returns a value of type AngularMomentum
impl<T> std::ops::Mul<&MomentOfInertia<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps * rhs.kgm2.clone()}
	}
}
/// Multiplying a AngularVelocity by a MomentOfInertia returns a value of type AngularMomentum
impl<T> std::ops::Mul<&MomentOfInertia<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps.clone() * rhs.kgm2.clone()}
	}
}

// AngularVelocity * AreaDensity -> AngularMomentum
/// Multiplying a AngularVelocity by a AreaDensity returns a value of type AngularMomentum
impl<T> std::ops::Mul<AreaDensity<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps * rhs.kgm2}
	}
}
/// Multiplying a AngularVelocity by a AreaDensity returns a value of type AngularMomentum
impl<T> std::ops::Mul<AreaDensity<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps.clone() * rhs.kgm2}
	}
}
/// Multiplying a AngularVelocity by a AreaDensity returns a value of type AngularMomentum
impl<T> std::ops::Mul<&AreaDensity<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps * rhs.kgm2.clone()}
	}
}
/// Multiplying a AngularVelocity by a AreaDensity returns a value of type AngularMomentum
impl<T> std::ops::Mul<&AreaDensity<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &AreaDensity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.radps.clone() * rhs.kgm2.clone()}
	}
}

// AngularVelocity * Frequency -> AngularAcceleration
/// Multiplying a AngularVelocity by a Frequency returns a value of type AngularAcceleration
impl<T> std::ops::Mul<Frequency<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps * rhs.Hz}
	}
}
/// Multiplying a AngularVelocity by a Frequency returns a value of type AngularAcceleration
impl<T> std::ops::Mul<Frequency<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps.clone() * rhs.Hz}
	}
}
/// Multiplying a AngularVelocity by a Frequency returns a value of type AngularAcceleration
impl<T> std::ops::Mul<&Frequency<T>> for AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps * rhs.Hz.clone()}
	}
}
/// Multiplying a AngularVelocity by a Frequency returns a value of type AngularAcceleration
impl<T> std::ops::Mul<&Frequency<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		AngularAcceleration{radps2: self.radps.clone() * rhs.Hz.clone()}
	}
}

// AngularVelocity / Frequency -> Angle
/// Dividing a AngularVelocity by a Frequency returns a value of type Angle
impl<T> std::ops::Div<Frequency<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Angle{rad: self.radps / rhs.Hz}
	}
}
/// Dividing a AngularVelocity by a Frequency returns a value of type Angle
impl<T> std::ops::Div<Frequency<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Angle{rad: self.radps.clone() / rhs.Hz}
	}
}
/// Dividing a AngularVelocity by a Frequency returns a value of type Angle
impl<T> std::ops::Div<&Frequency<T>> for AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Angle{rad: self.radps / rhs.Hz.clone()}
	}
}
/// Dividing a AngularVelocity by a Frequency returns a value of type Angle
impl<T> std::ops::Div<&Frequency<T>> for &AngularVelocity<T> where T: NumLike {
	type Output = Angle<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Angle{rad: self.radps.clone() / rhs.Hz.clone()}
	}
}

/// The angular acceleration unit type, defined as radians per second squared in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AngularAcceleration<T: NumLike>{
	/// The value of this Angular acceleration in radians per second squared
	pub radps2: T
}

impl<T> AngularAcceleration<T> where T: NumLike {

	/// Returns the standard unit name of angular acceleration: "radians per second squared"
	pub fn unit_name() -> &'static str {
		return "radians per second squared";
	}
	
	/// Returns the abbreviated name or symbol of angular acceleration: "radps2" for radians per second squared
	pub fn unit_symbol() -> &'static str {
		return "radps2";
	}
	
	/// Returns a new angular acceleration value from the given number of radians per second squared
	///
	/// # Arguments
	/// * `radps2` - Any number-like type, representing a quantity of radians per second squared
	pub fn from_radps2(radps2: T) -> Self {
		AngularAcceleration{radps2: radps2}
	}
	
	/// Returns a copy of this angular acceleration value in radians per second squared
	pub fn to_radps2(self) -> T {
		return self.radps2.clone();
	}

	/// Returns a new angular acceleration value from the given number of radians per second squared
	///
	/// # Arguments
	/// * `radians_per_second_squared` - Any number-like type, representing a quantity of radians per second squared
	pub fn from_radians_per_second_squared(radians_per_second_squared: T) -> Self {
		AngularAcceleration{radps2: radians_per_second_squared}
	}
	
	/// Returns a copy of this angular acceleration value in radians per second squared
	pub fn to_radians_per_second_squared(self) -> T {
		return self.radps2.clone();
	}

}

impl<T> fmt::Display for AngularAcceleration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.radps2, Self::unit_symbol())
	}
}

impl<T> AngularAcceleration<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this angular acceleration value in degrees per second squared
	pub fn to_degrees_per_second_squared(self) -> T {
		return self.radps2.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angular acceleration value from the given number of degrees per second squared
	///
	/// # Arguments
	/// * `degrees_per_second_squared` - Any number-like type, representing a quantity of degrees per second squared
	pub fn from_degrees_per_second_squared(degrees_per_second_squared: T) -> Self {
		AngularAcceleration{radps2: degrees_per_second_squared * T::from(0.0174532925199433_f64)}
	}

	/// Returns a copy of this angular acceleration value in revolutions per second squared
	pub fn to_rps2(self) -> T {
		return self.radps2.clone() * T::from(0.159154943091895_f64);
	}

	/// Returns a new angular acceleration value from the given number of revolutions per second squared
	///
	/// # Arguments
	/// * `rps2` - Any number-like type, representing a quantity of revolutions per second squared
	pub fn from_rps2(rps2: T) -> Self {
		AngularAcceleration{radps2: rps2 * T::from(6.28318530717959_f64)}
	}

	/// Returns a copy of this angular acceleration value in revolutions per minute squared
	pub fn to_rpm2(self) -> T {
		return self.radps2.clone() * T::from(572.957795130823_f64);
	}

	/// Returns a new angular acceleration value from the given number of revolutions per minute squared
	///
	/// # Arguments
	/// * `rpm2` - Any number-like type, representing a quantity of revolutions per minute squared
	pub fn from_rpm2(rpm2: T) -> Self {
		AngularAcceleration{radps2: rpm2 * T::from(0.0017453292519943_f64)}
	}

	/// Returns a copy of this angular acceleration value in degrees per second squared
	pub fn to_degps2(self) -> T {
		return self.radps2.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angular acceleration value from the given number of degrees per second squared
	///
	/// # Arguments
	/// * `degps2` - Any number-like type, representing a quantity of degrees per second squared
	pub fn from_degps2(degps2: T) -> Self {
		AngularAcceleration{radps2: degps2 * T::from(0.0174532925199433_f64)}
	}

	/// Returns a copy of this angular acceleration value in revolutions per hour squared
	pub fn to_rph2(self) -> T {
		return self.radps2.clone() * T::from(2062648.06247096_f64);
	}

	/// Returns a new angular acceleration value from the given number of revolutions per hour squared
	///
	/// # Arguments
	/// * `rph2` - Any number-like type, representing a quantity of revolutions per hour squared
	pub fn from_rph2(rph2: T) -> Self {
		AngularAcceleration{radps2: rph2 * T::from(4.84813681109536e-07_f64)}
	}

}

// AngularAcceleration * Time -> AngularVelocity
/// Multiplying a AngularAcceleration by a Time returns a value of type AngularVelocity
impl<T> std::ops::Mul<Time<T>> for AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2 * rhs.s}
	}
}
/// Multiplying a AngularAcceleration by a Time returns a value of type AngularVelocity
impl<T> std::ops::Mul<Time<T>> for &AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2.clone() * rhs.s}
	}
}
/// Multiplying a AngularAcceleration by a Time returns a value of type AngularVelocity
impl<T> std::ops::Mul<&Time<T>> for AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2 * rhs.s.clone()}
	}
}
/// Multiplying a AngularAcceleration by a Time returns a value of type AngularVelocity
impl<T> std::ops::Mul<&Time<T>> for &AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2.clone() * rhs.s.clone()}
	}
}

// AngularAcceleration / AngularVelocity -> Frequency
/// Dividing a AngularAcceleration by a AngularVelocity returns a value of type Frequency
impl<T> std::ops::Div<AngularVelocity<T>> for AngularAcceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.radps2 / rhs.radps}
	}
}
/// Dividing a AngularAcceleration by a AngularVelocity returns a value of type Frequency
impl<T> std::ops::Div<AngularVelocity<T>> for &AngularAcceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.radps2.clone() / rhs.radps}
	}
}
/// Dividing a AngularAcceleration by a AngularVelocity returns a value of type Frequency
impl<T> std::ops::Div<&AngularVelocity<T>> for AngularAcceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.radps2 / rhs.radps.clone()}
	}
}
/// Dividing a AngularAcceleration by a AngularVelocity returns a value of type Frequency
impl<T> std::ops::Div<&AngularVelocity<T>> for &AngularAcceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Frequency{Hz: self.radps2.clone() / rhs.radps.clone()}
	}
}

// AngularAcceleration / Frequency -> AngularVelocity
/// Dividing a AngularAcceleration by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Div<Frequency<T>> for AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2 / rhs.Hz}
	}
}
/// Dividing a AngularAcceleration by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Div<Frequency<T>> for &AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2.clone() / rhs.Hz}
	}
}
/// Dividing a AngularAcceleration by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Div<&Frequency<T>> for AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2 / rhs.Hz.clone()}
	}
}
/// Dividing a AngularAcceleration by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Div<&Frequency<T>> for &AngularAcceleration<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.radps2.clone() / rhs.Hz.clone()}
	}
}

/// The moment of inertia unit type, defined as kilogram meters squared in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MomentOfInertia<T: NumLike>{
	/// The value of this Moment of inertia in kilogram meters squared
	pub kgm2: T
}

impl<T> MomentOfInertia<T> where T: NumLike {

	/// Returns the standard unit name of moment of inertia: "kilogram meters squared"
	pub fn unit_name() -> &'static str {
		return "kilogram meters squared";
	}
	
	/// Returns the abbreviated name or symbol of moment of inertia: "kgm2" for kilogram meters squared
	pub fn unit_symbol() -> &'static str {
		return "kgm2";
	}
	
	/// Returns a new moment of inertia value from the given number of kilogram meters squared
	///
	/// # Arguments
	/// * `kgm2` - Any number-like type, representing a quantity of kilogram meters squared
	pub fn from_kgm2(kgm2: T) -> Self {
		MomentOfInertia{kgm2: kgm2}
	}
	
	/// Returns a copy of this moment of inertia value in kilogram meters squared
	pub fn to_kgm2(self) -> T {
		return self.kgm2.clone();
	}

	/// Returns a new moment of inertia value from the given number of kilogram meters squared
	///
	/// # Arguments
	/// * `kilogram_meters_squared` - Any number-like type, representing a quantity of kilogram meters squared
	pub fn from_kilogram_meters_squared(kilogram_meters_squared: T) -> Self {
		MomentOfInertia{kgm2: kilogram_meters_squared}
	}
	
	/// Returns a copy of this moment of inertia value in kilogram meters squared
	pub fn to_kilogram_meters_squared(self) -> T {
		return self.kgm2.clone();
	}

}

impl<T> fmt::Display for MomentOfInertia<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgm2, Self::unit_symbol())
	}
}

impl<T> MomentOfInertia<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this moment of inertia value in gram cm squared
	pub fn to_gcm2(self) -> T {
		return self.kgm2.clone() * T::from(0.1_f64);
	}

	/// Returns a new moment of inertia value from the given number of gram cm squared
	///
	/// # Arguments
	/// * `gcm2` - Any number-like type, representing a quantity of gram cm squared
	pub fn from_gcm2(gcm2: T) -> Self {
		MomentOfInertia{kgm2: gcm2 * T::from(10.0_f64)}
	}

	/// Returns a copy of this moment of inertia value in gram meters squared
	pub fn to_gm2(self) -> T {
		return self.kgm2.clone() * T::from(1000.0_f64);
	}

	/// Returns a new moment of inertia value from the given number of gram meters squared
	///
	/// # Arguments
	/// * `gm2` - Any number-like type, representing a quantity of gram meters squared
	pub fn from_gm2(gm2: T) -> Self {
		MomentOfInertia{kgm2: gm2 * T::from(0.001_f64)}
	}

}

// MomentOfInertia / Mass -> Area
/// Dividing a MomentOfInertia by a Mass returns a value of type Area
impl<T> std::ops::Div<Mass<T>> for MomentOfInertia<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Area{m2: self.kgm2 / rhs.kg}
	}
}
/// Dividing a MomentOfInertia by a Mass returns a value of type Area
impl<T> std::ops::Div<Mass<T>> for &MomentOfInertia<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Area{m2: self.kgm2.clone() / rhs.kg}
	}
}
/// Dividing a MomentOfInertia by a Mass returns a value of type Area
impl<T> std::ops::Div<&Mass<T>> for MomentOfInertia<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Area{m2: self.kgm2 / rhs.kg.clone()}
	}
}
/// Dividing a MomentOfInertia by a Mass returns a value of type Area
impl<T> std::ops::Div<&Mass<T>> for &MomentOfInertia<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Area{m2: self.kgm2.clone() / rhs.kg.clone()}
	}
}

// MomentOfInertia * AngularVelocity -> AngularMomentum
/// Multiplying a MomentOfInertia by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<AngularVelocity<T>> for MomentOfInertia<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2 * rhs.radps}
	}
}
/// Multiplying a MomentOfInertia by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<AngularVelocity<T>> for &MomentOfInertia<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2.clone() * rhs.radps}
	}
}
/// Multiplying a MomentOfInertia by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<&AngularVelocity<T>> for MomentOfInertia<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2 * rhs.radps.clone()}
	}
}
/// Multiplying a MomentOfInertia by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<&AngularVelocity<T>> for &MomentOfInertia<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2.clone() * rhs.radps.clone()}
	}
}

// MomentOfInertia / Area -> Mass
/// Dividing a MomentOfInertia by a Area returns a value of type Mass
impl<T> std::ops::Div<Area<T>> for MomentOfInertia<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Mass{kg: self.kgm2 / rhs.m2}
	}
}
/// Dividing a MomentOfInertia by a Area returns a value of type Mass
impl<T> std::ops::Div<Area<T>> for &MomentOfInertia<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Mass{kg: self.kgm2.clone() / rhs.m2}
	}
}
/// Dividing a MomentOfInertia by a Area returns a value of type Mass
impl<T> std::ops::Div<&Area<T>> for MomentOfInertia<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Mass{kg: self.kgm2 / rhs.m2.clone()}
	}
}
/// Dividing a MomentOfInertia by a Area returns a value of type Mass
impl<T> std::ops::Div<&Area<T>> for &MomentOfInertia<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Mass{kg: self.kgm2.clone() / rhs.m2.clone()}
	}
}

/// The angular momentum unit type, defined as kilogram meters squared radians per second in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AngularMomentum<T: NumLike>{
	/// The value of this Angular momentum in kilogram meters squared radians per second
	pub kgm2radps: T
}

impl<T> AngularMomentum<T> where T: NumLike {

	/// Returns the standard unit name of angular momentum: "kilogram meters squared radians per second"
	pub fn unit_name() -> &'static str {
		return "kilogram meters squared radians per second";
	}
	
	/// Returns the abbreviated name or symbol of angular momentum: "kgm2radps" for kilogram meters squared radians per second
	pub fn unit_symbol() -> &'static str {
		return "kgm2radps";
	}
	
	/// Returns a new angular momentum value from the given number of kilogram meters squared radians per second
	///
	/// # Arguments
	/// * `kgm2radps` - Any number-like type, representing a quantity of kilogram meters squared radians per second
	pub fn from_kgm2radps(kgm2radps: T) -> Self {
		AngularMomentum{kgm2radps: kgm2radps}
	}
	
	/// Returns a copy of this angular momentum value in kilogram meters squared radians per second
	pub fn to_kgm2radps(self) -> T {
		return self.kgm2radps.clone();
	}

	/// Returns a new angular momentum value from the given number of kilogram meters squared radians per second
	///
	/// # Arguments
	/// * `kilogram_meters_squared_radians_per_second` - Any number-like type, representing a quantity of kilogram meters squared radians per second
	pub fn from_kilogram_meters_squared_radians_per_second(kilogram_meters_squared_radians_per_second: T) -> Self {
		AngularMomentum{kgm2radps: kilogram_meters_squared_radians_per_second}
	}
	
	/// Returns a copy of this angular momentum value in kilogram meters squared radians per second
	pub fn to_kilogram_meters_squared_radians_per_second(self) -> T {
		return self.kgm2radps.clone();
	}

}

impl<T> fmt::Display for AngularMomentum<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgm2radps, Self::unit_symbol())
	}
}

impl<T> AngularMomentum<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this angular momentum value in gram cm squared radians per second
	pub fn to_gcm2radps(self) -> T {
		return self.kgm2radps.clone() * T::from(0.1_f64);
	}

	/// Returns a new angular momentum value from the given number of gram cm squared radians per second
	///
	/// # Arguments
	/// * `gcm2radps` - Any number-like type, representing a quantity of gram cm squared radians per second
	pub fn from_gcm2radps(gcm2radps: T) -> Self {
		AngularMomentum{kgm2radps: gcm2radps * T::from(10.0_f64)}
	}

}

// AngularMomentum / AngularVelocity -> AreaDensity
/// Dividing a AngularMomentum by a AngularVelocity returns a value of type AreaDensity
impl<T> std::ops::Div<AngularVelocity<T>> for AngularMomentum<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		AreaDensity{kgm2: self.kgm2radps / rhs.radps}
	}
}
/// Dividing a AngularMomentum by a AngularVelocity returns a value of type AreaDensity
impl<T> std::ops::Div<AngularVelocity<T>> for &AngularMomentum<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		AreaDensity{kgm2: self.kgm2radps.clone() / rhs.radps}
	}
}
/// Dividing a AngularMomentum by a AngularVelocity returns a value of type AreaDensity
impl<T> std::ops::Div<&AngularVelocity<T>> for AngularMomentum<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AreaDensity{kgm2: self.kgm2radps / rhs.radps.clone()}
	}
}
/// Dividing a AngularMomentum by a AngularVelocity returns a value of type AreaDensity
impl<T> std::ops::Div<&AngularVelocity<T>> for &AngularMomentum<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AreaDensity{kgm2: self.kgm2radps.clone() / rhs.radps.clone()}
	}
}

// AngularMomentum / MomentOfInertia -> AngularVelocity
/// Dividing a AngularMomentum by a MomentOfInertia returns a value of type AngularVelocity
impl<T> std::ops::Div<MomentOfInertia<T>> for AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps / rhs.kgm2}
	}
}
/// Dividing a AngularMomentum by a MomentOfInertia returns a value of type AngularVelocity
impl<T> std::ops::Div<MomentOfInertia<T>> for &AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps.clone() / rhs.kgm2}
	}
}
/// Dividing a AngularMomentum by a MomentOfInertia returns a value of type AngularVelocity
impl<T> std::ops::Div<&MomentOfInertia<T>> for AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps / rhs.kgm2.clone()}
	}
}
/// Dividing a AngularMomentum by a MomentOfInertia returns a value of type AngularVelocity
impl<T> std::ops::Div<&MomentOfInertia<T>> for &AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &MomentOfInertia<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps.clone() / rhs.kgm2.clone()}
	}
}

// AngularMomentum / AreaDensity -> AngularVelocity
/// Dividing a AngularMomentum by a AreaDensity returns a value of type AngularVelocity
impl<T> std::ops::Div<AreaDensity<T>> for AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps / rhs.kgm2}
	}
}
/// Dividing a AngularMomentum by a AreaDensity returns a value of type AngularVelocity
impl<T> std::ops::Div<AreaDensity<T>> for &AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: AreaDensity<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps.clone() / rhs.kgm2}
	}
}
/// Dividing a AngularMomentum by a AreaDensity returns a value of type AngularVelocity
impl<T> std::ops::Div<&AreaDensity<T>> for AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps / rhs.kgm2.clone()}
	}
}
/// Dividing a AngularMomentum by a AreaDensity returns a value of type AngularVelocity
impl<T> std::ops::Div<&AreaDensity<T>> for &AngularMomentum<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &AreaDensity<T>) -> Self::Output {
		AngularVelocity{radps: self.kgm2radps.clone() / rhs.kgm2.clone()}
	}
}

/// The torque unit type, defined as newton meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Torque<T: NumLike>{
	/// The value of this Torque in newton meters
	pub Nm: T
}

impl<T> Torque<T> where T: NumLike {

	/// Returns the standard unit name of torque: "newton meters"
	pub fn unit_name() -> &'static str {
		return "newton meters";
	}
	
	/// Returns the abbreviated name or symbol of torque: "Nm" for newton meters
	pub fn unit_symbol() -> &'static str {
		return "Nm";
	}
	
	/// Returns a new torque value from the given number of newton meters
	///
	/// # Arguments
	/// * `Nm` - Any number-like type, representing a quantity of newton meters
	pub fn from_Nm(Nm: T) -> Self {
		Torque{Nm: Nm}
	}
	
	/// Returns a copy of this torque value in newton meters
	pub fn to_Nm(self) -> T {
		return self.Nm.clone();
	}

	/// Returns a new torque value from the given number of newton meters
	///
	/// # Arguments
	/// * `newton_meters` - Any number-like type, representing a quantity of newton meters
	pub fn from_newton_meters(newton_meters: T) -> Self {
		Torque{Nm: newton_meters}
	}
	
	/// Returns a copy of this torque value in newton meters
	pub fn to_newton_meters(self) -> T {
		return self.Nm.clone();
	}

}

impl<T> fmt::Display for Torque<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Nm, Self::unit_symbol())
	}
}

impl<T> Torque<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this torque value in Foot-pounds
	pub fn to_ftlb(self) -> T {
		return self.Nm.clone() * T::from(1.35581794833139_f64);
	}

	/// Returns a new torque value from the given number of Foot-pounds
	///
	/// # Arguments
	/// * `ftlb` - Any number-like type, representing a quantity of Foot-pounds
	pub fn from_ftlb(ftlb: T) -> Self {
		Torque{Nm: ftlb * T::from(0.73756214927727_f64)}
	}

}

// Torque / Distance -> Force
/// Dividing a Torque by a Distance returns a value of type Force
impl<T> std::ops::Div<Distance<T>> for Torque<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Force{N: self.Nm / rhs.m}
	}
}
/// Dividing a Torque by a Distance returns a value of type Force
impl<T> std::ops::Div<Distance<T>> for &Torque<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Force{N: self.Nm.clone() / rhs.m}
	}
}
/// Dividing a Torque by a Distance returns a value of type Force
impl<T> std::ops::Div<&Distance<T>> for Torque<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Force{N: self.Nm / rhs.m.clone()}
	}
}
/// Dividing a Torque by a Distance returns a value of type Force
impl<T> std::ops::Div<&Distance<T>> for &Torque<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Force{N: self.Nm.clone() / rhs.m.clone()}
	}
}

// Torque / Time -> Power
/// Dividing a Torque by a Time returns a value of type Power
impl<T> std::ops::Div<Time<T>> for Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Power{W: self.Nm / rhs.s}
	}
}
/// Dividing a Torque by a Time returns a value of type Power
impl<T> std::ops::Div<Time<T>> for &Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Power{W: self.Nm.clone() / rhs.s}
	}
}
/// Dividing a Torque by a Time returns a value of type Power
impl<T> std::ops::Div<&Time<T>> for Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Power{W: self.Nm / rhs.s.clone()}
	}
}
/// Dividing a Torque by a Time returns a value of type Power
impl<T> std::ops::Div<&Time<T>> for &Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Power{W: self.Nm.clone() / rhs.s.clone()}
	}
}

// Torque / Current -> MagneticFlux
/// Dividing a Torque by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<Current<T>> for Torque<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.Nm / rhs.A}
	}
}
/// Dividing a Torque by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<Current<T>> for &Torque<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.Nm.clone() / rhs.A}
	}
}
/// Dividing a Torque by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<&Current<T>> for Torque<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.Nm / rhs.A.clone()}
	}
}
/// Dividing a Torque by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<&Current<T>> for &Torque<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.Nm.clone() / rhs.A.clone()}
	}
}

// Torque * Frequency -> Power
/// Multiplying a Torque by a Frequency returns a value of type Power
impl<T> std::ops::Mul<Frequency<T>> for Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Power{W: self.Nm * rhs.Hz}
	}
}
/// Multiplying a Torque by a Frequency returns a value of type Power
impl<T> std::ops::Mul<Frequency<T>> for &Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Power{W: self.Nm.clone() * rhs.Hz}
	}
}
/// Multiplying a Torque by a Frequency returns a value of type Power
impl<T> std::ops::Mul<&Frequency<T>> for Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Power{W: self.Nm * rhs.Hz.clone()}
	}
}
/// Multiplying a Torque by a Frequency returns a value of type Power
impl<T> std::ops::Mul<&Frequency<T>> for &Torque<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Power{W: self.Nm.clone() * rhs.Hz.clone()}
	}
}

// Torque / Volume -> Pressure
/// Dividing a Torque by a Volume returns a value of type Pressure
impl<T> std::ops::Div<Volume<T>> for Torque<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Pressure{Pa: self.Nm / rhs.m3}
	}
}
/// Dividing a Torque by a Volume returns a value of type Pressure
impl<T> std::ops::Div<Volume<T>> for &Torque<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Pressure{Pa: self.Nm.clone() / rhs.m3}
	}
}
/// Dividing a Torque by a Volume returns a value of type Pressure
impl<T> std::ops::Div<&Volume<T>> for Torque<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Pressure{Pa: self.Nm / rhs.m3.clone()}
	}
}
/// Dividing a Torque by a Volume returns a value of type Pressure
impl<T> std::ops::Div<&Volume<T>> for &Torque<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Pressure{Pa: self.Nm.clone() / rhs.m3.clone()}
	}
}

// Torque / Velocity -> Momentum
/// Dividing a Torque by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<Velocity<T>> for Torque<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.Nm / rhs.mps}
	}
}
/// Dividing a Torque by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<Velocity<T>> for &Torque<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.Nm.clone() / rhs.mps}
	}
}
/// Dividing a Torque by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<&Velocity<T>> for Torque<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.Nm / rhs.mps.clone()}
	}
}
/// Dividing a Torque by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<&Velocity<T>> for &Torque<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.Nm.clone() / rhs.mps.clone()}
	}
}

// Torque / Momentum -> Velocity
/// Dividing a Torque by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<Momentum<T>> for Torque<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Velocity{mps: self.Nm / rhs.kgmps}
	}
}
/// Dividing a Torque by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<Momentum<T>> for &Torque<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Velocity{mps: self.Nm.clone() / rhs.kgmps}
	}
}
/// Dividing a Torque by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<&Momentum<T>> for Torque<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Velocity{mps: self.Nm / rhs.kgmps.clone()}
	}
}
/// Dividing a Torque by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<&Momentum<T>> for &Torque<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Velocity{mps: self.Nm.clone() / rhs.kgmps.clone()}
	}
}

// Torque / Force -> Distance
/// Dividing a Torque by a Force returns a value of type Distance
impl<T> std::ops::Div<Force<T>> for Torque<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Distance{m: self.Nm / rhs.N}
	}
}
/// Dividing a Torque by a Force returns a value of type Distance
impl<T> std::ops::Div<Force<T>> for &Torque<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Distance{m: self.Nm.clone() / rhs.N}
	}
}
/// Dividing a Torque by a Force returns a value of type Distance
impl<T> std::ops::Div<&Force<T>> for Torque<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Distance{m: self.Nm / rhs.N.clone()}
	}
}
/// Dividing a Torque by a Force returns a value of type Distance
impl<T> std::ops::Div<&Force<T>> for &Torque<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Distance{m: self.Nm.clone() / rhs.N.clone()}
	}
}

// Torque / Pressure -> Volume
/// Dividing a Torque by a Pressure returns a value of type Volume
impl<T> std::ops::Div<Pressure<T>> for Torque<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		Volume{m3: self.Nm / rhs.Pa}
	}
}
/// Dividing a Torque by a Pressure returns a value of type Volume
impl<T> std::ops::Div<Pressure<T>> for &Torque<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		Volume{m3: self.Nm.clone() / rhs.Pa}
	}
}
/// Dividing a Torque by a Pressure returns a value of type Volume
impl<T> std::ops::Div<&Pressure<T>> for Torque<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		Volume{m3: self.Nm / rhs.Pa.clone()}
	}
}
/// Dividing a Torque by a Pressure returns a value of type Volume
impl<T> std::ops::Div<&Pressure<T>> for &Torque<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		Volume{m3: self.Nm.clone() / rhs.Pa.clone()}
	}
}

// Torque / Charge -> Voltage
/// Dividing a Torque by a Charge returns a value of type Voltage
impl<T> std::ops::Div<Charge<T>> for Torque<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Voltage{V: self.Nm / rhs.C}
	}
}
/// Dividing a Torque by a Charge returns a value of type Voltage
impl<T> std::ops::Div<Charge<T>> for &Torque<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Voltage{V: self.Nm.clone() / rhs.C}
	}
}
/// Dividing a Torque by a Charge returns a value of type Voltage
impl<T> std::ops::Div<&Charge<T>> for Torque<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Voltage{V: self.Nm / rhs.C.clone()}
	}
}
/// Dividing a Torque by a Charge returns a value of type Voltage
impl<T> std::ops::Div<&Charge<T>> for &Torque<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Voltage{V: self.Nm.clone() / rhs.C.clone()}
	}
}

// Torque / Power -> Time
/// Dividing a Torque by a Power returns a value of type Time
impl<T> std::ops::Div<Power<T>> for Torque<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		Time{s: self.Nm / rhs.W}
	}
}
/// Dividing a Torque by a Power returns a value of type Time
impl<T> std::ops::Div<Power<T>> for &Torque<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		Time{s: self.Nm.clone() / rhs.W}
	}
}
/// Dividing a Torque by a Power returns a value of type Time
impl<T> std::ops::Div<&Power<T>> for Torque<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		Time{s: self.Nm / rhs.W.clone()}
	}
}
/// Dividing a Torque by a Power returns a value of type Time
impl<T> std::ops::Div<&Power<T>> for &Torque<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		Time{s: self.Nm.clone() / rhs.W.clone()}
	}
}

// Torque / Voltage -> Charge
/// Dividing a Torque by a Voltage returns a value of type Charge
impl<T> std::ops::Div<Voltage<T>> for Torque<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.Nm / rhs.V}
	}
}
/// Dividing a Torque by a Voltage returns a value of type Charge
impl<T> std::ops::Div<Voltage<T>> for &Torque<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.Nm.clone() / rhs.V}
	}
}
/// Dividing a Torque by a Voltage returns a value of type Charge
impl<T> std::ops::Div<&Voltage<T>> for Torque<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.Nm / rhs.V.clone()}
	}
}
/// Dividing a Torque by a Voltage returns a value of type Charge
impl<T> std::ops::Div<&Voltage<T>> for &Torque<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.Nm.clone() / rhs.V.clone()}
	}
}

// Torque / MagneticFlux -> Current
/// Dividing a Torque by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<MagneticFlux<T>> for Torque<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Current{A: self.Nm / rhs.Wb}
	}
}
/// Dividing a Torque by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<MagneticFlux<T>> for &Torque<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Current{A: self.Nm.clone() / rhs.Wb}
	}
}
/// Dividing a Torque by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<&MagneticFlux<T>> for Torque<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Current{A: self.Nm / rhs.Wb.clone()}
	}
}
/// Dividing a Torque by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<&MagneticFlux<T>> for &Torque<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Current{A: self.Nm.clone() / rhs.Wb.clone()}
	}
}

/// The frequency unit type, defined as hertz in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Frequency<T: NumLike>{
	/// The value of this Frequency in hertz
	pub Hz: T
}

impl<T> Frequency<T> where T: NumLike {

	/// Returns the standard unit name of frequency: "hertz"
	pub fn unit_name() -> &'static str {
		return "hertz";
	}
	
	/// Returns the abbreviated name or symbol of frequency: "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "Hz";
	}
	
	/// Returns a new frequency value from the given number of hertz
	///
	/// # Arguments
	/// * `Hz` - Any number-like type, representing a quantity of hertz
	pub fn from_Hz(Hz: T) -> Self {
		Frequency{Hz: Hz}
	}
	
	/// Returns a copy of this frequency value in hertz
	pub fn to_Hz(self) -> T {
		return self.Hz.clone();
	}

	/// Returns a new frequency value from the given number of hertz
	///
	/// # Arguments
	/// * `hertz` - Any number-like type, representing a quantity of hertz
	pub fn from_hertz(hertz: T) -> Self {
		Frequency{Hz: hertz}
	}
	
	/// Returns a copy of this frequency value in hertz
	pub fn to_hertz(self) -> T {
		return self.Hz.clone();
	}

}

impl<T> fmt::Display for Frequency<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Hz, Self::unit_symbol())
	}
}

impl<T> Frequency<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this frequency value in kilohertz
	pub fn to_kHz(self) -> T {
		return self.Hz.clone() * T::from(0.001_f64);
	}

	/// Returns a new frequency value from the given number of kilohertz
	///
	/// # Arguments
	/// * `kHz` - Any number-like type, representing a quantity of kilohertz
	pub fn from_kHz(kHz: T) -> Self {
		Frequency{Hz: kHz * T::from(1000.0_f64)}
	}

	/// Returns a copy of this frequency value in megahertz
	pub fn to_MHz(self) -> T {
		return self.Hz.clone() * T::from(1e-06_f64);
	}

	/// Returns a new frequency value from the given number of megahertz
	///
	/// # Arguments
	/// * `MHz` - Any number-like type, representing a quantity of megahertz
	pub fn from_MHz(MHz: T) -> Self {
		Frequency{Hz: MHz * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this frequency value in gigahertz
	pub fn to_GHz(self) -> T {
		return self.Hz.clone() * T::from(1e-09_f64);
	}

	/// Returns a new frequency value from the given number of gigahertz
	///
	/// # Arguments
	/// * `GHz` - Any number-like type, representing a quantity of gigahertz
	pub fn from_GHz(GHz: T) -> Self {
		Frequency{Hz: GHz * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this frequency value in terahertz
	pub fn to_THz(self) -> T {
		return self.Hz.clone() * T::from(1e-12_f64);
	}

	/// Returns a new frequency value from the given number of terahertz
	///
	/// # Arguments
	/// * `THz` - Any number-like type, representing a quantity of terahertz
	pub fn from_THz(THz: T) -> Self {
		Frequency{Hz: THz * T::from(1000000000000.0_f64)}
	}

}

// Frequency * Distance -> Velocity
/// Multiplying a Frequency by a Distance returns a value of type Velocity
impl<T> std::ops::Mul<Distance<T>> for Frequency<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Velocity{mps: self.Hz * rhs.m}
	}
}
/// Multiplying a Frequency by a Distance returns a value of type Velocity
impl<T> std::ops::Mul<Distance<T>> for &Frequency<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Velocity{mps: self.Hz.clone() * rhs.m}
	}
}
/// Multiplying a Frequency by a Distance returns a value of type Velocity
impl<T> std::ops::Mul<&Distance<T>> for Frequency<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Velocity{mps: self.Hz * rhs.m.clone()}
	}
}
/// Multiplying a Frequency by a Distance returns a value of type Velocity
impl<T> std::ops::Mul<&Distance<T>> for &Frequency<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Velocity{mps: self.Hz.clone() * rhs.m.clone()}
	}
}

// Frequency * Amount -> CatalyticActivity
/// Multiplying a Frequency by a Amount returns a value of type CatalyticActivity
impl<T> std::ops::Mul<Amount<T>> for Frequency<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		CatalyticActivity{molps: self.Hz * rhs.mol}
	}
}
/// Multiplying a Frequency by a Amount returns a value of type CatalyticActivity
impl<T> std::ops::Mul<Amount<T>> for &Frequency<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: Amount<T>) -> Self::Output {
		CatalyticActivity{molps: self.Hz.clone() * rhs.mol}
	}
}
/// Multiplying a Frequency by a Amount returns a value of type CatalyticActivity
impl<T> std::ops::Mul<&Amount<T>> for Frequency<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		CatalyticActivity{molps: self.Hz * rhs.mol.clone()}
	}
}
/// Multiplying a Frequency by a Amount returns a value of type CatalyticActivity
impl<T> std::ops::Mul<&Amount<T>> for &Frequency<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: &Amount<T>) -> Self::Output {
		CatalyticActivity{molps: self.Hz.clone() * rhs.mol.clone()}
	}
}

// Frequency * Angle -> AngularVelocity
/// Multiplying a Frequency by a Angle returns a value of type AngularVelocity
impl<T> std::ops::Mul<Angle<T>> for Frequency<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Angle<T>) -> Self::Output {
		AngularVelocity{radps: self.Hz * rhs.rad}
	}
}
/// Multiplying a Frequency by a Angle returns a value of type AngularVelocity
impl<T> std::ops::Mul<Angle<T>> for &Frequency<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Angle<T>) -> Self::Output {
		AngularVelocity{radps: self.Hz.clone() * rhs.rad}
	}
}
/// Multiplying a Frequency by a Angle returns a value of type AngularVelocity
impl<T> std::ops::Mul<&Angle<T>> for Frequency<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Angle<T>) -> Self::Output {
		AngularVelocity{radps: self.Hz * rhs.rad.clone()}
	}
}
/// Multiplying a Frequency by a Angle returns a value of type AngularVelocity
impl<T> std::ops::Mul<&Angle<T>> for &Frequency<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Angle<T>) -> Self::Output {
		AngularVelocity{radps: self.Hz.clone() * rhs.rad.clone()}
	}
}

// Frequency * AngularVelocity -> AngularAcceleration
/// Multiplying a Frequency by a AngularVelocity returns a value of type AngularAcceleration
impl<T> std::ops::Mul<AngularVelocity<T>> for Frequency<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		AngularAcceleration{radps2: self.Hz * rhs.radps}
	}
}
/// Multiplying a Frequency by a AngularVelocity returns a value of type AngularAcceleration
impl<T> std::ops::Mul<AngularVelocity<T>> for &Frequency<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		AngularAcceleration{radps2: self.Hz.clone() * rhs.radps}
	}
}
/// Multiplying a Frequency by a AngularVelocity returns a value of type AngularAcceleration
impl<T> std::ops::Mul<&AngularVelocity<T>> for Frequency<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AngularAcceleration{radps2: self.Hz * rhs.radps.clone()}
	}
}
/// Multiplying a Frequency by a AngularVelocity returns a value of type AngularAcceleration
impl<T> std::ops::Mul<&AngularVelocity<T>> for &Frequency<T> where T: NumLike {
	type Output = AngularAcceleration<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AngularAcceleration{radps2: self.Hz.clone() * rhs.radps.clone()}
	}
}

// Frequency * Torque -> Power
/// Multiplying a Frequency by a Torque returns a value of type Power
impl<T> std::ops::Mul<Torque<T>> for Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Power{W: self.Hz * rhs.Nm}
	}
}
/// Multiplying a Frequency by a Torque returns a value of type Power
impl<T> std::ops::Mul<Torque<T>> for &Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Torque<T>) -> Self::Output {
		Power{W: self.Hz.clone() * rhs.Nm}
	}
}
/// Multiplying a Frequency by a Torque returns a value of type Power
impl<T> std::ops::Mul<&Torque<T>> for Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Power{W: self.Hz * rhs.Nm.clone()}
	}
}
/// Multiplying a Frequency by a Torque returns a value of type Power
impl<T> std::ops::Mul<&Torque<T>> for &Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Torque<T>) -> Self::Output {
		Power{W: self.Hz.clone() * rhs.Nm.clone()}
	}
}

// Frequency * Energy -> Power
/// Multiplying a Frequency by a Energy returns a value of type Power
impl<T> std::ops::Mul<Energy<T>> for Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Power{W: self.Hz * rhs.J}
	}
}
/// Multiplying a Frequency by a Energy returns a value of type Power
impl<T> std::ops::Mul<Energy<T>> for &Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Energy<T>) -> Self::Output {
		Power{W: self.Hz.clone() * rhs.J}
	}
}
/// Multiplying a Frequency by a Energy returns a value of type Power
impl<T> std::ops::Mul<&Energy<T>> for Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Power{W: self.Hz * rhs.J.clone()}
	}
}
/// Multiplying a Frequency by a Energy returns a value of type Power
impl<T> std::ops::Mul<&Energy<T>> for &Frequency<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Energy<T>) -> Self::Output {
		Power{W: self.Hz.clone() * rhs.J.clone()}
	}
}

// Frequency * Velocity -> Acceleration
/// Multiplying a Frequency by a Velocity returns a value of type Acceleration
impl<T> std::ops::Mul<Velocity<T>> for Frequency<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Acceleration{mps2: self.Hz * rhs.mps}
	}
}
/// Multiplying a Frequency by a Velocity returns a value of type Acceleration
impl<T> std::ops::Mul<Velocity<T>> for &Frequency<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Acceleration{mps2: self.Hz.clone() * rhs.mps}
	}
}
/// Multiplying a Frequency by a Velocity returns a value of type Acceleration
impl<T> std::ops::Mul<&Velocity<T>> for Frequency<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Acceleration{mps2: self.Hz * rhs.mps.clone()}
	}
}
/// Multiplying a Frequency by a Velocity returns a value of type Acceleration
impl<T> std::ops::Mul<&Velocity<T>> for &Frequency<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Acceleration{mps2: self.Hz.clone() * rhs.mps.clone()}
	}
}

// Frequency * Momentum -> Force
/// Multiplying a Frequency by a Momentum returns a value of type Force
impl<T> std::ops::Mul<Momentum<T>> for Frequency<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Force{N: self.Hz * rhs.kgmps}
	}
}
/// Multiplying a Frequency by a Momentum returns a value of type Force
impl<T> std::ops::Mul<Momentum<T>> for &Frequency<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Force{N: self.Hz.clone() * rhs.kgmps}
	}
}
/// Multiplying a Frequency by a Momentum returns a value of type Force
impl<T> std::ops::Mul<&Momentum<T>> for Frequency<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Force{N: self.Hz * rhs.kgmps.clone()}
	}
}
/// Multiplying a Frequency by a Momentum returns a value of type Force
impl<T> std::ops::Mul<&Momentum<T>> for &Frequency<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Force{N: self.Hz.clone() * rhs.kgmps.clone()}
	}
}

// Frequency * Charge -> Current
/// Multiplying a Frequency by a Charge returns a value of type Current
impl<T> std::ops::Mul<Charge<T>> for Frequency<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Current{A: self.Hz * rhs.C}
	}
}
/// Multiplying a Frequency by a Charge returns a value of type Current
impl<T> std::ops::Mul<Charge<T>> for &Frequency<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Current{A: self.Hz.clone() * rhs.C}
	}
}
/// Multiplying a Frequency by a Charge returns a value of type Current
impl<T> std::ops::Mul<&Charge<T>> for Frequency<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Current{A: self.Hz * rhs.C.clone()}
	}
}
/// Multiplying a Frequency by a Charge returns a value of type Current
impl<T> std::ops::Mul<&Charge<T>> for &Frequency<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Current{A: self.Hz.clone() * rhs.C.clone()}
	}
}

// Frequency * Capacitance -> Conductance
/// Multiplying a Frequency by a Capacitance returns a value of type Conductance
impl<T> std::ops::Mul<Capacitance<T>> for Frequency<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Conductance{S: self.Hz * rhs.F}
	}
}
/// Multiplying a Frequency by a Capacitance returns a value of type Conductance
impl<T> std::ops::Mul<Capacitance<T>> for &Frequency<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Conductance{S: self.Hz.clone() * rhs.F}
	}
}
/// Multiplying a Frequency by a Capacitance returns a value of type Conductance
impl<T> std::ops::Mul<&Capacitance<T>> for Frequency<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Conductance{S: self.Hz * rhs.F.clone()}
	}
}
/// Multiplying a Frequency by a Capacitance returns a value of type Conductance
impl<T> std::ops::Mul<&Capacitance<T>> for &Frequency<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Conductance{S: self.Hz.clone() * rhs.F.clone()}
	}
}

// Frequency * Inductance -> Resistance
/// Multiplying a Frequency by a Inductance returns a value of type Resistance
impl<T> std::ops::Mul<Inductance<T>> for Frequency<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		Resistance{Ohm: self.Hz * rhs.H}
	}
}
/// Multiplying a Frequency by a Inductance returns a value of type Resistance
impl<T> std::ops::Mul<Inductance<T>> for &Frequency<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		Resistance{Ohm: self.Hz.clone() * rhs.H}
	}
}
/// Multiplying a Frequency by a Inductance returns a value of type Resistance
impl<T> std::ops::Mul<&Inductance<T>> for Frequency<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		Resistance{Ohm: self.Hz * rhs.H.clone()}
	}
}
/// Multiplying a Frequency by a Inductance returns a value of type Resistance
impl<T> std::ops::Mul<&Inductance<T>> for &Frequency<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		Resistance{Ohm: self.Hz.clone() * rhs.H.clone()}
	}
}

// Frequency * MagneticFlux -> Voltage
/// Multiplying a Frequency by a MagneticFlux returns a value of type Voltage
impl<T> std::ops::Mul<MagneticFlux<T>> for Frequency<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Voltage{V: self.Hz * rhs.Wb}
	}
}
/// Multiplying a Frequency by a MagneticFlux returns a value of type Voltage
impl<T> std::ops::Mul<MagneticFlux<T>> for &Frequency<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Voltage{V: self.Hz.clone() * rhs.Wb}
	}
}
/// Multiplying a Frequency by a MagneticFlux returns a value of type Voltage
impl<T> std::ops::Mul<&MagneticFlux<T>> for Frequency<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Voltage{V: self.Hz * rhs.Wb.clone()}
	}
}
/// Multiplying a Frequency by a MagneticFlux returns a value of type Voltage
impl<T> std::ops::Mul<&MagneticFlux<T>> for &Frequency<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Voltage{V: self.Hz.clone() * rhs.Wb.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<Frequency<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
impl<T> std::ops::Div<&Frequency<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_bigfloat")]
impl<T> std::ops::Div<Frequency<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_bigfloat")]
impl<T> std::ops::Div<Frequency<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_bigfloat")]
impl<T> std::ops::Div<&Frequency<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_bigfloat")]
impl<T> std::ops::Div<&Frequency<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="astro_float")]
impl<T> std::ops::Div<Frequency<T>> for astro_float::BigFloat where T: NumLike+From<astro_float::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="astro_float")]
impl<T> std::ops::Div<Frequency<T>> for &astro_float::BigFloat where T: NumLike+From<astro_float::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="astro_float")]
impl<T> std::ops::Div<&Frequency<T>> for astro_float::BigFloat where T: NumLike+From<astro_float::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="astro_float")]
impl<T> std::ops::Div<&Frequency<T>> for &astro_float::BigFloat where T: NumLike+From<astro_float::BigFloat> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<Frequency<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<Frequency<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<&Frequency<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<&Frequency<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

// 1/Frequency -> Time
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<Frequency<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<Frequency<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<&Frequency<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self) / rhs.Hz.clone()}
	}
}
/// Dividing a scalar value by a Frequency unit value returns a value of type Time
#[cfg(feature="num_complex")]
impl<T> std::ops::Div<&Frequency<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Time<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Time{s: T::from(self.clone()) / rhs.Hz.clone()}
	}
}

/// The area density unit type, defined as kilograms per square meter in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct AreaDensity<T: NumLike>{
	/// The value of this Area density in kilograms per square meter
	pub kgm2: T
}

impl<T> AreaDensity<T> where T: NumLike {

	/// Returns the standard unit name of area density: "kilograms per square meter"
	pub fn unit_name() -> &'static str {
		return "kilograms per square meter";
	}
	
	/// Returns the abbreviated name or symbol of area density: "kgm2" for kilograms per square meter
	pub fn unit_symbol() -> &'static str {
		return "kgm2";
	}
	
	/// Returns a new area density value from the given number of kilograms per square meter
	///
	/// # Arguments
	/// * `kgpm2` - Any number-like type, representing a quantity of kilograms per square meter
	pub fn from_kgpm2(kgpm2: T) -> Self {
		AreaDensity{kgm2: kgpm2}
	}
	
	/// Returns a copy of this area density value in kilograms per square meter
	pub fn to_kgpm2(self) -> T {
		return self.kgm2.clone();
	}

	/// Returns a new area density value from the given number of kilograms per square meter
	///
	/// # Arguments
	/// * `kilograms_per_square_meter` - Any number-like type, representing a quantity of kilograms per square meter
	pub fn from_kilograms_per_square_meter(kilograms_per_square_meter: T) -> Self {
		AreaDensity{kgm2: kilograms_per_square_meter}
	}
	
	/// Returns a copy of this area density value in kilograms per square meter
	pub fn to_kilograms_per_square_meter(self) -> T {
		return self.kgm2.clone();
	}

}

impl<T> fmt::Display for AreaDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgm2, Self::unit_symbol())
	}
}

impl<T> AreaDensity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this area density value in grams per square meter
	pub fn to_gpm2(self) -> T {
		return self.kgm2.clone() * T::from(1000.0_f64);
	}

	/// Returns a new area density value from the given number of grams per square meter
	///
	/// # Arguments
	/// * `gpm2` - Any number-like type, representing a quantity of grams per square meter
	pub fn from_gpm2(gpm2: T) -> Self {
		AreaDensity{kgm2: gpm2 * T::from(0.001_f64)}
	}

	/// Returns a copy of this area density value in grams per square meter
	pub fn to_grams_per_square_meter(self) -> T {
		return self.kgm2.clone() * T::from(1000.0_f64);
	}

	/// Returns a new area density value from the given number of grams per square meter
	///
	/// # Arguments
	/// * `grams_per_square_meter` - Any number-like type, representing a quantity of grams per square meter
	pub fn from_grams_per_square_meter(grams_per_square_meter: T) -> Self {
		AreaDensity{kgm2: grams_per_square_meter * T::from(0.001_f64)}
	}

	/// Returns a copy of this area density value in grams per square cm
	pub fn to_gpcm2(self) -> T {
		return self.kgm2.clone() * T::from(0.1_f64);
	}

	/// Returns a new area density value from the given number of grams per square cm
	///
	/// # Arguments
	/// * `gpcm2` - Any number-like type, representing a quantity of grams per square cm
	pub fn from_gpcm2(gpcm2: T) -> Self {
		AreaDensity{kgm2: gpcm2 * T::from(10.0_f64)}
	}

	/// Returns a copy of this area density value in grams per square cm
	pub fn to_grams_per_square_cm(self) -> T {
		return self.kgm2.clone() * T::from(0.1_f64);
	}

	/// Returns a new area density value from the given number of grams per square cm
	///
	/// # Arguments
	/// * `grams_per_square_cm` - Any number-like type, representing a quantity of grams per square cm
	pub fn from_grams_per_square_cm(grams_per_square_cm: T) -> Self {
		AreaDensity{kgm2: grams_per_square_cm * T::from(10.0_f64)}
	}

}

// AreaDensity / Mass -> Area
/// Dividing a AreaDensity by a Mass returns a value of type Area
impl<T> std::ops::Div<Mass<T>> for AreaDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Area{m2: self.kgm2 / rhs.kg}
	}
}
/// Dividing a AreaDensity by a Mass returns a value of type Area
impl<T> std::ops::Div<Mass<T>> for &AreaDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Area{m2: self.kgm2.clone() / rhs.kg}
	}
}
/// Dividing a AreaDensity by a Mass returns a value of type Area
impl<T> std::ops::Div<&Mass<T>> for AreaDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Area{m2: self.kgm2 / rhs.kg.clone()}
	}
}
/// Dividing a AreaDensity by a Mass returns a value of type Area
impl<T> std::ops::Div<&Mass<T>> for &AreaDensity<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Area{m2: self.kgm2.clone() / rhs.kg.clone()}
	}
}

// AreaDensity * AngularVelocity -> AngularMomentum
/// Multiplying a AreaDensity by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<AngularVelocity<T>> for AreaDensity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2 * rhs.radps}
	}
}
/// Multiplying a AreaDensity by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<AngularVelocity<T>> for &AreaDensity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2.clone() * rhs.radps}
	}
}
/// Multiplying a AreaDensity by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<&AngularVelocity<T>> for AreaDensity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2 * rhs.radps.clone()}
	}
}
/// Multiplying a AreaDensity by a AngularVelocity returns a value of type AngularMomentum
impl<T> std::ops::Mul<&AngularVelocity<T>> for &AreaDensity<T> where T: NumLike {
	type Output = AngularMomentum<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		AngularMomentum{kgm2radps: self.kgm2.clone() * rhs.radps.clone()}
	}
}

// AreaDensity / Area -> Mass
/// Dividing a AreaDensity by a Area returns a value of type Mass
impl<T> std::ops::Div<Area<T>> for AreaDensity<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Mass{kg: self.kgm2 / rhs.m2}
	}
}
/// Dividing a AreaDensity by a Area returns a value of type Mass
impl<T> std::ops::Div<Area<T>> for &AreaDensity<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Mass{kg: self.kgm2.clone() / rhs.m2}
	}
}
/// Dividing a AreaDensity by a Area returns a value of type Mass
impl<T> std::ops::Div<&Area<T>> for AreaDensity<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Mass{kg: self.kgm2 / rhs.m2.clone()}
	}
}
/// Dividing a AreaDensity by a Area returns a value of type Mass
impl<T> std::ops::Div<&Area<T>> for &AreaDensity<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Mass{kg: self.kgm2.clone() / rhs.m2.clone()}
	}
}

/// The density unit type, defined as kilograms per cubic meter in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Density<T: NumLike>{
	/// The value of this Density in kilograms per cubic meter
	pub kgpm3: T
}

impl<T> Density<T> where T: NumLike {

	/// Returns the standard unit name of density: "kilograms per cubic meter"
	pub fn unit_name() -> &'static str {
		return "kilograms per cubic meter";
	}
	
	/// Returns the abbreviated name or symbol of density: "kgpm3" for kilograms per cubic meter
	pub fn unit_symbol() -> &'static str {
		return "kgpm3";
	}
	
	/// Returns a new density value from the given number of kilograms per cubic meter
	///
	/// # Arguments
	/// * `kgpm3` - Any number-like type, representing a quantity of kilograms per cubic meter
	pub fn from_kgpm3(kgpm3: T) -> Self {
		Density{kgpm3: kgpm3}
	}
	
	/// Returns a copy of this density value in kilograms per cubic meter
	pub fn to_kgpm3(self) -> T {
		return self.kgpm3.clone();
	}

	/// Returns a new density value from the given number of kilograms per cubic meter
	///
	/// # Arguments
	/// * `kilograms_per_cubic_meter` - Any number-like type, representing a quantity of kilograms per cubic meter
	pub fn from_kilograms_per_cubic_meter(kilograms_per_cubic_meter: T) -> Self {
		Density{kgpm3: kilograms_per_cubic_meter}
	}
	
	/// Returns a copy of this density value in kilograms per cubic meter
	pub fn to_kilograms_per_cubic_meter(self) -> T {
		return self.kgpm3.clone();
	}

}

impl<T> fmt::Display for Density<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgpm3, Self::unit_symbol())
	}
}

impl<T> Density<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this density value in kilograms per liter
	pub fn to_kgpL(self) -> T {
		return self.kgpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new density value from the given number of kilograms per liter
	///
	/// # Arguments
	/// * `kgpL` - Any number-like type, representing a quantity of kilograms per liter
	pub fn from_kgpL(kgpL: T) -> Self {
		Density{kgpm3: kgpL * T::from(1000.0_f64)}
	}

	/// Returns a copy of this density value in kilograms per liter
	pub fn to_kilograms_per_liter(self) -> T {
		return self.kgpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new density value from the given number of kilograms per liter
	///
	/// # Arguments
	/// * `kilograms_per_liter` - Any number-like type, representing a quantity of kilograms per liter
	pub fn from_kilograms_per_liter(kilograms_per_liter: T) -> Self {
		Density{kgpm3: kilograms_per_liter * T::from(1000.0_f64)}
	}

	/// Returns a copy of this density value in grams per cc
	pub fn to_gpcc(self) -> T {
		return self.kgpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new density value from the given number of grams per cc
	///
	/// # Arguments
	/// * `gpcc` - Any number-like type, representing a quantity of grams per cc
	pub fn from_gpcc(gpcc: T) -> Self {
		Density{kgpm3: gpcc * T::from(1000.0_f64)}
	}

	/// Returns a copy of this density value in grams per cc
	pub fn to_grams_per_cubic_centimeter(self) -> T {
		return self.kgpm3.clone() * T::from(0.001_f64);
	}

	/// Returns a new density value from the given number of grams per cc
	///
	/// # Arguments
	/// * `grams_per_cubic_centimeter` - Any number-like type, representing a quantity of grams per cc
	pub fn from_grams_per_cubic_centimeter(grams_per_cubic_centimeter: T) -> Self {
		Density{kgpm3: grams_per_cubic_centimeter * T::from(1000.0_f64)}
	}

	/// Returns a copy of this density value in grams per cubic meter
	pub fn to_gpm3(self) -> T {
		return self.kgpm3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new density value from the given number of grams per cubic meter
	///
	/// # Arguments
	/// * `gpm3` - Any number-like type, representing a quantity of grams per cubic meter
	pub fn from_gpm3(gpm3: T) -> Self {
		Density{kgpm3: gpm3 * T::from(0.001_f64)}
	}

}

// Density * Volume -> Mass
/// Multiplying a Density by a Volume returns a value of type Mass
impl<T> std::ops::Mul<Volume<T>> for Density<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Mass{kg: self.kgpm3 * rhs.m3}
	}
}
/// Multiplying a Density by a Volume returns a value of type Mass
impl<T> std::ops::Mul<Volume<T>> for &Density<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Mass{kg: self.kgpm3.clone() * rhs.m3}
	}
}
/// Multiplying a Density by a Volume returns a value of type Mass
impl<T> std::ops::Mul<&Volume<T>> for Density<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Mass{kg: self.kgpm3 * rhs.m3.clone()}
	}
}
/// Multiplying a Density by a Volume returns a value of type Mass
impl<T> std::ops::Mul<&Volume<T>> for &Density<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Mass{kg: self.kgpm3.clone() * rhs.m3.clone()}
	}
}

/// The velocity unit type, defined as meters per second in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Velocity<T: NumLike>{
	/// The value of this Velocity in meters per second
	pub mps: T
}

impl<T> Velocity<T> where T: NumLike {

	/// Returns the standard unit name of velocity: "meters per second"
	pub fn unit_name() -> &'static str {
		return "meters per second";
	}
	
	/// Returns the abbreviated name or symbol of velocity: "mps" for meters per second
	pub fn unit_symbol() -> &'static str {
		return "mps";
	}
	
	/// Returns a new velocity value from the given number of meters per second
	///
	/// # Arguments
	/// * `mps` - Any number-like type, representing a quantity of meters per second
	pub fn from_mps(mps: T) -> Self {
		Velocity{mps: mps}
	}
	
	/// Returns a copy of this velocity value in meters per second
	pub fn to_mps(self) -> T {
		return self.mps.clone();
	}

	/// Returns a new velocity value from the given number of meters per second
	///
	/// # Arguments
	/// * `meters_per_second` - Any number-like type, representing a quantity of meters per second
	pub fn from_meters_per_second(meters_per_second: T) -> Self {
		Velocity{mps: meters_per_second}
	}
	
	/// Returns a copy of this velocity value in meters per second
	pub fn to_meters_per_second(self) -> T {
		return self.mps.clone();
	}

}

impl<T> fmt::Display for Velocity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.mps, Self::unit_symbol())
	}
}

impl<T> Velocity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this velocity value in centimeters per second
	pub fn to_cmps(self) -> T {
		return self.mps.clone() * T::from(100.0_f64);
	}

	/// Returns a new velocity value from the given number of centimeters per second
	///
	/// # Arguments
	/// * `cmps` - Any number-like type, representing a quantity of centimeters per second
	pub fn from_cmps(cmps: T) -> Self {
		Velocity{mps: cmps * T::from(0.01_f64)}
	}

	/// Returns a copy of this velocity value in millimeters per second
	pub fn to_mmps(self) -> T {
		return self.mps.clone() * T::from(1000.0_f64);
	}

	/// Returns a new velocity value from the given number of millimeters per second
	///
	/// # Arguments
	/// * `mmps` - Any number-like type, representing a quantity of millimeters per second
	pub fn from_mmps(mmps: T) -> Self {
		Velocity{mps: mmps * T::from(0.001_f64)}
	}

	/// Returns a copy of this velocity value in millimeters per hour
	pub fn to_mmph(self) -> T {
		return self.mps.clone() * T::from(3600000.0_f64);
	}

	/// Returns a new velocity value from the given number of millimeters per hour
	///
	/// # Arguments
	/// * `mmph` - Any number-like type, representing a quantity of millimeters per hour
	pub fn from_mmph(mmph: T) -> Self {
		Velocity{mps: mmph * T::from(2.77777777777778e-07_f64)}
	}

	/// Returns a copy of this velocity value in kilometers per hour
	pub fn to_kph(self) -> T {
		return self.mps.clone() * T::from(3.6_f64);
	}

	/// Returns a new velocity value from the given number of kilometers per hour
	///
	/// # Arguments
	/// * `kph` - Any number-like type, representing a quantity of kilometers per hour
	pub fn from_kph(kph: T) -> Self {
		Velocity{mps: kph * T::from(0.277777777777778_f64)}
	}

	/// Returns a copy of this velocity value in miles per hour
	pub fn to_mph(self) -> T {
		return self.mps.clone() * T::from(5.79363839999999_f64);
	}

	/// Returns a new velocity value from the given number of miles per hour
	///
	/// # Arguments
	/// * `mph` - Any number-like type, representing a quantity of miles per hour
	pub fn from_mph(mph: T) -> Self {
		Velocity{mps: mph * T::from(0.172603108954815_f64)}
	}

	/// Returns a copy of this velocity value in kilometers per second
	pub fn to_kmps(self) -> T {
		return self.mps.clone() * T::from(0.001_f64);
	}

	/// Returns a new velocity value from the given number of kilometers per second
	///
	/// # Arguments
	/// * `kmps` - Any number-like type, representing a quantity of kilometers per second
	pub fn from_kmps(kmps: T) -> Self {
		Velocity{mps: kmps * T::from(1000.0_f64)}
	}

	/// Returns a copy of this velocity value in light speed
	pub fn to_c(self) -> T {
		return self.mps.clone() * T::from(3.3356409519815204e-09_f64);
	}

	/// Returns a new velocity value from the given number of light speed
	///
	/// # Arguments
	/// * `c` - Any number-like type, representing a quantity of light speed
	pub fn from_c(c: T) -> Self {
		Velocity{mps: c * T::from(299792458.0_f64)}
	}

}

// Velocity / Distance -> Frequency
/// Dividing a Velocity by a Distance returns a value of type Frequency
impl<T> std::ops::Div<Distance<T>> for Velocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Frequency{Hz: self.mps / rhs.m}
	}
}
/// Dividing a Velocity by a Distance returns a value of type Frequency
impl<T> std::ops::Div<Distance<T>> for &Velocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Frequency{Hz: self.mps.clone() / rhs.m}
	}
}
/// Dividing a Velocity by a Distance returns a value of type Frequency
impl<T> std::ops::Div<&Distance<T>> for Velocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Frequency{Hz: self.mps / rhs.m.clone()}
	}
}
/// Dividing a Velocity by a Distance returns a value of type Frequency
impl<T> std::ops::Div<&Distance<T>> for &Velocity<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Frequency{Hz: self.mps.clone() / rhs.m.clone()}
	}
}

// Velocity * Mass -> Momentum
/// Multiplying a Velocity by a Mass returns a value of type Momentum
impl<T> std::ops::Mul<Mass<T>> for Velocity<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Momentum{kgmps: self.mps * rhs.kg}
	}
}
/// Multiplying a Velocity by a Mass returns a value of type Momentum
impl<T> std::ops::Mul<Mass<T>> for &Velocity<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Momentum{kgmps: self.mps.clone() * rhs.kg}
	}
}
/// Multiplying a Velocity by a Mass returns a value of type Momentum
impl<T> std::ops::Mul<&Mass<T>> for Velocity<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Momentum{kgmps: self.mps * rhs.kg.clone()}
	}
}
/// Multiplying a Velocity by a Mass returns a value of type Momentum
impl<T> std::ops::Mul<&Mass<T>> for &Velocity<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Momentum{kgmps: self.mps.clone() * rhs.kg.clone()}
	}
}

// Velocity * Time -> Distance
/// Multiplying a Velocity by a Time returns a value of type Distance
impl<T> std::ops::Mul<Time<T>> for Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Distance{m: self.mps * rhs.s}
	}
}
/// Multiplying a Velocity by a Time returns a value of type Distance
impl<T> std::ops::Mul<Time<T>> for &Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Distance{m: self.mps.clone() * rhs.s}
	}
}
/// Multiplying a Velocity by a Time returns a value of type Distance
impl<T> std::ops::Mul<&Time<T>> for Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Distance{m: self.mps * rhs.s.clone()}
	}
}
/// Multiplying a Velocity by a Time returns a value of type Distance
impl<T> std::ops::Mul<&Time<T>> for &Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Distance{m: self.mps.clone() * rhs.s.clone()}
	}
}

// Velocity / Time -> Acceleration
/// Dividing a Velocity by a Time returns a value of type Acceleration
impl<T> std::ops::Div<Time<T>> for Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Acceleration{mps2: self.mps / rhs.s}
	}
}
/// Dividing a Velocity by a Time returns a value of type Acceleration
impl<T> std::ops::Div<Time<T>> for &Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Acceleration{mps2: self.mps.clone() / rhs.s}
	}
}
/// Dividing a Velocity by a Time returns a value of type Acceleration
impl<T> std::ops::Div<&Time<T>> for Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Acceleration{mps2: self.mps / rhs.s.clone()}
	}
}
/// Dividing a Velocity by a Time returns a value of type Acceleration
impl<T> std::ops::Div<&Time<T>> for &Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Acceleration{mps2: self.mps.clone() / rhs.s.clone()}
	}
}

// Velocity * Frequency -> Acceleration
/// Multiplying a Velocity by a Frequency returns a value of type Acceleration
impl<T> std::ops::Mul<Frequency<T>> for Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Acceleration{mps2: self.mps * rhs.Hz}
	}
}
/// Multiplying a Velocity by a Frequency returns a value of type Acceleration
impl<T> std::ops::Mul<Frequency<T>> for &Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Acceleration{mps2: self.mps.clone() * rhs.Hz}
	}
}
/// Multiplying a Velocity by a Frequency returns a value of type Acceleration
impl<T> std::ops::Mul<&Frequency<T>> for Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Acceleration{mps2: self.mps * rhs.Hz.clone()}
	}
}
/// Multiplying a Velocity by a Frequency returns a value of type Acceleration
impl<T> std::ops::Mul<&Frequency<T>> for &Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Acceleration{mps2: self.mps.clone() * rhs.Hz.clone()}
	}
}

// Velocity / Frequency -> Distance
/// Dividing a Velocity by a Frequency returns a value of type Distance
impl<T> std::ops::Div<Frequency<T>> for Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Distance{m: self.mps / rhs.Hz}
	}
}
/// Dividing a Velocity by a Frequency returns a value of type Distance
impl<T> std::ops::Div<Frequency<T>> for &Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Distance{m: self.mps.clone() / rhs.Hz}
	}
}
/// Dividing a Velocity by a Frequency returns a value of type Distance
impl<T> std::ops::Div<&Frequency<T>> for Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Distance{m: self.mps / rhs.Hz.clone()}
	}
}
/// Dividing a Velocity by a Frequency returns a value of type Distance
impl<T> std::ops::Div<&Frequency<T>> for &Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Distance{m: self.mps.clone() / rhs.Hz.clone()}
	}
}

// Velocity / Acceleration -> Time
/// Dividing a Velocity by a Acceleration returns a value of type Time
impl<T> std::ops::Div<Acceleration<T>> for Velocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		Time{s: self.mps / rhs.mps2}
	}
}
/// Dividing a Velocity by a Acceleration returns a value of type Time
impl<T> std::ops::Div<Acceleration<T>> for &Velocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		Time{s: self.mps.clone() / rhs.mps2}
	}
}
/// Dividing a Velocity by a Acceleration returns a value of type Time
impl<T> std::ops::Div<&Acceleration<T>> for Velocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		Time{s: self.mps / rhs.mps2.clone()}
	}
}
/// Dividing a Velocity by a Acceleration returns a value of type Time
impl<T> std::ops::Div<&Acceleration<T>> for &Velocity<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		Time{s: self.mps.clone() / rhs.mps2.clone()}
	}
}

// Velocity * Momentum -> Energy
/// Multiplying a Velocity by a Momentum returns a value of type Energy
impl<T> std::ops::Mul<Momentum<T>> for Velocity<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Energy{J: self.mps * rhs.kgmps}
	}
}
/// Multiplying a Velocity by a Momentum returns a value of type Energy
impl<T> std::ops::Mul<Momentum<T>> for &Velocity<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Energy{J: self.mps.clone() * rhs.kgmps}
	}
}
/// Multiplying a Velocity by a Momentum returns a value of type Energy
impl<T> std::ops::Mul<&Momentum<T>> for Velocity<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Energy{J: self.mps * rhs.kgmps.clone()}
	}
}
/// Multiplying a Velocity by a Momentum returns a value of type Energy
impl<T> std::ops::Mul<&Momentum<T>> for &Velocity<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Energy{J: self.mps.clone() * rhs.kgmps.clone()}
	}
}

// Velocity * Force -> Power
/// Multiplying a Velocity by a Force returns a value of type Power
impl<T> std::ops::Mul<Force<T>> for Velocity<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Power{W: self.mps * rhs.N}
	}
}
/// Multiplying a Velocity by a Force returns a value of type Power
impl<T> std::ops::Mul<Force<T>> for &Velocity<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Power{W: self.mps.clone() * rhs.N}
	}
}
/// Multiplying a Velocity by a Force returns a value of type Power
impl<T> std::ops::Mul<&Force<T>> for Velocity<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Power{W: self.mps * rhs.N.clone()}
	}
}
/// Multiplying a Velocity by a Force returns a value of type Power
impl<T> std::ops::Mul<&Force<T>> for &Velocity<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Power{W: self.mps.clone() * rhs.N.clone()}
	}
}

/// The acceleration unit type, defined as meters per second squared in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Acceleration<T: NumLike>{
	/// The value of this Acceleration in meters per second squared
	pub mps2: T
}

impl<T> Acceleration<T> where T: NumLike {

	/// Returns the standard unit name of acceleration: "meters per second squared"
	pub fn unit_name() -> &'static str {
		return "meters per second squared";
	}
	
	/// Returns the abbreviated name or symbol of acceleration: "mps2" for meters per second squared
	pub fn unit_symbol() -> &'static str {
		return "mps2";
	}
	
	/// Returns a new acceleration value from the given number of meters per second squared
	///
	/// # Arguments
	/// * `mps2` - Any number-like type, representing a quantity of meters per second squared
	pub fn from_mps2(mps2: T) -> Self {
		Acceleration{mps2: mps2}
	}
	
	/// Returns a copy of this acceleration value in meters per second squared
	pub fn to_mps2(self) -> T {
		return self.mps2.clone();
	}

	/// Returns a new acceleration value from the given number of meters per second squared
	///
	/// # Arguments
	/// * `meters_per_second_squared` - Any number-like type, representing a quantity of meters per second squared
	pub fn from_meters_per_second_squared(meters_per_second_squared: T) -> Self {
		Acceleration{mps2: meters_per_second_squared}
	}
	
	/// Returns a copy of this acceleration value in meters per second squared
	pub fn to_meters_per_second_squared(self) -> T {
		return self.mps2.clone();
	}

}

impl<T> fmt::Display for Acceleration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.mps2, Self::unit_symbol())
	}
}

impl<T> Acceleration<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this acceleration value in millimeters per second squared
	pub fn to_mmps2(self) -> T {
		return self.mps2.clone() * T::from(1000.0_f64);
	}

	/// Returns a new acceleration value from the given number of millimeters per second squared
	///
	/// # Arguments
	/// * `mmps2` - Any number-like type, representing a quantity of millimeters per second squared
	pub fn from_mmps2(mmps2: T) -> Self {
		Acceleration{mps2: mmps2 * T::from(0.001_f64)}
	}

	/// Returns a copy of this acceleration value in kilometers per hour squared
	pub fn to_kilometers_per_hour_squared(self) -> T {
		return self.mps2.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new acceleration value from the given number of kilometers per hour squared
	///
	/// # Arguments
	/// * `kilometers_per_hour_squared` - Any number-like type, representing a quantity of kilometers per hour squared
	pub fn from_kilometers_per_hour_squared(kilometers_per_hour_squared: T) -> Self {
		Acceleration{mps2: kilometers_per_hour_squared * T::from(1e-06_f64)}
	}

	/// Returns a copy of this acceleration value in kilometers per hour squared
	pub fn to_kph2(self) -> T {
		return self.mps2.clone() * T::from(12960.0_f64);
	}

	/// Returns a new acceleration value from the given number of kilometers per hour squared
	///
	/// # Arguments
	/// * `kph2` - Any number-like type, representing a quantity of kilometers per hour squared
	pub fn from_kph2(kph2: T) -> Self {
		Acceleration{mps2: kph2 * T::from(7.71604938271605e-05_f64)}
	}

}

// Acceleration * Mass -> Force
/// Multiplying a Acceleration by a Mass returns a value of type Force
impl<T> std::ops::Mul<Mass<T>> for Acceleration<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Force{N: self.mps2 * rhs.kg}
	}
}
/// Multiplying a Acceleration by a Mass returns a value of type Force
impl<T> std::ops::Mul<Mass<T>> for &Acceleration<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		Force{N: self.mps2.clone() * rhs.kg}
	}
}
/// Multiplying a Acceleration by a Mass returns a value of type Force
impl<T> std::ops::Mul<&Mass<T>> for Acceleration<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Force{N: self.mps2 * rhs.kg.clone()}
	}
}
/// Multiplying a Acceleration by a Mass returns a value of type Force
impl<T> std::ops::Mul<&Mass<T>> for &Acceleration<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		Force{N: self.mps2.clone() * rhs.kg.clone()}
	}
}

// Acceleration * Time -> Velocity
/// Multiplying a Acceleration by a Time returns a value of type Velocity
impl<T> std::ops::Mul<Time<T>> for Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.mps2 * rhs.s}
	}
}
/// Multiplying a Acceleration by a Time returns a value of type Velocity
impl<T> std::ops::Mul<Time<T>> for &Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.mps2.clone() * rhs.s}
	}
}
/// Multiplying a Acceleration by a Time returns a value of type Velocity
impl<T> std::ops::Mul<&Time<T>> for Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Velocity{mps: self.mps2 * rhs.s.clone()}
	}
}
/// Multiplying a Acceleration by a Time returns a value of type Velocity
impl<T> std::ops::Mul<&Time<T>> for &Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Velocity{mps: self.mps2.clone() * rhs.s.clone()}
	}
}

// Acceleration / Frequency -> Velocity
/// Dividing a Acceleration by a Frequency returns a value of type Velocity
impl<T> std::ops::Div<Frequency<T>> for Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Velocity{mps: self.mps2 / rhs.Hz}
	}
}
/// Dividing a Acceleration by a Frequency returns a value of type Velocity
impl<T> std::ops::Div<Frequency<T>> for &Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Velocity{mps: self.mps2.clone() / rhs.Hz}
	}
}
/// Dividing a Acceleration by a Frequency returns a value of type Velocity
impl<T> std::ops::Div<&Frequency<T>> for Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Velocity{mps: self.mps2 / rhs.Hz.clone()}
	}
}
/// Dividing a Acceleration by a Frequency returns a value of type Velocity
impl<T> std::ops::Div<&Frequency<T>> for &Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Velocity{mps: self.mps2.clone() / rhs.Hz.clone()}
	}
}

// Acceleration / Velocity -> Frequency
/// Dividing a Acceleration by a Velocity returns a value of type Frequency
impl<T> std::ops::Div<Velocity<T>> for Acceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Frequency{Hz: self.mps2 / rhs.mps}
	}
}
/// Dividing a Acceleration by a Velocity returns a value of type Frequency
impl<T> std::ops::Div<Velocity<T>> for &Acceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Frequency{Hz: self.mps2.clone() / rhs.mps}
	}
}
/// Dividing a Acceleration by a Velocity returns a value of type Frequency
impl<T> std::ops::Div<&Velocity<T>> for Acceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Frequency{Hz: self.mps2 / rhs.mps.clone()}
	}
}
/// Dividing a Acceleration by a Velocity returns a value of type Frequency
impl<T> std::ops::Div<&Velocity<T>> for &Acceleration<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Frequency{Hz: self.mps2.clone() / rhs.mps.clone()}
	}
}

// Acceleration * Momentum -> Power
/// Multiplying a Acceleration by a Momentum returns a value of type Power
impl<T> std::ops::Mul<Momentum<T>> for Acceleration<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Power{W: self.mps2 * rhs.kgmps}
	}
}
/// Multiplying a Acceleration by a Momentum returns a value of type Power
impl<T> std::ops::Mul<Momentum<T>> for &Acceleration<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Momentum<T>) -> Self::Output {
		Power{W: self.mps2.clone() * rhs.kgmps}
	}
}
/// Multiplying a Acceleration by a Momentum returns a value of type Power
impl<T> std::ops::Mul<&Momentum<T>> for Acceleration<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Power{W: self.mps2 * rhs.kgmps.clone()}
	}
}
/// Multiplying a Acceleration by a Momentum returns a value of type Power
impl<T> std::ops::Mul<&Momentum<T>> for &Acceleration<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Momentum<T>) -> Self::Output {
		Power{W: self.mps2.clone() * rhs.kgmps.clone()}
	}
}

/// The momentum unit type, defined as kilogram meters per second in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Momentum<T: NumLike>{
	/// The value of this Momentum in kilogram meters per second
	pub kgmps: T
}

impl<T> Momentum<T> where T: NumLike {

	/// Returns the standard unit name of momentum: "kilogram meters per second"
	pub fn unit_name() -> &'static str {
		return "kilogram meters per second";
	}
	
	/// Returns the abbreviated name or symbol of momentum: "kgmps" for kilogram meters per second
	pub fn unit_symbol() -> &'static str {
		return "kgmps";
	}
	
	/// Returns a new momentum value from the given number of kilogram meters per second
	///
	/// # Arguments
	/// * `kgmps` - Any number-like type, representing a quantity of kilogram meters per second
	pub fn from_kgmps(kgmps: T) -> Self {
		Momentum{kgmps: kgmps}
	}
	
	/// Returns a copy of this momentum value in kilogram meters per second
	pub fn to_kgmps(self) -> T {
		return self.kgmps.clone();
	}

	/// Returns a new momentum value from the given number of kilogram meters per second
	///
	/// # Arguments
	/// * `kilogram_meters_per_second` - Any number-like type, representing a quantity of kilogram meters per second
	pub fn from_kilogram_meters_per_second(kilogram_meters_per_second: T) -> Self {
		Momentum{kgmps: kilogram_meters_per_second}
	}
	
	/// Returns a copy of this momentum value in kilogram meters per second
	pub fn to_kilogram_meters_per_second(self) -> T {
		return self.kgmps.clone();
	}

}

impl<T> fmt::Display for Momentum<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgmps, Self::unit_symbol())
	}
}

impl<T> Momentum<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this momentum value in gram centimeters per second
	pub fn to_gram_centimeters_per_second(self) -> T {
		return self.kgmps.clone() * T::from(100000.0_f64);
	}

	/// Returns a new momentum value from the given number of gram centimeters per second
	///
	/// # Arguments
	/// * `gram_centimeters_per_second` - Any number-like type, representing a quantity of gram centimeters per second
	pub fn from_gram_centimeters_per_second(gram_centimeters_per_second: T) -> Self {
		Momentum{kgmps: gram_centimeters_per_second * T::from(1e-05_f64)}
	}

	/// Returns a copy of this momentum value in gram centimeters per second
	pub fn to_gcmps(self) -> T {
		return self.kgmps.clone() * T::from(100000.0_f64);
	}

	/// Returns a new momentum value from the given number of gram centimeters per second
	///
	/// # Arguments
	/// * `gcmps` - Any number-like type, representing a quantity of gram centimeters per second
	pub fn from_gcmps(gcmps: T) -> Self {
		Momentum{kgmps: gcmps * T::from(1e-05_f64)}
	}

}

// Momentum / Mass -> Velocity
/// Dividing a Momentum by a Mass returns a value of type Velocity
impl<T> std::ops::Div<Mass<T>> for Momentum<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Velocity{mps: self.kgmps / rhs.kg}
	}
}
/// Dividing a Momentum by a Mass returns a value of type Velocity
impl<T> std::ops::Div<Mass<T>> for &Momentum<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Velocity{mps: self.kgmps.clone() / rhs.kg}
	}
}
/// Dividing a Momentum by a Mass returns a value of type Velocity
impl<T> std::ops::Div<&Mass<T>> for Momentum<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Velocity{mps: self.kgmps / rhs.kg.clone()}
	}
}
/// Dividing a Momentum by a Mass returns a value of type Velocity
impl<T> std::ops::Div<&Mass<T>> for &Momentum<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Velocity{mps: self.kgmps.clone() / rhs.kg.clone()}
	}
}

// Momentum / Time -> Force
/// Dividing a Momentum by a Time returns a value of type Force
impl<T> std::ops::Div<Time<T>> for Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Force{N: self.kgmps / rhs.s}
	}
}
/// Dividing a Momentum by a Time returns a value of type Force
impl<T> std::ops::Div<Time<T>> for &Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Force{N: self.kgmps.clone() / rhs.s}
	}
}
/// Dividing a Momentum by a Time returns a value of type Force
impl<T> std::ops::Div<&Time<T>> for Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Force{N: self.kgmps / rhs.s.clone()}
	}
}
/// Dividing a Momentum by a Time returns a value of type Force
impl<T> std::ops::Div<&Time<T>> for &Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Force{N: self.kgmps.clone() / rhs.s.clone()}
	}
}

// Momentum * Frequency -> Force
/// Multiplying a Momentum by a Frequency returns a value of type Force
impl<T> std::ops::Mul<Frequency<T>> for Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Force{N: self.kgmps * rhs.Hz}
	}
}
/// Multiplying a Momentum by a Frequency returns a value of type Force
impl<T> std::ops::Mul<Frequency<T>> for &Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Force{N: self.kgmps.clone() * rhs.Hz}
	}
}
/// Multiplying a Momentum by a Frequency returns a value of type Force
impl<T> std::ops::Mul<&Frequency<T>> for Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Force{N: self.kgmps * rhs.Hz.clone()}
	}
}
/// Multiplying a Momentum by a Frequency returns a value of type Force
impl<T> std::ops::Mul<&Frequency<T>> for &Momentum<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Force{N: self.kgmps.clone() * rhs.Hz.clone()}
	}
}

// Momentum * Velocity -> Energy
/// Multiplying a Momentum by a Velocity returns a value of type Energy
impl<T> std::ops::Mul<Velocity<T>> for Momentum<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Energy{J: self.kgmps * rhs.mps}
	}
}
/// Multiplying a Momentum by a Velocity returns a value of type Energy
impl<T> std::ops::Mul<Velocity<T>> for &Momentum<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Energy{J: self.kgmps.clone() * rhs.mps}
	}
}
/// Multiplying a Momentum by a Velocity returns a value of type Energy
impl<T> std::ops::Mul<&Velocity<T>> for Momentum<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Energy{J: self.kgmps * rhs.mps.clone()}
	}
}
/// Multiplying a Momentum by a Velocity returns a value of type Energy
impl<T> std::ops::Mul<&Velocity<T>> for &Momentum<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Energy{J: self.kgmps.clone() * rhs.mps.clone()}
	}
}

// Momentum / Velocity -> Mass
/// Dividing a Momentum by a Velocity returns a value of type Mass
impl<T> std::ops::Div<Velocity<T>> for Momentum<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Mass{kg: self.kgmps / rhs.mps}
	}
}
/// Dividing a Momentum by a Velocity returns a value of type Mass
impl<T> std::ops::Div<Velocity<T>> for &Momentum<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Mass{kg: self.kgmps.clone() / rhs.mps}
	}
}
/// Dividing a Momentum by a Velocity returns a value of type Mass
impl<T> std::ops::Div<&Velocity<T>> for Momentum<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Mass{kg: self.kgmps / rhs.mps.clone()}
	}
}
/// Dividing a Momentum by a Velocity returns a value of type Mass
impl<T> std::ops::Div<&Velocity<T>> for &Momentum<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Mass{kg: self.kgmps.clone() / rhs.mps.clone()}
	}
}

// Momentum * Acceleration -> Power
/// Multiplying a Momentum by a Acceleration returns a value of type Power
impl<T> std::ops::Mul<Acceleration<T>> for Momentum<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Power{W: self.kgmps * rhs.mps2}
	}
}
/// Multiplying a Momentum by a Acceleration returns a value of type Power
impl<T> std::ops::Mul<Acceleration<T>> for &Momentum<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Power{W: self.kgmps.clone() * rhs.mps2}
	}
}
/// Multiplying a Momentum by a Acceleration returns a value of type Power
impl<T> std::ops::Mul<&Acceleration<T>> for Momentum<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Power{W: self.kgmps * rhs.mps2.clone()}
	}
}
/// Multiplying a Momentum by a Acceleration returns a value of type Power
impl<T> std::ops::Mul<&Acceleration<T>> for &Momentum<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Power{W: self.kgmps.clone() * rhs.mps2.clone()}
	}
}

// Momentum / Force -> Time
/// Dividing a Momentum by a Force returns a value of type Time
impl<T> std::ops::Div<Force<T>> for Momentum<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Time{s: self.kgmps / rhs.N}
	}
}
/// Dividing a Momentum by a Force returns a value of type Time
impl<T> std::ops::Div<Force<T>> for &Momentum<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Time{s: self.kgmps.clone() / rhs.N}
	}
}
/// Dividing a Momentum by a Force returns a value of type Time
impl<T> std::ops::Div<&Force<T>> for Momentum<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Time{s: self.kgmps / rhs.N.clone()}
	}
}
/// Dividing a Momentum by a Force returns a value of type Time
impl<T> std::ops::Div<&Force<T>> for &Momentum<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Time{s: self.kgmps.clone() / rhs.N.clone()}
	}
}

/// The force unit type, defined as newtons in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Force<T: NumLike>{
	/// The value of this Force in newtons
	pub N: T
}

impl<T> Force<T> where T: NumLike {

	/// Returns the standard unit name of force: "newtons"
	pub fn unit_name() -> &'static str {
		return "newtons";
	}
	
	/// Returns the abbreviated name or symbol of force: "N" for newtons
	pub fn unit_symbol() -> &'static str {
		return "N";
	}
	
	/// Returns a new force value from the given number of newtons
	///
	/// # Arguments
	/// * `N` - Any number-like type, representing a quantity of newtons
	pub fn from_N(N: T) -> Self {
		Force{N: N}
	}
	
	/// Returns a copy of this force value in newtons
	pub fn to_N(self) -> T {
		return self.N.clone();
	}

	/// Returns a new force value from the given number of newtons
	///
	/// # Arguments
	/// * `newtons` - Any number-like type, representing a quantity of newtons
	pub fn from_newtons(newtons: T) -> Self {
		Force{N: newtons}
	}
	
	/// Returns a copy of this force value in newtons
	pub fn to_newtons(self) -> T {
		return self.N.clone();
	}

}

impl<T> fmt::Display for Force<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.N, Self::unit_symbol())
	}
}

impl<T> Force<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this force value in pounds
	pub fn to_lb(self) -> T {
		return self.N.clone() * T::from(0.224337566199999_f64);
	}

	/// Returns a new force value from the given number of pounds
	///
	/// # Arguments
	/// * `lb` - Any number-like type, representing a quantity of pounds
	pub fn from_lb(lb: T) -> Self {
		Force{N: lb * T::from(4.45756819483586_f64)}
	}

	/// Returns a copy of this force value in kilogram-force
	pub fn to_kgG(self) -> T {
		return self.N.clone() * T::from(0.101971620999999_f64);
	}

	/// Returns a new force value from the given number of kilogram-force
	///
	/// # Arguments
	/// * `kgG` - Any number-like type, representing a quantity of kilogram-force
	pub fn from_kgG(kgG: T) -> Self {
		Force{N: kgG * T::from(9.8066500286389_f64)}
	}

	/// Returns a copy of this force value in millinewtons
	pub fn to_mN(self) -> T {
		return self.N.clone() * T::from(1000.0_f64);
	}

	/// Returns a new force value from the given number of millinewtons
	///
	/// # Arguments
	/// * `mN` - Any number-like type, representing a quantity of millinewtons
	pub fn from_mN(mN: T) -> Self {
		Force{N: mN * T::from(0.001_f64)}
	}

	/// Returns a copy of this force value in micronewtons
	pub fn to_uN(self) -> T {
		return self.N.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new force value from the given number of micronewtons
	///
	/// # Arguments
	/// * `uN` - Any number-like type, representing a quantity of micronewtons
	pub fn from_uN(uN: T) -> Self {
		Force{N: uN * T::from(1e-06_f64)}
	}

	/// Returns a copy of this force value in nanonewtons
	pub fn to_nN(self) -> T {
		return self.N.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new force value from the given number of nanonewtons
	///
	/// # Arguments
	/// * `nN` - Any number-like type, representing a quantity of nanonewtons
	pub fn from_nN(nN: T) -> Self {
		Force{N: nN * T::from(1e-09_f64)}
	}

	/// Returns a copy of this force value in kilonewtons
	pub fn to_kN(self) -> T {
		return self.N.clone() * T::from(0.001_f64);
	}

	/// Returns a new force value from the given number of kilonewtons
	///
	/// # Arguments
	/// * `kN` - Any number-like type, representing a quantity of kilonewtons
	pub fn from_kN(kN: T) -> Self {
		Force{N: kN * T::from(1000.0_f64)}
	}

	/// Returns a copy of this force value in meganewtons
	pub fn to_MN(self) -> T {
		return self.N.clone() * T::from(1e-06_f64);
	}

	/// Returns a new force value from the given number of meganewtons
	///
	/// # Arguments
	/// * `MN` - Any number-like type, representing a quantity of meganewtons
	pub fn from_MN(MN: T) -> Self {
		Force{N: MN * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this force value in giganewtons
	pub fn to_GN(self) -> T {
		return self.N.clone() * T::from(1e-09_f64);
	}

	/// Returns a new force value from the given number of giganewtons
	///
	/// # Arguments
	/// * `GN` - Any number-like type, representing a quantity of giganewtons
	pub fn from_GN(GN: T) -> Self {
		Force{N: GN * T::from(1000000000.0_f64)}
	}

}

// Force * Distance -> Energy
/// Multiplying a Force by a Distance returns a value of type Energy
impl<T> std::ops::Mul<Distance<T>> for Force<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Energy{J: self.N * rhs.m}
	}
}
/// Multiplying a Force by a Distance returns a value of type Energy
impl<T> std::ops::Mul<Distance<T>> for &Force<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Energy{J: self.N.clone() * rhs.m}
	}
}
/// Multiplying a Force by a Distance returns a value of type Energy
impl<T> std::ops::Mul<&Distance<T>> for Force<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Energy{J: self.N * rhs.m.clone()}
	}
}
/// Multiplying a Force by a Distance returns a value of type Energy
impl<T> std::ops::Mul<&Distance<T>> for &Force<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Energy{J: self.N.clone() * rhs.m.clone()}
	}
}

// Force / Mass -> Acceleration
/// Dividing a Force by a Mass returns a value of type Acceleration
impl<T> std::ops::Div<Mass<T>> for Force<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Acceleration{mps2: self.N / rhs.kg}
	}
}
/// Dividing a Force by a Mass returns a value of type Acceleration
impl<T> std::ops::Div<Mass<T>> for &Force<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: Mass<T>) -> Self::Output {
		Acceleration{mps2: self.N.clone() / rhs.kg}
	}
}
/// Dividing a Force by a Mass returns a value of type Acceleration
impl<T> std::ops::Div<&Mass<T>> for Force<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Acceleration{mps2: self.N / rhs.kg.clone()}
	}
}
/// Dividing a Force by a Mass returns a value of type Acceleration
impl<T> std::ops::Div<&Mass<T>> for &Force<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &Mass<T>) -> Self::Output {
		Acceleration{mps2: self.N.clone() / rhs.kg.clone()}
	}
}

// Force * Time -> Momentum
/// Multiplying a Force by a Time returns a value of type Momentum
impl<T> std::ops::Mul<Time<T>> for Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Momentum{kgmps: self.N * rhs.s}
	}
}
/// Multiplying a Force by a Time returns a value of type Momentum
impl<T> std::ops::Mul<Time<T>> for &Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Momentum{kgmps: self.N.clone() * rhs.s}
	}
}
/// Multiplying a Force by a Time returns a value of type Momentum
impl<T> std::ops::Mul<&Time<T>> for Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Momentum{kgmps: self.N * rhs.s.clone()}
	}
}
/// Multiplying a Force by a Time returns a value of type Momentum
impl<T> std::ops::Mul<&Time<T>> for &Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Momentum{kgmps: self.N.clone() * rhs.s.clone()}
	}
}

// Force / Frequency -> Momentum
/// Dividing a Force by a Frequency returns a value of type Momentum
impl<T> std::ops::Div<Frequency<T>> for Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Momentum{kgmps: self.N / rhs.Hz}
	}
}
/// Dividing a Force by a Frequency returns a value of type Momentum
impl<T> std::ops::Div<Frequency<T>> for &Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Momentum{kgmps: self.N.clone() / rhs.Hz}
	}
}
/// Dividing a Force by a Frequency returns a value of type Momentum
impl<T> std::ops::Div<&Frequency<T>> for Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Momentum{kgmps: self.N / rhs.Hz.clone()}
	}
}
/// Dividing a Force by a Frequency returns a value of type Momentum
impl<T> std::ops::Div<&Frequency<T>> for &Force<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Momentum{kgmps: self.N.clone() / rhs.Hz.clone()}
	}
}

// Force / Area -> Pressure
/// Dividing a Force by a Area returns a value of type Pressure
impl<T> std::ops::Div<Area<T>> for Force<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Pressure{Pa: self.N / rhs.m2}
	}
}
/// Dividing a Force by a Area returns a value of type Pressure
impl<T> std::ops::Div<Area<T>> for &Force<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Pressure{Pa: self.N.clone() / rhs.m2}
	}
}
/// Dividing a Force by a Area returns a value of type Pressure
impl<T> std::ops::Div<&Area<T>> for Force<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Pressure{Pa: self.N / rhs.m2.clone()}
	}
}
/// Dividing a Force by a Area returns a value of type Pressure
impl<T> std::ops::Div<&Area<T>> for &Force<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Pressure{Pa: self.N.clone() / rhs.m2.clone()}
	}
}

// Force * Velocity -> Power
/// Multiplying a Force by a Velocity returns a value of type Power
impl<T> std::ops::Mul<Velocity<T>> for Force<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Power{W: self.N * rhs.mps}
	}
}
/// Multiplying a Force by a Velocity returns a value of type Power
impl<T> std::ops::Mul<Velocity<T>> for &Force<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Power{W: self.N.clone() * rhs.mps}
	}
}
/// Multiplying a Force by a Velocity returns a value of type Power
impl<T> std::ops::Mul<&Velocity<T>> for Force<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Power{W: self.N * rhs.mps.clone()}
	}
}
/// Multiplying a Force by a Velocity returns a value of type Power
impl<T> std::ops::Mul<&Velocity<T>> for &Force<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Power{W: self.N.clone() * rhs.mps.clone()}
	}
}

// Force / Acceleration -> Mass
/// Dividing a Force by a Acceleration returns a value of type Mass
impl<T> std::ops::Div<Acceleration<T>> for Force<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		Mass{kg: self.N / rhs.mps2}
	}
}
/// Dividing a Force by a Acceleration returns a value of type Mass
impl<T> std::ops::Div<Acceleration<T>> for &Force<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		Mass{kg: self.N.clone() / rhs.mps2}
	}
}
/// Dividing a Force by a Acceleration returns a value of type Mass
impl<T> std::ops::Div<&Acceleration<T>> for Force<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		Mass{kg: self.N / rhs.mps2.clone()}
	}
}
/// Dividing a Force by a Acceleration returns a value of type Mass
impl<T> std::ops::Div<&Acceleration<T>> for &Force<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		Mass{kg: self.N.clone() / rhs.mps2.clone()}
	}
}

// Force / Momentum -> Frequency
/// Dividing a Force by a Momentum returns a value of type Frequency
impl<T> std::ops::Div<Momentum<T>> for Force<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Frequency{Hz: self.N / rhs.kgmps}
	}
}
/// Dividing a Force by a Momentum returns a value of type Frequency
impl<T> std::ops::Div<Momentum<T>> for &Force<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Frequency{Hz: self.N.clone() / rhs.kgmps}
	}
}
/// Dividing a Force by a Momentum returns a value of type Frequency
impl<T> std::ops::Div<&Momentum<T>> for Force<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Frequency{Hz: self.N / rhs.kgmps.clone()}
	}
}
/// Dividing a Force by a Momentum returns a value of type Frequency
impl<T> std::ops::Div<&Momentum<T>> for &Force<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Frequency{Hz: self.N.clone() / rhs.kgmps.clone()}
	}
}

// Force / Pressure -> Area
/// Dividing a Force by a Pressure returns a value of type Area
impl<T> std::ops::Div<Pressure<T>> for Force<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		Area{m2: self.N / rhs.Pa}
	}
}
/// Dividing a Force by a Pressure returns a value of type Area
impl<T> std::ops::Div<Pressure<T>> for &Force<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		Area{m2: self.N.clone() / rhs.Pa}
	}
}
/// Dividing a Force by a Pressure returns a value of type Area
impl<T> std::ops::Div<&Pressure<T>> for Force<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		Area{m2: self.N / rhs.Pa.clone()}
	}
}
/// Dividing a Force by a Pressure returns a value of type Area
impl<T> std::ops::Div<&Pressure<T>> for &Force<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		Area{m2: self.N.clone() / rhs.Pa.clone()}
	}
}

/// The pressure unit type, defined as pascals in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Pressure<T: NumLike>{
	/// The value of this Pressure in pascals
	pub Pa: T
}

impl<T> Pressure<T> where T: NumLike {

	/// Returns the standard unit name of pressure: "pascals"
	pub fn unit_name() -> &'static str {
		return "pascals";
	}
	
	/// Returns the abbreviated name or symbol of pressure: "Pa" for pascals
	pub fn unit_symbol() -> &'static str {
		return "Pa";
	}
	
	/// Returns a new pressure value from the given number of pascals
	///
	/// # Arguments
	/// * `Pa` - Any number-like type, representing a quantity of pascals
	pub fn from_Pa(Pa: T) -> Self {
		Pressure{Pa: Pa}
	}
	
	/// Returns a copy of this pressure value in pascals
	pub fn to_Pa(self) -> T {
		return self.Pa.clone();
	}

	/// Returns a new pressure value from the given number of pascals
	///
	/// # Arguments
	/// * `pascals` - Any number-like type, representing a quantity of pascals
	pub fn from_pascals(pascals: T) -> Self {
		Pressure{Pa: pascals}
	}
	
	/// Returns a copy of this pressure value in pascals
	pub fn to_pascals(self) -> T {
		return self.Pa.clone();
	}

}

impl<T> fmt::Display for Pressure<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Pa, Self::unit_symbol())
	}
}

impl<T> Pressure<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this pressure value in pounds per square inch
	pub fn to_psi(self) -> T {
		return self.Pa.clone() * T::from(0.00014503773773_f64);
	}

	/// Returns a new pressure value from the given number of pounds per square inch
	///
	/// # Arguments
	/// * `psi` - Any number-like type, representing a quantity of pounds per square inch
	pub fn from_psi(psi: T) -> Self {
		Pressure{Pa: psi * T::from(6894.7572931783_f64)}
	}

	/// Returns a copy of this pressure value in millipascals
	pub fn to_mPa(self) -> T {
		return self.Pa.clone() * T::from(1000.0_f64);
	}

	/// Returns a new pressure value from the given number of millipascals
	///
	/// # Arguments
	/// * `mPa` - Any number-like type, representing a quantity of millipascals
	pub fn from_mPa(mPa: T) -> Self {
		Pressure{Pa: mPa * T::from(0.001_f64)}
	}

	/// Returns a copy of this pressure value in micropascals
	pub fn to_uPa(self) -> T {
		return self.Pa.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new pressure value from the given number of micropascals
	///
	/// # Arguments
	/// * `uPa` - Any number-like type, representing a quantity of micropascals
	pub fn from_uPa(uPa: T) -> Self {
		Pressure{Pa: uPa * T::from(1e-06_f64)}
	}

	/// Returns a copy of this pressure value in nanopascals
	pub fn to_nPa(self) -> T {
		return self.Pa.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new pressure value from the given number of nanopascals
	///
	/// # Arguments
	/// * `nPa` - Any number-like type, representing a quantity of nanopascals
	pub fn from_nPa(nPa: T) -> Self {
		Pressure{Pa: nPa * T::from(1e-09_f64)}
	}

	/// Returns a copy of this pressure value in kilopascals
	pub fn to_kPa(self) -> T {
		return self.Pa.clone() * T::from(0.001_f64);
	}

	/// Returns a new pressure value from the given number of kilopascals
	///
	/// # Arguments
	/// * `kPa` - Any number-like type, representing a quantity of kilopascals
	pub fn from_kPa(kPa: T) -> Self {
		Pressure{Pa: kPa * T::from(1000.0_f64)}
	}

	/// Returns a copy of this pressure value in megapascals
	pub fn to_MPa(self) -> T {
		return self.Pa.clone() * T::from(1e-06_f64);
	}

	/// Returns a new pressure value from the given number of megapascals
	///
	/// # Arguments
	/// * `MPa` - Any number-like type, representing a quantity of megapascals
	pub fn from_MPa(MPa: T) -> Self {
		Pressure{Pa: MPa * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this pressure value in gigapascals
	pub fn to_GPa(self) -> T {
		return self.Pa.clone() * T::from(1e-09_f64);
	}

	/// Returns a new pressure value from the given number of gigapascals
	///
	/// # Arguments
	/// * `GPa` - Any number-like type, representing a quantity of gigapascals
	pub fn from_GPa(GPa: T) -> Self {
		Pressure{Pa: GPa * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this pressure value in hectopascals
	pub fn to_hPa(self) -> T {
		return self.Pa.clone() * T::from(0.01_f64);
	}

	/// Returns a new pressure value from the given number of hectopascals
	///
	/// # Arguments
	/// * `hPa` - Any number-like type, representing a quantity of hectopascals
	pub fn from_hPa(hPa: T) -> Self {
		Pressure{Pa: hPa * T::from(100.0_f64)}
	}

	/// Returns a copy of this pressure value in bar
	pub fn to_bar(self) -> T {
		return self.Pa.clone() * T::from(1e-05_f64);
	}

	/// Returns a new pressure value from the given number of bar
	///
	/// # Arguments
	/// * `bar` - Any number-like type, representing a quantity of bar
	pub fn from_bar(bar: T) -> Self {
		Pressure{Pa: bar * T::from(100000.0_f64)}
	}

	/// Returns a copy of this pressure value in millibar
	pub fn to_mbar(self) -> T {
		return self.Pa.clone() * T::from(0.01_f64);
	}

	/// Returns a new pressure value from the given number of millibar
	///
	/// # Arguments
	/// * `mbar` - Any number-like type, representing a quantity of millibar
	pub fn from_mbar(mbar: T) -> Self {
		Pressure{Pa: mbar * T::from(100.0_f64)}
	}

	/// Returns a copy of this pressure value in atmospheres
	pub fn to_atm(self) -> T {
		return self.Pa.clone() * T::from(9.86923266716013e-06_f64);
	}

	/// Returns a new pressure value from the given number of atmospheres
	///
	/// # Arguments
	/// * `atm` - Any number-like type, representing a quantity of atmospheres
	pub fn from_atm(atm: T) -> Self {
		Pressure{Pa: atm * T::from(101325.0_f64)}
	}

	/// Returns a copy of this pressure value in torr
	pub fn to_torr(self) -> T {
		return self.Pa.clone() * T::from(0.007500616827039_f64);
	}

	/// Returns a new pressure value from the given number of torr
	///
	/// # Arguments
	/// * `torr` - Any number-like type, representing a quantity of torr
	pub fn from_torr(torr: T) -> Self {
		Pressure{Pa: torr * T::from(133.3223684211_f64)}
	}

	/// Returns a copy of this pressure value in mm Hg
	pub fn to_mmHg(self) -> T {
		return self.Pa.clone() * T::from(0.007500616827039_f64);
	}

	/// Returns a new pressure value from the given number of mm Hg
	///
	/// # Arguments
	/// * `mmHg` - Any number-like type, representing a quantity of mm Hg
	pub fn from_mmHg(mmHg: T) -> Self {
		Pressure{Pa: mmHg * T::from(133.3223684211_f64)}
	}

}

// Pressure * Area -> Force
/// Multiplying a Pressure by a Area returns a value of type Force
impl<T> std::ops::Mul<Area<T>> for Pressure<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Force{N: self.Pa * rhs.m2}
	}
}
/// Multiplying a Pressure by a Area returns a value of type Force
impl<T> std::ops::Mul<Area<T>> for &Pressure<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Force{N: self.Pa.clone() * rhs.m2}
	}
}
/// Multiplying a Pressure by a Area returns a value of type Force
impl<T> std::ops::Mul<&Area<T>> for Pressure<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Force{N: self.Pa * rhs.m2.clone()}
	}
}
/// Multiplying a Pressure by a Area returns a value of type Force
impl<T> std::ops::Mul<&Area<T>> for &Pressure<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Force{N: self.Pa.clone() * rhs.m2.clone()}
	}
}

// Pressure * Volume -> Energy
/// Multiplying a Pressure by a Volume returns a value of type Energy
impl<T> std::ops::Mul<Volume<T>> for Pressure<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Energy{J: self.Pa * rhs.m3}
	}
}
/// Multiplying a Pressure by a Volume returns a value of type Energy
impl<T> std::ops::Mul<Volume<T>> for &Pressure<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Volume<T>) -> Self::Output {
		Energy{J: self.Pa.clone() * rhs.m3}
	}
}
/// Multiplying a Pressure by a Volume returns a value of type Energy
impl<T> std::ops::Mul<&Volume<T>> for Pressure<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Energy{J: self.Pa * rhs.m3.clone()}
	}
}
/// Multiplying a Pressure by a Volume returns a value of type Energy
impl<T> std::ops::Mul<&Volume<T>> for &Pressure<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Volume<T>) -> Self::Output {
		Energy{J: self.Pa.clone() * rhs.m3.clone()}
	}
}

/// The energy unit type, defined as joules in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Energy<T: NumLike>{
	/// The value of this Energy in joules
	pub J: T
}

impl<T> Energy<T> where T: NumLike {

	/// Returns the standard unit name of energy: "joules"
	pub fn unit_name() -> &'static str {
		return "joules";
	}
	
	/// Returns the abbreviated name or symbol of energy: "J" for joules
	pub fn unit_symbol() -> &'static str {
		return "J";
	}
	
	/// Returns a new energy value from the given number of joules
	///
	/// # Arguments
	/// * `J` - Any number-like type, representing a quantity of joules
	pub fn from_J(J: T) -> Self {
		Energy{J: J}
	}
	
	/// Returns a copy of this energy value in joules
	pub fn to_J(self) -> T {
		return self.J.clone();
	}

	/// Returns a new energy value from the given number of joules
	///
	/// # Arguments
	/// * `joules` - Any number-like type, representing a quantity of joules
	pub fn from_joules(joules: T) -> Self {
		Energy{J: joules}
	}
	
	/// Returns a copy of this energy value in joules
	pub fn to_joules(self) -> T {
		return self.J.clone();
	}

}

impl<T> fmt::Display for Energy<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.J, Self::unit_symbol())
	}
}

impl<T> Energy<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this energy value in millijoules
	pub fn to_mJ(self) -> T {
		return self.J.clone() * T::from(1000.0_f64);
	}

	/// Returns a new energy value from the given number of millijoules
	///
	/// # Arguments
	/// * `mJ` - Any number-like type, representing a quantity of millijoules
	pub fn from_mJ(mJ: T) -> Self {
		Energy{J: mJ * T::from(0.001_f64)}
	}

	/// Returns a copy of this energy value in microjoules
	pub fn to_uJ(self) -> T {
		return self.J.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new energy value from the given number of microjoules
	///
	/// # Arguments
	/// * `uJ` - Any number-like type, representing a quantity of microjoules
	pub fn from_uJ(uJ: T) -> Self {
		Energy{J: uJ * T::from(1e-06_f64)}
	}

	/// Returns a copy of this energy value in nanojoules
	pub fn to_nJ(self) -> T {
		return self.J.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new energy value from the given number of nanojoules
	///
	/// # Arguments
	/// * `nJ` - Any number-like type, representing a quantity of nanojoules
	pub fn from_nJ(nJ: T) -> Self {
		Energy{J: nJ * T::from(1e-09_f64)}
	}

	/// Returns a copy of this energy value in kilojoules
	pub fn to_kJ(self) -> T {
		return self.J.clone() * T::from(0.001_f64);
	}

	/// Returns a new energy value from the given number of kilojoules
	///
	/// # Arguments
	/// * `kJ` - Any number-like type, representing a quantity of kilojoules
	pub fn from_kJ(kJ: T) -> Self {
		Energy{J: kJ * T::from(1000.0_f64)}
	}

	/// Returns a copy of this energy value in megajoules
	pub fn to_MJ(self) -> T {
		return self.J.clone() * T::from(1e-06_f64);
	}

	/// Returns a new energy value from the given number of megajoules
	///
	/// # Arguments
	/// * `MJ` - Any number-like type, representing a quantity of megajoules
	pub fn from_MJ(MJ: T) -> Self {
		Energy{J: MJ * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this energy value in gigajoules
	pub fn to_GJ(self) -> T {
		return self.J.clone() * T::from(1e-09_f64);
	}

	/// Returns a new energy value from the given number of gigajoules
	///
	/// # Arguments
	/// * `GJ` - Any number-like type, representing a quantity of gigajoules
	pub fn from_GJ(GJ: T) -> Self {
		Energy{J: GJ * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this energy value in calories
	pub fn to_cal(self) -> T {
		return self.J.clone() * T::from(0.239005736137667_f64);
	}

	/// Returns a new energy value from the given number of calories
	///
	/// # Arguments
	/// * `cal` - Any number-like type, representing a quantity of calories
	pub fn from_cal(cal: T) -> Self {
		Energy{J: cal * T::from(4.184_f64)}
	}

	/// Returns a copy of this energy value in kilocalories
	pub fn to_kcal(self) -> T {
		return self.J.clone() * T::from(0.0002390057361376_f64);
	}

	/// Returns a new energy value from the given number of kilocalories
	///
	/// # Arguments
	/// * `kcal` - Any number-like type, representing a quantity of kilocalories
	pub fn from_kcal(kcal: T) -> Self {
		Energy{J: kcal * T::from(4184.0_f64)}
	}

	/// Returns a copy of this energy value in watt-hours
	pub fn to_Whr(self) -> T {
		return self.J.clone() * T::from(0.0002777777777777_f64);
	}

	/// Returns a new energy value from the given number of watt-hours
	///
	/// # Arguments
	/// * `Whr` - Any number-like type, representing a quantity of watt-hours
	pub fn from_Whr(Whr: T) -> Self {
		Energy{J: Whr * T::from(3600.0_f64)}
	}

	/// Returns a copy of this energy value in kilowatt-hours
	pub fn to_kWhr(self) -> T {
		return self.J.clone() * T::from(2.77777777777778e-07_f64);
	}

	/// Returns a new energy value from the given number of kilowatt-hours
	///
	/// # Arguments
	/// * `kWhr` - Any number-like type, representing a quantity of kilowatt-hours
	pub fn from_kWhr(kWhr: T) -> Self {
		Energy{J: kWhr * T::from(3600000.0_f64)}
	}

	/// Returns a copy of this energy value in electron-volts
	pub fn to_eV(self) -> T {
		return self.J.clone() * T::from(6.24150907446076e+18_f64);
	}

	/// Returns a new energy value from the given number of electron-volts
	///
	/// # Arguments
	/// * `eV` - Any number-like type, representing a quantity of electron-volts
	pub fn from_eV(eV: T) -> Self {
		Energy{J: eV * T::from(1.6021766340000001e-19_f64)}
	}

	/// Returns a copy of this energy value in british thermal units
	pub fn to_BTU(self) -> T {
		return self.J.clone() * T::from(0.0009478672985781_f64);
	}

	/// Returns a new energy value from the given number of british thermal units
	///
	/// # Arguments
	/// * `BTU` - Any number-like type, representing a quantity of british thermal units
	pub fn from_BTU(BTU: T) -> Self {
		Energy{J: BTU * T::from(1055.0_f64)}
	}

}

// Energy / Distance -> Force
/// Dividing a Energy by a Distance returns a value of type Force
impl<T> std::ops::Div<Distance<T>> for Energy<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Force{N: self.J / rhs.m}
	}
}
/// Dividing a Energy by a Distance returns a value of type Force
impl<T> std::ops::Div<Distance<T>> for &Energy<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Force{N: self.J.clone() / rhs.m}
	}
}
/// Dividing a Energy by a Distance returns a value of type Force
impl<T> std::ops::Div<&Distance<T>> for Energy<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Force{N: self.J / rhs.m.clone()}
	}
}
/// Dividing a Energy by a Distance returns a value of type Force
impl<T> std::ops::Div<&Distance<T>> for &Energy<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Force{N: self.J.clone() / rhs.m.clone()}
	}
}

// Energy / Time -> Power
/// Dividing a Energy by a Time returns a value of type Power
impl<T> std::ops::Div<Time<T>> for Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Power{W: self.J / rhs.s}
	}
}
/// Dividing a Energy by a Time returns a value of type Power
impl<T> std::ops::Div<Time<T>> for &Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Power{W: self.J.clone() / rhs.s}
	}
}
/// Dividing a Energy by a Time returns a value of type Power
impl<T> std::ops::Div<&Time<T>> for Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Power{W: self.J / rhs.s.clone()}
	}
}
/// Dividing a Energy by a Time returns a value of type Power
impl<T> std::ops::Div<&Time<T>> for &Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Power{W: self.J.clone() / rhs.s.clone()}
	}
}

// Energy / Current -> MagneticFlux
/// Dividing a Energy by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<Current<T>> for Energy<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.J / rhs.A}
	}
}
/// Dividing a Energy by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<Current<T>> for &Energy<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.J.clone() / rhs.A}
	}
}
/// Dividing a Energy by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<&Current<T>> for Energy<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.J / rhs.A.clone()}
	}
}
/// Dividing a Energy by a Current returns a value of type MagneticFlux
impl<T> std::ops::Div<&Current<T>> for &Energy<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.J.clone() / rhs.A.clone()}
	}
}

// Energy * Frequency -> Power
/// Multiplying a Energy by a Frequency returns a value of type Power
impl<T> std::ops::Mul<Frequency<T>> for Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Power{W: self.J * rhs.Hz}
	}
}
/// Multiplying a Energy by a Frequency returns a value of type Power
impl<T> std::ops::Mul<Frequency<T>> for &Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Power{W: self.J.clone() * rhs.Hz}
	}
}
/// Multiplying a Energy by a Frequency returns a value of type Power
impl<T> std::ops::Mul<&Frequency<T>> for Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Power{W: self.J * rhs.Hz.clone()}
	}
}
/// Multiplying a Energy by a Frequency returns a value of type Power
impl<T> std::ops::Mul<&Frequency<T>> for &Energy<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Power{W: self.J.clone() * rhs.Hz.clone()}
	}
}

// Energy / Volume -> Pressure
/// Dividing a Energy by a Volume returns a value of type Pressure
impl<T> std::ops::Div<Volume<T>> for Energy<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Pressure{Pa: self.J / rhs.m3}
	}
}
/// Dividing a Energy by a Volume returns a value of type Pressure
impl<T> std::ops::Div<Volume<T>> for &Energy<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Pressure{Pa: self.J.clone() / rhs.m3}
	}
}
/// Dividing a Energy by a Volume returns a value of type Pressure
impl<T> std::ops::Div<&Volume<T>> for Energy<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Pressure{Pa: self.J / rhs.m3.clone()}
	}
}
/// Dividing a Energy by a Volume returns a value of type Pressure
impl<T> std::ops::Div<&Volume<T>> for &Energy<T> where T: NumLike {
	type Output = Pressure<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Pressure{Pa: self.J.clone() / rhs.m3.clone()}
	}
}

// Energy / Velocity -> Momentum
/// Dividing a Energy by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<Velocity<T>> for Energy<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.J / rhs.mps}
	}
}
/// Dividing a Energy by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<Velocity<T>> for &Energy<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.J.clone() / rhs.mps}
	}
}
/// Dividing a Energy by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<&Velocity<T>> for Energy<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.J / rhs.mps.clone()}
	}
}
/// Dividing a Energy by a Velocity returns a value of type Momentum
impl<T> std::ops::Div<&Velocity<T>> for &Energy<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.J.clone() / rhs.mps.clone()}
	}
}

// Energy / Momentum -> Velocity
/// Dividing a Energy by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<Momentum<T>> for Energy<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Velocity{mps: self.J / rhs.kgmps}
	}
}
/// Dividing a Energy by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<Momentum<T>> for &Energy<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Velocity{mps: self.J.clone() / rhs.kgmps}
	}
}
/// Dividing a Energy by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<&Momentum<T>> for Energy<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Velocity{mps: self.J / rhs.kgmps.clone()}
	}
}
/// Dividing a Energy by a Momentum returns a value of type Velocity
impl<T> std::ops::Div<&Momentum<T>> for &Energy<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Velocity{mps: self.J.clone() / rhs.kgmps.clone()}
	}
}

// Energy / Force -> Distance
/// Dividing a Energy by a Force returns a value of type Distance
impl<T> std::ops::Div<Force<T>> for Energy<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Distance{m: self.J / rhs.N}
	}
}
/// Dividing a Energy by a Force returns a value of type Distance
impl<T> std::ops::Div<Force<T>> for &Energy<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Distance{m: self.J.clone() / rhs.N}
	}
}
/// Dividing a Energy by a Force returns a value of type Distance
impl<T> std::ops::Div<&Force<T>> for Energy<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Distance{m: self.J / rhs.N.clone()}
	}
}
/// Dividing a Energy by a Force returns a value of type Distance
impl<T> std::ops::Div<&Force<T>> for &Energy<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Distance{m: self.J.clone() / rhs.N.clone()}
	}
}

// Energy / Pressure -> Volume
/// Dividing a Energy by a Pressure returns a value of type Volume
impl<T> std::ops::Div<Pressure<T>> for Energy<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		Volume{m3: self.J / rhs.Pa}
	}
}
/// Dividing a Energy by a Pressure returns a value of type Volume
impl<T> std::ops::Div<Pressure<T>> for &Energy<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Pressure<T>) -> Self::Output {
		Volume{m3: self.J.clone() / rhs.Pa}
	}
}
/// Dividing a Energy by a Pressure returns a value of type Volume
impl<T> std::ops::Div<&Pressure<T>> for Energy<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		Volume{m3: self.J / rhs.Pa.clone()}
	}
}
/// Dividing a Energy by a Pressure returns a value of type Volume
impl<T> std::ops::Div<&Pressure<T>> for &Energy<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Pressure<T>) -> Self::Output {
		Volume{m3: self.J.clone() / rhs.Pa.clone()}
	}
}

// Energy / Charge -> Voltage
/// Dividing a Energy by a Charge returns a value of type Voltage
impl<T> std::ops::Div<Charge<T>> for Energy<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Voltage{V: self.J / rhs.C}
	}
}
/// Dividing a Energy by a Charge returns a value of type Voltage
impl<T> std::ops::Div<Charge<T>> for &Energy<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Voltage{V: self.J.clone() / rhs.C}
	}
}
/// Dividing a Energy by a Charge returns a value of type Voltage
impl<T> std::ops::Div<&Charge<T>> for Energy<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Voltage{V: self.J / rhs.C.clone()}
	}
}
/// Dividing a Energy by a Charge returns a value of type Voltage
impl<T> std::ops::Div<&Charge<T>> for &Energy<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Voltage{V: self.J.clone() / rhs.C.clone()}
	}
}

// Energy / Power -> Time
/// Dividing a Energy by a Power returns a value of type Time
impl<T> std::ops::Div<Power<T>> for Energy<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		Time{s: self.J / rhs.W}
	}
}
/// Dividing a Energy by a Power returns a value of type Time
impl<T> std::ops::Div<Power<T>> for &Energy<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Power<T>) -> Self::Output {
		Time{s: self.J.clone() / rhs.W}
	}
}
/// Dividing a Energy by a Power returns a value of type Time
impl<T> std::ops::Div<&Power<T>> for Energy<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		Time{s: self.J / rhs.W.clone()}
	}
}
/// Dividing a Energy by a Power returns a value of type Time
impl<T> std::ops::Div<&Power<T>> for &Energy<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Power<T>) -> Self::Output {
		Time{s: self.J.clone() / rhs.W.clone()}
	}
}

// Energy / Voltage -> Charge
/// Dividing a Energy by a Voltage returns a value of type Charge
impl<T> std::ops::Div<Voltage<T>> for Energy<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.J / rhs.V}
	}
}
/// Dividing a Energy by a Voltage returns a value of type Charge
impl<T> std::ops::Div<Voltage<T>> for &Energy<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.J.clone() / rhs.V}
	}
}
/// Dividing a Energy by a Voltage returns a value of type Charge
impl<T> std::ops::Div<&Voltage<T>> for Energy<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.J / rhs.V.clone()}
	}
}
/// Dividing a Energy by a Voltage returns a value of type Charge
impl<T> std::ops::Div<&Voltage<T>> for &Energy<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.J.clone() / rhs.V.clone()}
	}
}

// Energy / MagneticFlux -> Current
/// Dividing a Energy by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<MagneticFlux<T>> for Energy<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Current{A: self.J / rhs.Wb}
	}
}
/// Dividing a Energy by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<MagneticFlux<T>> for &Energy<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Current{A: self.J.clone() / rhs.Wb}
	}
}
/// Dividing a Energy by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<&MagneticFlux<T>> for Energy<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Current{A: self.J / rhs.Wb.clone()}
	}
}
/// Dividing a Energy by a MagneticFlux returns a value of type Current
impl<T> std::ops::Div<&MagneticFlux<T>> for &Energy<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Current{A: self.J.clone() / rhs.Wb.clone()}
	}
}

// Energy / AbsorbedDose -> Mass
/// Dividing a Energy by a AbsorbedDose returns a value of type Mass
impl<T> std::ops::Div<AbsorbedDose<T>> for Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: AbsorbedDose<T>) -> Self::Output {
		Mass{kg: self.J / rhs.Gy}
	}
}
/// Dividing a Energy by a AbsorbedDose returns a value of type Mass
impl<T> std::ops::Div<AbsorbedDose<T>> for &Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: AbsorbedDose<T>) -> Self::Output {
		Mass{kg: self.J.clone() / rhs.Gy}
	}
}
/// Dividing a Energy by a AbsorbedDose returns a value of type Mass
impl<T> std::ops::Div<&AbsorbedDose<T>> for Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &AbsorbedDose<T>) -> Self::Output {
		Mass{kg: self.J / rhs.Gy.clone()}
	}
}
/// Dividing a Energy by a AbsorbedDose returns a value of type Mass
impl<T> std::ops::Div<&AbsorbedDose<T>> for &Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &AbsorbedDose<T>) -> Self::Output {
		Mass{kg: self.J.clone() / rhs.Gy.clone()}
	}
}

// Energy / DoseEquivalent -> Mass
/// Dividing a Energy by a DoseEquivalent returns a value of type Mass
impl<T> std::ops::Div<DoseEquivalent<T>> for Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: DoseEquivalent<T>) -> Self::Output {
		Mass{kg: self.J / rhs.Sv}
	}
}
/// Dividing a Energy by a DoseEquivalent returns a value of type Mass
impl<T> std::ops::Div<DoseEquivalent<T>> for &Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: DoseEquivalent<T>) -> Self::Output {
		Mass{kg: self.J.clone() / rhs.Sv}
	}
}
/// Dividing a Energy by a DoseEquivalent returns a value of type Mass
impl<T> std::ops::Div<&DoseEquivalent<T>> for Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &DoseEquivalent<T>) -> Self::Output {
		Mass{kg: self.J / rhs.Sv.clone()}
	}
}
/// Dividing a Energy by a DoseEquivalent returns a value of type Mass
impl<T> std::ops::Div<&DoseEquivalent<T>> for &Energy<T> where T: NumLike {
	type Output = Mass<T>;
	fn div(self, rhs: &DoseEquivalent<T>) -> Self::Output {
		Mass{kg: self.J.clone() / rhs.Sv.clone()}
	}
}

/// The power (aka watts) unit type, defined as watts in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Power<T: NumLike>{
	/// The value of this Power in watts
	pub W: T
}

impl<T> Power<T> where T: NumLike {

	/// Returns the standard unit name of power: "watts"
	pub fn unit_name() -> &'static str {
		return "watts";
	}
	
	/// Returns the abbreviated name or symbol of power: "W" for watts
	pub fn unit_symbol() -> &'static str {
		return "W";
	}
	
	/// Returns a new power value from the given number of watts
	///
	/// # Arguments
	/// * `W` - Any number-like type, representing a quantity of watts
	pub fn from_W(W: T) -> Self {
		Power{W: W}
	}
	
	/// Returns a copy of this power value in watts
	pub fn to_W(self) -> T {
		return self.W.clone();
	}

	/// Returns a new power value from the given number of watts
	///
	/// # Arguments
	/// * `watts` - Any number-like type, representing a quantity of watts
	pub fn from_watts(watts: T) -> Self {
		Power{W: watts}
	}
	
	/// Returns a copy of this power value in watts
	pub fn to_watts(self) -> T {
		return self.W.clone();
	}

}

impl<T> fmt::Display for Power<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.W, Self::unit_symbol())
	}
}

impl<T> Power<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this power value in milliwatts
	pub fn to_mW(self) -> T {
		return self.W.clone() * T::from(1000.0_f64);
	}

	/// Returns a new power value from the given number of milliwatts
	///
	/// # Arguments
	/// * `mW` - Any number-like type, representing a quantity of milliwatts
	pub fn from_mW(mW: T) -> Self {
		Power{W: mW * T::from(0.001_f64)}
	}

	/// Returns a copy of this power value in microwatts
	pub fn to_uW(self) -> T {
		return self.W.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new power value from the given number of microwatts
	///
	/// # Arguments
	/// * `uW` - Any number-like type, representing a quantity of microwatts
	pub fn from_uW(uW: T) -> Self {
		Power{W: uW * T::from(1e-06_f64)}
	}

	/// Returns a copy of this power value in nanowatts
	pub fn to_nW(self) -> T {
		return self.W.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new power value from the given number of nanowatts
	///
	/// # Arguments
	/// * `nW` - Any number-like type, representing a quantity of nanowatts
	pub fn from_nW(nW: T) -> Self {
		Power{W: nW * T::from(1e-09_f64)}
	}

	/// Returns a copy of this power value in kilowatts
	pub fn to_kW(self) -> T {
		return self.W.clone() * T::from(0.001_f64);
	}

	/// Returns a new power value from the given number of kilowatts
	///
	/// # Arguments
	/// * `kW` - Any number-like type, representing a quantity of kilowatts
	pub fn from_kW(kW: T) -> Self {
		Power{W: kW * T::from(1000.0_f64)}
	}

	/// Returns a copy of this power value in megawatts
	pub fn to_MW(self) -> T {
		return self.W.clone() * T::from(1e-06_f64);
	}

	/// Returns a new power value from the given number of megawatts
	///
	/// # Arguments
	/// * `MW` - Any number-like type, representing a quantity of megawatts
	pub fn from_MW(MW: T) -> Self {
		Power{W: MW * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this power value in gigawatts
	pub fn to_GW(self) -> T {
		return self.W.clone() * T::from(1e-09_f64);
	}

	/// Returns a new power value from the given number of gigawatts
	///
	/// # Arguments
	/// * `GW` - Any number-like type, representing a quantity of gigawatts
	pub fn from_GW(GW: T) -> Self {
		Power{W: GW * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this power value in horse power
	pub fn to_horsepower(self) -> T {
		return self.W.clone() * T::from(0.0013410218586563_f64);
	}

	/// Returns a new power value from the given number of horse power
	///
	/// # Arguments
	/// * `horsepower` - Any number-like type, representing a quantity of horse power
	pub fn from_horsepower(horsepower: T) -> Self {
		Power{W: horsepower * T::from(745.7_f64)}
	}

}

// Power * Time -> Energy
/// Multiplying a Power by a Time returns a value of type Energy
impl<T> std::ops::Mul<Time<T>> for Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Energy{J: self.W * rhs.s}
	}
}
/// Multiplying a Power by a Time returns a value of type Energy
impl<T> std::ops::Mul<Time<T>> for &Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Energy{J: self.W.clone() * rhs.s}
	}
}
/// Multiplying a Power by a Time returns a value of type Energy
impl<T> std::ops::Mul<&Time<T>> for Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Energy{J: self.W * rhs.s.clone()}
	}
}
/// Multiplying a Power by a Time returns a value of type Energy
impl<T> std::ops::Mul<&Time<T>> for &Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Energy{J: self.W.clone() * rhs.s.clone()}
	}
}

// Power / Current -> Voltage
/// Dividing a Power by a Current returns a value of type Voltage
impl<T> std::ops::Div<Current<T>> for Power<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Voltage{V: self.W / rhs.A}
	}
}
/// Dividing a Power by a Current returns a value of type Voltage
impl<T> std::ops::Div<Current<T>> for &Power<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Voltage{V: self.W.clone() / rhs.A}
	}
}
/// Dividing a Power by a Current returns a value of type Voltage
impl<T> std::ops::Div<&Current<T>> for Power<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Voltage{V: self.W / rhs.A.clone()}
	}
}
/// Dividing a Power by a Current returns a value of type Voltage
impl<T> std::ops::Div<&Current<T>> for &Power<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Voltage{V: self.W.clone() / rhs.A.clone()}
	}
}

// Power / Torque -> Frequency
/// Dividing a Power by a Torque returns a value of type Frequency
impl<T> std::ops::Div<Torque<T>> for Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		Frequency{Hz: self.W / rhs.Nm}
	}
}
/// Dividing a Power by a Torque returns a value of type Frequency
impl<T> std::ops::Div<Torque<T>> for &Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Torque<T>) -> Self::Output {
		Frequency{Hz: self.W.clone() / rhs.Nm}
	}
}
/// Dividing a Power by a Torque returns a value of type Frequency
impl<T> std::ops::Div<&Torque<T>> for Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		Frequency{Hz: self.W / rhs.Nm.clone()}
	}
}
/// Dividing a Power by a Torque returns a value of type Frequency
impl<T> std::ops::Div<&Torque<T>> for &Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Torque<T>) -> Self::Output {
		Frequency{Hz: self.W.clone() / rhs.Nm.clone()}
	}
}

// Power / Energy -> Frequency
/// Dividing a Power by a Energy returns a value of type Frequency
impl<T> std::ops::Div<Energy<T>> for Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		Frequency{Hz: self.W / rhs.J}
	}
}
/// Dividing a Power by a Energy returns a value of type Frequency
impl<T> std::ops::Div<Energy<T>> for &Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Energy<T>) -> Self::Output {
		Frequency{Hz: self.W.clone() / rhs.J}
	}
}
/// Dividing a Power by a Energy returns a value of type Frequency
impl<T> std::ops::Div<&Energy<T>> for Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		Frequency{Hz: self.W / rhs.J.clone()}
	}
}
/// Dividing a Power by a Energy returns a value of type Frequency
impl<T> std::ops::Div<&Energy<T>> for &Power<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Energy<T>) -> Self::Output {
		Frequency{Hz: self.W.clone() / rhs.J.clone()}
	}
}

// Power / Frequency -> Energy
/// Dividing a Power by a Frequency returns a value of type Energy
impl<T> std::ops::Div<Frequency<T>> for Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Energy{J: self.W / rhs.Hz}
	}
}
/// Dividing a Power by a Frequency returns a value of type Energy
impl<T> std::ops::Div<Frequency<T>> for &Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Energy{J: self.W.clone() / rhs.Hz}
	}
}
/// Dividing a Power by a Frequency returns a value of type Energy
impl<T> std::ops::Div<&Frequency<T>> for Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Energy{J: self.W / rhs.Hz.clone()}
	}
}
/// Dividing a Power by a Frequency returns a value of type Energy
impl<T> std::ops::Div<&Frequency<T>> for &Power<T> where T: NumLike {
	type Output = Energy<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Energy{J: self.W.clone() / rhs.Hz.clone()}
	}
}

// Power / Velocity -> Force
/// Dividing a Power by a Velocity returns a value of type Force
impl<T> std::ops::Div<Velocity<T>> for Power<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Force{N: self.W / rhs.mps}
	}
}
/// Dividing a Power by a Velocity returns a value of type Force
impl<T> std::ops::Div<Velocity<T>> for &Power<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Force{N: self.W.clone() / rhs.mps}
	}
}
/// Dividing a Power by a Velocity returns a value of type Force
impl<T> std::ops::Div<&Velocity<T>> for Power<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Force{N: self.W / rhs.mps.clone()}
	}
}
/// Dividing a Power by a Velocity returns a value of type Force
impl<T> std::ops::Div<&Velocity<T>> for &Power<T> where T: NumLike {
	type Output = Force<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Force{N: self.W.clone() / rhs.mps.clone()}
	}
}

// Power / Acceleration -> Momentum
/// Dividing a Power by a Acceleration returns a value of type Momentum
impl<T> std::ops::Div<Acceleration<T>> for Power<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		Momentum{kgmps: self.W / rhs.mps2}
	}
}
/// Dividing a Power by a Acceleration returns a value of type Momentum
impl<T> std::ops::Div<Acceleration<T>> for &Power<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: Acceleration<T>) -> Self::Output {
		Momentum{kgmps: self.W.clone() / rhs.mps2}
	}
}
/// Dividing a Power by a Acceleration returns a value of type Momentum
impl<T> std::ops::Div<&Acceleration<T>> for Power<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		Momentum{kgmps: self.W / rhs.mps2.clone()}
	}
}
/// Dividing a Power by a Acceleration returns a value of type Momentum
impl<T> std::ops::Div<&Acceleration<T>> for &Power<T> where T: NumLike {
	type Output = Momentum<T>;
	fn div(self, rhs: &Acceleration<T>) -> Self::Output {
		Momentum{kgmps: self.W.clone() / rhs.mps2.clone()}
	}
}

// Power / Momentum -> Acceleration
/// Dividing a Power by a Momentum returns a value of type Acceleration
impl<T> std::ops::Div<Momentum<T>> for Power<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Acceleration{mps2: self.W / rhs.kgmps}
	}
}
/// Dividing a Power by a Momentum returns a value of type Acceleration
impl<T> std::ops::Div<Momentum<T>> for &Power<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: Momentum<T>) -> Self::Output {
		Acceleration{mps2: self.W.clone() / rhs.kgmps}
	}
}
/// Dividing a Power by a Momentum returns a value of type Acceleration
impl<T> std::ops::Div<&Momentum<T>> for Power<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Acceleration{mps2: self.W / rhs.kgmps.clone()}
	}
}
/// Dividing a Power by a Momentum returns a value of type Acceleration
impl<T> std::ops::Div<&Momentum<T>> for &Power<T> where T: NumLike {
	type Output = Acceleration<T>;
	fn div(self, rhs: &Momentum<T>) -> Self::Output {
		Acceleration{mps2: self.W.clone() / rhs.kgmps.clone()}
	}
}

// Power / Force -> Velocity
/// Dividing a Power by a Force returns a value of type Velocity
impl<T> std::ops::Div<Force<T>> for Power<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Velocity{mps: self.W / rhs.N}
	}
}
/// Dividing a Power by a Force returns a value of type Velocity
impl<T> std::ops::Div<Force<T>> for &Power<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Force<T>) -> Self::Output {
		Velocity{mps: self.W.clone() / rhs.N}
	}
}
/// Dividing a Power by a Force returns a value of type Velocity
impl<T> std::ops::Div<&Force<T>> for Power<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Velocity{mps: self.W / rhs.N.clone()}
	}
}
/// Dividing a Power by a Force returns a value of type Velocity
impl<T> std::ops::Div<&Force<T>> for &Power<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Force<T>) -> Self::Output {
		Velocity{mps: self.W.clone() / rhs.N.clone()}
	}
}

// Power / Voltage -> Current
/// Dividing a Power by a Voltage returns a value of type Current
impl<T> std::ops::Div<Voltage<T>> for Power<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Current{A: self.W / rhs.V}
	}
}
/// Dividing a Power by a Voltage returns a value of type Current
impl<T> std::ops::Div<Voltage<T>> for &Power<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Current{A: self.W.clone() / rhs.V}
	}
}
/// Dividing a Power by a Voltage returns a value of type Current
impl<T> std::ops::Div<&Voltage<T>> for Power<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Current{A: self.W / rhs.V.clone()}
	}
}
/// Dividing a Power by a Voltage returns a value of type Current
impl<T> std::ops::Div<&Voltage<T>> for &Power<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Current{A: self.W.clone() / rhs.V.clone()}
	}
}



