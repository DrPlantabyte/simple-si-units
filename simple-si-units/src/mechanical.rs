
//! This module provides mechanical SI units, such as angular velocity 
//! and velocity.
use std::fmt;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;


/// The angular velocity unit type, defined as radians per second in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct AngularVelocity<T: NumLike>{
	pub radps: T
}

impl<T> AngularVelocity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "radians per second";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// AngularVelocity * Time -> Angle
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for AngularVelocity<T> where T: NumLike {
		type Output = Angle<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Angle{rad: self.radps * rhs.s}
		}
	}

	// AngularVelocity / Time -> AngularAcceleration
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for AngularVelocity<T> where T: NumLike {
		type Output = AngularAcceleration<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			AngularAcceleration{radps2: self.radps / rhs.s}
		}
	}

	// AngularVelocity / Angle -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Angle<T>> for AngularVelocity<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Angle<T>) -> Self::Output {
			Frequency{Hz: self.radps / rhs.rad}
		}
	}

	// AngularVelocity / AngularAcceleration -> Time
	// TODO: docstring
	impl<T> std::ops::Div<AngularAcceleration<T>> for AngularVelocity<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: AngularAcceleration<T>) -> Self::Output {
			Time{s: self.radps / rhs.radps2}
		}
	}

	// AngularVelocity * MomentOfInertia -> AngularMomentum
	// TODO: docstring
	impl<T> std::ops::Mul<MomentOfInertia<T>> for AngularVelocity<T> where T: NumLike {
		type Output = AngularMomentum<T>;
		fn mul(self, rhs: MomentOfInertia<T>) -> Self::Output {
			AngularMomentum{kgm2radps: self.radps * rhs.kgm2}
		}
	}

	// AngularVelocity * AreaDensity -> AngularMomentum
	// TODO: docstring
	impl<T> std::ops::Mul<AreaDensity<T>> for AngularVelocity<T> where T: NumLike {
		type Output = AngularMomentum<T>;
		fn mul(self, rhs: AreaDensity<T>) -> Self::Output {
			AngularMomentum{kgm2radps: self.radps * rhs.kgm2}
		}
	}

	// AngularVelocity * Frequency -> AngularAcceleration
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for AngularVelocity<T> where T: NumLike {
		type Output = AngularAcceleration<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			AngularAcceleration{radps2: self.radps * rhs.Hz}
		}
	}

	// AngularVelocity / Frequency -> Angle
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for AngularVelocity<T> where T: NumLike {
		type Output = Angle<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Angle{rad: self.radps / rhs.Hz}
		}
	}

}

/// The angular acceleration unit type, defined as radians per second squared in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct AngularAcceleration<T: NumLike>{
	pub radps2: T
}

impl<T> AngularAcceleration<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "radians per second squared";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// AngularAcceleration * Time -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for AngularAcceleration<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			AngularVelocity{radps: self.radps2 * rhs.s}
		}
	}

	// AngularAcceleration / AngularVelocity -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<AngularVelocity<T>> for AngularAcceleration<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
			Frequency{Hz: self.radps2 / rhs.radps}
		}
	}

	// AngularAcceleration / Frequency -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for AngularAcceleration<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			AngularVelocity{radps: self.radps2 / rhs.Hz}
		}
	}

}

/// The moment of inertia unit type, defined as kilogram meters squared in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct MomentOfInertia<T: NumLike>{
	pub kgm2: T
}

