
//! This module provides geometry SI units, such as angle 
//! and volume.
use std::fmt;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;


/// The angle unit type, defined as radians in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Angle<T: NumLike>{
	pub rad: T
}

impl<T> Angle<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "radians";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Angle / Time -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Angle<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			AngularVelocity{radps: self.rad / rhs.s}
		}
	}

	// Angle / AngularVelocity -> Time
	// TODO: docstring
	impl<T> std::ops::Div<AngularVelocity<T>> for Angle<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
			Time{s: self.rad / rhs.radps}
		}
	}

	// Angle * Frequency -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Angle<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			AngularVelocity{radps: self.rad * rhs.Hz}
		}
	}

}

/// The solid angle unit type, defined as steradian in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct SolidAngle<T: NumLike>{
	pub sr: T
}

impl<T> SolidAngle<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "steradian";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// SolidAngle * Luminosity -> LuminousFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Luminosity<T>> for SolidAngle<T> where T: NumLike {
		type Output = LuminousFlux<T>;
		fn mul(self, rhs: Luminosity<T>) -> Self::Output {
			LuminousFlux{lm: self.sr * rhs.cd}
		}
	}

}

/// The area unit type, defined as square meters in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Area<T: NumLike>{
	pub m2: T
}

impl<T> Area<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "square meters";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Area * Distance -> Volume
	// TODO: docstring
	impl<T> std::ops::Mul<Distance<T>> for Area<T> where T: NumLike {
		type Output = Volume<T>;
		fn mul(self, rhs: Distance<T>) -> Self::Output {
			Volume{m3: self.m2 * rhs.m}
		}
	}

	// Area / Distance -> Distance
	// TODO: docstring
	impl<T> std::ops::Div<Distance<T>> for Area<T> where T: NumLike {
		type Output = Distance<T>;
		fn div(self, rhs: Distance<T>) -> Self::Output {
			Distance{m: self.m2 / rhs.m}
		}
	}

	// Area * Mass -> AreaDensity
	// TODO: docstring
	impl<T> std::ops::Mul<Mass<T>> for Area<T> where T: NumLike {
		type Output = AreaDensity<T>;
		fn mul(self, rhs: Mass<T>) -> Self::Output {
			AreaDensity{kgm2: self.m2 * rhs.kg}
		}
	}

	// Area * Pressure -> Force
	// TODO: docstring
	impl<T> std::ops::Mul<Pressure<T>> for Area<T> where T: NumLike {
		type Output = Force<T>;
		fn mul(self, rhs: Pressure<T>) -> Self::Output {
			Force{N: self.m2 * rhs.Pa}
		}
	}

	// Area * MagneticFluxDensity -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<MagneticFluxDensity<T>> for Area<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
			MagneticFlux{Wb: self.m2 * rhs.T}
		}
	}

}

/// The volume unit type, defined as cubic meters in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Volume<T: NumLike>{
	pub m3: T
}

impl<T> Volume<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "cubic meters";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Volume / Distance -> Area
	// TODO: docstring
	impl<T> std::ops::Div<Distance<T>> for Volume<T> where T: NumLike {
		type Output = Area<T>;
		fn div(self, rhs: Distance<T>) -> Self::Output {
			Area{m2: self.m3 / rhs.m}
		}
	}

	// Volume / Area -> Distance
	// TODO: docstring
	impl<T> std::ops::Div<Area<T>> for Volume<T> where T: NumLike {
		type Output = Distance<T>;
		fn div(self, rhs: Area<T>) -> Self::Output {
			Distance{m: self.m3 / rhs.m2}
		}
	}

	// Volume * Pressure -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Pressure<T>> for Volume<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Pressure<T>) -> Self::Output {
			Energy{J: self.m3 * rhs.Pa}
		}
	}

	// Volume * Concentration -> Amount
	// TODO: docstring
	impl<T> std::ops::Mul<Concentration<T>> for Volume<T> where T: NumLike {
		type Output = Amount<T>;
		fn mul(self, rhs: Concentration<T>) -> Self::Output {
			Amount{mol: self.m3 * rhs.molpm3}
		}
	}

}


