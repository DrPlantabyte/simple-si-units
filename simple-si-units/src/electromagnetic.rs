
//! This module provides electromagnetic SI units, such as electric charge (aka coulombs) 
//! and magnetic flux.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::geometry::*;
use super::mechanical::*;
use super::base::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;


/// The electric charge (aka coulombs) unit type, defined as coulombs in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Charge<T: NumLike>{
	/// The value of this Electric charge in coulombs
	pub C: T
}

impl<T> Charge<T> where T: NumLike {

	/// Returns the standard unit name of electric charge: "coulombs"
	pub fn unit_name() -> &'static str {
		return "coulombs";
	}
	
	/// Returns the abbreviated name or symbol of electric charge: "C" for coulombs
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
	
}

// Charge / Time -> Current
/// Dividing a Charge by a Time returns a value of type Current
impl<T> std::ops::Div<Time<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Current{A: self.C / rhs.s}
	}
}
/// Dividing a Charge by a Time returns a value of type Current
impl<T> std::ops::Div<Time<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Current{A: self.C.clone() / rhs.s}
	}
}
/// Dividing a Charge by a Time returns a value of type Current
impl<T> std::ops::Div<&Time<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Current{A: self.C / rhs.s.clone()}
	}
}
/// Dividing a Charge by a Time returns a value of type Current
impl<T> std::ops::Div<&Time<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Current{A: self.C.clone() / rhs.s.clone()}
	}
}

// Charge / Current -> Time
/// Dividing a Charge by a Current returns a value of type Time
impl<T> std::ops::Div<Current<T>> for Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Time{s: self.C / rhs.A}
	}
}
/// Dividing a Charge by a Current returns a value of type Time
impl<T> std::ops::Div<Current<T>> for &Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Time{s: self.C.clone() / rhs.A}
	}
}
/// Dividing a Charge by a Current returns a value of type Time
impl<T> std::ops::Div<&Current<T>> for Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Time{s: self.C / rhs.A.clone()}
	}
}
/// Dividing a Charge by a Current returns a value of type Time
impl<T> std::ops::Div<&Current<T>> for &Charge<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Time{s: self.C.clone() / rhs.A.clone()}
	}
}

// Charge * Frequency -> Current
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> std::ops::Mul<Frequency<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Current{A: self.C * rhs.Hz}
	}
}
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> std::ops::Mul<Frequency<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Current{A: self.C.clone() * rhs.Hz}
	}
}
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> std::ops::Mul<&Frequency<T>> for Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Current{A: self.C * rhs.Hz.clone()}
	}
}
/// Multiplying a Charge by a Frequency returns a value of type Current
impl<T> std::ops::Mul<&Frequency<T>> for &Charge<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Current{A: self.C.clone() * rhs.Hz.clone()}
	}
}

// Charge * Voltage -> Energy
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> std::ops::Mul<Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Energy{J: self.C * rhs.V}
	}
}
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> std::ops::Mul<Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Energy{J: self.C.clone() * rhs.V}
	}
}
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> std::ops::Mul<&Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Energy{J: self.C * rhs.V.clone()}
	}
}
/// Multiplying a Charge by a Voltage returns a value of type Energy
impl<T> std::ops::Mul<&Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Energy{J: self.C.clone() * rhs.V.clone()}
	}
}

// Charge / Voltage -> Capacitance
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> std::ops::Div<Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Capacitance{F: self.C / rhs.V}
	}
}
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> std::ops::Div<Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Capacitance{F: self.C.clone() / rhs.V}
	}
}
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> std::ops::Div<&Voltage<T>> for Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Capacitance{F: self.C / rhs.V.clone()}
	}
}
/// Dividing a Charge by a Voltage returns a value of type Capacitance
impl<T> std::ops::Div<&Voltage<T>> for &Charge<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Capacitance{F: self.C.clone() / rhs.V.clone()}
	}
}

// Charge * Resistance -> MagneticFlux
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> std::ops::Mul<Resistance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C * rhs.Ohm}
	}
}
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> std::ops::Mul<Resistance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() * rhs.Ohm}
	}
}
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Resistance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C * rhs.Ohm.clone()}
	}
}
/// Multiplying a Charge by a Resistance returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Resistance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() * rhs.Ohm.clone()}
	}
}

