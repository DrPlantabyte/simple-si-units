
//! This module provides electromagnetic SI units, such as electrical capacitance 
//! and magnetic flux.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::base::*;
use super::geometry::*;
use super::mechanical::*;

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num-bigfloat")]
use num_bigfloat;
#[cfg(feature="num-complex")]
use num_complex;


/// The electrical capacitance unit type, defined as farads in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Capacitance<T: NumLike>{
	/// The value of this Electrical capacitance in farads
	pub F: T
}

impl<T> Capacitance<T> where T: NumLike {

	/// Returns the standard unit name of electrical capacitance: "farads"
	pub fn unit_name() -> &'static str { "farads" }
	
	/// Returns the abbreviated name or symbol of electrical capacitance: "F" for farads
	pub fn unit_symbol() -> &'static str { "F" }
	
	/// Returns a new electrical capacitance value from the given number of farads
	///
	/// # Arguments
	/// * `F` - Any number-like type, representing a quantity of farads
	pub fn from_F(F: T) -> Self { Capacitance{F: F} }
	
	/// Returns a copy of this electrical capacitance value in farads
	pub fn to_F(&self) -> T { self.F.clone() }

	/// Returns a new electrical capacitance value from the given number of farads
	///
	/// # Arguments
	/// * `farads` - Any number-like type, representing a quantity of farads
	pub fn from_farads(farads: T) -> Self { Capacitance{F: farads} }
	
	/// Returns a copy of this electrical capacitance value in farads
	pub fn to_farads(&self) -> T { self.F.clone() }

}

impl<T> fmt::Display for Capacitance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.F, Self::unit_symbol())
	}
}