impl<T> MomentOfInertia<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "kilogram meters squared";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// MomentOfInertia / Mass -> Area
	// TODO: docstring
	impl<T> std::ops::Div<Mass<T>> for MomentOfInertia<T> where T: NumLike {
		type Output = Area<T>;
		fn div(self, rhs: Mass<T>) -> Self::Output {
			Area{m2: self.kgm2 / rhs.kg}
		}
	}

	// MomentOfInertia * AngularVelocity -> AngularMomentum
	// TODO: docstring
	impl<T> std::ops::Mul<AngularVelocity<T>> for MomentOfInertia<T> where T: NumLike {
		type Output = AngularMomentum<T>;
		fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
			AngularMomentum{kgm2radps: self.kgm2 * rhs.radps}
		}
	}

	// MomentOfInertia / Area -> Mass
	// TODO: docstring
	impl<T> std::ops::Div<Area<T>> for MomentOfInertia<T> where T: NumLike {
		type Output = Mass<T>;
		fn div(self, rhs: Area<T>) -> Self::Output {
			Mass{kg: self.kgm2 / rhs.m2}
		}
	}

}

/// The angular momentum unit type, defined as kilogram meters squared radians per second in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct AngularMomentum<T: NumLike>{
	pub kgm2radps: T
}

impl<T> AngularMomentum<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "kilogram meters squared radians per second";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// AngularMomentum / AngularVelocity -> AreaDensity
	// TODO: docstring
	impl<T> std::ops::Div<AngularVelocity<T>> for AngularMomentum<T> where T: NumLike {
		type Output = AreaDensity<T>;
		fn div(self, rhs: AngularVelocity<T>) -> Self::Output {
			AreaDensity{kgm2: self.kgm2radps / rhs.radps}
		}
	}

	// AngularMomentum / MomentOfInertia -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Div<MomentOfInertia<T>> for AngularMomentum<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn div(self, rhs: MomentOfInertia<T>) -> Self::Output {
			AngularVelocity{radps: self.kgm2radps / rhs.kgm2}
		}
	}

	// AngularMomentum / AreaDensity -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Div<AreaDensity<T>> for AngularMomentum<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn div(self, rhs: AreaDensity<T>) -> Self::Output {
			AngularVelocity{radps: self.kgm2radps / rhs.kgm2}
		}
	}

}

/// The torque unit type, defined as newton meters in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Torque<T: NumLike>{
	pub Nm: T
}