// Charge / Conductance -> MagneticFlux
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> std::ops::Div<Conductance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C / rhs.S}
	}
}
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> std::ops::Div<Conductance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() / rhs.S}
	}
}
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> std::ops::Div<&Conductance<T>> for Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C / rhs.S.clone()}
	}
}
/// Dividing a Charge by a Conductance returns a value of type MagneticFlux
impl<T> std::ops::Div<&Conductance<T>> for &Charge<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.C.clone() / rhs.S.clone()}
	}
}

// Charge / Capacitance -> Voltage
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> std::ops::Div<Capacitance<T>> for Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Voltage{V: self.C / rhs.F}
	}
}
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> std::ops::Div<Capacitance<T>> for &Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Voltage{V: self.C.clone() / rhs.F}
	}
}
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> std::ops::Div<&Capacitance<T>> for Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Voltage{V: self.C / rhs.F.clone()}
	}
}
/// Dividing a Charge by a Capacitance returns a value of type Voltage
impl<T> std::ops::Div<&Capacitance<T>> for &Charge<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Voltage{V: self.C.clone() / rhs.F.clone()}
	}
}

// Charge / MagneticFlux -> Conductance
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> std::ops::Div<MagneticFlux<T>> for Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C / rhs.Wb}
	}
}
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> std::ops::Div<MagneticFlux<T>> for &Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C.clone() / rhs.Wb}
	}
}
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> std::ops::Div<&MagneticFlux<T>> for Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C / rhs.Wb.clone()}
	}
}
/// Dividing a Charge by a MagneticFlux returns a value of type Conductance
impl<T> std::ops::Div<&MagneticFlux<T>> for &Charge<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Conductance{S: self.C.clone() / rhs.Wb.clone()}
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
	pub fn unit_name() -> &'static str {
		return "volts";
	}
	
	/// Returns the abbreviated name or symbol of voltage: "V" for volts
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
	
}

// Voltage * Time -> MagneticFlux
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> std::ops::Mul<Time<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V * rhs.s}
	}
}
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> std::ops::Mul<Time<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() * rhs.s}
	}
}
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Time<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V * rhs.s.clone()}
	}
}
/// Multiplying a Voltage by a Time returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Time<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() * rhs.s.clone()}
	}
}

// Voltage * Current -> Power
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> std::ops::Mul<Current<T>> for Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Power{W: self.V * rhs.A}
	}
}
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> std::ops::Mul<Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Power{W: self.V.clone() * rhs.A}
	}
}
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> std::ops::Mul<&Current<T>> for Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Power{W: self.V * rhs.A.clone()}
	}
}
/// Multiplying a Voltage by a Current returns a value of type Power
impl<T> std::ops::Mul<&Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Power{W: self.V.clone() * rhs.A.clone()}
	}
}

// Voltage / Current -> Resistance
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> std::ops::Div<Current<T>> for Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Resistance{Ohm: self.V / rhs.A}
	}
}
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> std::ops::Div<Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Resistance{Ohm: self.V.clone() / rhs.A}
	}
}
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> std::ops::Div<&Current<T>> for Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Resistance{Ohm: self.V / rhs.A.clone()}
	}
}
/// Dividing a Voltage by a Current returns a value of type Resistance
impl<T> std::ops::Div<&Current<T>> for &Voltage<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Resistance{Ohm: self.V.clone() / rhs.A.clone()}
	}
}

// Voltage / Frequency -> MagneticFlux
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> std::ops::Div<Frequency<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V / rhs.Hz}
	}
}
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> std::ops::Div<Frequency<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() / rhs.Hz}
	}
}
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> std::ops::Div<&Frequency<T>> for Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V / rhs.Hz.clone()}
	}
}
/// Dividing a Voltage by a Frequency returns a value of type MagneticFlux
impl<T> std::ops::Div<&Frequency<T>> for &Voltage<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		MagneticFlux{Wb: self.V.clone() / rhs.Hz.clone()}
	}
}

