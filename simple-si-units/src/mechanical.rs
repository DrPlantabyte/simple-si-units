
//! This module provides mechanical SI units, such as angular velocity 
//! and velocity.
use std::fmt;


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
	
	/// Returns this angular velocity value in radians per second
	pub fn to_radps(self) -> T {
		return self.radps;
	}
}

impl<T> fmt::Display for AngularVelocity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.radps, Self::unit_symbol())
	}
}

impl<T> AngularVelocity<T> where T: NumLike+From<f64> {
	
	// TODO: AngularVelocity * Time -> Angle

	// TODO: AngularVelocity / Time -> AngularAcceleration

	// TODO: AngularVelocity / Angle -> Frequency

	// TODO: AngularVelocity / AngularAcceleration -> Time

	// TODO: AngularVelocity * MomentOfInertia -> AngularMomentum

	// TODO: AngularVelocity * AreaDensity -> AngularMomentum

	// TODO: AngularVelocity * Frequency -> AngularAcceleration

	// TODO: AngularVelocity / Frequency -> Angle

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
	
	/// Returns this angular acceleration value in radians per second squared
	pub fn to_radps2(self) -> T {
		return self.radps2;
	}
}

impl<T> fmt::Display for AngularAcceleration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.radps2, Self::unit_symbol())
	}
}

impl<T> AngularAcceleration<T> where T: NumLike+From<f64> {
	
	// TODO: AngularAcceleration * Time -> AngularVelocity

	// TODO: AngularAcceleration / AngularVelocity -> Frequency

	// TODO: AngularAcceleration / Frequency -> AngularVelocity

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
	
	/// Returns this moment of inertia value in kilogram meters squared
	pub fn to_kgm2(self) -> T {
		return self.kgm2;
	}
}

impl<T> fmt::Display for MomentOfInertia<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.kgm2, Self::unit_symbol())
	}
}

impl<T> MomentOfInertia<T> where T: NumLike+From<f64> {
	
	// TODO: MomentOfInertia / Mass -> Area

	// TODO: MomentOfInertia * AngularVelocity -> AngularMomentum

	// TODO: MomentOfInertia / Area -> Mass

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
	
	/// Returns this angular momentum value in kilogram meters squared radians per second
	pub fn to_kgm2radps(self) -> T {
		return self.kgm2radps;
	}
}

impl<T> fmt::Display for AngularMomentum<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.kgm2radps, Self::unit_symbol())
	}
}

impl<T> AngularMomentum<T> where T: NumLike+From<f64> {
	
	// TODO: AngularMomentum / AngularVelocity -> AreaDensity

	// TODO: AngularMomentum / MomentOfInertia -> AngularVelocity

	// TODO: AngularMomentum / AreaDensity -> AngularVelocity

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
	
	/// Returns this torque value in newton meters
	pub fn to_Nm(self) -> T {
		return self.Nm;
	}
}

impl<T> fmt::Display for Torque<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.Nm, Self::unit_symbol())
	}
}

impl<T> Torque<T> where T: NumLike+From<f64> {
	
	// TODO: Torque / Distance -> Force

	// TODO: Torque / Mass -> DoseEquivalent

	// TODO: Torque / Time -> Power

	// TODO: Torque / Current -> MagneticFlux

	// TODO: Torque * Frequency -> Power

	// TODO: Torque / Volume -> Pressure

	// TODO: Torque / Velocity -> Momentum

	// TODO: Torque / Momentum -> Velocity

	// TODO: Torque / Force -> Distance

	// TODO: Torque / Pressure -> Volume

	// TODO: Torque / Charge -> Voltage

	// TODO: Torque / Power -> Time

	// TODO: Torque / Voltage -> Charge

	// TODO: Torque / MagneticFlux -> Current

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
	
	/// Returns this frequency value in hertz
	pub fn to_Hz(self) -> T {
		return self.Hz;
	}
}

impl<T> fmt::Display for Frequency<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.Hz, Self::unit_symbol())
	}
}

impl<T> Frequency<T> where T: NumLike+From<f64> {
	
	// TODO: Frequency * Distance -> Velocity

	// TODO: Frequency * Amount -> CatalyticActivity

	// TODO: Frequency * Angle -> AngularVelocity

	// TODO: Frequency * AngularVelocity -> AngularAcceleration

	// TODO: Frequency * Torque -> Power

	// TODO: Frequency * Energy -> Power

	// TODO: Frequency * Velocity -> Acceleration

	// TODO: Frequency * Momentum -> Force

	// TODO: Frequency * Charge -> Current

	// TODO: Frequency * Capacitance -> Conductance

	// TODO: Frequency * Inductance -> Resistance

	// TODO: Frequency * MagneticFlux -> Voltage

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
	
	/// Returns this area density value in kilograms per square meter
	pub fn to_kgm2(self) -> T {
		return self.kgm2;
	}
}

impl<T> fmt::Display for AreaDensity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.kgm2, Self::unit_symbol())
	}
}

impl<T> AreaDensity<T> where T: NumLike+From<f64> {
	
	// TODO: AreaDensity / Mass -> Area

	// TODO: AreaDensity * AngularVelocity -> AngularMomentum

	// TODO: AreaDensity / Area -> Mass

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
	