impl<T> Torque<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "newton meters";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Torque / Distance -> Force
	// TODO: docstring
	impl<T> std::ops::Div<Distance<T>> for Torque<T> where T: NumLike {
		type Output = Force<T>;
		fn div(self, rhs: Distance<T>) -> Self::Output {
			Force{N: self.Nm / rhs.m}
		}
	}

	// Torque / Mass -> DoseEquivalent
	// TODO: docstring
	impl<T> std::ops::Div<Mass<T>> for Torque<T> where T: NumLike {
		type Output = DoseEquivalent<T>;
		fn div(self, rhs: Mass<T>) -> Self::Output {
			DoseEquivalent{Sv: self.Nm / rhs.kg}
		}
	}

	// Torque / Time -> Power
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Torque<T> where T: NumLike {
		type Output = Power<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Power{W: self.Nm / rhs.s}
		}
	}

	// Torque / Current -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Div<Current<T>> for Torque<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn div(self, rhs: Current<T>) -> Self::Output {
			MagneticFlux{Wb: self.Nm / rhs.A}
		}
	}

	// Torque * Frequency -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Torque<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Power{W: self.Nm * rhs.Hz}
		}
	}

	// Torque / Volume -> Pressure
	// TODO: docstring
	impl<T> std::ops::Div<Volume<T>> for Torque<T> where T: NumLike {
		type Output = Pressure<T>;
		fn div(self, rhs: Volume<T>) -> Self::Output {
			Pressure{Pa: self.Nm / rhs.m3}
		}
	}

	// Torque / Velocity -> Momentum
	// TODO: docstring
	impl<T> std::ops::Div<Velocity<T>> for Torque<T> where T: NumLike {
		type Output = Momentum<T>;
		fn div(self, rhs: Velocity<T>) -> Self::Output {
			Momentum{kgmps: self.Nm / rhs.mps}
		}
	}

	// Torque / Momentum -> Velocity
	// TODO: docstring
	impl<T> std::ops::Div<Momentum<T>> for Torque<T> where T: NumLike {
		type Output = Velocity<T>;
		fn div(self, rhs: Momentum<T>) -> Self::Output {
			Velocity{mps: self.Nm / rhs.kgmps}
		}
	}

	// Torque / Force -> Distance
	// TODO: docstring
	impl<T> std::ops::Div<Force<T>> for Torque<T> where T: NumLike {
		type Output = Distance<T>;
		fn div(self, rhs: Force<T>) -> Self::Output {
			Distance{m: self.Nm / rhs.N}
		}
	}

	// Torque / Pressure -> Volume
	// TODO: docstring
	impl<T> std::ops::Div<Pressure<T>> for Torque<T> where T: NumLike {
		type Output = Volume<T>;
		fn div(self, rhs: Pressure<T>) -> Self::Output {
			Volume{m3: self.Nm / rhs.Pa}
		}
	}

	// Torque / Charge -> Voltage
	// TODO: docstring
	impl<T> std::ops::Div<Charge<T>> for Torque<T> where T: NumLike {
		type Output = Voltage<T>;
		fn div(self, rhs: Charge<T>) -> Self::Output {
			Voltage{V: self.Nm / rhs.C}
		}
	}

	// Torque / Power -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Power<T>> for Torque<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Power<T>) -> Self::Output {
			Time{s: self.Nm / rhs.W}
		}
	}

	// Torque / Voltage -> Charge
	// TODO: docstring
	impl<T> std::ops::Div<Voltage<T>> for Torque<T> where T: NumLike {
		type Output = Charge<T>;
		fn div(self, rhs: Voltage<T>) -> Self::Output {
			Charge{C: self.Nm / rhs.V}
		}
	}

	// Torque / MagneticFlux -> Current
	// TODO: docstring
	impl<T> std::ops::Div<MagneticFlux<T>> for Torque<T> where T: NumLike {
		type Output = Current<T>;
		fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
			Current{A: self.Nm / rhs.Wb}
		}
	}

}

/// The frequency unit type, defined as hertz in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Frequency<T: NumLike>{
	pub Hz: T
}