// Voltage * Charge -> Energy
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> std::ops::Mul<Charge<T>> for Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Energy{J: self.V * rhs.C}
	}
}
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> std::ops::Mul<Charge<T>> for &Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		Energy{J: self.V.clone() * rhs.C}
	}
}
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> std::ops::Mul<&Charge<T>> for Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Energy{J: self.V * rhs.C.clone()}
	}
}
/// Multiplying a Voltage by a Charge returns a value of type Energy
impl<T> std::ops::Mul<&Charge<T>> for &Voltage<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		Energy{J: self.V.clone() * rhs.C.clone()}
	}
}

// Voltage / Resistance -> Current
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> std::ops::Div<Resistance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Current{A: self.V / rhs.Ohm}
	}
}
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> std::ops::Div<Resistance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Current{A: self.V.clone() / rhs.Ohm}
	}
}
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> std::ops::Div<&Resistance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Current{A: self.V / rhs.Ohm.clone()}
	}
}
/// Dividing a Voltage by a Resistance returns a value of type Current
impl<T> std::ops::Div<&Resistance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Current{A: self.V.clone() / rhs.Ohm.clone()}
	}
}

// Voltage * Conductance -> Current
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> std::ops::Mul<Conductance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Current{A: self.V * rhs.S}
	}
}
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> std::ops::Mul<Conductance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Current{A: self.V.clone() * rhs.S}
	}
}
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> std::ops::Mul<&Conductance<T>> for Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Current{A: self.V * rhs.S.clone()}
	}
}
/// Multiplying a Voltage by a Conductance returns a value of type Current
impl<T> std::ops::Mul<&Conductance<T>> for &Voltage<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Current{A: self.V.clone() * rhs.S.clone()}
	}
}

// Voltage * Capacitance -> Charge
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> std::ops::Mul<Capacitance<T>> for Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Charge{C: self.V * rhs.F}
	}
}
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> std::ops::Mul<Capacitance<T>> for &Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Charge{C: self.V.clone() * rhs.F}
	}
}
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> std::ops::Mul<&Capacitance<T>> for Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Charge{C: self.V * rhs.F.clone()}
	}
}
/// Multiplying a Voltage by a Capacitance returns a value of type Charge
impl<T> std::ops::Mul<&Capacitance<T>> for &Voltage<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Charge{C: self.V.clone() * rhs.F.clone()}
	}
}

// Voltage / MagneticFlux -> Frequency
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> std::ops::Div<MagneticFlux<T>> for Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V / rhs.Wb}
	}
}
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> std::ops::Div<MagneticFlux<T>> for &Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V.clone() / rhs.Wb}
	}
}
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> std::ops::Div<&MagneticFlux<T>> for Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V / rhs.Wb.clone()}
	}
}
/// Dividing a Voltage by a MagneticFlux returns a value of type Frequency
impl<T> std::ops::Div<&MagneticFlux<T>> for &Voltage<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Frequency{Hz: self.V.clone() / rhs.Wb.clone()}
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
	pub fn unit_name() -> &'static str {
		return "ohms";
	}
	
	/// Returns the abbreviated name or symbol of electrical resistance: "Ohm" for ohms
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
	
}

// Resistance * Time -> Inductance
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> std::ops::Mul<Time<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Inductance{H: self.Ohm * rhs.s}
	}
}
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> std::ops::Mul<Time<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() * rhs.s}
	}
}
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> std::ops::Mul<&Time<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Inductance{H: self.Ohm * rhs.s.clone()}
	}
}
/// Multiplying a Resistance by a Time returns a value of type Inductance
impl<T> std::ops::Mul<&Time<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() * rhs.s.clone()}
	}
}

// Resistance * Current -> Voltage
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> std::ops::Mul<Current<T>> for Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Voltage{V: self.Ohm * rhs.A}
	}
}
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> std::ops::Mul<Current<T>> for &Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Voltage{V: self.Ohm.clone() * rhs.A}
	}
}
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> std::ops::Mul<&Current<T>> for Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Voltage{V: self.Ohm * rhs.A.clone()}
	}
}
/// Multiplying a Resistance by a Current returns a value of type Voltage
impl<T> std::ops::Mul<&Current<T>> for &Resistance<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Voltage{V: self.Ohm.clone() * rhs.A.clone()}
	}
}

