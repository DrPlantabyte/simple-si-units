
//! This module provides base SI units, such as distance (aka length) 
//! and amount.
use std::fmt;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;


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
	
	/// Returns a copy of this distance value in meters
	pub fn to_m(self) -> T {
		return self.m.clone();
	}
}

impl<T> fmt::Display for Distance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.m, Self::unit_symbol())
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// Distance * Distance -> Area
	// TODO: docstring
	impl<T> std::ops::Mul<Distance<T>> for Distance<T> where T: NumLike {
		type Output = Area<T>;
		fn mul(self, rhs: Distance<T>) -> Self::Output {
			Area{m2: self.m * rhs.m}
		}
	}

	// Distance / Time -> Velocity
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Distance<T> where T: NumLike {
		type Output = Velocity<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			Velocity{mps: self.m / rhs.s}
		}
	}

	// Distance * Frequency -> Velocity
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Distance<T> where T: NumLike {
		type Output = Velocity<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			Velocity{mps: self.m * rhs.Hz}
		}
	}

	// Distance * Area -> Volume
	// TODO: docstring
	impl<T> std::ops::Mul<Area<T>> for Distance<T> where T: NumLike {
		type Output = Volume<T>;
		fn mul(self, rhs: Area<T>) -> Self::Output {
			Volume{m3: self.m * rhs.m2}
		}
	}

	// Distance / Velocity -> Time
	// TODO: docstring
	impl<T> std::ops::Div<Velocity<T>> for Distance<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: Velocity<T>) -> Self::Output {
			Time{s: self.m / rhs.mps}
		}
	}

	// Distance * Acceleration -> DoseEquivalent
	// TODO: docstring
	impl<T> std::ops::Mul<Acceleration<T>> for Distance<T> where T: NumLike {
		type Output = DoseEquivalent<T>;
		fn mul(self, rhs: Acceleration<T>) -> Self::Output {
			DoseEquivalent{Sv: self.m * rhs.mps2}
		}
	}

	// Distance * Force -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Force<T>> for Distance<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Force<T>) -> Self::Output {
			Energy{J: self.m * rhs.N}
		}
	}

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
	
	/// Returns a copy of this mass value in kilograms
	pub fn to_kg(self) -> T {
		return self.kg.clone();
	}
}

impl<T> fmt::Display for Mass<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.kg, Self::unit_symbol())
	}
}

impl<T> Mass<T> where T: NumLike+From<f64> {
	
	// Mass * Area -> AreaDensity
	// TODO: docstring
	impl<T> std::ops::Mul<Area<T>> for Mass<T> where T: NumLike {
		type Output = AreaDensity<T>;
		fn mul(self, rhs: Area<T>) -> Self::Output {
			AreaDensity{kgm2: self.kg * rhs.m2}
		}
	}

	// Mass * Velocity -> Momentum
	// TODO: docstring
	impl<T> std::ops::Mul<Velocity<T>> for Mass<T> where T: NumLike {
		type Output = Momentum<T>;
		fn mul(self, rhs: Velocity<T>) -> Self::Output {
			Momentum{kgmps: self.kg * rhs.mps}
		}
	}

	// Mass * Acceleration -> Force
	// TODO: docstring
	impl<T> std::ops::Mul<Acceleration<T>> for Mass<T> where T: NumLike {
		type Output = Force<T>;
		fn mul(self, rhs: Acceleration<T>) -> Self::Output {
			Force{N: self.kg * rhs.mps2}
		}
	}

	// Mass * AbsorbedDose -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<AbsorbedDose<T>> for Mass<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: AbsorbedDose<T>) -> Self::Output {
			Energy{J: self.kg * rhs.Gy}
		}
	}

	// Mass * DoseEquivalent -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<DoseEquivalent<T>> for Mass<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: DoseEquivalent<T>) -> Self::Output {
			Energy{J: self.kg * rhs.Sv}
		}
	}

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
	
	/// Returns a copy of this time value in seconds
	pub fn to_s(self) -> T {
		return self.s.clone();
	}
}

impl<T> fmt::Display for Time<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.s, Self::unit_symbol())
	}
}

impl<T> Time<T> where T: NumLike+From<f64> {
	
	// Time * Current -> Charge
	// TODO: docstring
	impl<T> std::ops::Mul<Current<T>> for Time<T> where T: NumLike {
		type Output = Charge<T>;
		fn mul(self, rhs: Current<T>) -> Self::Output {
			Charge{C: self.s * rhs.A}
		}
	}