impl<T> Capacitance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical capacitance value in millifarads
	pub fn to_mF(&self) -> T {
		return self.F.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of millifarads
	///
	/// # Arguments
	/// * `mF` - Any number-like type, representing a quantity of millifarads
	pub fn from_mF(mF: T) -> Self {
		Capacitance{F: mF * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical capacitance value in microfarads
	pub fn to_uF(&self) -> T {
		return self.F.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of microfarads
	///
	/// # Arguments
	/// * `uF` - Any number-like type, representing a quantity of microfarads
	pub fn from_uF(uF: T) -> Self {
		Capacitance{F: uF * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical capacitance value in nanofarads
	pub fn to_nF(&self) -> T {
		return self.F.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of nanofarads
	///
	/// # Arguments
	/// * `nF` - Any number-like type, representing a quantity of nanofarads
	pub fn from_nF(nF: T) -> Self {
		Capacitance{F: nF * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical capacitance value in picofarads
	pub fn to_pF(&self) -> T {
		return self.F.clone() * T::from(1000000000000.0_f64);
	}

	/// Returns a new electrical capacitance value from the given number of picofarads
	///
	/// # Arguments
	/// * `pF` - Any number-like type, representing a quantity of picofarads
	pub fn from_pF(pF: T) -> Self {
		Capacitance{F: pF * T::from(1e-12_f64)}
	}

	/// Returns a copy of this electrical capacitance value in kilofarads
	pub fn to_kF(&self) -> T {
		return self.F.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical capacitance value from the given number of kilofarads
	///
	/// # Arguments
	/// * `kF` - Any number-like type, representing a quantity of kilofarads
	pub fn from_kF(kF: T) -> Self {
		Capacitance{F: kF * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical capacitance value in megafarads
	pub fn to_MF(&self) -> T {
		return self.F.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical capacitance value from the given number of megafarads
	///
	/// # Arguments
	/// * `MF` - Any number-like type, representing a quantity of megafarads
	pub fn from_MF(MF: T) -> Self {
		Capacitance{F: MF * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical capacitance value in gigafarads
	pub fn to_GF(&self) -> T {
		return self.F.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical capacitance value from the given number of gigafarads
	///
	/// # Arguments
	/// * `GF` - Any number-like type, representing a quantity of gigafarads
	pub fn from_GF(GF: T) -> Self {
		Capacitance{F: GF * T::from(1000000000.0_f64)}
	}

}


/// Converts a Capacitance into the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Capacitance> for Capacitance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Capacitance {
		uom::si::f32::Capacitance::new::<uom::si::capacitance::farad>(self.F.into())
	}
}

/// Creates a Capacitance from the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Capacitance> for Capacitance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Capacitance) -> Self {
		Capacitance{F: T::from(src.value)}
	}
}

/// Converts a Capacitance into the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Capacitance> for Capacitance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Capacitance {
		uom::si::f64::Capacitance::new::<uom::si::capacitance::farad>(self.F.into())
	}
}

/// Creates a Capacitance from the equivalent [uom](https://crates.io/crates/uom) type [Capacitance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Capacitance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Capacitance> for Capacitance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Capacitance) -> Self {
		Capacitance{F: T::from(src.value)}
	}
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

/// The electric charge (aka coulombs) unit type, defined as coulombs in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Charge<T: NumLike>{
	/// The value of this Electric charge in coulombs
	pub C: T
}

impl<T> Charge<T> where T: NumLike {

	/// Returns the standard unit name of electric charge: "coulombs"
	pub fn unit_name() -> &'static str { "coulombs" }
	
	/// Returns the abbreviated name or symbol of electric charge: "C" for coulombs
	pub fn unit_symbol() -> &'static str { "C" }
	
	/// Returns a new electric charge value from the given number of coulombs
	///
	/// # Arguments
	/// * `C` - Any number-like type, representing a quantity of coulombs
	pub fn from_C(C: T) -> Self { Charge{C: C} }
	
	/// Returns a copy of this electric charge value in coulombs
	pub fn to_C(&self) -> T { self.C.clone() }

	/// Returns a new electric charge value from the given number of coulombs
	///
	/// # Arguments
	/// * `coulombs` - Any number-like type, representing a quantity of coulombs
	pub fn from_coulombs(coulombs: T) -> Self { Charge{C: coulombs} }
	
	/// Returns a copy of this electric charge value in coulombs
	pub fn to_coulombs(&self) -> T { self.C.clone() }

}

impl<T> fmt::Display for Charge<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.C, Self::unit_symbol())
	}
}

impl<T> Charge<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electric charge value in millicoulombs
	pub fn to_mC(&self) -> T {
		return self.C.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electric charge value from the given number of millicoulombs
	///
	/// # Arguments
	/// * `mC` - Any number-like type, representing a quantity of millicoulombs
	pub fn from_mC(mC: T) -> Self {
		Charge{C: mC * T::from(0.001_f64)}
	}

	/// Returns a copy of this electric charge value in microcoulombs
	pub fn to_uC(&self) -> T {
		return self.C.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electric charge value from the given number of microcoulombs
	///
	/// # Arguments
	/// * `uC` - Any number-like type, representing a quantity of microcoulombs
	pub fn from_uC(uC: T) -> Self {
		Charge{C: uC * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electric charge value in nanocoulombs
	pub fn to_nC(&self) -> T {
		return self.C.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electric charge value from the given number of nanocoulombs
	///
	/// # Arguments
	/// * `nC` - Any number-like type, representing a quantity of nanocoulombs
	pub fn from_nC(nC: T) -> Self {
		Charge{C: nC * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electric charge value in kilocoulombs
	pub fn to_kC(&self) -> T {
		return self.C.clone() * T::from(0.001_f64);
	}

	/// Returns a new electric charge value from the given number of kilocoulombs
	///
	/// # Arguments
	/// * `kC` - Any number-like type, representing a quantity of kilocoulombs
	pub fn from_kC(kC: T) -> Self {
		Charge{C: kC * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electric charge value in megacoulombs
	pub fn to_MC(&self) -> T {
		return self.C.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electric charge value from the given number of megacoulombs
	///
	/// # Arguments
	/// * `MC` - Any number-like type, representing a quantity of megacoulombs
	pub fn from_MC(MC: T) -> Self {
		Charge{C: MC * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electric charge value in gigacoulombs
	pub fn to_GC(&self) -> T {
		return self.C.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electric charge value from the given number of gigacoulombs
	///
	/// # Arguments
	/// * `GC` - Any number-like type, representing a quantity of gigacoulombs
	pub fn from_GC(GC: T) -> Self {
		Charge{C: GC * T::from(1000000000.0_f64)}
	}

	/// Returns a copy of this electric charge value in proton
	pub fn to_p(&self) -> T {
		return self.C.clone() * T::from(6.24150907446076e+18_f64);
	}

	/// Returns a new electric charge value from the given number of proton
	///
	/// # Arguments
	/// * `p` - Any number-like type, representing a quantity of proton
	pub fn from_p(p: T) -> Self {
		Charge{C: p * T::from(1.6021766340000001e-19_f64)}
	}

	/// Returns a copy of this electric charge value in electron
	pub fn to_e(&self) -> T {
		return self.C.clone() * T::from(-6.24150907446076e+18_f64);
	}

	/// Returns a new electric charge value from the given number of electron
	///
	/// # Arguments
	/// * `e` - Any number-like type, representing a quantity of electron
	pub fn from_e(e: T) -> Self {
		Charge{C: e * T::from(-1.6021766340000001e-19_f64)}
	}

}


/// Converts a Charge into the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricCharge> for Charge<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricCharge {
		uom::si::f32::ElectricCharge::new::<uom::si::electric_charge::coulomb>(self.C.into())
	}
}

/// Creates a Charge from the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricCharge> for Charge<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricCharge) -> Self {
		Charge{C: T::from(src.value)}
	}
}

/// Converts a Charge into the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricCharge> for Charge<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricCharge {
		uom::si::f64::ElectricCharge::new::<uom::si::electric_charge::coulomb>(self.C.into())
	}
}

/// Creates a Charge from the equivalent [uom](https://crates.io/crates/uom) type [ElectricCharge](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricCharge.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricCharge> for Charge<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricCharge) -> Self {
		Charge{C: T::from(src.value)}
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

/// The electrical conductance unit type, defined as siemens in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Conductance<T: NumLike>{
	/// The value of this Electrical conductance in siemens
	pub S: T
}

impl<T> Conductance<T> where T: NumLike {

	/// Returns the standard unit name of electrical conductance: "siemens"
	pub fn unit_name() -> &'static str { "siemens" }
	
	/// Returns the abbreviated name or symbol of electrical conductance: "S" for siemens
	pub fn unit_symbol() -> &'static str { "S" }
	
	/// Returns a new electrical conductance value from the given number of siemens
	///
	/// # Arguments
	/// * `S` - Any number-like type, representing a quantity of siemens
	pub fn from_S(S: T) -> Self { Conductance{S: S} }
	
	/// Returns a copy of this electrical conductance value in siemens
	pub fn to_S(&self) -> T { self.S.clone() }

	/// Returns a new electrical conductance value from the given number of siemens
	///
	/// # Arguments
	/// * `siemens` - Any number-like type, representing a quantity of siemens
	pub fn from_siemens(siemens: T) -> Self { Conductance{S: siemens} }
	
	/// Returns a copy of this electrical conductance value in siemens
	pub fn to_siemens(&self) -> T { self.S.clone() }

}

impl<T> fmt::Display for Conductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.S, Self::unit_symbol())
	}
}

impl<T> Conductance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical conductance value in millisiemens
	pub fn to_mS(&self) -> T {
		return self.S.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical conductance value from the given number of millisiemens
	///
	/// # Arguments
	/// * `mS` - Any number-like type, representing a quantity of millisiemens
	pub fn from_mS(mS: T) -> Self {
		Conductance{S: mS * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical conductance value in microsiemens
	pub fn to_uS(&self) -> T {
		return self.S.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical conductance value from the given number of microsiemens
	///
	/// # Arguments
	/// * `uS` - Any number-like type, representing a quantity of microsiemens
	pub fn from_uS(uS: T) -> Self {
		Conductance{S: uS * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical conductance value in nanosiemens
	pub fn to_nS(&self) -> T {
		return self.S.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical conductance value from the given number of nanosiemens
	///
	/// # Arguments
	/// * `nS` - Any number-like type, representing a quantity of nanosiemens
	pub fn from_nS(nS: T) -> Self {
		Conductance{S: nS * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical conductance value in kilosiemens
	pub fn to_kS(&self) -> T {
		return self.S.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical conductance value from the given number of kilosiemens
	///
	/// # Arguments
	/// * `kS` - Any number-like type, representing a quantity of kilosiemens
	pub fn from_kS(kS: T) -> Self {
		Conductance{S: kS * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical conductance value in megasiemens
	pub fn to_MS(&self) -> T {
		return self.S.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical conductance value from the given number of megasiemens
	///
	/// # Arguments
	/// * `MS` - Any number-like type, representing a quantity of megasiemens
	pub fn from_MS(MS: T) -> Self {
		Conductance{S: MS * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical conductance value in gigasiemens
	pub fn to_GS(&self) -> T {
		return self.S.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical conductance value from the given number of gigasiemens
	///
	/// # Arguments
	/// * `GS` - Any number-like type, representing a quantity of gigasiemens
	pub fn from_GS(GS: T) -> Self {
		Conductance{S: GS * T::from(1000000000.0_f64)}
	}

}


/// Converts a Conductance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricalConductance> for Conductance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricalConductance {
		uom::si::f32::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(self.S.into())
	}
}

/// Creates a Conductance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricalConductance> for Conductance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricalConductance) -> Self {
		Conductance{S: T::from(src.value)}
	}
}

/// Converts a Conductance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricalConductance> for Conductance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricalConductance {
		uom::si::f64::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(self.S.into())
	}
}

/// Creates a Conductance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalConductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalConductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricalConductance> for Conductance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricalConductance) -> Self {
		Conductance{S: T::from(src.value)}
	}
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

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<Conductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
impl<T> std::ops::Div<&Conductance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<Conductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<Conductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<&Conductance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<&Conductance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Conductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Conductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Conductance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Conductance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
	}
}

// 1/Conductance -> Resistance
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Conductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Conductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Conductance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self) / rhs.S.clone()}
	}
}
/// Dividing a scalar value by a Conductance unit value returns a value of type Resistance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Conductance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Resistance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Resistance{Ohm: T::from(self.clone()) / rhs.S.clone()}
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
	pub fn unit_name() -> &'static str { "lux" }
	
	/// Returns the abbreviated name or symbol of illuminance: "lux" for lux
	pub fn unit_symbol() -> &'static str { "lux" }
	
	/// Returns a new illuminance value from the given number of lux
	///
	/// # Arguments
	/// * `lux` - Any number-like type, representing a quantity of lux
	pub fn from_lux(lux: T) -> Self { Illuminance{lux: lux} }
	
	/// Returns a copy of this illuminance value in lux
	pub fn to_lux(&self) -> T { self.lux.clone() }

}

impl<T> fmt::Display for Illuminance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.lux, Self::unit_symbol())
	}
}

impl<T> Illuminance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this illuminance value in millilux
	pub fn to_mlux(&self) -> T {
		return self.lux.clone() * T::from(1000.0_f64);
	}

	/// Returns a new illuminance value from the given number of millilux
	///
	/// # Arguments
	/// * `mlux` - Any number-like type, representing a quantity of millilux
	pub fn from_mlux(mlux: T) -> Self {
		Illuminance{lux: mlux * T::from(0.001_f64)}
	}

	/// Returns a copy of this illuminance value in microlux
	pub fn to_ulux(&self) -> T {
		return self.lux.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new illuminance value from the given number of microlux
	///
	/// # Arguments
	/// * `ulux` - Any number-like type, representing a quantity of microlux
	pub fn from_ulux(ulux: T) -> Self {
		Illuminance{lux: ulux * T::from(1e-06_f64)}
	}

	/// Returns a copy of this illuminance value in nanolux
	pub fn to_nlux(&self) -> T {
		return self.lux.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new illuminance value from the given number of nanolux
	///
	/// # Arguments
	/// * `nlux` - Any number-like type, representing a quantity of nanolux
	pub fn from_nlux(nlux: T) -> Self {
		Illuminance{lux: nlux * T::from(1e-09_f64)}
	}

	/// Returns a copy of this illuminance value in kilolux
	pub fn to_klux(&self) -> T {
		return self.lux.clone() * T::from(0.001_f64);
	}

	/// Returns a new illuminance value from the given number of kilolux
	///
	/// # Arguments
	/// * `klux` - Any number-like type, representing a quantity of kilolux
	pub fn from_klux(klux: T) -> Self {
		Illuminance{lux: klux * T::from(1000.0_f64)}
	}

	/// Returns a copy of this illuminance value in megalux
	pub fn to_Mlux(&self) -> T {
		return self.lux.clone() * T::from(1e-06_f64);
	}

	/// Returns a new illuminance value from the given number of megalux
	///
	/// # Arguments
	/// * `Mlux` - Any number-like type, representing a quantity of megalux
	pub fn from_Mlux(Mlux: T) -> Self {
		Illuminance{lux: Mlux * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this illuminance value in gigalux
	pub fn to_Glux(&self) -> T {
		return self.lux.clone() * T::from(1e-09_f64);
	}

	/// Returns a new illuminance value from the given number of gigalux
	///
	/// # Arguments
	/// * `Glux` - Any number-like type, representing a quantity of gigalux
	pub fn from_Glux(Glux: T) -> Self {
		Illuminance{lux: Glux * T::from(1000000000.0_f64)}
	}

}


/// Converts a Illuminance into the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Luminance> for Illuminance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Luminance {
		uom::si::f32::Luminance::new::<uom::si::luminance::candela_per_square_meter>(self.lux.into())
	}
}

/// Creates a Illuminance from the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Luminance> for Illuminance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Luminance) -> Self {
		Illuminance{lux: T::from(src.value)}
	}
}

/// Converts a Illuminance into the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Luminance> for Illuminance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Luminance {
		uom::si::f64::Luminance::new::<uom::si::luminance::candela_per_square_meter>(self.lux.into())
	}
}

/// Creates a Illuminance from the equivalent [uom](https://crates.io/crates/uom) type [Luminance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Luminance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Luminance> for Illuminance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Luminance) -> Self {
		Illuminance{lux: T::from(src.value)}
	}
}