// Resistance / Frequency -> Inductance
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> std::ops::Div<Frequency<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm / rhs.Hz}
	}
}
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> std::ops::Div<Frequency<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() / rhs.Hz}
	}
}
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> std::ops::Div<&Frequency<T>> for Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm / rhs.Hz.clone()}
	}
}
/// Dividing a Resistance by a Frequency returns a value of type Inductance
impl<T> std::ops::Div<&Frequency<T>> for &Resistance<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Inductance{H: self.Ohm.clone() / rhs.Hz.clone()}
	}
}

// Resistance * Charge -> MagneticFlux
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> std::ops::Mul<Charge<T>> for Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm * rhs.C}
	}
}
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> std::ops::Mul<Charge<T>> for &Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm.clone() * rhs.C}
	}
}
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Charge<T>> for Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm * rhs.C.clone()}
	}
}
/// Multiplying a Resistance by a Charge returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Charge<T>> for &Resistance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Charge<T>) -> Self::Output {
		MagneticFlux{Wb: self.Ohm.clone() * rhs.C.clone()}
	}
}

// Resistance * Capacitance -> Time
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> std::ops::Mul<Capacitance<T>> for Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm * rhs.F}
	}
}
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> std::ops::Mul<Capacitance<T>> for &Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm.clone() * rhs.F}
	}
}
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> std::ops::Mul<&Capacitance<T>> for Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm * rhs.F.clone()}
	}
}
/// Multiplying a Resistance by a Capacitance returns a value of type Time
impl<T> std::ops::Mul<&Capacitance<T>> for &Resistance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Capacitance<T>) -> Self::Output {
		Time{s: self.Ohm.clone() * rhs.F.clone()}
	}
}

// Resistance / Inductance -> Frequency
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> std::ops::Div<Inductance<T>> for Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm / rhs.H}
	}
}
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> std::ops::Div<Inductance<T>> for &Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm.clone() / rhs.H}
	}
}
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> std::ops::Div<&Inductance<T>> for Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm / rhs.H.clone()}
	}
}
/// Dividing a Resistance by a Inductance returns a value of type Frequency
impl<T> std::ops::Div<&Inductance<T>> for &Resistance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Frequency{Hz: self.Ohm.clone() / rhs.H.clone()}
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
	pub fn unit_name() -> &'static str {
		return "siemens";
	}
	
	/// Returns the abbreviated name or symbol of electrical conductance: "S" for siemens
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
	
}

// Conductance * Time -> Capacitance
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> std::ops::Mul<Time<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Capacitance{F: self.S * rhs.s}
	}
}
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> std::ops::Mul<Time<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Capacitance{F: self.S.clone() * rhs.s}
	}
}
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> std::ops::Mul<&Time<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Capacitance{F: self.S * rhs.s.clone()}
	}
}
/// Multiplying a Conductance by a Time returns a value of type Capacitance
impl<T> std::ops::Mul<&Time<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Capacitance{F: self.S.clone() * rhs.s.clone()}
	}
}

// Conductance / Frequency -> Capacitance
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> std::ops::Div<Frequency<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Capacitance{F: self.S / rhs.Hz}
	}
}
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> std::ops::Div<Frequency<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Capacitance{F: self.S.clone() / rhs.Hz}
	}
}
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> std::ops::Div<&Frequency<T>> for Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Capacitance{F: self.S / rhs.Hz.clone()}
	}
}
/// Dividing a Conductance by a Frequency returns a value of type Capacitance
impl<T> std::ops::Div<&Frequency<T>> for &Conductance<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Capacitance{F: self.S.clone() / rhs.Hz.clone()}
	}
}

// Conductance * Voltage -> Current
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> std::ops::Mul<Voltage<T>> for Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Current{A: self.S * rhs.V}
	}
}
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> std::ops::Mul<Voltage<T>> for &Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Current{A: self.S.clone() * rhs.V}
	}
}
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> std::ops::Mul<&Voltage<T>> for Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Current{A: self.S * rhs.V.clone()}
	}
}
/// Multiplying a Conductance by a Voltage returns a value of type Current
impl<T> std::ops::Mul<&Voltage<T>> for &Conductance<T> where T: NumLike {
	type Output = Current<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Current{A: self.S.clone() * rhs.V.clone()}
	}
}