	// Time * AngularVelocity -> Angle
	// TODO: docstring
	impl<T> std::ops::Mul<AngularVelocity<T>> for Time<T> where T: NumLike {
		type Output = Angle<T>;
		fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
			Angle{rad: self.s * rhs.radps}
		}
	}

	// Time * AngularAcceleration -> AngularVelocity
	// TODO: docstring
	impl<T> std::ops::Mul<AngularAcceleration<T>> for Time<T> where T: NumLike {
		type Output = AngularVelocity<T>;
		fn mul(self, rhs: AngularAcceleration<T>) -> Self::Output {
			AngularVelocity{radps: self.s * rhs.radps2}
		}
	}

	// Time * Velocity -> Distance
	// TODO: docstring
	impl<T> std::ops::Mul<Velocity<T>> for Time<T> where T: NumLike {
		type Output = Distance<T>;
		fn mul(self, rhs: Velocity<T>) -> Self::Output {
			Distance{m: self.s * rhs.mps}
		}
	}

	// Time * Acceleration -> Velocity
	// TODO: docstring
	impl<T> std::ops::Mul<Acceleration<T>> for Time<T> where T: NumLike {
		type Output = Velocity<T>;
		fn mul(self, rhs: Acceleration<T>) -> Self::Output {
			Velocity{mps: self.s * rhs.mps2}
		}
	}

	// Time * Force -> Momentum
	// TODO: docstring
	impl<T> std::ops::Mul<Force<T>> for Time<T> where T: NumLike {
		type Output = Momentum<T>;
		fn mul(self, rhs: Force<T>) -> Self::Output {
			Momentum{kgmps: self.s * rhs.N}
		}
	}

	// Time * Power -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<Power<T>> for Time<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: Power<T>) -> Self::Output {
			Energy{J: self.s * rhs.W}
		}
	}

	// Time * Voltage -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Voltage<T>> for Time<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Voltage<T>) -> Self::Output {
			MagneticFlux{Wb: self.s * rhs.V}
		}
	}

	// Time * Resistance -> Inductance
	// TODO: docstring
	impl<T> std::ops::Mul<Resistance<T>> for Time<T> where T: NumLike {
		type Output = Inductance<T>;
		fn mul(self, rhs: Resistance<T>) -> Self::Output {
			Inductance{H: self.s * rhs.Ohm}
		}
	}

	// Time / Resistance -> Capacitance
	// TODO: docstring
	impl<T> std::ops::Div<Resistance<T>> for Time<T> where T: NumLike {
		type Output = Capacitance<T>;
		fn div(self, rhs: Resistance<T>) -> Self::Output {
			Capacitance{F: self.s / rhs.Ohm}
		}
	}

	// Time * Conductance -> Capacitance
	// TODO: docstring
	impl<T> std::ops::Mul<Conductance<T>> for Time<T> where T: NumLike {
		type Output = Capacitance<T>;
		fn mul(self, rhs: Conductance<T>) -> Self::Output {
			Capacitance{F: self.s * rhs.S}
		}
	}

	// Time / Conductance -> Inductance
	// TODO: docstring
	impl<T> std::ops::Div<Conductance<T>> for Time<T> where T: NumLike {
		type Output = Inductance<T>;
		fn div(self, rhs: Conductance<T>) -> Self::Output {
			Inductance{H: self.s / rhs.S}
		}
	}

	// Time / Capacitance -> Resistance
	// TODO: docstring
	impl<T> std::ops::Div<Capacitance<T>> for Time<T> where T: NumLike {
		type Output = Resistance<T>;
		fn div(self, rhs: Capacitance<T>) -> Self::Output {
			Resistance{Ohm: self.s / rhs.F}
		}
	}

	// Time / Inductance -> Conductance
	// TODO: docstring
	impl<T> std::ops::Div<Inductance<T>> for Time<T> where T: NumLike {
		type Output = Conductance<T>;
		fn div(self, rhs: Inductance<T>) -> Self::Output {
			Conductance{S: self.s / rhs.H}
		}
	}

	// Time * CatalyticActivity -> Amount
	// TODO: docstring
	impl<T> std::ops::Mul<CatalyticActivity<T>> for Time<T> where T: NumLike {
		type Output = Amount<T>;
		fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
			Amount{mol: self.s * rhs.molps}
		}
	}

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
	
	/// Returns a copy of this temperature value in degrees kelvin
	pub fn to_K(self) -> T {
		return self.K.clone();
	}
}

impl<T> fmt::Display for Temperature<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.K, Self::unit_symbol())
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
	
	/// Returns a copy of this amount value in moles
	pub fn to_mol(self) -> T {
		return self.mol.clone();
	}
}

impl<T> fmt::Display for Amount<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.mol, Self::unit_symbol())
	}
}

impl<T> Amount<T> where T: NumLike+From<f64> {
	
	// Amount / Time -> CatalyticActivity
	// TODO: docstring
	impl<T> std::ops::Div<Time<T>> for Amount<T> where T: NumLike {
		type Output = CatalyticActivity<T>;
		fn div(self, rhs: Time<T>) -> Self::Output {
			CatalyticActivity{molps: self.mol / rhs.s}
		}
	}

	// Amount * Frequency -> CatalyticActivity
	// TODO: docstring
	impl<T> std::ops::Mul<Frequency<T>> for Amount<T> where T: NumLike {
		type Output = CatalyticActivity<T>;
		fn mul(self, rhs: Frequency<T>) -> Self::Output {
			CatalyticActivity{molps: self.mol * rhs.Hz}
		}
	}

