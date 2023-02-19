
//! This module provides electromagnetic SI units, such as electric charge (aka coulombs) 
//! and magnetic flux.
use std::fmt;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;


/// The electric charge (aka coulombs) unit type, defined as coulombs in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Charge<T: NumLike>{
	pub C: T
}

impl<T> Charge<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "coulombs";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "C";
	}

	/// Returns a new electric charge value from the given number of coulombs
	///
	/// # Arguments
	/// * `C` - Any number-like type, representing a quantity of coulombs
	pub fn from_C(C: T) -> Self {
		Charge{C}
	}
	
	/// Returns a copy of this electric charge value in coulombs
	pub fn to_C(self) -> T {
		return self.C.clone();
	}
}

impl<T> fmt::Display for Charge<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.C, Self::unit_symbol())
	}
}

impl<T> Charge<T> where T: NumLike+From<f64> {
	
	// Charge / Time -> Current
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Charge<T> where T: NumLike {
		type Output = Current<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Current{A: self.C / rhs.s}
		}
	}

	// Charge / Current -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Current<T>> for Charge<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Current<T>) -> Self::Output {
			Time{s: self.C / rhs.A}
		}
	}

	// Charge * Frequency -> Current
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Charge<T> where T: NumLike {
		type Output = Current<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Current{A: self.C * rhs.Hz}
		}
	}

	// Charge * Voltage -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Voltage<T>> for Charge<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Voltage<T>) -> Self::Output {
			Energy{J: self.C * rhs.V}
		}
	}

	// Charge / Voltage -> Capacitance
	// TODO: docstring
	impl<T> std::ops::Div<Voltage<T>> for Charge<T> where T: NumLike {
		type Output = Capacitance<T>;
		fn div(self, rhs: Voltage<T>) -> Self::Output {
			Capacitance{F: self.C / rhs.V}
		}
	}

	// Charge * Resistance -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Resistance<T>> for Charge<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Resistance<T>) -> Self::Output {
			MagneticFlux{Wb: self.C * rhs.Ohm}
		}
	}

	// Charge / Conductance -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Div<Conductance<T>> for Charge<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn div(self, rhs: Conductance<T>) -> Self::Output {
			MagneticFlux{Wb: self.C / rhs.S}
		}
	}

	// Charge / Capacitance -> Voltage
	// TODO: docstring
	impl<T> std::ops::Div<Capacitance<T>> for Charge<T> where T: NumLike {
		type Output = Voltage<T>;
		fn div(self, rhs: Capacitance<T>) -> Self::Output {
			Voltage{V: self.C / rhs.F}
		}
	}

	// Charge / MagneticFlux -> Conductance
	// TODO: docstring
	impl<T> std::ops::Div<MagneticFlux<T>> for Charge<T> where T: NumLike {
		type Output = Conductance<T>;
		fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
			Conductance{S: self.C / rhs.Wb}
		}
	}

}

/// The voltage unit type, defined as volts in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Voltage<T: NumLike>{
	pub V: T
}

impl<T> Voltage<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "volts";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "V";
	}

	/// Returns a new voltage value from the given number of volts
	///
	/// # Arguments
	/// * `V` - Any number-like type, representing a quantity of volts
	pub fn from_V(V: T) -> Self {
		Voltage{V}
	}
	
	/// Returns a copy of this voltage value in volts
	pub fn to_V(self) -> T {
		return self.V.clone();
	}
}

impl<T> fmt::Display for Voltage<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.V, Self::unit_symbol())
	}
}

impl<T> Voltage<T> where T: NumLike+From<f64> {
	
