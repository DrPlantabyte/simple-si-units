
//! This module provides geometry SI units, such as angle 
//! and volume.

/// The angle unit type, defined as radians in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Angle<T: NumLike>{
	pub rad: T
}

impl<T> Angle<T> where T: NumLike {

	/// Returns a new angle value from the given number of radians
	///
	/// # Arguments
	/// * `rad` - Any number-like type, representing a quantity of radians
	pub fn from_rad(rad: T) -> Self {
		Angle{rad}
	}
	
	/// Returns this angle value in radians
	pub fn to_rad(self) -> T {
		return self.rad;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: Angle / Time -> AngularVelocity

	// TODO: Angle / AngularVelocity -> Time

	// TODO: Angle * Frequency -> AngularVelocity

}

/// The solid angle unit type, defined as steradian in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct SolidAngle<T: NumLike>{
	pub sr: T
}

impl<T> SolidAngle<T> where T: NumLike {

	/// Returns a new solid angle value from the given number of steradian
	///
	/// # Arguments
	/// * `sr` - Any number-like type, representing a quantity of steradian
	pub fn from_sr(sr: T) -> Self {
		SolidAngle{sr}
	}
	
	/// Returns this solid angle value in steradian
	pub fn to_sr(self) -> T {
		return self.sr;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: SolidAngle * Luminosity -> LuminousFlux

}

/// The area unit type, defined as square meters in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Area<T: NumLike>{
	pub m2: T
}

impl<T> Area<T> where T: NumLike {

	/// Returns a new area value from the given number of square meters
	///
	/// # Arguments
	/// * `m2` - Any number-like type, representing a quantity of square meters
	pub fn from_m2(m2: T) -> Self {
		Area{m2}
	}
	
	/// Returns this area value in square meters
	pub fn to_m2(self) -> T {
		return self.m2;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: Area * Distance -> Volume

	// TODO: Area / Distance -> Distance

	// TODO: Area * Mass -> AreaDensity

	// TODO: Area * Pressure -> Force

	// TODO: Area * MagneticFluxDensity -> MagneticFlux

}

/// The volume unit type, defined as cubic meters in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Volume<T: NumLike>{
	pub m3: T
}

impl<T> Volume<T> where T: NumLike {

	/// Returns a new volume value from the given number of cubic meters
	///
	/// # Arguments
	/// * `m3` - Any number-like type, representing a quantity of cubic meters
	pub fn from_m3(m3: T) -> Self {
		Volume{m3}
	}
	
	/// Returns this volume value in cubic meters
	pub fn to_m3(self) -> T {
		return self.m3;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: Volume / Distance -> Area

	// TODO: Volume / Area -> Distance

	// TODO: Volume * Pressure -> Energy

	// TODO: Volume * Concentration -> Amount

}


