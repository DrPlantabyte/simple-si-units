
//! This module provides geometry SI units, such as angle 
//! and volume.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::electromagnetic::*;
use super::mechanical::*;
use super::base::*;
use super::chemical::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;


/// The angle unit type, defined as radians in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Angle<T: NumLike>{
	/// The value of this Angle in radians
	pub rad: T
}

impl<T> Angle<T> where T: NumLike {

	/// Returns the standard unit name of angle: "radians"
	pub fn unit_name() -> &'static str {
		return "radians";
	}
	
	/// Returns the abbreviated name or symbol of angle: "rad" for radians
	pub fn unit_symbol() -> &'static str {
		return "rad";
	}

	/// Returns a new angle value from the given number of radians
	///
	/// # Arguments
	/// * `rad` - Any number-like type, representing a quantity of radians
	pub fn from_rad(rad: T) -> Self {
		Angle{rad}
	}
	
	/// Returns a copy of this angle value in radians
	pub fn to_rad(self) -> T {
		return self.rad.clone();
	}
}

impl<T> fmt::Display for Angle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.rad, Self::unit_symbol())
	}
}

impl<T> Angle<T> where T: NumLike+From<f64> {
	
}

// Angle / Time -> AngularVelocity
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> std::ops::Div<Time<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad / rhs.s}
	}
}
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> std::ops::Div<Time<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() / rhs.s}
	}
}
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> std::ops::Div<&Time<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad / rhs.s.clone()}
	}
}
/// Dividing a Angle by a Time returns a value of type AngularVelocity
impl<T> std::ops::Div<&Time<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() / rhs.s.clone()}
	}
}

// Angle / AngularVelocity -> Time
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> std::ops::Div<AngularVelocity<T>> for Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad / rhs.radps}
	}
}
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> std::ops::Div<AngularVelocity<T>> for &Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad.clone() / rhs.radps}
	}
}
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> std::ops::Div<&AngularVelocity<T>> for Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad / rhs.radps.clone()}
	}
}
/// Dividing a Angle by a AngularVelocity returns a value of type Time
impl<T> std::ops::Div<&AngularVelocity<T>> for &Angle<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Time{s: self.rad.clone() / rhs.radps.clone()}
	}
}

// Angle * Frequency -> AngularVelocity
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Mul<Frequency<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad * rhs.Hz}
	}
}
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Mul<Frequency<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() * rhs.Hz}
	}
}
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Mul<&Frequency<T>> for Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad * rhs.Hz.clone()}
	}
}
/// Multiplying a Angle by a Frequency returns a value of type AngularVelocity
impl<T> std::ops::Mul<&Frequency<T>> for &Angle<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		AngularVelocity{radps: self.rad.clone() * rhs.Hz.clone()}
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
	pub fn unit_name() -> &'static str {
		return "steradian";
	}
	
	/// Returns the abbreviated name or symbol of solid angle: "sr" for steradian
	pub fn unit_symbol() -> &'static str {
		return "sr";
	}

	/// Returns a new solid angle value from the given number of steradian
	///
	/// # Arguments
	/// * `sr` - Any number-like type, representing a quantity of steradian
	pub fn from_sr(sr: T) -> Self {
		SolidAngle{sr}
	}
	
	/// Returns a copy of this solid angle value in steradian
	pub fn to_sr(self) -> T {
		return self.sr.clone();
	}
}

impl<T> fmt::Display for SolidAngle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.sr, Self::unit_symbol())
	}
}

impl<T> SolidAngle<T> where T: NumLike+From<f64> {
	
}