// Illuminance * Area -> LuminousFlux
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> std::ops::Mul<Area<T>> for Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux * rhs.m2}
	}
}
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> std::ops::Mul<Area<T>> for &Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux.clone() * rhs.m2}
	}
}
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> std::ops::Mul<&Area<T>> for Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux * rhs.m2.clone()}
	}
}
/// Multiplying a Illuminance by a Area returns a value of type LuminousFlux
impl<T> std::ops::Mul<&Area<T>> for &Illuminance<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		LuminousFlux{lm: self.lux.clone() * rhs.m2.clone()}
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
	pub fn unit_name() -> &'static str { "henries" }
	
	/// Returns the abbreviated name or symbol of inductance: "H" for henries
	pub fn unit_symbol() -> &'static str { "H" }
	
	/// Returns a new inductance value from the given number of henries
	///
	/// # Arguments
	/// * `H` - Any number-like type, representing a quantity of henries
	pub fn from_H(H: T) -> Self { Inductance{H: H} }
	
	/// Returns a copy of this inductance value in henries
	pub fn to_H(&self) -> T { self.H.clone() }

	/// Returns a new inductance value from the given number of henries
	///
	/// # Arguments
	/// * `henries` - Any number-like type, representing a quantity of henries
	pub fn from_henries(henries: T) -> Self { Inductance{H: henries} }
	
	/// Returns a copy of this inductance value in henries
	pub fn to_henries(&self) -> T { self.H.clone() }

}