// Conductance / Capacitance -> Frequency
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> std::ops::Div<Capacitance<T>> for Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S / rhs.F}
	}
}
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> std::ops::Div<Capacitance<T>> for &Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S.clone() / rhs.F}
	}
}
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> std::ops::Div<&Capacitance<T>> for Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S / rhs.F.clone()}
	}
}
/// Dividing a Conductance by a Capacitance returns a value of type Frequency
impl<T> std::ops::Div<&Capacitance<T>> for &Conductance<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Frequency{Hz: self.S.clone() / rhs.F.clone()}
	}
}

// Conductance * Inductance -> Time
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> std::ops::Mul<Inductance<T>> for Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		Time{s: self.S * rhs.H}
	}
}
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> std::ops::Mul<Inductance<T>> for &Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		Time{s: self.S.clone() * rhs.H}
	}
}
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> std::ops::Mul<&Inductance<T>> for Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		Time{s: self.S * rhs.H.clone()}
	}
}
/// Multiplying a Conductance by a Inductance returns a value of type Time
impl<T> std::ops::Mul<&Inductance<T>> for &Conductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		Time{s: self.S.clone() * rhs.H.clone()}
	}
}

// Conductance * MagneticFlux -> Charge
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> std::ops::Mul<MagneticFlux<T>> for Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S * rhs.Wb}
	}
}
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> std::ops::Mul<MagneticFlux<T>> for &Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S.clone() * rhs.Wb}
	}
}
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> std::ops::Mul<&MagneticFlux<T>> for Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S * rhs.Wb.clone()}
	}
}
/// Multiplying a Conductance by a MagneticFlux returns a value of type Charge
impl<T> std::ops::Mul<&MagneticFlux<T>> for &Conductance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Charge{C: self.S.clone() * rhs.Wb.clone()}
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
	pub fn unit_name() -> &'static str {
		return "farads";
	}
	
	/// Returns the abbreviated name or symbol of electrical capacitance: "F" for farads
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
	
}

// Capacitance / Time -> Conductance
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> std::ops::Div<Time<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Conductance{S: self.F / rhs.s}
	}
}
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> std::ops::Div<Time<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Conductance{S: self.F.clone() / rhs.s}
	}
}
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> std::ops::Div<&Time<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Conductance{S: self.F / rhs.s.clone()}
	}
}
/// Dividing a Capacitance by a Time returns a value of type Conductance
impl<T> std::ops::Div<&Time<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Conductance{S: self.F.clone() / rhs.s.clone()}
	}
}

// Capacitance * Frequency -> Conductance
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> std::ops::Mul<Frequency<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Conductance{S: self.F * rhs.Hz}
	}
}
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> std::ops::Mul<Frequency<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Conductance{S: self.F.clone() * rhs.Hz}
	}
}
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> std::ops::Mul<&Frequency<T>> for Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Conductance{S: self.F * rhs.Hz.clone()}
	}
}
/// Multiplying a Capacitance by a Frequency returns a value of type Conductance
impl<T> std::ops::Mul<&Frequency<T>> for &Capacitance<T> where T: NumLike {
	type Output = Conductance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Conductance{S: self.F.clone() * rhs.Hz.clone()}
	}
}

// Capacitance * Voltage -> Charge
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> std::ops::Mul<Voltage<T>> for Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.F * rhs.V}
	}
}
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> std::ops::Mul<Voltage<T>> for &Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Charge{C: self.F.clone() * rhs.V}
	}
}
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> std::ops::Mul<&Voltage<T>> for Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.F * rhs.V.clone()}
	}
}
/// Multiplying a Capacitance by a Voltage returns a value of type Charge
impl<T> std::ops::Mul<&Voltage<T>> for &Capacitance<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Charge{C: self.F.clone() * rhs.V.clone()}
	}
}