// SolidAngle * Luminosity -> LuminousFlux
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> std::ops::Mul<Luminosity<T>> for SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr * rhs.cd}
	}
}
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> std::ops::Mul<Luminosity<T>> for &SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr.clone() * rhs.cd}
	}
}
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> std::ops::Mul<&Luminosity<T>> for SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr * rhs.cd.clone()}
	}
}
/// Multiplying a SolidAngle by a Luminosity returns a value of type LuminousFlux
impl<T> std::ops::Mul<&Luminosity<T>> for &SolidAngle<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Luminosity<T>) -> Self::Output {
		LuminousFlux{lm: self.sr.clone() * rhs.cd.clone()}
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
	pub fn unit_name() -> &'static str {
		return "square meters";
	}
	
	/// Returns the abbreviated name or symbol of area: "m2" for square meters
	pub fn unit_symbol() -> &'static str {
		return "m2";
	}

	/// Returns a new area value from the given number of square meters
	///
	/// # Arguments
	/// * `m2` - Any number-like type, representing a quantity of square meters
	pub fn from_m2(m2: T) -> Self {
		Area{m2}
	}
	
	/// Returns a copy of this area value in square meters
	pub fn to_m2(self) -> T {
		return self.m2.clone();
	}
}

impl<T> fmt::Display for Area<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m2, Self::unit_symbol())
	}
}

impl<T> Area<T> where T: NumLike+From<f64> {
	
}

// Area * Distance -> Volume
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> std::ops::Mul<Distance<T>> for Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Volume{m3: self.m2 * rhs.m}
	}
}
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> std::ops::Mul<Distance<T>> for &Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Volume{m3: self.m2.clone() * rhs.m}
	}
}
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> std::ops::Mul<&Distance<T>> for Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Volume{m3: self.m2 * rhs.m.clone()}
	}
}
/// Multiplying a Area by a Distance returns a value of type Volume
impl<T> std::ops::Mul<&Distance<T>> for &Area<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Volume{m3: self.m2.clone() * rhs.m.clone()}
	}
}

// Area / Distance -> Distance
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> std::ops::Div<Distance<T>> for Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Distance{m: self.m2 / rhs.m}
	}
}
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> std::ops::Div<Distance<T>> for &Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Distance{m: self.m2.clone() / rhs.m}
	}
}
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> std::ops::Div<&Distance<T>> for Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Distance{m: self.m2 / rhs.m.clone()}
	}
}
/// Dividing a Area by a Distance returns a value of type Distance
impl<T> std::ops::Div<&Distance<T>> for &Area<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Distance{m: self.m2.clone() / rhs.m.clone()}
	}
}

// Area * Mass -> AreaDensity
/// Multiplying a Area by a Mass returns a value of type AreaDensity
impl<T> std::ops::Mul<Mass<T>> for Area<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		AreaDensity{kgm2: self.m2 * rhs.kg}
	}
}
/// Multiplying a Area by a Mass returns a value of type AreaDensity
impl<T> std::ops::Mul<Mass<T>> for &Area<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Mass<T>) -> Self::Output {
		AreaDensity{kgm2: self.m2.clone() * rhs.kg}
	}
}
/// Multiplying a Area by a Mass returns a value of type AreaDensity
impl<T> std::ops::Mul<&Mass<T>> for Area<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		AreaDensity{kgm2: self.m2 * rhs.kg.clone()}
	}
}
/// Multiplying a Area by a Mass returns a value of type AreaDensity
impl<T> std::ops::Mul<&Mass<T>> for &Area<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Mass<T>) -> Self::Output {
		AreaDensity{kgm2: self.m2.clone() * rhs.kg.clone()}
	}
}

// Area * Pressure -> Force
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> std::ops::Mul<Pressure<T>> for Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Force{N: self.m2 * rhs.Pa}
	}
}
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> std::ops::Mul<Pressure<T>> for &Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Force{N: self.m2.clone() * rhs.Pa}
	}
}
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> std::ops::Mul<&Pressure<T>> for Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Force{N: self.m2 * rhs.Pa.clone()}
	}
}
/// Multiplying a Area by a Pressure returns a value of type Force
impl<T> std::ops::Mul<&Pressure<T>> for &Area<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Force{N: self.m2.clone() * rhs.Pa.clone()}
	}
}