	// Voltage * Time -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Voltage<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			MagneticFlux{Wb: self.V * rhs.s}
		}
	}

	// Voltage * Current -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Current<T>> for Voltage<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Current<T>) -> Self::Output {
			Power{W: self.V * rhs.A}
		}
	}

	// Voltage / Current -> Resistance
	// TODO: docstring
	impl<T> std::ops::Div<Current<T>> for Voltage<T> where T: NumLike {
		type Output = Resistance<T>;
		fn div(self, rhs: Current<T>) -> Self::Output {
			Resistance{Ohm: self.V / rhs.A}
		}
	}

	// Voltage / Frequency -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Voltage<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			MagneticFlux{Wb: self.V / rhs.Hz}
		}
	}

	// Voltage * Charge -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Charge<T>> for Voltage<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Charge<T>) -> Self::Output {
			Energy{J: self.V * rhs.C}
		}
	}

	// Voltage / Resistance -> Current
	// TODO: docstring
	impl<T> std::ops::Div<Resistance<T>> for Voltage<T> where T: NumLike {
		type Output = Current<T>;
		fn div(self, rhs: Resistance<T>) -> Self::Output {
			Current{A: self.V / rhs.Ohm}
		}
	}

	// Voltage * Conductance -> Current
	// TODO: docstring
	impl<T> std::ops::Mul<Conductance<T>> for Voltage<T> where T: NumLike {
		type Output = Current<T>;
		fn mul(self, rhs: Conductance<T>) -> Self::Output {
			Current{A: self.V * rhs.S}
		}
	}

	// Voltage * Capacitance -> Charge
	// TODO: docstring
	impl<T> std::ops::Mul<Capacitance<T>> for Voltage<T> where T: NumLike {
		type Output = Charge<T>;
		fn mul(self, rhs: Capacitance<T>) -> Self::Output {
			Charge{C: self.V * rhs.F}
		}
	}

	// Voltage / MagneticFlux -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<MagneticFlux<T>> for Voltage<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
			Frequency{Hz: self.V / rhs.Wb}
		}
	}

}

/// The electrical resistance unit type, defined as ohms in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Resistance<T: NumLike>{
	pub Ohm: T
}

impl<T> Resistance<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "ohms";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "Ohm";
	}

	/// Returns a new electrical resistance value from the given number of ohms
	///
	/// # Arguments
	/// * `Ohm` - Any number-like type, representing a quantity of ohms
	pub fn from_Ohm(Ohm: T) -> Self {
		Resistance{Ohm}
	}
	
	/// Returns a copy of this electrical resistance value in ohms
	pub fn to_Ohm(self) -> T {
		return self.Ohm.clone();
	}
}

impl<T> fmt::Display for Resistance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Ohm, Self::unit_symbol())
	}
}

impl<T> Resistance<T> where T: NumLike+From<f64> {
	
	// Resistance * Time -> Inductance
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Resistance<T> where T: NumLike {
		type Output = Inductance<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Inductance{H: self.Ohm * rhs.s}
		}
	}

	// Resistance * Current -> Voltage
	// TODO: docstring
	impl<T> std::ops::Mul<Current<T>> for Resistance<T> where T: NumLike {
		type Output = Voltage<T>;
		fn mul(self, rhs: Current<T>) -> Self::Output {
			Voltage{V: self.Ohm * rhs.A}
		}
	}

	// Resistance / Frequency -> Inductance
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Resistance<T> where T: NumLike {
		type Output = Inductance<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Inductance{H: self.Ohm / rhs.Hz}
		}
	}

	// Resistance * Charge -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Charge<T>> for Resistance<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Charge<T>) -> Self::Output {
			MagneticFlux{Wb: self.Ohm * rhs.C}
		}
	}

	// Resistance * Capacitance -> Time
	// TODO: docstring
	impl<T> std::ops::Mul<Capacitance<T>> for Resistance<T> where T: NumLike {
		type Output = Time<T>;
		fn mul(self, rhs: Capacitance<T>) -> Self::Output {
			Time{s: self.Ohm * rhs.F}
		}
	}

	// Resistance / Inductance -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Inductance<T>> for Resistance<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Inductance<T>) -> Self::Output {
			Frequency{Hz: self.Ohm / rhs.H}
		}
	}

}

/// The electrical conductance unit type, defined as siemens in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Conductance<T: NumLike>{
	pub S: T
}