// Capacitance * Resistance -> Time
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> std::ops::Mul<Resistance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.F * rhs.Ohm}
	}
}
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> std::ops::Mul<Resistance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.F.clone() * rhs.Ohm}
	}
}
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> std::ops::Mul<&Resistance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.F * rhs.Ohm.clone()}
	}
}
/// Multiplying a Capacitance by a Resistance returns a value of type Time
impl<T> std::ops::Mul<&Resistance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.F.clone() * rhs.Ohm.clone()}
	}
}

// Capacitance / Conductance -> Time
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> std::ops::Div<Conductance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.F / rhs.S}
	}
}
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> std::ops::Div<Conductance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.F.clone() / rhs.S}
	}
}
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> std::ops::Div<&Conductance<T>> for Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.F / rhs.S.clone()}
	}
}
/// Dividing a Capacitance by a Conductance returns a value of type Time
impl<T> std::ops::Div<&Conductance<T>> for &Capacitance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.F.clone() / rhs.S.clone()}
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
	pub fn unit_name() -> &'static str {
		return "henries";
	}
	
	/// Returns the abbreviated name or symbol of inductance: "H" for henries
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
	
}

// Inductance / Time -> Resistance
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> std::ops::Div<Time<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Resistance{Ohm: self.H / rhs.s}
	}
}
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> std::ops::Div<Time<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() / rhs.s}
	}
}
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> std::ops::Div<&Time<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Resistance{Ohm: self.H / rhs.s.clone()}
	}
}
/// Dividing a Inductance by a Time returns a value of type Resistance
impl<T> std::ops::Div<&Time<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() / rhs.s.clone()}
	}
}

// Inductance * Current -> MagneticFlux
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> std::ops::Mul<Current<T>> for Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H * rhs.A}
	}
}
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> std::ops::Mul<Current<T>> for &Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H.clone() * rhs.A}
	}
}
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Current<T>> for Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H * rhs.A.clone()}
	}
}
/// Multiplying a Inductance by a Current returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Current<T>> for &Inductance<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		MagneticFlux{Wb: self.H.clone() * rhs.A.clone()}
	}
}

// Inductance * Frequency -> Resistance
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> std::ops::Mul<Frequency<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H * rhs.Hz}
	}
}
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> std::ops::Mul<Frequency<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() * rhs.Hz}
	}
}
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> std::ops::Mul<&Frequency<T>> for Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H * rhs.Hz.clone()}
	}
}
/// Multiplying a Inductance by a Frequency returns a value of type Resistance
impl<T> std::ops::Mul<&Frequency<T>> for &Inductance<T> where T: NumLike {
	type Output = Resistance<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Resistance{Ohm: self.H.clone() * rhs.Hz.clone()}
	}
}

// Inductance / Resistance -> Time
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> std::ops::Div<Resistance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.H / rhs.Ohm}
	}
}
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> std::ops::Div<Resistance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Time{s: self.H.clone() / rhs.Ohm}
	}
}
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> std::ops::Div<&Resistance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.H / rhs.Ohm.clone()}
	}
}
/// Dividing a Inductance by a Resistance returns a value of type Time
impl<T> std::ops::Div<&Resistance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Time{s: self.H.clone() / rhs.Ohm.clone()}
	}
}

// Inductance * Conductance -> Time
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> std::ops::Mul<Conductance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.H * rhs.S}
	}
}
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> std::ops::Mul<Conductance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Time{s: self.H.clone() * rhs.S}
	}
}
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> std::ops::Mul<&Conductance<T>> for Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.H * rhs.S.clone()}
	}
}
/// Multiplying a Inductance by a Conductance returns a value of type Time
impl<T> std::ops::Mul<&Conductance<T>> for &Inductance<T> where T: NumLike {
	type Output = Time<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Time{s: self.H.clone() * rhs.S.clone()}
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
	pub fn unit_name() -> &'static str {
		return "webers";
	}
	
	/// Returns the abbreviated name or symbol of magnetic flux: "Wb" for webers
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
	
}

// MagneticFlux / Time -> Voltage
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> std::ops::Div<Time<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Voltage{V: self.Wb / rhs.s}
	}
}
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> std::ops::Div<Time<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() / rhs.s}
	}
}
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> std::ops::Div<&Time<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Voltage{V: self.Wb / rhs.s.clone()}
	}
}
/// Dividing a MagneticFlux by a Time returns a value of type Voltage
impl<T> std::ops::Div<&Time<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() / rhs.s.clone()}
	}
}