impl<T> Frequency<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "hertz";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Frequency * Distance -> Velocity
	// TODO: docstring
	impl<T> std::ops::Mul<Distance<T>> for Frequency<T> where T: NumLike {
		type Output = Velocity<T>;
		fn mul(self, rhs: Distance<T>) -> Self::Output {
			Velocity{mps: self.Hz * rhs.m}
		}
	}

	// Frequency * Amount -> CatalyticActivity
	// TODO: docstring
	impl<T> std::ops::Mul<Amount<T>> for Frequency<T> where T: NumLike {
		type Output = CatalyticActivity<T>;
		fn mul(self, rhs: Amount<T>) -> Self::Output {
			CatalyticActivity{molps: self.Hz * rhs.mol}
		}
	}

	// Frequency * Angle -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Mul<Angle<T>> for Frequency<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn mul(self, rhs: Angle<T>) -> Self::Output {
			AngularVelocity{radps: self.Hz * rhs.rad}
		}
	}

	// Frequency * AngularVelocity -> AngularAcceleration
	// TODO: docstring
	impl<T> std::ops::Mul<AngularVelocity<T>> for Frequency<T> where T: NumLike {
		type Output = AngularAcceleration<T>;
		fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
			AngularAcceleration{radps2: self.Hz * rhs.radps}
		}
	}

	// Frequency * Torque -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Torque<T>> for Frequency<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Torque<T>) -> Self::Output {
			Power{W: self.Hz * rhs.Nm}
		}
	}

	// Frequency * Energy -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Energy<T>> for Frequency<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Energy<T>) -> Self::Output {
			Power{W: self.Hz * rhs.J}
		}
	}

	// Frequency * Velocity -> Acceleration
	// TODO: docstring
	impl<T> std::ops::Mul<Velocity<T>> for Frequency<T> where T: NumLike {
		type Output = Acceleration<T>;
		fn mul(self, rhs: Velocity<T>) -> Self::Output {
			Acceleration{mps2: self.Hz * rhs.mps}
		}
	}

	// Frequency * Momentum -> Force
	// TODO: docstring
	impl<T> std::ops::Mul<Momentum<T>> for Frequency<T> where T: NumLike {
		type Output = Force<T>;
		fn mul(self, rhs: Momentum<T>) -> Self::Output {
			Force{N: self.Hz * rhs.kgmps}
		}
	}

	// Frequency * Charge -> Current
	// TODO: docstring
	impl<T> std::ops::Mul<Charge<T>> for Frequency<T> where T: NumLike {
		type Output = Current<T>;
		fn mul(self, rhs: Charge<T>) -> Self::Output {
			Current{A: self.Hz * rhs.C}
		}
	}

	// Frequency * Capacitance -> Conductance
	// TODO: docstring
	impl<T> std::ops::Mul<Capacitance<T>> for Frequency<T> where T: NumLike {
		type Output = Conductance<T>;
		fn mul(self, rhs: Capacitance<T>) -> Self::Output {
			Conductance{S: self.Hz * rhs.F}
		}
	}

	// Frequency * Inductance -> Resistance
	// TODO: docstring
	impl<T> std::ops::Mul<Inductance<T>> for Frequency<T> where T: NumLike {
		type Output = Resistance<T>;
		fn mul(self, rhs: Inductance<T>) -> Self::Output {
			Resistance{Ohm: self.Hz * rhs.H}
		}
	}

	// Frequency * MagneticFlux -> Voltage
	// TODO: docstring
	impl<T> std::ops::Mul<MagneticFlux<T>> for Frequency<T> where T: NumLike {
		type Output = Voltage<T>;
		fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
			Voltage{V: self.Hz * rhs.Wb}
		}
	}

}

/// The area density unit type, defined as kilograms per square meter in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct AreaDensity<T: NumLike>{
	pub kgm2: T
}

impl<T> AreaDensity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "kilograms per square meter";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// AreaDensity / Mass -> Area
	// TODO: docstring
	impl<T> std::ops::Div<Mass<T>> for AreaDensity<T> where T: NumLike {
		type Output = Area<T>;
		fn div(self, rhs: Mass<T>) -> Self::Output {
			Area{m2: self.kgm2 / rhs.kg}
		}
	}

	// AreaDensity * AngularVelocity -> AngularMomentum
	// TODO: docstring
	impl<T> std::ops::Mul<AngularVelocity<T>> for AreaDensity<T> where T: NumLike {
		type Output = AngularMomentum<T>;
		fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
			AngularMomentum{kgm2radps: self.kgm2 * rhs.radps}
		}
	}

	// AreaDensity / Area -> Mass
	// TODO: docstring
	impl<T> std::ops::Div<Area<T>> for AreaDensity<T> where T: NumLike {
		type Output = Mass<T>;
		fn div(self, rhs: Area<T>) -> Self::Output {
			Mass{kg: self.kgm2 / rhs.m2}
		}
	}

}

/// The density unit type, defined as kilograms per liter in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Density<T: NumLike>{
	pub kgpL: T
}

impl<T> Density<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "kilograms per liter";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
pub struct Velocity<T: NumLike>{
	pub mps: T
}