// Area * MagneticFluxDensity -> MagneticFlux
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> std::ops::Mul<MagneticFluxDensity<T>> for Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2 * rhs.T}
	}
}
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> std::ops::Mul<MagneticFluxDensity<T>> for &Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2.clone() * rhs.T}
	}
}
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> std::ops::Mul<&MagneticFluxDensity<T>> for Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2 * rhs.T.clone()}
	}
}
/// Multiplying a Area by a MagneticFluxDensity returns a value of type MagneticFlux
impl<T> std::ops::Mul<&MagneticFluxDensity<T>> for &Area<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		MagneticFlux{Wb: self.m2.clone() * rhs.T.clone()}
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
	pub fn unit_name() -> &'static str {
		return "cubic meters";
	}
	
	/// Returns the abbreviated name or symbol of volume: "m3" for cubic meters
	pub fn unit_symbol() -> &'static str {
		return "m3";
	}

	/// Returns a new volume value from the given number of cubic meters
	///
	/// # Arguments
	/// * `m3` - Any number-like type, representing a quantity of cubic meters
	pub fn from_m3(m3: T) -> Self {
		Volume{m3}
	}
	
	/// Returns a copy of this volume value in cubic meters
	pub fn to_m3(self) -> T {
		return self.m3.clone();
	}
}

impl<T> fmt::Display for Volume<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m3, Self::unit_symbol())
	}
}

impl<T> Volume<T> where T: NumLike+From<f64> {
	
}

// Volume / Distance -> Area
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> std::ops::Div<Distance<T>> for Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m3 / rhs.m}
	}
}
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> std::ops::Div<Distance<T>> for &Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m3.clone() / rhs.m}
	}
}
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> std::ops::Div<&Distance<T>> for Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m3 / rhs.m.clone()}
	}
}
/// Dividing a Volume by a Distance returns a value of type Area
impl<T> std::ops::Div<&Distance<T>> for &Volume<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m3.clone() / rhs.m.clone()}
	}
}

// Volume / Area -> Distance
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> std::ops::Div<Area<T>> for Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Distance{m: self.m3 / rhs.m2}
	}
}
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> std::ops::Div<Area<T>> for &Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Distance{m: self.m3.clone() / rhs.m2}
	}
}
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> std::ops::Div<&Area<T>> for Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Distance{m: self.m3 / rhs.m2.clone()}
	}
}
/// Dividing a Volume by a Area returns a value of type Distance
impl<T> std::ops::Div<&Area<T>> for &Volume<T> where T: NumLike {
	type Output = Distance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Distance{m: self.m3.clone() / rhs.m2.clone()}
	}
}

// Volume * Pressure -> Energy
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> std::ops::Mul<Pressure<T>> for Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Energy{J: self.m3 * rhs.Pa}
	}
}
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> std::ops::Mul<Pressure<T>> for &Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Pressure<T>) -> Self::Output {
		Energy{J: self.m3.clone() * rhs.Pa}
	}
}
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> std::ops::Mul<&Pressure<T>> for Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Energy{J: self.m3 * rhs.Pa.clone()}
	}
}
/// Multiplying a Volume by a Pressure returns a value of type Energy
impl<T> std::ops::Mul<&Pressure<T>> for &Volume<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Pressure<T>) -> Self::Output {
		Energy{J: self.m3.clone() * rhs.Pa.clone()}
	}
}

// Volume * Concentration -> Amount
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> std::ops::Mul<Concentration<T>> for Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		Amount{mol: self.m3 * rhs.molpm3}
	}
}
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> std::ops::Mul<Concentration<T>> for &Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: Concentration<T>) -> Self::Output {
		Amount{mol: self.m3.clone() * rhs.molpm3}
	}
}
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> std::ops::Mul<&Concentration<T>> for Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		Amount{mol: self.m3 * rhs.molpm3.clone()}
	}
}
/// Multiplying a Volume by a Concentration returns a value of type Amount
impl<T> std::ops::Mul<&Concentration<T>> for &Volume<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &Concentration<T>) -> Self::Output {
		Amount{mol: self.m3.clone() * rhs.molpm3.clone()}
	}
}