impl<T> fmt::Display for Inductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.H, Self::unit_symbol())
	}
}

impl<T> Inductance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this inductance value in millihenries
	pub fn to_mH(&self) -> T {
		return self.H.clone() * T::from(1000.0_f64);
	}

	/// Returns a new inductance value from the given number of millihenries
	///
	/// # Arguments
	/// * `mH` - Any number-like type, representing a quantity of millihenries
	pub fn from_mH(mH: T) -> Self {
		Inductance{H: mH * T::from(0.001_f64)}
	}

	/// Returns a copy of this inductance value in microhenries
	pub fn to_uH(&self) -> T {
		return self.H.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new inductance value from the given number of microhenries
	///
	/// # Arguments
	/// * `uH` - Any number-like type, representing a quantity of microhenries
	pub fn from_uH(uH: T) -> Self {
		Inductance{H: uH * T::from(1e-06_f64)}
	}

	/// Returns a copy of this inductance value in nanohenries
	pub fn to_nH(&self) -> T {
		return self.H.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new inductance value from the given number of nanohenries
	///
	/// # Arguments
	/// * `nH` - Any number-like type, representing a quantity of nanohenries
	pub fn from_nH(nH: T) -> Self {
		Inductance{H: nH * T::from(1e-09_f64)}
	}

	/// Returns a copy of this inductance value in kilohenries
	pub fn to_kH(&self) -> T {
		return self.H.clone() * T::from(0.001_f64);
	}

	/// Returns a new inductance value from the given number of kilohenries
	///
	/// # Arguments
	/// * `kH` - Any number-like type, representing a quantity of kilohenries
	pub fn from_kH(kH: T) -> Self {
		Inductance{H: kH * T::from(1000.0_f64)}
	}

	/// Returns a copy of this inductance value in megahenries
	pub fn to_MH(&self) -> T {
		return self.H.clone() * T::from(1e-06_f64);
	}

	/// Returns a new inductance value from the given number of megahenries
	///
	/// # Arguments
	/// * `MH` - Any number-like type, representing a quantity of megahenries
	pub fn from_MH(MH: T) -> Self {
		Inductance{H: MH * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this inductance value in gigahenries
	pub fn to_GH(&self) -> T {
		return self.H.clone() * T::from(1e-09_f64);
	}

	/// Returns a new inductance value from the given number of gigahenries
	///
	/// # Arguments
	/// * `GH` - Any number-like type, representing a quantity of gigahenries
	pub fn from_GH(GH: T) -> Self {
		Inductance{H: GH * T::from(1000000000.0_f64)}
	}

}


/// Converts a Inductance into the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::Inductance> for Inductance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::Inductance {
		uom::si::f32::Inductance::new::<uom::si::inductance::henry>(self.H.into())
	}
}

/// Creates a Inductance from the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f32/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::Inductance> for Inductance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::Inductance) -> Self {
		Inductance{H: T::from(src.value)}
	}
}

/// Converts a Inductance into the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::Inductance> for Inductance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::Inductance {
		uom::si::f64::Inductance::new::<uom::si::inductance::henry>(self.H.into())
	}
}

/// Creates a Inductance from the equivalent [uom](https://crates.io/crates/uom) type [Inductance](https://docs.rs/uom/0.34.0/uom/si/f64/type.Inductance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::Inductance> for Inductance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::Inductance) -> Self {
		Inductance{H: T::from(src.value)}
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

/// The luminous flux unit type, defined as lumens in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct LuminousFlux<T: NumLike>{
	/// The value of this Luminous flux in lumens
	pub lm: T
}

impl<T> LuminousFlux<T> where T: NumLike {

	/// Returns the standard unit name of luminous flux: "lumens"
	pub fn unit_name() -> &'static str { "lumens" }
	
	/// Returns the abbreviated name or symbol of luminous flux: "lm" for lumens
	pub fn unit_symbol() -> &'static str { "lm" }
	
	/// Returns a new luminous flux value from the given number of lumens
	///
	/// # Arguments
	/// * `lm` - Any number-like type, representing a quantity of lumens
	pub fn from_lm(lm: T) -> Self { LuminousFlux{lm: lm} }
	
	/// Returns a copy of this luminous flux value in lumens
	pub fn to_lm(&self) -> T { self.lm.clone() }

	/// Returns a new luminous flux value from the given number of lumens
	///
	/// # Arguments
	/// * `lumens` - Any number-like type, representing a quantity of lumens
	pub fn from_lumens(lumens: T) -> Self { LuminousFlux{lm: lumens} }
	
	/// Returns a copy of this luminous flux value in lumens
	pub fn to_lumens(&self) -> T { self.lm.clone() }

}

impl<T> fmt::Display for LuminousFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.lm, Self::unit_symbol())
	}
}

