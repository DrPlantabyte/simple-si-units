
//! This module provides geometry SI units, such as angle 
//! and volume.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::mechanical::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;
#[cfg(feature="num_bigfloat")]
extern crate num_bigfloat;
#[cfg(feature="num_bigfloat")]
use num_bigfloat;
#[cfg(feature="num_complex")]
extern crate num_complex;
#[cfg(feature="num_complex")]
use num_complex;
#[cfg(feature="astro_float")]
extern crate astro_float;
#[cfg(feature="astro_float")]
use astro_float;


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
		Angle{rad: rad}
	}
	
	/// Returns a copy of this angle value in radians
	pub fn to_rad(self) -> T {
		return self.rad.clone();
	}

	/// Returns a new angle value from the given number of radians
	///
	/// # Arguments
	/// * `radians` - Any number-like type, representing a quantity of radians
	pub fn from_radians(radians: T) -> Self {
		Angle{rad: radians}
	}
	
	/// Returns a copy of this angle value in radians
	pub fn to_radians(self) -> T {
		return self.rad.clone();
	}

}

impl<T> fmt::Display for Angle<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.rad, Self::unit_symbol())
	}
}

impl<T> Angle<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this angle value in degrees
	pub fn to_degrees(self) -> T {
		return self.rad.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angle value from the given number of degrees
	///
	/// # Arguments
	/// * `degrees` - Any number-like type, representing a quantity of degrees
	pub fn from_degrees(degrees: T) -> Self {
		Angle{rad: degrees * T::from(0.0174532925199433_f64)}
	}

	/// Returns a copy of this angle value in degrees
	pub fn to_deg(self) -> T {
		return self.rad.clone() * T::from(57.2957795130823_f64);
	}

	/// Returns a new angle value from the given number of degrees
	///
	/// # Arguments
	/// * `deg` - Any number-like type, representing a quantity of degrees
	pub fn from_deg(deg: T) -> Self {
		Angle{rad: deg * T::from(0.0174532925199433_f64)}
	}

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
	
	/// Returns a new solid angle value from the given number of steradians
	///
	/// # Arguments
	/// * `sr` - Any number-like type, representing a quantity of steradian
	pub fn from_sr(sr: T) -> Self {
		SolidAngle{sr: sr}
	}
	
	/// Returns a copy of this solid angle value in steradians
	pub fn to_sr(self) -> T {
		return self.sr.clone();
	}

	/// Returns a new solid angle value from the given number of steradians
	///
	/// # Arguments
	/// * `steradians` - Any number-like type, representing a quantity of steradian
	pub fn from_steradians(steradians: T) -> Self {
		SolidAngle{sr: steradians}
	}
	
	/// Returns a copy of this solid angle value in steradians
	pub fn to_steradians(self) -> T {
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
		Area{m2: m2}
	}
	
	/// Returns a copy of this area value in square meters
	pub fn to_m2(self) -> T {
		return self.m2.clone();
	}

	/// Returns a new area value from the given number of square meters
	///
	/// # Arguments
	/// * `square_meters` - Any number-like type, representing a quantity of square meters
	pub fn from_square_meters(square_meters: T) -> Self {
		Area{m2: square_meters}
	}
	
	/// Returns a copy of this area value in square meters
	pub fn to_square_meters(self) -> T {
		return self.m2.clone();
	}

}

impl<T> fmt::Display for Area<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m2, Self::unit_symbol())
	}
}