impl<T> Velocity<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "meters per second";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Velocity / Distance -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Distance<T>> for Velocity<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Distance<T>) -> Self::Output {
			Frequency{Hz: self.mps / rhs.m}
		}
	}

	// Velocity * Mass -> Momentum
	// TODO: docstring
	impl<T> std::ops::Mul<Mass<T>> for Velocity<T> where T: NumLike {
		type Output = Momentum<T>;
		fn mul(self, rhs: Mass<T>) -> Self::Output {
			Momentum{kgmps: self.mps * rhs.kg}
		}
	}

	// Velocity * Time -> Distance
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Velocity<T> where T: NumLike {
		type Output = Distance<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Distance{m: self.mps * rhs.s}
		}
	}

	// Velocity / Time -> Acceleration
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Velocity<T> where T: NumLike {
		type Output = Acceleration<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Acceleration{mps2: self.mps / rhs.s}
		}
	}

	// Velocity * Frequency -> Acceleration
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Velocity<T> where T: NumLike {
		type Output = Acceleration<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Acceleration{mps2: self.mps * rhs.Hz}
		}
	}

	// Velocity / Frequency -> Distance
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Velocity<T> where T: NumLike {
		type Output = Distance<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Distance{m: self.mps / rhs.Hz}
		}
	}

	// Velocity * Velocity -> DoseEquivalent
	// TODO: docstring
	impl<T> std::ops::Mul<Velocity<T>> for Velocity<T> where T: NumLike {
		type Output = DoseEquivalent<T>;
		fn mul(self, rhs: Velocity<T>) -> Self::Output {
			DoseEquivalent{Sv: self.mps * rhs.mps}
		}
	}

	// Velocity / Acceleration -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Acceleration<T>> for Velocity<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Acceleration<T>) -> Self::Output {
			Time{s: self.mps / rhs.mps2}
		}
	}

	// Velocity * Momentum -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Momentum<T>> for Velocity<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Momentum<T>) -> Self::Output {
			Energy{J: self.mps * rhs.kgmps}
		}
	}

	// Velocity * Force -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Force<T>> for Velocity<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Force<T>) -> Self::Output {
			Power{W: self.mps * rhs.N}
		}
	}

}

/// The acceleration unit type, defined as meters per second squared in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Acceleration<T: NumLike>{
	pub mps2: T
}

impl<T> Acceleration<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "meters per second squared";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Acceleration * Distance -> DoseEquivalent
	// TODO: docstring
	impl<T> std::ops::Mul<Distance<T>> for Acceleration<T> where T: NumLike {
		type Output = DoseEquivalent<T>;
		fn mul(self, rhs: Distance<T>) -> Self::Output {
			DoseEquivalent{Sv: self.mps2 * rhs.m}
		}
	}

	// Acceleration * Mass -> Force
	// TODO: docstring
	impl<T> std::ops::Mul<Mass<T>> for Acceleration<T> where T: NumLike {
		type Output = Force<T>;
		fn mul(self, rhs: Mass<T>) -> Self::Output {
			Force{N: self.mps2 * rhs.kg}
		}
	}

	// Acceleration * Time -> Velocity
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Acceleration<T> where T: NumLike {
		type Output = Velocity<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Velocity{mps: self.mps2 * rhs.s}
		}
	}

	// Acceleration / Frequency -> Velocity
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Acceleration<T> where T: NumLike {
		type Output = Velocity<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Velocity{mps: self.mps2 / rhs.Hz}
		}
	}

	// Acceleration / Velocity -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Velocity<T>> for Acceleration<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Velocity<T>) -> Self::Output {
			Frequency{Hz: self.mps2 / rhs.mps}
		}
	}

	// Acceleration * Momentum -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Momentum<T>> for Acceleration<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Momentum<T>) -> Self::Output {
			Power{W: self.mps2 * rhs.kgmps}
		}
	}

}

/// The momentum unit type, defined as kilogram meters per second in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Momentum<T: NumLike>{
	pub kgmps: T
}