impl<T> LuminousFlux<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this luminous flux value in millilumens
	pub fn to_mlm(&self) -> T {
		return self.lm.clone() * T::from(1000.0_f64);
	}

	/// Returns a new luminous flux value from the given number of millilumens
	///
	/// # Arguments
	/// * `mlm` - Any number-like type, representing a quantity of millilumens
	pub fn from_mlm(mlm: T) -> Self {
		LuminousFlux{lm: mlm * T::from(0.001_f64)}
	}

	/// Returns a copy of this luminous flux value in microlumens
	pub fn to_ulm(&self) -> T {
		return self.lm.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new luminous flux value from the given number of microlumens
	///
	/// # Arguments
	/// * `ulm` - Any number-like type, representing a quantity of microlumens
	pub fn from_ulm(ulm: T) -> Self {
		LuminousFlux{lm: ulm * T::from(1e-06_f64)}
	}

	/// Returns a copy of this luminous flux value in nanolumens
	pub fn to_nlm(&self) -> T {
		return self.lm.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new luminous flux value from the given number of nanolumens
	///
	/// # Arguments
	/// * `nlm` - Any number-like type, representing a quantity of nanolumens
	pub fn from_nlm(nlm: T) -> Self {
		LuminousFlux{lm: nlm * T::from(1e-09_f64)}
	}

	/// Returns a copy of this luminous flux value in kilolumens
	pub fn to_klm(&self) -> T {
		return self.lm.clone() * T::from(0.001_f64);
	}

	/// Returns a new luminous flux value from the given number of kilolumens
	///
	/// # Arguments
	/// * `klm` - Any number-like type, representing a quantity of kilolumens
	pub fn from_klm(klm: T) -> Self {
		LuminousFlux{lm: klm * T::from(1000.0_f64)}
	}

	/// Returns a copy of this luminous flux value in megalumens
	pub fn to_Mlm(&self) -> T {
		return self.lm.clone() * T::from(1e-06_f64);
	}

	/// Returns a new luminous flux value from the given number of megalumens
	///
	/// # Arguments
	/// * `Mlm` - Any number-like type, representing a quantity of megalumens
	pub fn from_Mlm(Mlm: T) -> Self {
		LuminousFlux{lm: Mlm * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this luminous flux value in gigalumens
	pub fn to_Glm(&self) -> T {
		return self.lm.clone() * T::from(1e-09_f64);
	}

	/// Returns a new luminous flux value from the given number of gigalumens
	///
	/// # Arguments
	/// * `Glm` - Any number-like type, representing a quantity of gigalumens
	pub fn from_Glm(Glm: T) -> Self {
		LuminousFlux{lm: Glm * T::from(1000000000.0_f64)}
	}

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

// LuminousFlux / Illuminance -> Area
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> std::ops::Div<Illuminance<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		Area{m2: self.lm / rhs.lux}
	}
}
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> std::ops::Div<Illuminance<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: Illuminance<T>) -> Self::Output {
		Area{m2: self.lm.clone() / rhs.lux}
	}
}
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> std::ops::Div<&Illuminance<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		Area{m2: self.lm / rhs.lux.clone()}
	}
}
/// Dividing a LuminousFlux by a Illuminance returns a value of type Area
impl<T> std::ops::Div<&Illuminance<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Area<T>;
	fn div(self, rhs: &Illuminance<T>) -> Self::Output {
		Area{m2: self.lm.clone() / rhs.lux.clone()}
	}
}

// LuminousFlux / Area -> Illuminance
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> std::ops::Div<Area<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Illuminance{lux: self.lm / rhs.m2}
	}
}
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> std::ops::Div<Area<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: Area<T>) -> Self::Output {
		Illuminance{lux: self.lm.clone() / rhs.m2}
	}
}
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> std::ops::Div<&Area<T>> for LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Illuminance{lux: self.lm / rhs.m2.clone()}
	}
}
/// Dividing a LuminousFlux by a Area returns a value of type Illuminance
impl<T> std::ops::Div<&Area<T>> for &LuminousFlux<T> where T: NumLike {
	type Output = Illuminance<T>;
	fn div(self, rhs: &Area<T>) -> Self::Output {
		Illuminance{lux: self.lm.clone() / rhs.m2.clone()}
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

/// The magnetic flux unit type, defined as webers in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MagneticFlux<T: NumLike>{
	/// The value of this Magnetic flux in webers
	pub Wb: T
}

impl<T> MagneticFlux<T> where T: NumLike {

	/// Returns the standard unit name of magnetic flux: "webers"
	pub fn unit_name() -> &'static str { "webers" }
	
	/// Returns the abbreviated name or symbol of magnetic flux: "Wb" for webers
	pub fn unit_symbol() -> &'static str { "Wb" }
	
	/// Returns a new magnetic flux value from the given number of webers
	///
	/// # Arguments
	/// * `Wb` - Any number-like type, representing a quantity of webers
	pub fn from_Wb(Wb: T) -> Self { MagneticFlux{Wb: Wb} }
	
	/// Returns a copy of this magnetic flux value in webers
	pub fn to_Wb(&self) -> T { self.Wb.clone() }

	/// Returns a new magnetic flux value from the given number of webers
	///
	/// # Arguments
	/// * `webers` - Any number-like type, representing a quantity of webers
	pub fn from_webers(webers: T) -> Self { MagneticFlux{Wb: webers} }
	
	/// Returns a copy of this magnetic flux value in webers
	pub fn to_webers(&self) -> T { self.Wb.clone() }

}

impl<T> fmt::Display for MagneticFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Wb, Self::unit_symbol())
	}
}

