
//! This module provides base SI units, such as distance (aka length) 
//! and amount.
use std::fmt;


/// The distance (aka length) unit type, defined as meters in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Distance<T: NumLike>{
	pub m: T
}

impl<T> Distance<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "meters";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "m";
	}

	/// Returns a new distance value from the given number of meters
	///
	/// # Arguments
	/// * `m` - Any number-like type, representing a quantity of meters
	pub fn from_m(m: T) -> Self {
		Distance{m}
	}
	
	/// Returns this distance value in meters
	pub fn to_m(self) -> T {
		return self.m;
	}
}

impl<T> fmt::Display for Distance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.m, Self::unit_symbol())
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: Distance * Distance -> Area

	// TODO: Distance / Time -> Velocity

	// TODO: Distance * Frequency -> Velocity

	// TODO: Distance * Area -> Volume

	// TODO: Distance / Velocity -> Time

	// TODO: Distance * Acceleration -> DoseEquivalent

	// TODO: Distance * Force -> Energy

}

/// The mass unit type, defined as kilograms in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Mass<T: NumLike>{
	pub kg: T
}

impl<T> Mass<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "kilograms";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "kg";
	}

	/// Returns a new mass value from the given number of kilograms
	///
	/// # Arguments
	/// * `kg` - Any number-like type, representing a quantity of kilograms
	pub fn from_kg(kg: T) -> Self {
		Mass{kg}
	}
	
	/// Returns this mass value in kilograms
	pub fn to_kg(self) -> T {
		return self.kg;
	}
}

impl<T> fmt::Display for Mass<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.kg, Self::unit_symbol())
	}
}

impl<T> Mass<T> where T: NumLike+From<f64> {
	
	// TODO: Mass * Area -> AreaDensity

	// TODO: Mass * Velocity -> Momentum

	// TODO: Mass * Acceleration -> Force

	// TODO: Mass * AbsorbedDose -> Energy

	// TODO: Mass * DoseEquivalent -> Energy

}

/// The time unit type, defined as seconds in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Time<T: NumLike>{
	pub s: T
}

impl<T> Time<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "seconds";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "s";
	}

	/// Returns a new time value from the given number of seconds
	///
	/// # Arguments
	/// * `s` - Any number-like type, representing a quantity of seconds
	pub fn from_s(s: T) -> Self {
		Time{s}
	}
	
	/// Returns this time value in seconds
	pub fn to_s(self) -> T {
		return self.s;
	}
}

impl<T> fmt::Display for Time<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.s, Self::unit_symbol())
	}
}

impl<T> Time<T> where T: NumLike+From<f64> {
	
	// TODO: Time * Current -> Charge

	// TODO: Time * AngularVelocity -> Angle

	// TODO: Time * AngularAcceleration -> AngularVelocity

	// TODO: Time * Velocity -> Distance

	// TODO: Time * Acceleration -> Velocity

	// TODO: Time * Force -> Momentum

	// TODO: Time * Power -> Energy

	// TODO: Time * Voltage -> MagneticFlux

	// TODO: Time * Resistance -> Inductance

	// TODO: Time / Resistance -> Capacitance

	// TODO: Time * Conductance -> Capacitance

	// TODO: Time / Conductance -> Inductance

	// TODO: Time / Capacitance -> Resistance

	// TODO: Time / Inductance -> Conductance

	// TODO: Time * CatalyticActivity -> Amount

}

/// The temperature unit type, defined as degrees kelvin in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Temperature<T: NumLike>{
	pub K: T
}

impl<T> Temperature<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "degrees kelvin";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "K";
	}

	/// Returns a new temperature value from the given number of degrees kelvin
	///
	/// # Arguments
	/// * `K` - Any number-like type, representing a quantity of degrees kelvin
	pub fn from_K(K: T) -> Self {
		Temperature{K}
	}
	
	/// Returns this temperature value in degrees kelvin
	pub fn to_K(self) -> T {
		return self.K;
	}
}

impl<T> fmt::Display for Temperature<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.K, Self::unit_symbol())
	}
}

impl<T> Temperature<T> where T: NumLike+From<f64> {
	
}

/// The amount unit type, defined as moles in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Amount<T: NumLike>{
	pub mol: T
}

impl<T> Amount<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "moles";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "mol";
	}

	/// Returns a new amount value from the given number of moles
	///
	/// # Arguments
	/// * `mol` - Any number-like type, representing a quantity of moles
	pub fn from_mol(mol: T) -> Self {
		Amount{mol}
	}
	
	/// Returns this amount value in moles
	pub fn to_mol(self) -> T {
		return self.mol;
	}
}

impl<T> fmt::Display for Amount<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.mol, Self::unit_symbol())
	}
}

impl<T> Amount<T> where T: NumLike+From<f64> {
	
	// TODO: Amount / Time -> CatalyticActivity

	// TODO: Amount * Frequency -> CatalyticActivity

	// TODO: Amount / Volume -> Concentration

	// TODO: Amount / CatalyticActivity -> Time

	// TODO: Amount / Concentration -> Volume

}

/// The electrical current unit type, defined as amperes in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Current<T: NumLike>{
	pub A: T
}

impl<T> Current<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "amperes";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "A";
	}

	/// Returns a new electrical current value from the given number of amperes
	///
	/// # Arguments
	/// * `A` - Any number-like type, representing a quantity of amperes
	pub fn from_A(A: T) -> Self {
		Current{A}
	}
	
	/// Returns this electrical current value in amperes
	pub fn to_A(self) -> T {
		return self.A;
	}
}

impl<T> fmt::Display for Current<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.A, Self::unit_symbol())
	}
}

impl<T> Current<T> where T: NumLike+From<f64> {
	
	// TODO: Current * Time -> Charge

	// TODO: Current / Frequency -> Charge

	// TODO: Current / Charge -> Frequency

	// TODO: Current * Voltage -> Power

	// TODO: Current / Voltage -> Conductance

	// TODO: Current * Resistance -> Voltage

	// TODO: Current / Conductance -> Voltage

	// TODO: Current * Inductance -> MagneticFlux

	// TODO: Current * MagneticFlux -> Energy

}

/// The luminosity unit type, defined as candela in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Luminosity<T: NumLike>{
	pub cd: T
}

impl<T> Luminosity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "candela";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "cd";
	}

	/// Returns a new luminosity value from the given number of candela
	///
	/// # Arguments
	/// * `cd` - Any number-like type, representing a quantity of candela
	pub fn from_cd(cd: T) -> Self {
		Luminosity{cd}
	}
	
	/// Returns this luminosity value in candela
	pub fn to_cd(self) -> T {
		return self.cd;
	}
}

impl<T> fmt::Display for Luminosity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.cd, Self::unit_symbol())
	}
}

impl<T> Luminosity<T> where T: NumLike+From<f64> {
	
	// TODO: Luminosity * SolidAngle -> LuminousFlux

}


