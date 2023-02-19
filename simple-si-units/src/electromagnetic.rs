
//! This module provides electromagnetic SI units, such as electric charge (aka coulombs) 
//! and magnetic flux.
use std::fmt;


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
	
	/// Returns this electric charge value in coulombs
	pub fn to_C(self) -> T {
		return self.C;
	}
}

impl<T> fmt::Display for Charge<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.C, Self::unit_symbol())
	}
}

impl<T> Charge<T> where T: NumLike+From<f64> {
	
	// TODO: Charge / Time -> Current

	// TODO: Charge / Current -> Time

	// TODO: Charge * Frequency -> Current

	// TODO: Charge * Voltage -> Energy

	// TODO: Charge / Voltage -> Capacitance

	// TODO: Charge * Resistance -> MagneticFlux

	// TODO: Charge / Conductance -> MagneticFlux

	// TODO: Charge / Capacitance -> Voltage

	// TODO: Charge / MagneticFlux -> Conductance

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
	
	/// Returns this voltage value in volts
	pub fn to_V(self) -> T {
		return self.V;
	}
}

impl<T> fmt::Display for Voltage<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.V, Self::unit_symbol())
	}
}

impl<T> Voltage<T> where T: NumLike+From<f64> {
	
	// TODO: Voltage * Time -> MagneticFlux

	// TODO: Voltage * Current -> Power

	// TODO: Voltage / Current -> Resistance

	// TODO: Voltage / Frequency -> MagneticFlux

	// TODO: Voltage * Charge -> Energy

	// TODO: Voltage / Resistance -> Current

	// TODO: Voltage * Conductance -> Current

	// TODO: Voltage * Capacitance -> Charge

	// TODO: Voltage / MagneticFlux -> Frequency

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
	
	/// Returns this electrical resistance value in ohms
	pub fn to_Ohm(self) -> T {
		return self.Ohm;
	}
}

impl<T> fmt::Display for Resistance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.Ohm, Self::unit_symbol())
	}
}

impl<T> Resistance<T> where T: NumLike+From<f64> {
	
	// TODO: Resistance * Time -> Inductance

	// TODO: Resistance * Current -> Voltage

	// TODO: Resistance / Frequency -> Inductance

	// TODO: Resistance * Charge -> MagneticFlux

	// TODO: Resistance * Capacitance -> Time

	// TODO: Resistance / Inductance -> Frequency

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
	
	/// Returns this electrical conductance value in siemens
	pub fn to_S(self) -> T {
		return self.S;
	}
}

impl<T> fmt::Display for Conductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.S, Self::unit_symbol())
	}
}

impl<T> Conductance<T> where T: NumLike+From<f64> {
	
	// TODO: Conductance * Time -> Capacitance

	// TODO: Conductance / Frequency -> Capacitance

	// TODO: Conductance * Voltage -> Current

	// TODO: Conductance / Capacitance -> Frequency

	// TODO: Conductance * Inductance -> Time

	// TODO: Conductance * MagneticFlux -> Charge

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
	
	/// Returns this electrical capacitance value in farads
	pub fn to_F(self) -> T {
		return self.F;
	}
}

impl<T> fmt::Display for Capacitance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.F, Self::unit_symbol())
	}
}

impl<T> Capacitance<T> where T: NumLike+From<f64> {
	
	// TODO: Capacitance / Time -> Conductance

	// TODO: Capacitance * Frequency -> Conductance

	// TODO: Capacitance * Voltage -> Charge

	// TODO: Capacitance * Resistance -> Time

	// TODO: Capacitance / Conductance -> Time

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
	
	/// Returns this inductance value in henries
	pub fn to_H(self) -> T {
		return self.H;
	}
}

impl<T> fmt::Display for Inductance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.H, Self::unit_symbol())
	}
}

impl<T> Inductance<T> where T: NumLike+From<f64> {
	
	// TODO: Inductance / Time -> Resistance

	// TODO: Inductance * Current -> MagneticFlux

	// TODO: Inductance * Frequency -> Resistance

	// TODO: Inductance / Resistance -> Time

	// TODO: Inductance * Conductance -> Time

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
	
	/// Returns this magnetic flux value in webers
	pub fn to_Wb(self) -> T {
		return self.Wb;
	}
}

impl<T> fmt::Display for MagneticFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.Wb, Self::unit_symbol())
	}
}

impl<T> MagneticFlux<T> where T: NumLike+From<f64> {
	
	// TODO: MagneticFlux / Time -> Voltage

	// TODO: MagneticFlux * Current -> Energy

	// TODO: MagneticFlux / Current -> Inductance

	// TODO: MagneticFlux * Frequency -> Voltage

	// TODO: MagneticFlux / Area -> MagneticFluxDensity

	// TODO: MagneticFlux / Charge -> Resistance

	// TODO: MagneticFlux / Voltage -> Time

	// TODO: MagneticFlux / Resistance -> Charge

	// TODO: MagneticFlux * Conductance -> Charge

	// TODO: MagneticFlux / Inductance -> Current

	// TODO: MagneticFlux / MagneticFluxDensity -> Area

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
	
	/// Returns this magnetic flux density value in teslas
	pub fn to_T(self) -> T {
		return self.T;
	}
}

impl<T> fmt::Display for MagneticFluxDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.T, Self::unit_symbol())
	}
}

impl<T> MagneticFluxDensity<T> where T: NumLike+From<f64> {
	
	// TODO: MagneticFluxDensity * Area -> MagneticFlux

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
	
	/// Returns this luminous flux value in lumens
	pub fn to_lm(self) -> T {
		return self.lm;
	}
}

impl<T> fmt::Display for LuminousFlux<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.lm, Self::unit_symbol())
	}
}

impl<T> LuminousFlux<T> where T: NumLike+From<f64> {
	
	// TODO: LuminousFlux / Luminosity -> SolidAngle

	// TODO: LuminousFlux / SolidAngle -> Luminosity

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
	
	/// Returns this illuminance value in lux
	pub fn to_lux(self) -> T {
		return self.lux;
	}
}

impl<T> fmt::Display for Illuminance<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.lux, Self::unit_symbol())
	}
}

impl<T> Illuminance<T> where T: NumLike+From<f64> {
	
}