impl<T> MagneticFlux<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this magnetic flux value in milliwebers
	pub fn to_mWb(&self) -> T {
		return self.Wb.clone() * T::from(1000.0_f64);
	}

	/// Returns a new magnetic flux value from the given number of milliwebers
	///
	/// # Arguments
	/// * `mWb` - Any number-like type, representing a quantity of milliwebers
	pub fn from_mWb(mWb: T) -> Self {
		MagneticFlux{Wb: mWb * T::from(0.001_f64)}
	}

	/// Returns a copy of this magnetic flux value in microwebers
	pub fn to_uWb(&self) -> T {
		return self.Wb.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new magnetic flux value from the given number of microwebers
	///
	/// # Arguments
	/// * `uWb` - Any number-like type, representing a quantity of microwebers
	pub fn from_uWb(uWb: T) -> Self {
		MagneticFlux{Wb: uWb * T::from(1e-06_f64)}
	}

	/// Returns a copy of this magnetic flux value in nanowebers
	pub fn to_nWb(&self) -> T {
		return self.Wb.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new magnetic flux value from the given number of nanowebers
	///
	/// # Arguments
	/// * `nWb` - Any number-like type, representing a quantity of nanowebers
	pub fn from_nWb(nWb: T) -> Self {
		MagneticFlux{Wb: nWb * T::from(1e-09_f64)}
	}

	/// Returns a copy of this magnetic flux value in kilowebers
	pub fn to_kWb(&self) -> T {
		return self.Wb.clone() * T::from(0.001_f64);
	}

	/// Returns a new magnetic flux value from the given number of kilowebers
	///
	/// # Arguments
	/// * `kWb` - Any number-like type, representing a quantity of kilowebers
	pub fn from_kWb(kWb: T) -> Self {
		MagneticFlux{Wb: kWb * T::from(1000.0_f64)}
	}

	/// Returns a copy of this magnetic flux value in megawebers
	pub fn to_MWb(&self) -> T {
		return self.Wb.clone() * T::from(1e-06_f64);
	}

	/// Returns a new magnetic flux value from the given number of megawebers
	///
	/// # Arguments
	/// * `MWb` - Any number-like type, representing a quantity of megawebers
	pub fn from_MWb(MWb: T) -> Self {
		MagneticFlux{Wb: MWb * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this magnetic flux value in gigawebers
	pub fn to_GWb(&self) -> T {
		return self.Wb.clone() * T::from(1e-09_f64);
	}

	/// Returns a new magnetic flux value from the given number of gigawebers
	///
	/// # Arguments
	/// * `GWb` - Any number-like type, representing a quantity of gigawebers
	pub fn from_GWb(GWb: T) -> Self {
		MagneticFlux{Wb: GWb * T::from(1000000000.0_f64)}
	}

}


/// Converts a MagneticFlux into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::MagneticFlux> for MagneticFlux<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::MagneticFlux {
		uom::si::f32::MagneticFlux::new::<uom::si::magnetic_flux::weber>(self.Wb.into())
	}
}

/// Creates a MagneticFlux from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::MagneticFlux> for MagneticFlux<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::MagneticFlux) -> Self {
		MagneticFlux{Wb: T::from(src.value)}
	}
}

/// Converts a MagneticFlux into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::MagneticFlux> for MagneticFlux<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::MagneticFlux {
		uom::si::f64::MagneticFlux::new::<uom::si::magnetic_flux::weber>(self.Wb.into())
	}
}

/// Creates a MagneticFlux from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFlux](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFlux.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::MagneticFlux> for MagneticFlux<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::MagneticFlux) -> Self {
		MagneticFlux{Wb: T::from(src.value)}
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

/// The magnetic flux density unit type, defined as teslas in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct MagneticFluxDensity<T: NumLike>{
	/// The value of this Magnetic flux density in teslas
	pub T: T
}

impl<T> MagneticFluxDensity<T> where T: NumLike {

	/// Returns the standard unit name of magnetic flux density: "teslas"
	pub fn unit_name() -> &'static str { "teslas" }
	
	/// Returns the abbreviated name or symbol of magnetic flux density: "T" for teslas
	pub fn unit_symbol() -> &'static str { "T" }
	
	/// Returns a new magnetic flux density value from the given number of teslas
	///
	/// # Arguments
	/// * `T` - Any number-like type, representing a quantity of teslas
	pub fn from_T(T: T) -> Self { MagneticFluxDensity{T: T} }
	
	/// Returns a copy of this magnetic flux density value in teslas
	pub fn to_T(&self) -> T { self.T.clone() }

	/// Returns a new magnetic flux density value from the given number of teslas
	///
	/// # Arguments
	/// * `teslas` - Any number-like type, representing a quantity of teslas
	pub fn from_teslas(teslas: T) -> Self { MagneticFluxDensity{T: teslas} }
	
	/// Returns a copy of this magnetic flux density value in teslas
	pub fn to_teslas(&self) -> T { self.T.clone() }

}

impl<T> fmt::Display for MagneticFluxDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.T, Self::unit_symbol())
	}
}

impl<T> MagneticFluxDensity<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this magnetic flux density value in milliteslas
	pub fn to_mT(&self) -> T {
		return self.T.clone() * T::from(1000.0_f64);
	}

	/// Returns a new magnetic flux density value from the given number of milliteslas
	///
	/// # Arguments
	/// * `mT` - Any number-like type, representing a quantity of milliteslas
	pub fn from_mT(mT: T) -> Self {
		MagneticFluxDensity{T: mT * T::from(0.001_f64)}
	}

	/// Returns a copy of this magnetic flux density value in microteslas
	pub fn to_uT(&self) -> T {
		return self.T.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new magnetic flux density value from the given number of microteslas
	///
	/// # Arguments
	/// * `uT` - Any number-like type, representing a quantity of microteslas
	pub fn from_uT(uT: T) -> Self {
		MagneticFluxDensity{T: uT * T::from(1e-06_f64)}
	}

	/// Returns a copy of this magnetic flux density value in nanoteslas
	pub fn to_nT(&self) -> T {
		return self.T.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new magnetic flux density value from the given number of nanoteslas
	///
	/// # Arguments
	/// * `nT` - Any number-like type, representing a quantity of nanoteslas
	pub fn from_nT(nT: T) -> Self {
		MagneticFluxDensity{T: nT * T::from(1e-09_f64)}
	}

	/// Returns a copy of this magnetic flux density value in kiloteslas
	pub fn to_kT(&self) -> T {
		return self.T.clone() * T::from(0.001_f64);
	}

	/// Returns a new magnetic flux density value from the given number of kiloteslas
	///
	/// # Arguments
	/// * `kT` - Any number-like type, representing a quantity of kiloteslas
	pub fn from_kT(kT: T) -> Self {
		MagneticFluxDensity{T: kT * T::from(1000.0_f64)}
	}

	/// Returns a copy of this magnetic flux density value in megateslas
	pub fn to_MT(&self) -> T {
		return self.T.clone() * T::from(1e-06_f64);
	}

	/// Returns a new magnetic flux density value from the given number of megateslas
	///
	/// # Arguments
	/// * `MT` - Any number-like type, representing a quantity of megateslas
	pub fn from_MT(MT: T) -> Self {
		MagneticFluxDensity{T: MT * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this magnetic flux density value in gigateslas
	pub fn to_GT(&self) -> T {
		return self.T.clone() * T::from(1e-09_f64);
	}

	/// Returns a new magnetic flux density value from the given number of gigateslas
	///
	/// # Arguments
	/// * `GT` - Any number-like type, representing a quantity of gigateslas
	pub fn from_GT(GT: T) -> Self {
		MagneticFluxDensity{T: GT * T::from(1000000000.0_f64)}
	}

}


/// Converts a MagneticFluxDensity into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::MagneticFluxDensity {
		uom::si::f32::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(self.T.into())
	}
}

/// Creates a MagneticFluxDensity from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f32/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::MagneticFluxDensity) -> Self {
		MagneticFluxDensity{T: T::from(src.value)}
	}
}