impl<T> Conductance<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "siemens";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "S";
	}

	/// Returns a new electrical conductance value from the given number of siemens
	///
	/// # Arguments
	/// * `S` - Any number-like type, representing a quantity of siemens
	pub fn from_S(S: T) -> Self {
		Conductance{S}
	}
	
	/// Returns a copy of this electrical conductance value in siemens
	pub fn to_S(self) -> T {
		return self.S.clone();
	}
}

impl<T> fmt::Display for Conductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.S, Self::unit_symbol())
	}
}

impl<T> Conductance<T> where T: NumLike+From<f64> {
	
	// Conductance * Time -> Capacitance
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Conductance<T> where T: NumLike {
		type Output = Capacitance<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Capacitance{F: self.S * rhs.s}
		}
	}

	// Conductance / Frequency -> Capacitance
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Conductance<T> where T: NumLike {
		type Output = Capacitance<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Capacitance{F: self.S / rhs.Hz}
		}
	}

	// Conductance * Voltage -> Current
	// TODO: docstring
	impl<T> std::ops::Mul<Voltage<T>> for Conductance<T> where T: NumLike {
		type Output = Current<T>;
		fn mul(self, rhs: Voltage<T>) -> Self::Output {
			Current{A: self.S * rhs.V}
		}
	}

	// Conductance / Capacitance -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Capacitance<T>> for Conductance<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Capacitance<T>) -> Self::Output {
			Frequency{Hz: self.S / rhs.F}
		}
	}

	// Conductance * Inductance -> Time
	// TODO: docstring
	impl<T> std::ops::Mul<Inductance<T>> for Conductance<T> where T: NumLike {
		type Output = Time<T>;
		fn mul(self, rhs: Inductance<T>) -> Self::Output {
			Time{s: self.S * rhs.H}
		}
	}

	// Conductance * MagneticFlux -> Charge
	// TODO: docstring
	impl<T> std::ops::Mul<MagneticFlux<T>> for Conductance<T> where T: NumLike {
		type Output = Charge<T>;
		fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
			Charge{C: self.S * rhs.Wb}
		}
	}

}

/// The electrical capacitance unit type, defined as farads in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Capacitance<T: NumLike>{
	pub F: T
}

impl<T> Capacitance<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "farads";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "F";
	}

	/// Returns a new electrical capacitance value from the given number of farads
	///
	/// # Arguments
	/// * `F` - Any number-like type, representing a quantity of farads
	pub fn from_F(F: T) -> Self {
		Capacitance{F}
	}
	
	/// Returns a copy of this electrical capacitance value in farads
	pub fn to_F(self) -> T {
		return self.F.clone();
	}
}

impl<T> fmt::Display for Capacitance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.F, Self::unit_symbol())
	}
}

impl<T> Capacitance<T> where T: NumLike+From<f64> {
	
	// Capacitance / Time -> Conductance
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Capacitance<T> where T: NumLike {
		type Output = Conductance<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Conductance{S: self.F / rhs.s}
		}
	}

	// Capacitance * Frequency -> Conductance
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Capacitance<T> where T: NumLike {
		type Output = Conductance<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Conductance{S: self.F * rhs.Hz}
		}
	}

	// Capacitance * Voltage -> Charge
	// TODO: docstring
	impl<T> std::ops::Mul<Voltage<T>> for Capacitance<T> where T: NumLike {
		type Output = Charge<T>;
		fn mul(self, rhs: Voltage<T>) -> Self::Output {
			Charge{C: self.F * rhs.V}
		}
	}

	// Capacitance * Resistance -> Time
	// TODO: docstring
	impl<T> std::ops::Mul<Resistance<T>> for Capacitance<T> where T: NumLike {
		type Output = Time<T>;
		fn mul(self, rhs: Resistance<T>) -> Self::Output {
			Time{s: self.F * rhs.Ohm}
		}
	}

	// Capacitance / Conductance -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Conductance<T>> for Capacitance<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Conductance<T>) -> Self::Output {
			Time{s: self.F / rhs.S}
		}
	}

}

/// The inductance unit type, defined as henries in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Inductance<T: NumLike>{
	pub H: T
}