impl<T> Area<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this area value in square cm
	pub fn to_cm2(self) -> T {
		return self.m2.clone() * T::from(10000.0_f64);
	}

	/// Returns a new area value from the given number of square cm
	///
	/// # Arguments
	/// * `cm2` - Any number-like type, representing a quantity of square cm
	pub fn from_cm2(cm2: T) -> Self {
		Area{m2: cm2 * T::from(0.0001_f64)}
	}

	/// Returns a copy of this area value in square cm
	pub fn to_square_cm(self) -> T {
		return self.m2.clone() * T::from(10000.0_f64);
	}

	/// Returns a new area value from the given number of square cm
	///
	/// # Arguments
	/// * `square_cm` - Any number-like type, representing a quantity of square cm
	pub fn from_square_cm(square_cm: T) -> Self {
		Area{m2: square_cm * T::from(0.0001_f64)}
	}

	/// Returns a copy of this area value in square mm
	pub fn to_mm2(self) -> T {
		return self.m2.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new area value from the given number of square mm
	///
	/// # Arguments
	/// * `mm2` - Any number-like type, representing a quantity of square mm
	pub fn from_mm2(mm2: T) -> Self {
		Area{m2: mm2 * T::from(1e-06_f64)}
	}

	/// Returns a copy of this area value in square um
	pub fn to_um2(self) -> T {
		return self.m2.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new area value from the given number of square um
	///
	/// # Arguments
	/// * `um2` - Any number-like type, representing a quantity of square um
	pub fn from_um2(um2: T) -> Self {
		Area{m2: um2 * T::from(1e-12_f64)}
	}

	/// Returns a copy of this area value in square nm
	pub fn to_nm2(self) -> T {
		return self.m2.clone() * T::from(1e+18_f64);
	}

	/// Returns a new area value from the given number of square nm
	///
	/// # Arguments
	/// * `nm2` - Any number-like type, representing a quantity of square nm
	pub fn from_nm2(nm2: T) -> Self {
		Area{m2: nm2 * T::from(1e-18_f64)}
	}

	/// Returns a copy of this area value in square km
	pub fn to_km2(self) -> T {
		return self.m2.clone() * T::from(1e-06_f64);
	}

	/// Returns a new area value from the given number of square km
	///
	/// # Arguments
	/// * `km2` - Any number-like type, representing a quantity of square km
	pub fn from_km2(km2: T) -> Self {
		Area{m2: km2 * T::from(1000000.0_f64)}
	}

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

// Area * Illuminance -> LuminousFlux
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> std::ops::Mul<Illuminance<T>> for Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2 * rhs.lux}
	}
}
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> std::ops::Mul<Illuminance<T>> for &Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2.clone() * rhs.lux}
	}
}
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> std::ops::Mul<&Illuminance<T>> for Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2 * rhs.lux.clone()}
	}
}
/// Multiplying a Area by a Illuminance returns a value of type LuminousFlux
impl<T> std::ops::Mul<&Illuminance<T>> for &Area<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Illuminance<T>) -> Self::Output {
		LuminousFlux{lm: self.m2.clone() * rhs.lux.clone()}
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
		Volume{m3: m3}
	}
	
	/// Returns a copy of this volume value in cubic meters
	pub fn to_m3(self) -> T {
		return self.m3.clone();
	}

	/// Returns a new volume value from the given number of cubic meters
	///
	/// # Arguments
	/// * `cubic_meters` - Any number-like type, representing a quantity of cubic meters
	pub fn from_cubic_meters(cubic_meters: T) -> Self {
		Volume{m3: cubic_meters}
	}
	
	/// Returns a copy of this volume value in cubic meters
	pub fn to_cubic_meters(self) -> T {
		return self.m3.clone();
	}

	/// Returns a new volume value from the given number of kiloliters
	///
	/// # Arguments
	/// * `kL` - Any number-like type, representing a quantity of cubic meters
	pub fn from_kL(kL: T) -> Self {
		Volume{m3: kL}
	}
	
	/// Returns a copy of this volume value in kiloliters
	pub fn to_kL(self) -> T {
		return self.m3.clone();
	}

}

impl<T> fmt::Display for Volume<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m3, Self::unit_symbol())
	}
}