/// Converts a MagneticFluxDensity into the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::MagneticFluxDensity {
		uom::si::f64::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(self.T.into())
	}
}

/// Creates a MagneticFluxDensity from the equivalent [uom](https://crates.io/crates/uom) type [MagneticFluxDensity](https://docs.rs/uom/0.34.0/uom/si/f64/type.MagneticFluxDensity.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::MagneticFluxDensity> for MagneticFluxDensity<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::MagneticFluxDensity) -> Self {
		MagneticFluxDensity{T: T::from(src.value)}
	}
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

/// The electrical resistance unit type, defined as ohms in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Resistance<T: NumLike>{
	/// The value of this Electrical resistance in ohms
	pub Ohm: T
}

impl<T> Resistance<T> where T: NumLike {

	/// Returns the standard unit name of electrical resistance: "ohms"
	pub fn unit_name() -> &'static str { "ohms" }
	
	/// Returns the abbreviated name or symbol of electrical resistance: "Ohm" for ohms
	pub fn unit_symbol() -> &'static str { "Ohm" }
	
	/// Returns a new electrical resistance value from the given number of ohms
	///
	/// # Arguments
	/// * `Ohm` - Any number-like type, representing a quantity of ohms
	pub fn from_Ohm(Ohm: T) -> Self { Resistance{Ohm: Ohm} }
	
	/// Returns a copy of this electrical resistance value in ohms
	pub fn to_Ohm(&self) -> T { self.Ohm.clone() }

	/// Returns a new electrical resistance value from the given number of ohms
	///
	/// # Arguments
	/// * `ohms` - Any number-like type, representing a quantity of ohms
	pub fn from_ohms(ohms: T) -> Self { Resistance{Ohm: ohms} }
	
	/// Returns a copy of this electrical resistance value in ohms
	pub fn to_ohms(&self) -> T { self.Ohm.clone() }

}

impl<T> fmt::Display for Resistance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.Ohm, Self::unit_symbol())
	}
}

impl<T> Resistance<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this electrical resistance value in milliohms
	pub fn to_mOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1000.0_f64);
	}

	/// Returns a new electrical resistance value from the given number of milliohms
	///
	/// # Arguments
	/// * `mOhm` - Any number-like type, representing a quantity of milliohms
	pub fn from_mOhm(mOhm: T) -> Self {
		Resistance{Ohm: mOhm * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical resistance value in microohms
	pub fn to_uOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new electrical resistance value from the given number of microohms
	///
	/// # Arguments
	/// * `uOhm` - Any number-like type, representing a quantity of microohms
	pub fn from_uOhm(uOhm: T) -> Self {
		Resistance{Ohm: uOhm * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical resistance value in nanoohms
	pub fn to_nOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new electrical resistance value from the given number of nanoohms
	///
	/// # Arguments
	/// * `nOhm` - Any number-like type, representing a quantity of nanoohms
	pub fn from_nOhm(nOhm: T) -> Self {
		Resistance{Ohm: nOhm * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical resistance value in kiloohms
	pub fn to_kOhm(&self) -> T {
		return self.Ohm.clone() * T::from(0.001_f64);
	}

	/// Returns a new electrical resistance value from the given number of kiloohms
	///
	/// # Arguments
	/// * `kOhm` - Any number-like type, representing a quantity of kiloohms
	pub fn from_kOhm(kOhm: T) -> Self {
		Resistance{Ohm: kOhm * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical resistance value in megaohms
	pub fn to_MOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1e-06_f64);
	}

	/// Returns a new electrical resistance value from the given number of megaohms
	///
	/// # Arguments
	/// * `MOhm` - Any number-like type, representing a quantity of megaohms
	pub fn from_MOhm(MOhm: T) -> Self {
		Resistance{Ohm: MOhm * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical resistance value in gigaohms
	pub fn to_GOhm(&self) -> T {
		return self.Ohm.clone() * T::from(1e-09_f64);
	}

	/// Returns a new electrical resistance value from the given number of gigaohms
	///
	/// # Arguments
	/// * `GOhm` - Any number-like type, representing a quantity of gigaohms
	pub fn from_GOhm(GOhm: T) -> Self {
		Resistance{Ohm: GOhm * T::from(1000000000.0_f64)}
	}

}


/// Converts a Resistance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricalResistance> for Resistance<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricalResistance {
		uom::si::f32::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(self.Ohm.into())
	}
}

/// Creates a Resistance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricalResistance> for Resistance<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricalResistance) -> Self {
		Resistance{Ohm: T::from(src.value)}
	}
}

/// Converts a Resistance into the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricalResistance> for Resistance<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricalResistance {
		uom::si::f64::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(self.Ohm.into())
	}
}

/// Creates a Resistance from the equivalent [uom](https://crates.io/crates/uom) type [ElectricalResistance](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricalResistance.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricalResistance> for Resistance<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricalResistance) -> Self {
		Resistance{Ohm: T::from(src.value)}
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

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for &f64 where T: NumLike+From<f64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for &f32 where T: NumLike+From<f32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for &i64 where T: NumLike+From<i64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<Resistance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
impl<T> std::ops::Div<&Resistance<T>> for &i32 where T: NumLike+From<i32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<Resistance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<Resistance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<&Resistance<T>> for num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-bigfloat")]
impl<T> std::ops::Div<&Resistance<T>> for &num_bigfloat::BigFloat where T: NumLike+From<num_bigfloat::BigFloat> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Resistance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Resistance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Resistance<T>> for num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Resistance<T>> for &num_complex::Complex32 where T: NumLike+From<num_complex::Complex32> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
	}
}