impl<T> Momentum<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "kilogram meters per second";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Momentum / Mass -> Velocity
	// TODO: docstring
	impl<T> std::ops::Div<Mass<T>> for Momentum<T> where T: NumLike {
		type Output = Velocity<T>;
		fn div(self, rhs: Mass<T>) -> Self::Output {
			Velocity{mps: self.kgmps / rhs.kg}
		}
	}

	// Momentum / Time -> Force
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Momentum<T> where T: NumLike {
		type Output = Force<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Force{N: self.kgmps / rhs.s}
		}
	}

	// Momentum * Frequency -> Force
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Momentum<T> where T: NumLike {
		type Output = Force<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Force{N: self.kgmps * rhs.Hz}
		}
	}

	// Momentum * Velocity -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Velocity<T>> for Momentum<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Velocity<T>) -> Self::Output {
			Energy{J: self.kgmps * rhs.mps}
		}
	}

	// Momentum / Velocity -> Mass
	// TODO: docstring
	impl<T> std::ops::Div<Velocity<T>> for Momentum<T> where T: NumLike {
		type Output = Mass<T>;
		fn div(self, rhs: Velocity<T>) -> Self::Output {
			Mass{kg: self.kgmps / rhs.mps}
		}
	}

	// Momentum * Acceleration -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Acceleration<T>> for Momentum<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Acceleration<T>) -> Self::Output {
			Power{W: self.kgmps * rhs.mps2}
		}
	}

	// Momentum / Force -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Force<T>> for Momentum<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Force<T>) -> Self::Output {
			Time{s: self.kgmps / rhs.N}
		}
	}

}

/// The force unit type, defined as newtons in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Force<T: NumLike>{
	pub N: T
}

impl<T> Force<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "newtons";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Force * Distance -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Distance<T>> for Force<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Distance<T>) -> Self::Output {
			Energy{J: self.N * rhs.m}
		}
	}

	// Force / Mass -> Acceleration
	// TODO: docstring
	impl<T> std::ops::Div<Mass<T>> for Force<T> where T: NumLike {
		type Output = Acceleration<T>;
		fn div(self, rhs: Mass<T>) -> Self::Output {
			Acceleration{mps2: self.N / rhs.kg}
		}
	}

	// Force * Time -> Momentum
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Force<T> where T: NumLike {
		type Output = Momentum<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Momentum{kgmps: self.N * rhs.s}
		}
	}

	// Force / Frequency -> Momentum
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Force<T> where T: NumLike {
		type Output = Momentum<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Momentum{kgmps: self.N / rhs.Hz}
		}
	}

	// Force / Area -> Pressure
	// TODO: docstring
	impl<T> std::ops::Div<Area<T>> for Force<T> where T: NumLike {
		type Output = Pressure<T>;
		fn div(self, rhs: Area<T>) -> Self::Output {
			Pressure{Pa: self.N / rhs.m2}
		}
	}

	// Force * Velocity -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Velocity<T>> for Force<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Velocity<T>) -> Self::Output {
			Power{W: self.N * rhs.mps}
		}
	}

	// Force / Acceleration -> Mass
	// TODO: docstring
	impl<T> std::ops::Div<Acceleration<T>> for Force<T> where T: NumLike {
		type Output = Mass<T>;
		fn div(self, rhs: Acceleration<T>) -> Self::Output {
			Mass{kg: self.N / rhs.mps2}
		}
	}

	// Force / Momentum -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Momentum<T>> for Force<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Momentum<T>) -> Self::Output {
			Frequency{Hz: self.N / rhs.kgmps}
		}
	}

	// Force / Pressure -> Area
	// TODO: docstring
	impl<T> std::ops::Div<Pressure<T>> for Force<T> where T: NumLike {
		type Output = Area<T>;
		fn div(self, rhs: Pressure<T>) -> Self::Output {
			Area{m2: self.N / rhs.Pa}
		}
	}

}

/// The pressure unit type, defined as pascals in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Pressure<T: NumLike>{
	pub Pa: T
}