impl<T> Inductance<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "henries";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "H";
	}

	/// Returns a new inductance value from the given number of henries
	///
	/// # Arguments
	/// * `H` - Any number-like type, representing a quantity of henries
	pub fn from_H(H: T) -> Self {
		Inductance{H}
	}
	
	/// Returns a copy of this inductance value in henries
	pub fn to_H(self) -> T {
		return self.H.clone();
	}
}

impl<T> fmt::Display for Inductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.H, Self::unit_symbol())
	}
}

impl<T> Inductance<T> where T: NumLike+From<f64> {
	
	// Inductance / Time -> Resistance
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Inductance<T> where T: NumLike {
		type Output = Resistance<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Resistance{Ohm: self.H / rhs.s}
		}
	}

	// Inductance * Current -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Current<T>> for Inductance<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Current<T>) -> Self::Output {
			MagneticFlux{Wb: self.H * rhs.A}
		}
	}

	// Inductance * Frequency -> Resistance
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Inductance<T> where T: NumLike {
		type Output = Resistance<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Resistance{Ohm: self.H * rhs.Hz}
		}
	}

	// Inductance / Resistance -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Resistance<T>> for Inductance<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Resistance<T>) -> Self::Output {
			Time{s: self.H / rhs.Ohm}
		}
	}

	// Inductance * Conductance -> Time
	// TODO: docstring
	impl<T> std::ops::Mul<Conductance<T>> for Inductance<T> where T: NumLike {
		type Output = Time<T>;
		fn mul(self, rhs: Conductance<T>) -> Self::Output {
			Time{s: self.H * rhs.S}
		}
	}

}

/// The magnetic flux unit type, defined as webers in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct MagneticFlux<T: NumLike>{
	pub Wb: T
}

impl<T> MagneticFlux<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "webers";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "Wb";
	}

	/// Returns a new magnetic flux value from the given number of webers
	///
	/// # Arguments
	/// * `Wb` - Any number-like type, representing a quantity of webers
	pub fn from_Wb(Wb: T) -> Self {
		MagneticFlux{Wb}
	}
	
	/// Returns a copy of this magnetic flux value in webers
	pub fn to_Wb(self) -> T {
		return self.Wb.clone();
	}
}

impl<T> fmt::Display for MagneticFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Wb, Self::unit_symbol())
	}
}

impl<T> MagneticFlux<T> where T: NumLike+From<f64> {
	
	// MagneticFlux / Time -> Voltage
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Voltage<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Voltage{V: self.Wb / rhs.s}
		}
	}

	// MagneticFlux * Current -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Current<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Current<T>) -> Self::Output {
			Energy{J: self.Wb * rhs.A}
		}
	}

	// MagneticFlux / Current -> Inductance
	// TODO: docstring
	impl<T> std::ops::Div<Current<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Inductance<T>;
		fn div(self, rhs: Current<T>) -> Self::Output {
			Inductance{H: self.Wb / rhs.A}
		}
	}

	// MagneticFlux * Frequency -> Voltage
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Voltage<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Voltage{V: self.Wb * rhs.Hz}
		}
	}

	// MagneticFlux / Area -> MagneticFluxDensity
	// TODO: docstring
	impl<T> std::ops::Div<Area<T>> for MagneticFlux<T> where T: NumLike {
		type Output = MagneticFluxDensity<T>;
		fn div(self, rhs: Area<T>) -> Self::Output {
			MagneticFluxDensity{T: self.Wb / rhs.m2}
		}
	}

	// MagneticFlux / Charge -> Resistance
	// TODO: docstring
	impl<T> std::ops::Div<Charge<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Resistance<T>;
		fn div(self, rhs: Charge<T>) -> Self::Output {
			Resistance{Ohm: self.Wb / rhs.C}
		}
	}

	// MagneticFlux / Voltage -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Voltage<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Voltage<T>) -> Self::Output {
			Time{s: self.Wb / rhs.V}
		}
	}

	// MagneticFlux / Resistance -> Charge
	// TODO: docstring
	impl<T> std::ops::Div<Resistance<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Charge<T>;
		fn div(self, rhs: Resistance<T>) -> Self::Output {
			Charge{C: self.Wb / rhs.Ohm}
		}
	}

	// MagneticFlux * Conductance -> Charge
	// TODO: docstring
	impl<T> std::ops::Mul<Conductance<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Charge<T>;
		fn mul(self, rhs: Conductance<T>) -> Self::Output {
			Charge{C: self.Wb * rhs.S}
		}
	}

	// MagneticFlux / Inductance -> Current
	// TODO: docstring
	impl<T> std::ops::Div<Inductance<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Current<T>;
		fn div(self, rhs: Inductance<T>) -> Self::Output {
			Current{A: self.Wb / rhs.H}
		}
	}

	// MagneticFlux / MagneticFluxDensity -> Area
	// TODO: docstring
	impl<T> std::ops::Div<MagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
		type Output = Area<T>;
		fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
			Area{m2: self.Wb / rhs.T}
		}
	}

}

