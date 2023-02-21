
//! This module provides mechanical SI units, such as angular velocity 
//! and velocity.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::chemical::*;
use super::nuclear::*;
use super::geometry::*;
use super::base::*;
use super::electromagnetic::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;


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
		AngularVelocity{radps}
	}
	
	/// Returns a copy of this angular velocity value in radians per second
	pub fn to_radps(self) -> T {
		return self.radps.clone();
	}
}

impl<T> fmt::Display for AngularVelocity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.radps, Self::unit_symbol())
	}
}

impl<T> AngularVelocity<T> where T: NumLike+From<f64> {
	
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
		AngularAcceleration{radps2}
	}
	
	/// Returns a copy of this angular acceleration value in radians per second squared
	pub fn to_radps2(self) -> T {
		return self.radps2.clone();
	}
}

impl<T> fmt::Display for AngularAcceleration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.radps2, Self::unit_symbol())
	}
}

impl<T> AngularAcceleration<T> where T: NumLike+From<f64> {
	
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
		MomentOfInertia{kgm2}
	}
	
	/// Returns a copy of this moment of inertia value in kilogram meters squared
	pub fn to_kgm2(self) -> T {
		return self.kgm2.clone();
	}
}

impl<T> fmt::Display for MomentOfInertia<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgm2, Self::unit_symbol())
	}
}

impl<T> MomentOfInertia<T> where T: NumLike+From<f64> {
	
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
		AngularMomentum{kgm2radps}
	}
	
	/// Returns a copy of this angular momentum value in kilogram meters squared radians per second
	pub fn to_kgm2radps(self) -> T {
		return self.kgm2radps.clone();
	}
}

impl<T> fmt::Display for AngularMomentum<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgm2radps, Self::unit_symbol())
	}
}

impl<T> AngularMomentum<T> where T: NumLike+From<f64> {
	
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
		Torque{Nm}
	}
	
	/// Returns a copy of this torque value in newton meters
	pub fn to_Nm(self) -> T {
		return self.Nm.clone();
	}
}

impl<T> fmt::Display for Torque<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Nm, Self::unit_symbol())
	}
}

impl<T> Torque<T> where T: NumLike+From<f64> {
	
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
		Frequency{Hz}
	}
	
	/// Returns a copy of this frequency value in hertz
	pub fn to_Hz(self) -> T {
		return self.Hz.clone();
	}
}

impl<T> fmt::Display for Frequency<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Hz, Self::unit_symbol())
	}
}

impl<T> Frequency<T> where T: NumLike+From<f64> {
	
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
	/// * `kgm2` - Any number-like type, representing a quantity of kilograms per square meter
	pub fn from_kgm2(kgm2: T) -> Self {
		AreaDensity{kgm2}
	}
	
	/// Returns a copy of this area density value in kilograms per square meter
	pub fn to_kgm2(self) -> T {
		return self.kgm2.clone();
	}
}

impl<T> fmt::Display for AreaDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgm2, Self::unit_symbol())
	}
}

impl<T> AreaDensity<T> where T: NumLike+From<f64> {
	
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

/// The density unit type, defined as kilograms per liter in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Density<T: NumLike>{
	/// The value of this Density in kilograms per liter
	pub kgpL: T
}

impl<T> Density<T> where T: NumLike {

	/// Returns the standard unit name of density: "kilograms per liter"
	pub fn unit_name() -> &'static str {
		return "kilograms per liter";
	}
	
	/// Returns the abbreviated name or symbol of density: "kgpL" for kilograms per liter
	pub fn unit_symbol() -> &'static str {
		return "kgpL";
	}

	/// Returns a new density value from the given number of kilograms per liter
	///
	/// # Arguments
	/// * `kgpL` - Any number-like type, representing a quantity of kilograms per liter
	pub fn from_kgpL(kgpL: T) -> Self {
		Density{kgpL}
	}
	
	/// Returns a copy of this density value in kilograms per liter
	pub fn to_kgpL(self) -> T {
		return self.kgpL.clone();
	}
}

impl<T> fmt::Display for Density<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgpL, Self::unit_symbol())
	}
}

impl<T> Density<T> where T: NumLike+From<f64> {
	
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
		Velocity{mps}
	}
	
	/// Returns a copy of this velocity value in meters per second
	pub fn to_mps(self) -> T {
		return self.mps.clone();
	}
}

impl<T> fmt::Display for Velocity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.mps, Self::unit_symbol())
	}
}

impl<T> Velocity<T> where T: NumLike+From<f64> {
	
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
		Acceleration{mps2}
	}
	
	/// Returns a copy of this acceleration value in meters per second squared
	pub fn to_mps2(self) -> T {
		return self.mps2.clone();
	}
}

impl<T> fmt::Display for Acceleration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.mps2, Self::unit_symbol())
	}
}

impl<T> Acceleration<T> where T: NumLike+From<f64> {
	
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
		Momentum{kgmps}
	}
	
	/// Returns a copy of this momentum value in kilogram meters per second
	pub fn to_kgmps(self) -> T {
		return self.kgmps.clone();
	}
}

impl<T> fmt::Display for Momentum<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kgmps, Self::unit_symbol())
	}
}

impl<T> Momentum<T> where T: NumLike+From<f64> {
	
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
		Force{N}
	}
	
	/// Returns a copy of this force value in newtons
	pub fn to_N(self) -> T {
		return self.N.clone();
	}
}

impl<T> fmt::Display for Force<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.N, Self::unit_symbol())
	}
}

impl<T> Force<T> where T: NumLike+From<f64> {
	
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
		Pressure{Pa}
	}
	
	/// Returns a copy of this pressure value in pascals
	pub fn to_Pa(self) -> T {
		return self.Pa.clone();
	}
}

impl<T> fmt::Display for Pressure<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Pa, Self::unit_symbol())
	}
}

impl<T> Pressure<T> where T: NumLike+From<f64> {
	
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
		Energy{J}
	}
	
	/// Returns a copy of this energy value in joules
	pub fn to_J(self) -> T {
		return self.J.clone();
	}
}

impl<T> fmt::Display for Energy<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.J, Self::unit_symbol())
	}
}

impl<T> Energy<T> where T: NumLike+From<f64> {
	
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
		Power{W}
	}
	
	/// Returns a copy of this power value in watts
	pub fn to_W(self) -> T {
		return self.W.clone();
	}
}

impl<T> fmt::Display for Power<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.W, Self::unit_symbol())
	}
}

impl<T> Power<T> where T: NumLike+From<f64> {
	
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