impl<T> Pressure<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "pascals";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Pressure * Area -> Force
	// TODO: docstring
	impl<T> std::ops::Mul<Area<T>> for Pressure<T> where T: NumLike {
		type Output = Force<T>;
		fn mul(self, rhs: Area<T>) -> Self::Output {
			Force{N: self.Pa * rhs.m2}
		}
	}

	// Pressure * Volume -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Volume<T>> for Pressure<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Volume<T>) -> Self::Output {
			Energy{J: self.Pa * rhs.m3}
		}
	}

}

/// The energy unit type, defined as joules in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Energy<T: NumLike>{
	pub J: T
}

impl<T> Energy<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "joules";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Energy / Distance -> Force
	// TODO: docstring
	impl<T> std::ops::Div<Distance<T>> for Energy<T> where T: NumLike {
		type Output = Force<T>;
		fn div(self, rhs: Distance<T>) -> Self::Output {
			Force{N: self.J / rhs.m}
		}
	}

	// Energy / Mass -> DoseEquivalent
	// TODO: docstring
	impl<T> std::ops::Div<Mass<T>> for Energy<T> where T: NumLike {
		type Output = DoseEquivalent<T>;
		fn div(self, rhs: Mass<T>) -> Self::Output {
			DoseEquivalent{Sv: self.J / rhs.kg}
		}
	}

	// Energy / Time -> Power
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Energy<T> where T: NumLike {
		type Output = Power<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Power{W: self.J / rhs.s}
		}
	}

	// Energy / Current -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Div<Current<T>> for Energy<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn div(self, rhs: Current<T>) -> Self::Output {
			MagneticFlux{Wb: self.J / rhs.A}
		}
	}

	// Energy * Frequency -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Energy<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Power{W: self.J * rhs.Hz}
		}
	}

	// Energy / Volume -> Pressure
	// TODO: docstring
	impl<T> std::ops::Div<Volume<T>> for Energy<T> where T: NumLike {
		type Output = Pressure<T>;
		fn div(self, rhs: Volume<T>) -> Self::Output {
			Pressure{Pa: self.J / rhs.m3}
		}
	}

	// Energy / Velocity -> Momentum
	// TODO: docstring
	impl<T> std::ops::Div<Velocity<T>> for Energy<T> where T: NumLike {
		type Output = Momentum<T>;
		fn div(self, rhs: Velocity<T>) -> Self::Output {
			Momentum{kgmps: self.J / rhs.mps}
		}
	}

	// Energy / Momentum -> Velocity
	// TODO: docstring
	impl<T> std::ops::Div<Momentum<T>> for Energy<T> where T: NumLike {
		type Output = Velocity<T>;
		fn div(self, rhs: Momentum<T>) -> Self::Output {
			Velocity{mps: self.J / rhs.kgmps}
		}
	}

	// Energy / Force -> Distance
	// TODO: docstring
	impl<T> std::ops::Div<Force<T>> for Energy<T> where T: NumLike {
		type Output = Distance<T>;
		fn div(self, rhs: Force<T>) -> Self::Output {
			Distance{m: self.J / rhs.N}
		}
	}

	// Energy / Pressure -> Volume
	// TODO: docstring
	impl<T> std::ops::Div<Pressure<T>> for Energy<T> where T: NumLike {
		type Output = Volume<T>;
		fn div(self, rhs: Pressure<T>) -> Self::Output {
			Volume{m3: self.J / rhs.Pa}
		}
	}

	// Energy / Charge -> Voltage
	// TODO: docstring
	impl<T> std::ops::Div<Charge<T>> for Energy<T> where T: NumLike {
		type Output = Voltage<T>;
		fn div(self, rhs: Charge<T>) -> Self::Output {
			Voltage{V: self.J / rhs.C}
		}
	}

	// Energy / Power -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Power<T>> for Energy<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Power<T>) -> Self::Output {
			Time{s: self.J / rhs.W}
		}
	}

	// Energy / Voltage -> Charge
	// TODO: docstring
	impl<T> std::ops::Div<Voltage<T>> for Energy<T> where T: NumLike {
		type Output = Charge<T>;
		fn div(self, rhs: Voltage<T>) -> Self::Output {
			Charge{C: self.J / rhs.V}
		}
	}

	// Energy / MagneticFlux -> Current
	// TODO: docstring
	impl<T> std::ops::Div<MagneticFlux<T>> for Energy<T> where T: NumLike {
		type Output = Current<T>;
		fn div(self, rhs: MagneticFlux<T>) -> Self::Output {
			Current{A: self.J / rhs.Wb}
		}
	}

	// Energy / AbsorbedDose -> Mass
	// TODO: docstring
	impl<T> std::ops::Div<AbsorbedDose<T>> for Energy<T> where T: NumLike {
		type Output = Mass<T>;
		fn div(self, rhs: AbsorbedDose<T>) -> Self::Output {
			Mass{kg: self.J / rhs.Gy}
		}
	}

	// Energy / DoseEquivalent -> Mass
	// TODO: docstring
	impl<T> std::ops::Div<DoseEquivalent<T>> for Energy<T> where T: NumLike {
		type Output = Mass<T>;
		fn div(self, rhs: DoseEquivalent<T>) -> Self::Output {
			Mass{kg: self.J / rhs.Sv}
		}
	}

}