/// The magnetic flux density unit type, defined as teslas in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct MagneticFluxDensity<T: NumLike>{
	pub T: T
}

impl<T> MagneticFluxDensity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "teslas";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "T";
	}

	/// Returns a new magnetic flux density value from the given number of teslas
	///
	/// # Arguments
	/// * `T` - Any number-like type, representing a quantity of teslas
	pub fn from_T(T: T) -> Self {
		MagneticFluxDensity{T}
	}
	
	/// Returns a copy of this magnetic flux density value in teslas
	pub fn to_T(self) -> T {
		return self.T.clone();
	}
}

impl<T> fmt::Display for MagneticFluxDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.T, Self::unit_symbol())
	}
}

impl<T> MagneticFluxDensity<T> where T: NumLike+From<f64> {
	
	// MagneticFluxDensity * Area -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Area<T>> for MagneticFluxDensity<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Area<T>) -> Self::Output {
			MagneticFlux{Wb: self.T * rhs.m2}
		}
	}

}

/// The luminous flux unit type, defined as lumens in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct LuminousFlux<T: NumLike>{
	pub lm: T
}

impl<T> LuminousFlux<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "lumens";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "lm";
	}

	/// Returns a new luminous flux value from the given number of lumens
	///
	/// # Arguments
	/// * `lm` - Any number-like type, representing a quantity of lumens
	pub fn from_lm(lm: T) -> Self {
		LuminousFlux{lm}
	}
	
	/// Returns a copy of this luminous flux value in lumens
	pub fn to_lm(self) -> T {
		return self.lm.clone();
	}
}

impl<T> fmt::Display for LuminousFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.lm, Self::unit_symbol())
	}
}

impl<T> LuminousFlux<T> where T: NumLike+From<f64> {
	
	// LuminousFlux / Luminosity -> SolidAngle
	// TODO: docstring
	impl<T> std::ops::Div<Luminosity<T>> for LuminousFlux<T> where T: NumLike {
		type Output = SolidAngle<T>;
		fn div(self, rhs: Luminosity<T>) -> Self::Output {
			SolidAngle{sr: self.lm / rhs.cd}
		}
	}

	// LuminousFlux / SolidAngle -> Luminosity
	// TODO: docstring
	impl<T> std::ops::Div<SolidAngle<T>> for LuminousFlux<T> where T: NumLike {
		type Output = Luminosity<T>;
		fn div(self, rhs: SolidAngle<T>) -> Self::Output {
			Luminosity{cd: self.lm / rhs.sr}
		}
	}

}

/// The illuminance unit type, defined as lux in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Illuminance<T: NumLike>{
	pub lux: T
}

impl<T> Illuminance<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "lux";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "lux";
	}

	/// Returns a new illuminance value from the given number of lux
	///
	/// # Arguments
	/// * `lux` - Any number-like type, representing a quantity of lux
	pub fn from_lux(lux: T) -> Self {
		Illuminance{lux}
	}
	
	/// Returns a copy of this illuminance value in lux
	pub fn to_lux(self) -> T {
		return self.lux.clone();
	}
}

impl<T> fmt::Display for Illuminance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.lux, Self::unit_symbol())
	}
}

impl<T> Illuminance<T> where T: NumLike+From<f64> {
	
}