	/// Returns this density value in kilograms per liter
	pub fn to_kgpL(self) -> T {
		return self.kgpL;
	}
}

impl<T> fmt::Display for Density<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.kgpL, Self::unit_symbol())
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
	
	/// Returns this velocity value in meters per second
	pub fn to_mps(self) -> T {
		return self.mps;
	}
}

impl<T> fmt::Display for Velocity<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.mps, Self::unit_symbol())
	}
}

impl<T> Velocity<T> where T: NumLike+From<f64> {
	
	// TODO: Velocity / Distance -> Frequency

	// TODO: Velocity * Mass -> Momentum

	// TODO: Velocity * Time -> Distance

	// TODO: Velocity / Time -> Acceleration

	// TODO: Velocity * Frequency -> Acceleration

	// TODO: Velocity / Frequency -> Distance

	// TODO: Velocity * Velocity -> DoseEquivalent

	// TODO: Velocity / Acceleration -> Time

	// TODO: Velocity * Momentum -> Energy

	// TODO: Velocity * Force -> Power

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
	
	/// Returns this acceleration value in meters per second squared
	pub fn to_mps2(self) -> T {
		return self.mps2;
	}
}

impl<T> fmt::Display for Acceleration<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.mps2, Self::unit_symbol())
	}
}

impl<T> Acceleration<T> where T: NumLike+From<f64> {
	
	// TODO: Acceleration * Distance -> DoseEquivalent

	// TODO: Acceleration * Mass -> Force

	// TODO: Acceleration * Time -> Velocity

	// TODO: Acceleration / Frequency -> Velocity

	// TODO: Acceleration / Velocity -> Frequency

	// TODO: Acceleration * Momentum -> Power

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
	
	/// Returns this momentum value in kilogram meters per second
	pub fn to_kgmps(self) -> T {
		return self.kgmps;
	}
}

impl<T> fmt::Display for Momentum<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.kgmps, Self::unit_symbol())
	}
}

impl<T> Momentum<T> where T: NumLike+From<f64> {
	
	// TODO: Momentum / Mass -> Velocity

	// TODO: Momentum / Time -> Force

	// TODO: Momentum * Frequency -> Force

	// TODO: Momentum * Velocity -> Energy

	// TODO: Momentum / Velocity -> Mass

	// TODO: Momentum * Acceleration -> Power

	// TODO: Momentum / Force -> Time

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
	
	/// Returns this force value in newtons
	pub fn to_N(self) -> T {
		return self.N;
	}
}

impl<T> fmt::Display for Force<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.N, Self::unit_symbol())
	}
}

impl<T> Force<T> where T: NumLike+From<f64> {
	
	// TODO: Force * Distance -> Energy

	// TODO: Force / Mass -> Acceleration

	// TODO: Force * Time -> Momentum

	// TODO: Force / Frequency -> Momentum

	// TODO: Force / Area -> Pressure

	// TODO: Force * Velocity -> Power

	// TODO: Force / Acceleration -> Mass

	// TODO: Force / Momentum -> Frequency

	// TODO: Force / Pressure -> Area

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
	
	/// Returns this pressure value in pascals
	pub fn to_Pa(self) -> T {
		return self.Pa;
	}
}

impl<T> fmt::Display for Pressure<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.Pa, Self::unit_symbol())
	}
}

impl<T> Pressure<T> where T: NumLike+From<f64> {
	
	// TODO: Pressure * Area -> Force

	// TODO: Pressure * Volume -> Energy

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
	
	/// Returns this energy value in joules
	pub fn to_J(self) -> T {
		return self.J;
	}
}

impl<T> fmt::Display for Energy<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.J, Self::unit_symbol())
	}
}

impl<T> Energy<T> where T: NumLike+From<f64> {
	
	// TODO: Energy / Distance -> Force

	// TODO: Energy / Mass -> DoseEquivalent

	// TODO: Energy / Time -> Power

	// TODO: Energy / Current -> MagneticFlux

	// TODO: Energy * Frequency -> Power

	// TODO: Energy / Volume -> Pressure

	// TODO: Energy / Velocity -> Momentum

	// TODO: Energy / Momentum -> Velocity

	// TODO: Energy / Force -> Distance

	// TODO: Energy / Pressure -> Volume

	// TODO: Energy / Charge -> Voltage

	// TODO: Energy / Power -> Time

	// TODO: Energy / Voltage -> Charge

	// TODO: Energy / MagneticFlux -> Current

	// TODO: Energy / AbsorbedDose -> Mass

	// TODO: Energy / DoseEquivalent -> Mass

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
	
	/// Returns this power value in watts
	pub fn to_W(self) -> T {
		return self.W;
	}
}

impl<T> fmt::Display for Power<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.W, Self::unit_symbol())
	}
}

impl<T> Power<T> where T: NumLike+From<f64> {
	
	// TODO: Power * Time -> Energy

	// TODO: Power / Current -> Voltage

	// TODO: Power / Torque -> Frequency

	// TODO: Power / Energy -> Frequency

	// TODO: Power / Frequency -> Energy

	// TODO: Power / Velocity -> Force

	// TODO: Power / Acceleration -> Momentum

	// TODO: Power / Momentum -> Acceleration

	// TODO: Power / Force -> Velocity

	// TODO: Power / Voltage -> Current

}