/// The power (aka watts) unit type, defined as watts in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Power<T: NumLike>{
	pub W: T
}

impl<T> Power<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "watts";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
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
	
	// Power * Time -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Power<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Energy{J: self.W * rhs.s}
		}
	}

	// Power / Current -> Voltage
	// TODO: docstring
	impl<T> std::ops::Div<Current<T>> for Power<T> where T: NumLike {
		type Output = Voltage<T>;
		fn div(self, rhs: Current<T>) -> Self::Output {
			Voltage{V: self.W / rhs.A}
		}
	}

	// Power / Torque -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Torque<T>> for Power<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Torque<T>) -> Self::Output {
			Frequency{Hz: self.W / rhs.Nm}
		}
	}

	// Power / Energy -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Energy<T>> for Power<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Energy<T>) -> Self::Output {
			Frequency{Hz: self.W / rhs.J}
		}
	}

	// Power / Frequency -> Energy
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Power<T> where T: NumLike {
		type Output = Energy<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Energy{J: self.W / rhs.Hz}
		}
	}

	// Power / Velocity -> Force
	// TODO: docstring
	impl<T> std::ops::Div<Velocity<T>> for Power<T> where T: NumLike {
		type Output = Force<T>;
		fn div(self, rhs: Velocity<T>) -> Self::Output {
			Force{N: self.W / rhs.mps}
		}
	}

	// Power / Acceleration -> Momentum
	// TODO: docstring
	impl<T> std::ops::Div<Acceleration<T>> for Power<T> where T: NumLike {
		type Output = Momentum<T>;
		fn div(self, rhs: Acceleration<T>) -> Self::Output {
			Momentum{kgmps: self.W / rhs.mps2}
		}
	}

	// Power / Momentum -> Acceleration
	// TODO: docstring
	impl<T> std::ops::Div<Momentum<T>> for Power<T> where T: NumLike {
		type Output = Acceleration<T>;
		fn div(self, rhs: Momentum<T>) -> Self::Output {
			Acceleration{mps2: self.W / rhs.kgmps}
		}
	}

	// Power / Force -> Velocity
	// TODO: docstring
	impl<T> std::ops::Div<Force<T>> for Power<T> where T: NumLike {
		type Output = Velocity<T>;
		fn div(self, rhs: Force<T>) -> Self::Output {
			Velocity{mps: self.W / rhs.N}
		}
	}

	// Power / Voltage -> Current
	// TODO: docstring
	impl<T> std::ops::Div<Voltage<T>> for Power<T> where T: NumLike {
		type Output = Current<T>;
		fn div(self, rhs: Voltage<T>) -> Self::Output {
			Current{A: self.W / rhs.V}
		}
	}

}