// 1/Resistance -> Conductance
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Resistance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<Resistance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Resistance<T>> for num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self) / rhs.Ohm.clone()}
	}
}
/// Dividing a scalar value by a Resistance unit value returns a value of type Conductance
#[cfg(feature="num-complex")]
impl<T> std::ops::Div<&Resistance<T>> for &num_complex::Complex64 where T: NumLike+From<num_complex::Complex64> {
	type Output = Conductance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Conductance{S: T::from(self.clone()) / rhs.Ohm.clone()}
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
	pub fn unit_name() -> &'static str { "volts" }
	
	/// Returns the abbreviated name or symbol of voltage: "V" for volts
	pub fn unit_symbol() -> &'static str { "V" }
	
	/// Returns a new voltage value from the given number of volts
	///
	/// # Arguments
	/// * `V` - Any number-like type, representing a quantity of volts
	pub fn from_V(V: T) -> Self { Voltage{V: V} }
	
	/// Returns a copy of this voltage value in volts
	pub fn to_V(&self) -> T { self.V.clone() }

	/// Returns a new voltage value from the given number of volts
	///
	/// # Arguments
	/// * `volts` - Any number-like type, representing a quantity of volts
	pub fn from_volts(volts: T) -> Self { Voltage{V: volts} }
	
	/// Returns a copy of this voltage value in volts
	pub fn to_volts(&self) -> T { self.V.clone() }

}

impl<T> fmt::Display for Voltage<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.V, Self::unit_symbol())
	}
}

impl<T> Voltage<T> where T: NumLike+From<f64> {
	
	/// Returns a copy of this voltage value in millivolts
	pub fn to_mV(&self) -> T {
		return self.V.clone() * T::from(1000.0_f64);
	}

	/// Returns a new voltage value from the given number of millivolts
	///
	/// # Arguments
	/// * `mV` - Any number-like type, representing a quantity of millivolts
	pub fn from_mV(mV: T) -> Self {
		Voltage{V: mV * T::from(0.001_f64)}
	}

	/// Returns a copy of this voltage value in microvolts
	pub fn to_uV(&self) -> T {
		return self.V.clone() * T::from(1000000.0_f64);
	}

	/// Returns a new voltage value from the given number of microvolts
	///
	/// # Arguments
	/// * `uV` - Any number-like type, representing a quantity of microvolts
	pub fn from_uV(uV: T) -> Self {
		Voltage{V: uV * T::from(1e-06_f64)}
	}

	/// Returns a copy of this voltage value in nanovolts
	pub fn to_nV(&self) -> T {
		return self.V.clone() * T::from(1000000000.0_f64);
	}

	/// Returns a new voltage value from the given number of nanovolts
	///
	/// # Arguments
	/// * `nV` - Any number-like type, representing a quantity of nanovolts
	pub fn from_nV(nV: T) -> Self {
		Voltage{V: nV * T::from(1e-09_f64)}
	}

	/// Returns a copy of this voltage value in kilovolts
	pub fn to_kV(&self) -> T {
		return self.V.clone() * T::from(0.001_f64);
	}

	/// Returns a new voltage value from the given number of kilovolts
	///
	/// # Arguments
	/// * `kV` - Any number-like type, representing a quantity of kilovolts
	pub fn from_kV(kV: T) -> Self {
		Voltage{V: kV * T::from(1000.0_f64)}
	}

	/// Returns a copy of this voltage value in megavolts
	pub fn to_MV(&self) -> T {
		return self.V.clone() * T::from(1e-06_f64);
	}

	/// Returns a new voltage value from the given number of megavolts
	///
	/// # Arguments
	/// * `MV` - Any number-like type, representing a quantity of megavolts
	pub fn from_MV(MV: T) -> Self {
		Voltage{V: MV * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this voltage value in gigavolts
	pub fn to_GV(&self) -> T {
		return self.V.clone() * T::from(1e-09_f64);
	}

	/// Returns a new voltage value from the given number of gigavolts
	///
	/// # Arguments
	/// * `GV` - Any number-like type, representing a quantity of gigavolts
	pub fn from_GV(GV: T) -> Self {
		Voltage{V: GV * T::from(1000000000.0_f64)}
	}

}


/// Converts a Voltage into the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f32::ElectricPotential> for Voltage<T> where T: NumLike+Into<f32> {
	fn into(self) -> uom::si::f32::ElectricPotential {
		uom::si::f32::ElectricPotential::new::<uom::si::electric_potential::volt>(self.V.into())
	}
}

/// Creates a Voltage from the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f32/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f32::ElectricPotential> for Voltage<T> where T: NumLike+From<f32> {
	fn from(src: uom::si::f32::ElectricPotential) -> Self {
		Voltage{V: T::from(src.value)}
	}
}

/// Converts a Voltage into the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::f64::ElectricPotential> for Voltage<T> where T: NumLike+Into<f64> {
	fn into(self) -> uom::si::f64::ElectricPotential {
		uom::si::f64::ElectricPotential::new::<uom::si::electric_potential::volt>(self.V.into())
	}
}

/// Creates a Voltage from the equivalent [uom](https://crates.io/crates/uom) type [ElectricPotential](https://docs.rs/uom/0.34.0/uom/si/f64/type.ElectricPotential.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::f64::ElectricPotential> for Voltage<T> where T: NumLike+From<f64> {
	fn from(src: uom::si::f64::ElectricPotential) -> Self {
		Voltage{V: T::from(src.value)}
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