// MagneticFlux * Current -> Energy
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> std::ops::Mul<Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Energy{J: self.Wb * rhs.A}
	}
}
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> std::ops::Mul<Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Energy{J: self.Wb.clone() * rhs.A}
	}
}
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> std::ops::Mul<&Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Energy{J: self.Wb * rhs.A.clone()}
	}
}
/// Multiplying a MagneticFlux by a Current returns a value of type Energy
impl<T> std::ops::Mul<&Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Energy{J: self.Wb.clone() * rhs.A.clone()}
	}
}

// MagneticFlux / Current -> Inductance
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> std::ops::Div<Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Inductance{H: self.Wb / rhs.A}
	}
}
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> std::ops::Div<Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Current<T>) -> Self::Output {
		Inductance{H: self.Wb.clone() / rhs.A}
	}
}
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> std::ops::Div<&Current<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Inductance{H: self.Wb / rhs.A.clone()}
	}
}
/// Dividing a MagneticFlux by a Current returns a value of type Inductance
impl<T> std::ops::Div<&Current<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Current<T>) -> Self::Output {
		Inductance{H: self.Wb.clone() / rhs.A.clone()}
	}
}

// MagneticFlux * Frequency -> Voltage
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> std::ops::Mul<Frequency<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb * rhs.Hz}
	}
}
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> std::ops::Mul<Frequency<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() * rhs.Hz}
	}
}
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> std::ops::Mul<&Frequency<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb * rhs.Hz.clone()}
	}
}
/// Multiplying a MagneticFlux by a Frequency returns a value of type Voltage
impl<T> std::ops::Mul<&Frequency<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Voltage{V: self.Wb.clone() * rhs.Hz.clone()}
	}
}

// MagneticFlux / Area -> MagneticFluxDensity
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> std::ops::Div<Area<T>> for MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb / rhs.m2}
	}
}
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> std::ops::Div<Area<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb.clone() / rhs.m2}
	}
}
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> std::ops::Div<&Area<T>> for MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb / rhs.m2.clone()}
	}
}
/// Dividing a MagneticFlux by a Area returns a value of type MagneticFluxDensity
impl<T> std::ops::Div<&Area<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = MagneticFluxDensity<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		MagneticFluxDensity{T: self.Wb.clone() / rhs.m2.clone()}
	}
}

// MagneticFlux / Charge -> Resistance
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> std::ops::Div<Charge<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb / rhs.C}
	}
}
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> std::ops::Div<Charge<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb.clone() / rhs.C}
	}
}
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> std::ops::Div<&Charge<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb / rhs.C.clone()}
	}
}
/// Dividing a MagneticFlux by a Charge returns a value of type Resistance
impl<T> std::ops::Div<&Charge<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Resistance{Ohm: self.Wb.clone() / rhs.C.clone()}
	}
}

// MagneticFlux / Voltage -> Time
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> std::ops::Div<Voltage<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Time{s: self.Wb / rhs.V}
	}
}
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> std::ops::Div<Voltage<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Time{s: self.Wb.clone() / rhs.V}
	}
}
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> std::ops::Div<&Voltage<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Time{s: self.Wb / rhs.V.clone()}
	}
}
/// Dividing a MagneticFlux by a Voltage returns a value of type Time
impl<T> std::ops::Div<&Voltage<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Time{s: self.Wb.clone() / rhs.V.clone()}
	}
}

// MagneticFlux / Resistance -> Charge
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> std::ops::Div<Resistance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Charge{C: self.Wb / rhs.Ohm}
	}
}
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> std::ops::Div<Resistance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() / rhs.Ohm}
	}
}
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> std::ops::Div<&Resistance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Charge{C: self.Wb / rhs.Ohm.clone()}
	}
}
/// Dividing a MagneticFlux by a Resistance returns a value of type Charge
impl<T> std::ops::Div<&Resistance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() / rhs.Ohm.clone()}
	}
}