	// Amount / Volume -> Concentration
	// TODO: docstring
	impl<T> std::ops::Div<Volume<T>> for Amount<T> where T: NumLike {
		type Output = Concentration<T>;
		fn div(self, rhs: Volume<T>) -> Self::Output {
			Concentration{molpm3: self.mol / rhs.m3}
		}
	}

	// Amount / CatalyticActivity -> Time
	// TODO: docstring
	impl<T> std::ops::Div<CatalyticActivity<T>> for Amount<T> where T: NumLike {
		type Output = Time<T>;
		fn div(self, rhs: CatalyticActivity<T>) -> Self::Output {
			Time{s: self.mol / rhs.molps}
		}
	}

	// Amount / Concentration -> Volume
	// TODO: docstring
	impl<T> std::ops::Div<Concentration<T>> for Amount<T> where T: NumLike {
		type Output = Volume<T>;
		fn div(self, rhs: Concentration<T>) -> Self::Output {
			Volume{m3: self.mol / rhs.molpm3}
		}
	}

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
	
	/// Returns a copy of this electrical current value in amperes
	pub fn to_A(self) -> T {
		return self.A.clone();
	}
}

impl<T> fmt::Display for Current<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.A, Self::unit_symbol())
	}
}

impl<T> Current<T> where T: NumLike+From<f64> {
	
	// Current * Time -> Charge
	// TODO: docstring
	impl<T> std::ops::Mul<Time<T>> for Current<T> where T: NumLike {
		type Output = Charge<T>;
		fn mul(self, rhs: Time<T>) -> Self::Output {
			Charge{C: self.A * rhs.s}
		}
	}

	// Current / Frequency -> Charge
	// TODO: docstring
	impl<T> std::ops::Div<Frequency<T>> for Current<T> where T: NumLike {
		type Output = Charge<T>;
		fn div(self, rhs: Frequency<T>) -> Self::Output {
			Charge{C: self.A / rhs.Hz}
		}
	}

	// Current / Charge -> Frequency
	// TODO: docstring
	impl<T> std::ops::Div<Charge<T>> for Current<T> where T: NumLike {
		type Output = Frequency<T>;
		fn div(self, rhs: Charge<T>) -> Self::Output {
			Frequency{Hz: self.A / rhs.C}
		}
	}

	// Current * Voltage -> Power
	// TODO: docstring
	impl<T> std::ops::Mul<Voltage<T>> for Current<T> where T: NumLike {
		type Output = Power<T>;
		fn mul(self, rhs: Voltage<T>) -> Self::Output {
			Power{W: self.A * rhs.V}
		}
	}

	// Current / Voltage -> Conductance
	// TODO: docstring
	impl<T> std::ops::Div<Voltage<T>> for Current<T> where T: NumLike {
		type Output = Conductance<T>;
		fn div(self, rhs: Voltage<T>) -> Self::Output {
			Conductance{S: self.A / rhs.V}
		}
	}

	// Current * Resistance -> Voltage
	// TODO: docstring
	impl<T> std::ops::Mul<Resistance<T>> for Current<T> where T: NumLike {
		type Output = Voltage<T>;
		fn mul(self, rhs: Resistance<T>) -> Self::Output {
			Voltage{V: self.A * rhs.Ohm}
		}
	}

	// Current / Conductance -> Voltage
	// TODO: docstring
	impl<T> std::ops::Div<Conductance<T>> for Current<T> where T: NumLike {
		type Output = Voltage<T>;
		fn div(self, rhs: Conductance<T>) -> Self::Output {
			Voltage{V: self.A / rhs.S}
		}
	}

	// Current * Inductance -> MagneticFlux
	// TODO: docstring
	impl<T> std::ops::Mul<Inductance<T>> for Current<T> where T: NumLike {
		type Output = MagneticFlux<T>;
		fn mul(self, rhs: Inductance<T>) -> Self::Output {
			MagneticFlux{Wb: self.A * rhs.H}
		}
	}

	// Current * MagneticFlux -> Energy
	// TODO: docstring
	impl<T> std::ops::Mul<MagneticFlux<T>> for Current<T> where T: NumLike {
		type Output = Energy<T>;
		fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
			Energy{J: self.A * rhs.Wb}
		}
	}

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
	
	/// Returns a copy of this luminosity value in candela
	pub fn to_cd(self) -> T {
		return self.cd.clone();
	}
}

impl<T> fmt::Display for Luminosity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.cd, Self::unit_symbol())
	}
}

impl<T> Luminosity<T> where T: NumLike+From<f64> {
	
	// Luminosity * SolidAngle -> LuminousFlux
	// TODO: docstring
	impl<T> std::ops::Mul<SolidAngle<T>> for Luminosity<T> where T: NumLike {
		type Output = LuminousFlux<T>;
		fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
			LuminousFlux{lm: self.cd * rhs.sr}
		}
	}

}