impl<T> Volume<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this volume value in cubic cm
	pub fn to_cc(self) -> T {
		return self.m3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new volume value from the given number of cubic cm
	///
	/// # Arguments
	/// * `cc` - Any number-like type, representing a quantity of cubic cm
	pub fn from_cc(cc: T) -> Self {
		Volume{m3: cc * T::from(1e-06_f64)}
	}

	/// Returns a copy of this volume value in liters
	pub fn to_L(self) -> T {
		return self.m3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new volume value from the given number of liters
	///
	/// # Arguments
	/// * `L` - Any number-like type, representing a quantity of liters
	pub fn from_L(L: T) -> Self {
		Volume{m3: L * T::from(0.001_f64)}
	}

	/// Returns a copy of this volume value in liters
	pub fn to_liters(self) -> T {
		return self.m3.clone() * T::from(1000.0_f64);
	}

	/// Returns a new volume value from the given number of liters
	///
	/// # Arguments
	/// * `liters` - Any number-like type, representing a quantity of liters
	pub fn from_liters(liters: T) -> Self {
		Volume{m3: liters * T::from(0.001_f64)}
	}

	/// Returns a copy of this volume value in milliliters
	pub fn to_mL(self) -> T {
		return self.m3.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new volume value from the given number of milliliters
	///
	/// # Arguments
	/// * `mL` - Any number-like type, representing a quantity of milliliters
	pub fn from_mL(mL: T) -> Self {
		Volume{m3: mL * T::from(1e-06_f64)}
	}

	/// Returns a copy of this volume value in microliters
	pub fn to_uL(self) -> T {
		return self.m3.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new volume value from the given number of microliters
	///
	/// # Arguments
	/// * `uL` - Any number-like type, representing a quantity of microliters
	pub fn from_uL(uL: T) -> Self {
		Volume{m3: uL * T::from(1e-09_f64)}
	}

	/// Returns a copy of this volume value in nanoliters
	pub fn to_nL(self) -> T {
		return self.m3.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new volume value from the given number of nanoliters
	///
	/// # Arguments
	/// * `nL` - Any number-like type, representing a quantity of nanoliters
	pub fn from_nL(nL: T) -> Self {
		Volume{m3: nL * T::from(1e-12_f64)}
	}

	/// Returns a copy of this volume value in picoliters
	pub fn to_pL(self) -> T {
		return self.m3.clone() * T::from(1000000000000000.0_f64);
	}

	/// Returns a new volume value from the given number of picoliters
	///
	/// # Arguments
	/// * `pL` - Any number-like type, representing a quantity of picoliters
	pub fn from_pL(pL: T) -> Self {
		Volume{m3: pL * T::from(1e-15_f64)}
	}

	/// Returns a copy of this volume value in megaliters
	pub fn to_ML(self) -> T {
		return self.m3.clone() * T::from(0.001_f64);
	}

	/// Returns a new volume value from the given number of megaliters
	///
	/// # Arguments
	/// * `ML` - Any number-like type, representing a quantity of megaliters
	pub fn from_ML(ML: T) -> Self {
		Volume{m3: ML * T::from(1000.0_f64)}
	}

	/// Returns a copy of this volume value in gigaliters
	pub fn to_GL(self) -> T {
		return self.m3.clone() * T::from(1e-06_f64);
	}

	/// Returns a new volume value from the given number of gigaliters
	///
	/// # Arguments
	/// * `GL` - Any number-like type, representing a quantity of gigaliters
	pub fn from_GL(GL: T) -> Self {
		Volume{m3: GL * T::from(1000000.0_f64)}
	}

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

// Volume * Density -> Mass
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> std::ops::Mul<Density<T>> for Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		Mass{kg: self.m3 * rhs.kgpm3}
	}
}
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> std::ops::Mul<Density<T>> for &Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: Density<T>) -> Self::Output {
		Mass{kg: self.m3.clone() * rhs.kgpm3}
	}
}
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> std::ops::Mul<&Density<T>> for Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		Mass{kg: self.m3 * rhs.kgpm3.clone()}
	}
}
/// Multiplying a Volume by a Density returns a value of type Mass
impl<T> std::ops::Mul<&Density<T>> for &Volume<T> where T: NumLike {
	type Output = Mass<T>;
	fn mul(self, rhs: &Density<T>) -> Self::Output {
		Mass{kg: self.m3.clone() * rhs.kgpm3.clone()}
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