// MagneticFlux * Conductance -> Charge
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> std::ops::Mul<Conductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Charge{C: self.Wb * rhs.S}
	}
}
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> std::ops::Mul<Conductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() * rhs.S}
	}
}
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> std::ops::Mul<&Conductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Charge{C: self.Wb * rhs.S.clone()}
	}
}
/// Multiplying a MagneticFlux by a Conductance returns a value of type Charge
impl<T> std::ops::Mul<&Conductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Charge{C: self.Wb.clone() * rhs.S.clone()}
	}
}

// MagneticFlux / Inductance -> Current
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> std::ops::Div<Inductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Current{A: self.Wb / rhs.H}
	}
}
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> std::ops::Div<Inductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Current{A: self.Wb.clone() / rhs.H}
	}
}
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> std::ops::Div<&Inductance<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Current{A: self.Wb / rhs.H.clone()}
	}
}
/// Dividing a MagneticFlux by a Inductance returns a value of type Current
impl<T> std::ops::Div<&Inductance<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Current<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Current{A: self.Wb.clone() / rhs.H.clone()}
	}
}

// MagneticFlux / MagneticFluxDensity -> Area
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> std::ops::Div<MagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb / rhs.T}
	}
}
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> std::ops::Div<MagneticFluxDensity<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb.clone() / rhs.T}
	}
}
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> std::ops::Div<&MagneticFluxDensity<T>> for MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb / rhs.T.clone()}
	}
}
/// Dividing a MagneticFlux by a MagneticFluxDensity returns a value of type Area
impl<T> std::ops::Div<&MagneticFluxDensity<T>> for &MagneticFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &MagneticFluxDensity<T>) -> Self::Output {
		Area{m2: self.Wb.clone() / rhs.T.clone()}
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
	pub fn unit_name() -> &'static str {
		return "teslas";
	}
	
	/// Returns the abbreviated name or symbol of magnetic flux density: "T" for teslas
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
	
}

// MagneticFluxDensity * Area -> MagneticFlux
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> std::ops::Mul<Area<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T * rhs.m2}
	}
}
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> std::ops::Mul<Area<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T.clone() * rhs.m2}
	}
}
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Area<T>> for MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T * rhs.m2.clone()}
	}
}
/// Multiplying a MagneticFluxDensity by a Area returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Area<T>> for &MagneticFluxDensity<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		MagneticFlux{Wb: self.T.clone() * rhs.m2.clone()}
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
	pub fn unit_name() -> &'static str {
		return "lumens";
	}
	
	/// Returns the abbreviated name or symbol of luminous flux: "lm" for lumens
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
	
}

// LuminousFlux / Luminosity -> SolidAngle
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> std::ops::Div<Luminosity<T>> for LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm / rhs.cd}
	}
}
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> std::ops::Div<Luminosity<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm.clone() / rhs.cd}
	}
}
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> std::ops::Div<&Luminosity<T>> for LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm / rhs.cd.clone()}
	}
}
/// Dividing a LuminousFlux by a Luminosity returns a value of type SolidAngle
impl<T> std::ops::Div<&Luminosity<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = SolidAngle<T>;
	fn div(self, rhs: &Luminosity<T>) -> Self::Output {
		SolidAngle{sr: self.lm.clone() / rhs.cd.clone()}
	}
}

// LuminousFlux / SolidAngle -> Luminosity
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> std::ops::Div<SolidAngle<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm / rhs.sr}
	}
}
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> std::ops::Div<SolidAngle<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm.clone() / rhs.sr}
	}
}
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> std::ops::Div<&SolidAngle<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm / rhs.sr.clone()}
	}
}
/// Dividing a LuminousFlux by a SolidAngle returns a value of type Luminosity
impl<T> std::ops::Div<&SolidAngle<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Luminosity<T>;
	fn div(self, rhs: &SolidAngle<T>) -> Self::Output {
		Luminosity{cd: self.lm.clone() / rhs.sr.clone()}
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
	pub fn unit_name() -> &'static str {
		return "lux";
	}
	
	/// Returns the abbreviated name or symbol of illuminance: "lux" for lux
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


