
//! This module provides base SI units, such as distance (aka length) 
//! and amount.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
use super::chemical::*;
use super::nuclear::*;
use super::mechanical::*;
use super::geometry::*;
use super::electromagnetic::*;

// optional supports
#[cfg(feature="serde")]
#[macro_use]
extern crate serde;


/// The distance (aka length) unit type, defined as meters in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Distance<T: NumLike>{
	/// The value of this Distance in meters
	pub m: T
}

impl<T> Distance<T> where T: NumLike {

	/// Returns the standard unit name of distance: "meters"
	pub fn unit_name() -> &'static str {
		return "meters";
	}
	
	/// Returns the abbreviated name or symbol of distance: "m" for meters
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
	
	/// Returns a copy of this distance value in meters
	pub fn to_meters(self) -> T {
		return self.m.clone() / T::from(1.0_f64);
	}

	/// Returns a new distance value from the given number of meters
	///
	/// # Arguments
	/// * `meters` - Any number-like type, representing a quantity of meters
	pub fn from_meters(meters: T) -> Self {
		Distance{m: meters * T::from(1.0_f64)}
	}

	/// Returns a copy of this distance value in millimeters
	pub fn to_cm(self) -> T {
		return self.m.clone() / T::from(0.01_f64);
	}

	/// Returns a new distance value from the given number of millimeters
	///
	/// # Arguments
	/// * `cm` - Any number-like type, representing a quantity of millimeters
	pub fn from_cm(cm: T) -> Self {
		Distance{m: cm * T::from(0.01_f64)}
	}

	/// Returns a copy of this distance value in millimeters
	pub fn to_mm(self) -> T {
		return self.m.clone() / T::from(0.001_f64);
	}

	/// Returns a new distance value from the given number of millimeters
	///
	/// # Arguments
	/// * `mm` - Any number-like type, representing a quantity of millimeters
	pub fn from_mm(mm: T) -> Self {
		Distance{m: mm * T::from(0.001_f64)}
	}

	/// Returns a copy of this distance value in micrometers
	pub fn to_um(self) -> T {
		return self.m.clone() / T::from(1e-06_f64);
	}

	/// Returns a new distance value from the given number of micrometers
	///
	/// # Arguments
	/// * `um` - Any number-like type, representing a quantity of micrometers
	pub fn from_um(um: T) -> Self {
		Distance{m: um * T::from(1e-06_f64)}
	}

	/// Returns a copy of this distance value in nanometers
	pub fn to_nm(self) -> T {
		return self.m.clone() / T::from(1e-09_f64);
	}

	/// Returns a new distance value from the given number of nanometers
	///
	/// # Arguments
	/// * `nm` - Any number-like type, representing a quantity of nanometers
	pub fn from_nm(nm: T) -> Self {
		Distance{m: nm * T::from(1e-09_f64)}
	}

	/// Returns a copy of this distance value in picometers
	pub fn to_pm(self) -> T {
		return self.m.clone() / T::from(1e-12_f64);
	}

	/// Returns a new distance value from the given number of picometers
	///
	/// # Arguments
	/// * `pm` - Any number-like type, representing a quantity of picometers
	pub fn from_pm(pm: T) -> Self {
		Distance{m: pm * T::from(1e-12_f64)}
	}

	/// Returns a copy of this distance value in kilometers
	pub fn to_km(self) -> T {
		return self.m.clone() / T::from(1000.0_f64);
	}

	/// Returns a new distance value from the given number of kilometers
	///
	/// # Arguments
	/// * `km` - Any number-like type, representing a quantity of kilometers
	pub fn from_km(km: T) -> Self {
		Distance{m: km * T::from(1000.0_f64)}
	}

	/// Returns a copy of this distance value in astronomical units
	pub fn to_au(self) -> T {
		return self.m.clone() / T::from(149597870700.0_f64);
	}

	/// Returns a new distance value from the given number of astronomical units
	///
	/// # Arguments
	/// * `au` - Any number-like type, representing a quantity of astronomical units
	pub fn from_au(au: T) -> Self {
		Distance{m: au * T::from(149597870700.0_f64)}
	}

	/// Returns a copy of this distance value in parsecs
	pub fn to_parsec(self) -> T {
		return self.m.clone() / T::from(3.08568047999355e+16_f64);
	}

	/// Returns a new distance value from the given number of parsecs
	///
	/// # Arguments
	/// * `parsec` - Any number-like type, representing a quantity of parsecs
	pub fn from_parsec(parsec: T) -> Self {
		Distance{m: parsec * T::from(3.08568047999355e+16_f64)}
	}

	/// Returns a copy of this distance value in light-years
	pub fn to_lyr(self) -> T {
		return self.m.clone() / T::from(9460528169656200.0_f64);
	}

	/// Returns a new distance value from the given number of light-years
	///
	/// # Arguments
	/// * `lyr` - Any number-like type, representing a quantity of light-years
	pub fn from_lyr(lyr: T) -> Self {
		Distance{m: lyr * T::from(9460528169656200.0_f64)}
	}

}

// Distance * Distance -> Area
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> std::ops::Mul<Distance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m * rhs.m}
	}
}
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> std::ops::Mul<Distance<T>> for &Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m.clone() * rhs.m}
	}
}
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> std::ops::Mul<&Distance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m * rhs.m.clone()}
	}
}
/// Multiplying a Distance by a Distance returns a value of type Area
impl<T> std::ops::Mul<&Distance<T>> for &Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: &Distance<T>) -> Self::Output {
		Area{m2: self.m.clone() * rhs.m.clone()}
	}
}

// Distance / Time -> Velocity
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> std::ops::Div<Time<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.m / rhs.s}
	}
}
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> std::ops::Div<Time<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.m.clone() / rhs.s}
	}
}
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> std::ops::Div<&Time<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Velocity{mps: self.m / rhs.s.clone()}
	}
}
/// Dividing a Distance by a Time returns a value of type Velocity
impl<T> std::ops::Div<&Time<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		Velocity{mps: self.m.clone() / rhs.s.clone()}
	}
}

// Distance * Frequency -> Velocity
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> std::ops::Mul<Frequency<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Velocity{mps: self.m * rhs.Hz}
	}
}
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> std::ops::Mul<Frequency<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		Velocity{mps: self.m.clone() * rhs.Hz}
	}
}
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> std::ops::Mul<&Frequency<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Velocity{mps: self.m * rhs.Hz.clone()}
	}
}
/// Multiplying a Distance by a Frequency returns a value of type Velocity
impl<T> std::ops::Mul<&Frequency<T>> for &Distance<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		Velocity{mps: self.m.clone() * rhs.Hz.clone()}
	}
}

// Distance * Area -> Volume
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> std::ops::Mul<Area<T>> for Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Volume{m3: self.m * rhs.m2}
	}
}
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> std::ops::Mul<Area<T>> for &Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		Volume{m3: self.m.clone() * rhs.m2}
	}
}
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> std::ops::Mul<&Area<T>> for Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Volume{m3: self.m * rhs.m2.clone()}
	}
}
/// Multiplying a Distance by a Area returns a value of type Volume
impl<T> std::ops::Mul<&Area<T>> for &Distance<T> where T: NumLike {
	type Output = Volume<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		Volume{m3: self.m.clone() * rhs.m2.clone()}
	}
}

// Distance / Velocity -> Time
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> std::ops::Div<Velocity<T>> for Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Time{s: self.m / rhs.mps}
	}
}
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> std::ops::Div<Velocity<T>> for &Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: Velocity<T>) -> Self::Output {
		Time{s: self.m.clone() / rhs.mps}
	}
}
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> std::ops::Div<&Velocity<T>> for Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Time{s: self.m / rhs.mps.clone()}
	}
}
/// Dividing a Distance by a Velocity returns a value of type Time
impl<T> std::ops::Div<&Velocity<T>> for &Distance<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &Velocity<T>) -> Self::Output {
		Time{s: self.m.clone() / rhs.mps.clone()}
	}
}

// Distance * Force -> Energy
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> std::ops::Mul<Force<T>> for Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Energy{J: self.m * rhs.N}
	}
}
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> std::ops::Mul<Force<T>> for &Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Energy{J: self.m.clone() * rhs.N}
	}
}
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> std::ops::Mul<&Force<T>> for Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Energy{J: self.m * rhs.N.clone()}
	}
}
/// Multiplying a Distance by a Force returns a value of type Energy
impl<T> std::ops::Mul<&Force<T>> for &Distance<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Energy{J: self.m.clone() * rhs.N.clone()}
	}
}

/// The mass unit type, defined as kilograms in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Mass<T: NumLike>{
	/// The value of this Mass in kilograms
	pub kg: T
}

impl<T> Mass<T> where T: NumLike {

	/// Returns the standard unit name of mass: "kilograms"
	pub fn unit_name() -> &'static str {
		return "kilograms";
	}
	
	/// Returns the abbreviated name or symbol of mass: "kg" for kilograms
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
	
	/// Returns a copy of this mass value in kilograms
	pub fn to_kilograms(self) -> T {
		return self.kg.clone() / T::from(1.0_f64);
	}

	/// Returns a new mass value from the given number of kilograms
	///
	/// # Arguments
	/// * `kilograms` - Any number-like type, representing a quantity of kilograms
	pub fn from_kilograms(kilograms: T) -> Self {
		Mass{kg: kilograms * T::from(1.0_f64)}
	}

	/// Returns a copy of this mass value in grams
	pub fn to_g(self) -> T {
		return self.kg.clone() / T::from(0.001_f64);
	}

	/// Returns a new mass value from the given number of grams
	///
	/// # Arguments
	/// * `g` - Any number-like type, representing a quantity of grams
	pub fn from_g(g: T) -> Self {
		Mass{kg: g * T::from(0.001_f64)}
	}

	/// Returns a copy of this mass value in milligrams
	pub fn to_mg(self) -> T {
		return self.kg.clone() / T::from(1e-06_f64);
	}

	/// Returns a new mass value from the given number of milligrams
	///
	/// # Arguments
	/// * `mg` - Any number-like type, representing a quantity of milligrams
	pub fn from_mg(mg: T) -> Self {
		Mass{kg: mg * T::from(1e-06_f64)}
	}

	/// Returns a copy of this mass value in micrograms
	pub fn to_ug(self) -> T {
		return self.kg.clone() / T::from(1e-09_f64);
	}

	/// Returns a new mass value from the given number of micrograms
	///
	/// # Arguments
	/// * `ug` - Any number-like type, representing a quantity of micrograms
	pub fn from_ug(ug: T) -> Self {
		Mass{kg: ug * T::from(1e-09_f64)}
	}

	/// Returns a copy of this mass value in nanograms
	pub fn to_ng(self) -> T {
		return self.kg.clone() / T::from(1e-12_f64);
	}

	/// Returns a new mass value from the given number of nanograms
	///
	/// # Arguments
	/// * `ng` - Any number-like type, representing a quantity of nanograms
	pub fn from_ng(ng: T) -> Self {
		Mass{kg: ng * T::from(1e-12_f64)}
	}

	/// Returns a copy of this mass value in picograms
	pub fn to_pg(self) -> T {
		return self.kg.clone() / T::from(1e-15_f64);
	}

	/// Returns a new mass value from the given number of picograms
	///
	/// # Arguments
	/// * `pg` - Any number-like type, representing a quantity of picograms
	pub fn from_pg(pg: T) -> Self {
		Mass{kg: pg * T::from(1e-15_f64)}
	}

	/// Returns a copy of this mass value in tons
	pub fn to_ton(self) -> T {
		return self.kg.clone() / T::from(1000.0_f64);
	}

	/// Returns a new mass value from the given number of tons
	///
	/// # Arguments
	/// * `ton` - Any number-like type, representing a quantity of tons
	pub fn from_ton(ton: T) -> Self {
		Mass{kg: ton * T::from(1000.0_f64)}
	}

	/// Returns a copy of this mass value in earth masses
	pub fn to_earth_mass(self) -> T {
		return self.kg.clone() / T::from(5.9722e+24_f64);
	}

	/// Returns a new mass value from the given number of earth masses
	///
	/// # Arguments
	/// * `earth_mass` - Any number-like type, representing a quantity of earth masses
	pub fn from_earth_mass(earth_mass: T) -> Self {
		Mass{kg: earth_mass * T::from(5.9722e+24_f64)}
	}

	/// Returns a copy of this mass value in jupiter masses
	pub fn to_jupiter_mass(self) -> T {
		return self.kg.clone() / T::from(1.8986e+27_f64);
	}

	/// Returns a new mass value from the given number of jupiter masses
	///
	/// # Arguments
	/// * `jupiter_mass` - Any number-like type, representing a quantity of jupiter masses
	pub fn from_jupiter_mass(jupiter_mass: T) -> Self {
		Mass{kg: jupiter_mass * T::from(1.8986e+27_f64)}
	}

	/// Returns a copy of this mass value in solar masses
	pub fn to_solar_mass(self) -> T {
		return self.kg.clone() / T::from(1.9885500000000002e+30_f64);
	}

	/// Returns a new mass value from the given number of solar masses
	///
	/// # Arguments
	/// * `solar_mass` - Any number-like type, representing a quantity of solar masses
	pub fn from_solar_mass(solar_mass: T) -> Self {
		Mass{kg: solar_mass * T::from(1.9885500000000002e+30_f64)}
	}

}

// Mass * Area -> AreaDensity
/// Multiplying a Mass by a Area returns a value of type AreaDensity
impl<T> std::ops::Mul<Area<T>> for Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		AreaDensity{kgm2: self.kg * rhs.m2}
	}
}
/// Multiplying a Mass by a Area returns a value of type AreaDensity
impl<T> std::ops::Mul<Area<T>> for &Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: Area<T>) -> Self::Output {
		AreaDensity{kgm2: self.kg.clone() * rhs.m2}
	}
}
/// Multiplying a Mass by a Area returns a value of type AreaDensity
impl<T> std::ops::Mul<&Area<T>> for Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		AreaDensity{kgm2: self.kg * rhs.m2.clone()}
	}
}
/// Multiplying a Mass by a Area returns a value of type AreaDensity
impl<T> std::ops::Mul<&Area<T>> for &Mass<T> where T: NumLike {
	type Output = AreaDensity<T>;
	fn mul(self, rhs: &Area<T>) -> Self::Output {
		AreaDensity{kgm2: self.kg.clone() * rhs.m2.clone()}
	}
}

// Mass * Velocity -> Momentum
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> std::ops::Mul<Velocity<T>> for Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg * rhs.mps}
	}
}
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> std::ops::Mul<Velocity<T>> for &Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg.clone() * rhs.mps}
	}
}
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> std::ops::Mul<&Velocity<T>> for Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg * rhs.mps.clone()}
	}
}
/// Multiplying a Mass by a Velocity returns a value of type Momentum
impl<T> std::ops::Mul<&Velocity<T>> for &Mass<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Momentum{kgmps: self.kg.clone() * rhs.mps.clone()}
	}
}

// Mass * Acceleration -> Force
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> std::ops::Mul<Acceleration<T>> for Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Force{N: self.kg * rhs.mps2}
	}
}
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> std::ops::Mul<Acceleration<T>> for &Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Force{N: self.kg.clone() * rhs.mps2}
	}
}
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> std::ops::Mul<&Acceleration<T>> for Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Force{N: self.kg * rhs.mps2.clone()}
	}
}
/// Multiplying a Mass by a Acceleration returns a value of type Force
impl<T> std::ops::Mul<&Acceleration<T>> for &Mass<T> where T: NumLike {
	type Output = Force<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Force{N: self.kg.clone() * rhs.mps2.clone()}
	}
}

// Mass * AbsorbedDose -> Energy
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> std::ops::Mul<AbsorbedDose<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Gy}
	}
}
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> std::ops::Mul<AbsorbedDose<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Gy}
	}
}
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> std::ops::Mul<&AbsorbedDose<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Gy.clone()}
	}
}
/// Multiplying a Mass by a AbsorbedDose returns a value of type Energy
impl<T> std::ops::Mul<&AbsorbedDose<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &AbsorbedDose<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Gy.clone()}
	}
}

// Mass * DoseEquivalent -> Energy
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> std::ops::Mul<DoseEquivalent<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Sv}
	}
}
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> std::ops::Mul<DoseEquivalent<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Sv}
	}
}
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> std::ops::Mul<&DoseEquivalent<T>> for Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg * rhs.Sv.clone()}
	}
}
/// Multiplying a Mass by a DoseEquivalent returns a value of type Energy
impl<T> std::ops::Mul<&DoseEquivalent<T>> for &Mass<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &DoseEquivalent<T>) -> Self::Output {
		Energy{J: self.kg.clone() * rhs.Sv.clone()}
	}
}

/// The time unit type, defined as seconds in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Time<T: NumLike>{
	/// The value of this Time in seconds
	pub s: T
}

impl<T> Time<T> where T: NumLike {

	/// Returns the standard unit name of time: "seconds"
	pub fn unit_name() -> &'static str {
		return "seconds";
	}
	
	/// Returns the abbreviated name or symbol of time: "s" for seconds
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
	
	/// Returns a copy of this time value in seconds
	pub fn to_seconds(self) -> T {
		return self.s.clone() / T::from(1.0_f64);
	}

	/// Returns a new time value from the given number of seconds
	///
	/// # Arguments
	/// * `seconds` - Any number-like type, representing a quantity of seconds
	pub fn from_seconds(seconds: T) -> Self {
		Time{s: seconds * T::from(1.0_f64)}
	}

	/// Returns a copy of this time value in milliseconds
	pub fn to_ms(self) -> T {
		return self.s.clone() / T::from(0.001_f64);
	}

	/// Returns a new time value from the given number of milliseconds
	///
	/// # Arguments
	/// * `ms` - Any number-like type, representing a quantity of milliseconds
	pub fn from_ms(ms: T) -> Self {
		Time{s: ms * T::from(0.001_f64)}
	}

	/// Returns a copy of this time value in microseconds
	pub fn to_us(self) -> T {
		return self.s.clone() / T::from(1e-06_f64);
	}

	/// Returns a new time value from the given number of microseconds
	///
	/// # Arguments
	/// * `us` - Any number-like type, representing a quantity of microseconds
	pub fn from_us(us: T) -> Self {
		Time{s: us * T::from(1e-06_f64)}
	}

	/// Returns a copy of this time value in nanoseconds
	pub fn to_ns(self) -> T {
		return self.s.clone() / T::from(1e-09_f64);
	}

	/// Returns a new time value from the given number of nanoseconds
	///
	/// # Arguments
	/// * `ns` - Any number-like type, representing a quantity of nanoseconds
	pub fn from_ns(ns: T) -> Self {
		Time{s: ns * T::from(1e-09_f64)}
	}

	/// Returns a copy of this time value in picoseconds
	pub fn to_ps(self) -> T {
		return self.s.clone() / T::from(1e-12_f64);
	}

	/// Returns a new time value from the given number of picoseconds
	///
	/// # Arguments
	/// * `ps` - Any number-like type, representing a quantity of picoseconds
	pub fn from_ps(ps: T) -> Self {
		Time{s: ps * T::from(1e-12_f64)}
	}

	/// Returns a copy of this time value in minutes
	pub fn to_min(self) -> T {
		return self.s.clone() / T::from(60.0_f64);
	}

	/// Returns a new time value from the given number of minutes
	///
	/// # Arguments
	/// * `min` - Any number-like type, representing a quantity of minutes
	pub fn from_min(min: T) -> Self {
		Time{s: min * T::from(60.0_f64)}
	}

	/// Returns a copy of this time value in hours
	pub fn to_hr(self) -> T {
		return self.s.clone() / T::from(3600.0_f64);
	}

	/// Returns a new time value from the given number of hours
	///
	/// # Arguments
	/// * `hr` - Any number-like type, representing a quantity of hours
	pub fn from_hr(hr: T) -> Self {
		Time{s: hr * T::from(3600.0_f64)}
	}

	/// Returns a copy of this time value in days
	pub fn to_d(self) -> T {
		return self.s.clone() / T::from(86400.0_f64);
	}

	/// Returns a new time value from the given number of days
	///
	/// # Arguments
	/// * `d` - Any number-like type, representing a quantity of days
	pub fn from_d(d: T) -> Self {
		Time{s: d * T::from(86400.0_f64)}
	}

	/// Returns a copy of this time value in years
	pub fn to_yr(self) -> T {
		return self.s.clone() / T::from(31556925.19008_f64);
	}

	/// Returns a new time value from the given number of years
	///
	/// # Arguments
	/// * `yr` - Any number-like type, representing a quantity of years
	pub fn from_yr(yr: T) -> Self {
		Time{s: yr * T::from(31556925.19008_f64)}
	}

	/// Returns a copy of this time value in millennia
	pub fn to_kyr(self) -> T {
		return self.s.clone() / T::from(31556925190.08_f64);
	}

	/// Returns a new time value from the given number of millennia
	///
	/// # Arguments
	/// * `kyr` - Any number-like type, representing a quantity of millennia
	pub fn from_kyr(kyr: T) -> Self {
		Time{s: kyr * T::from(31556925190.08_f64)}
	}

	/// Returns a copy of this time value in million years
	pub fn to_Myr(self) -> T {
		return self.s.clone() / T::from(31556925190080.0_f64);
	}

	/// Returns a new time value from the given number of million years
	///
	/// # Arguments
	/// * `Myr` - Any number-like type, representing a quantity of million years
	pub fn from_Myr(Myr: T) -> Self {
		Time{s: Myr * T::from(31556925190080.0_f64)}
	}

	/// Returns a copy of this time value in billion years
	pub fn to_Gyr(self) -> T {
		return self.s.clone() / T::from(3.155692519008e+16_f64);
	}

	/// Returns a new time value from the given number of billion years
	///
	/// # Arguments
	/// * `Gyr` - Any number-like type, representing a quantity of billion years
	pub fn from_Gyr(Gyr: T) -> Self {
		Time{s: Gyr * T::from(3.155692519008e+16_f64)}
	}

}

// Time * Current -> Charge
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> std::ops::Mul<Current<T>> for Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Charge{C: self.s * rhs.A}
	}
}
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> std::ops::Mul<Current<T>> for &Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Current<T>) -> Self::Output {
		Charge{C: self.s.clone() * rhs.A}
	}
}
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> std::ops::Mul<&Current<T>> for Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Charge{C: self.s * rhs.A.clone()}
	}
}
/// Multiplying a Time by a Current returns a value of type Charge
impl<T> std::ops::Mul<&Current<T>> for &Time<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Current<T>) -> Self::Output {
		Charge{C: self.s.clone() * rhs.A.clone()}
	}
}

// Time * AngularVelocity -> Angle
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> std::ops::Mul<AngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s * rhs.radps}
	}
}
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> std::ops::Mul<AngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s.clone() * rhs.radps}
	}
}
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> std::ops::Mul<&AngularVelocity<T>> for Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s * rhs.radps.clone()}
	}
}
/// Multiplying a Time by a AngularVelocity returns a value of type Angle
impl<T> std::ops::Mul<&AngularVelocity<T>> for &Time<T> where T: NumLike {
	type Output = Angle<T>;
	fn mul(self, rhs: &AngularVelocity<T>) -> Self::Output {
		Angle{rad: self.s.clone() * rhs.radps.clone()}
	}
}

// Time * AngularAcceleration -> AngularVelocity
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> std::ops::Mul<AngularAcceleration<T>> for Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s * rhs.radps2}
	}
}
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> std::ops::Mul<AngularAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s.clone() * rhs.radps2}
	}
}
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> std::ops::Mul<&AngularAcceleration<T>> for Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s * rhs.radps2.clone()}
	}
}
/// Multiplying a Time by a AngularAcceleration returns a value of type AngularVelocity
impl<T> std::ops::Mul<&AngularAcceleration<T>> for &Time<T> where T: NumLike {
	type Output = AngularVelocity<T>;
	fn mul(self, rhs: &AngularAcceleration<T>) -> Self::Output {
		AngularVelocity{radps: self.s.clone() * rhs.radps2.clone()}
	}
}

// Time * Velocity -> Distance
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> std::ops::Mul<Velocity<T>> for Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Distance{m: self.s * rhs.mps}
	}
}
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> std::ops::Mul<Velocity<T>> for &Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Velocity<T>) -> Self::Output {
		Distance{m: self.s.clone() * rhs.mps}
	}
}
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> std::ops::Mul<&Velocity<T>> for Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Distance{m: self.s * rhs.mps.clone()}
	}
}
/// Multiplying a Time by a Velocity returns a value of type Distance
impl<T> std::ops::Mul<&Velocity<T>> for &Time<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: &Velocity<T>) -> Self::Output {
		Distance{m: self.s.clone() * rhs.mps.clone()}
	}
}

// Time * Acceleration -> Velocity
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> std::ops::Mul<Acceleration<T>> for Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s * rhs.mps2}
	}
}
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> std::ops::Mul<Acceleration<T>> for &Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s.clone() * rhs.mps2}
	}
}
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> std::ops::Mul<&Acceleration<T>> for Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s * rhs.mps2.clone()}
	}
}
/// Multiplying a Time by a Acceleration returns a value of type Velocity
impl<T> std::ops::Mul<&Acceleration<T>> for &Time<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: &Acceleration<T>) -> Self::Output {
		Velocity{mps: self.s.clone() * rhs.mps2.clone()}
	}
}

// Time * Force -> Momentum
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> std::ops::Mul<Force<T>> for Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Momentum{kgmps: self.s * rhs.N}
	}
}
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> std::ops::Mul<Force<T>> for &Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: Force<T>) -> Self::Output {
		Momentum{kgmps: self.s.clone() * rhs.N}
	}
}
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> std::ops::Mul<&Force<T>> for Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Momentum{kgmps: self.s * rhs.N.clone()}
	}
}
/// Multiplying a Time by a Force returns a value of type Momentum
impl<T> std::ops::Mul<&Force<T>> for &Time<T> where T: NumLike {
	type Output = Momentum<T>;
	fn mul(self, rhs: &Force<T>) -> Self::Output {
		Momentum{kgmps: self.s.clone() * rhs.N.clone()}
	}
}

// Time * Power -> Energy
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> std::ops::Mul<Power<T>> for Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Energy{J: self.s * rhs.W}
	}
}
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> std::ops::Mul<Power<T>> for &Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: Power<T>) -> Self::Output {
		Energy{J: self.s.clone() * rhs.W}
	}
}
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> std::ops::Mul<&Power<T>> for Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Energy{J: self.s * rhs.W.clone()}
	}
}
/// Multiplying a Time by a Power returns a value of type Energy
impl<T> std::ops::Mul<&Power<T>> for &Time<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &Power<T>) -> Self::Output {
		Energy{J: self.s.clone() * rhs.W.clone()}
	}
}

// Time * Voltage -> MagneticFlux
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> std::ops::Mul<Voltage<T>> for Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s * rhs.V}
	}
}
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> std::ops::Mul<Voltage<T>> for &Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s.clone() * rhs.V}
	}
}
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Voltage<T>> for Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s * rhs.V.clone()}
	}
}
/// Multiplying a Time by a Voltage returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Voltage<T>> for &Time<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		MagneticFlux{Wb: self.s.clone() * rhs.V.clone()}
	}
}

// Time * Resistance -> Inductance
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> std::ops::Mul<Resistance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Inductance{H: self.s * rhs.Ohm}
	}
}
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> std::ops::Mul<Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Inductance{H: self.s.clone() * rhs.Ohm}
	}
}
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> std::ops::Mul<&Resistance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Inductance{H: self.s * rhs.Ohm.clone()}
	}
}
/// Multiplying a Time by a Resistance returns a value of type Inductance
impl<T> std::ops::Mul<&Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Inductance{H: self.s.clone() * rhs.Ohm.clone()}
	}
}

// Time / Resistance -> Capacitance
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> std::ops::Div<Resistance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Capacitance{F: self.s / rhs.Ohm}
	}
}
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> std::ops::Div<Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: Resistance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() / rhs.Ohm}
	}
}
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> std::ops::Div<&Resistance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Capacitance{F: self.s / rhs.Ohm.clone()}
	}
}
/// Dividing a Time by a Resistance returns a value of type Capacitance
impl<T> std::ops::Div<&Resistance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn div(self, rhs: &Resistance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() / rhs.Ohm.clone()}
	}
}

// Time * Conductance -> Capacitance
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> std::ops::Mul<Conductance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Capacitance{F: self.s * rhs.S}
	}
}
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> std::ops::Mul<Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: Conductance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() * rhs.S}
	}
}
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> std::ops::Mul<&Conductance<T>> for Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Capacitance{F: self.s * rhs.S.clone()}
	}
}
/// Multiplying a Time by a Conductance returns a value of type Capacitance
impl<T> std::ops::Mul<&Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Capacitance<T>;
	fn mul(self, rhs: &Conductance<T>) -> Self::Output {
		Capacitance{F: self.s.clone() * rhs.S.clone()}
	}
}

// Time / Conductance -> Inductance
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> std::ops::Div<Conductance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Inductance{H: self.s / rhs.S}
	}
}
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> std::ops::Div<Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Inductance{H: self.s.clone() / rhs.S}
	}
}
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> std::ops::Div<&Conductance<T>> for Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Inductance{H: self.s / rhs.S.clone()}
	}
}
/// Dividing a Time by a Conductance returns a value of type Inductance
impl<T> std::ops::Div<&Conductance<T>> for &Time<T> where T: NumLike {
	type Output = Inductance<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Inductance{H: self.s.clone() / rhs.S.clone()}
	}
}

// Time / Capacitance -> Resistance
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> std::ops::Div<Capacitance<T>> for Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s / rhs.F}
	}
}
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> std::ops::Div<Capacitance<T>> for &Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s.clone() / rhs.F}
	}
}
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> std::ops::Div<&Capacitance<T>> for Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s / rhs.F.clone()}
	}
}
/// Dividing a Time by a Capacitance returns a value of type Resistance
impl<T> std::ops::Div<&Capacitance<T>> for &Time<T> where T: NumLike {
	type Output = Resistance<T>;
	fn div(self, rhs: &Capacitance<T>) -> Self::Output {
		Resistance{Ohm: self.s.clone() / rhs.F.clone()}
	}
}

// Time / Inductance -> Conductance
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> std::ops::Div<Inductance<T>> for Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Conductance{S: self.s / rhs.H}
	}
}
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> std::ops::Div<Inductance<T>> for &Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Inductance<T>) -> Self::Output {
		Conductance{S: self.s.clone() / rhs.H}
	}
}
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> std::ops::Div<&Inductance<T>> for Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Conductance{S: self.s / rhs.H.clone()}
	}
}
/// Dividing a Time by a Inductance returns a value of type Conductance
impl<T> std::ops::Div<&Inductance<T>> for &Time<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Inductance<T>) -> Self::Output {
		Conductance{S: self.s.clone() / rhs.H.clone()}
	}
}

// Time * CatalyticActivity -> Amount
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> std::ops::Mul<CatalyticActivity<T>> for Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s * rhs.molps}
	}
}
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> std::ops::Mul<CatalyticActivity<T>> for &Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s.clone() * rhs.molps}
	}
}
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> std::ops::Mul<&CatalyticActivity<T>> for Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s * rhs.molps.clone()}
	}
}
/// Multiplying a Time by a CatalyticActivity returns a value of type Amount
impl<T> std::ops::Mul<&CatalyticActivity<T>> for &Time<T> where T: NumLike {
	type Output = Amount<T>;
	fn mul(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Amount{mol: self.s.clone() * rhs.molps.clone()}
	}
}

/// The temperature unit type, defined as degrees kelvin in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Temperature<T: NumLike>{
	/// The value of this Temperature in degrees kelvin
	pub K: T
}

impl<T> Temperature<T> where T: NumLike {

	/// Returns the standard unit name of temperature: "degrees kelvin"
	pub fn unit_name() -> &'static str {
		return "degrees kelvin";
	}
	
	/// Returns the abbreviated name or symbol of temperature: "K" for degrees kelvin
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
	
	/// Returns a copy of this temperature value in degrees celsius
	pub fn to_C(self) -> T {
		return (self.K.clone() - T::from(273.15_f64)) / T::from(1.0_f64);
	}

	/// Returns a new temperature value from the given number of degrees celsius
	///
	/// # Arguments
	/// * `C` - Any number-like type, representing a quantity of degrees celsius
	pub fn from_C(C: T) -> Self {
		Temperature{K: C * T::from(1.0_f64) + T::from(273.15_f64)}
	}

	/// Returns a copy of this temperature value in degrees celsius
	pub fn to_celsius(self) -> T {
		return (self.K.clone() - T::from(273.15_f64)) / T::from(1.0_f64);
	}

	/// Returns a new temperature value from the given number of degrees celsius
	///
	/// # Arguments
	/// * `celsius` - Any number-like type, representing a quantity of degrees celsius
	pub fn from_celsius(celsius: T) -> Self {
		Temperature{K: celsius * T::from(1.0_f64) + T::from(273.15_f64)}
	}

	/// Returns a copy of this temperature value in degrees fahrenheit
	pub fn to_F(self) -> T {
		return (self.K.clone() - T::from(459.67_f64)) / T::from(0.555555555555556_f64);
	}

	/// Returns a new temperature value from the given number of degrees fahrenheit
	///
	/// # Arguments
	/// * `F` - Any number-like type, representing a quantity of degrees fahrenheit
	pub fn from_F(F: T) -> Self {
		Temperature{K: F * T::from(0.555555555555556_f64) + T::from(459.67_f64)}
	}

}

/// The amount unit type, defined as moles in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Amount<T: NumLike>{
	/// The value of this Amount in moles
	pub mol: T
}

impl<T> Amount<T> where T: NumLike {

	/// Returns the standard unit name of amount: "moles"
	pub fn unit_name() -> &'static str {
		return "moles";
	}
	
	/// Returns the abbreviated name or symbol of amount: "mol" for moles
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
	
}

// Amount / Time -> CatalyticActivity
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> std::ops::Div<Time<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol / rhs.s}
	}
}
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> std::ops::Div<Time<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() / rhs.s}
	}
}
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> std::ops::Div<&Time<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol / rhs.s.clone()}
	}
}
/// Dividing a Amount by a Time returns a value of type CatalyticActivity
impl<T> std::ops::Div<&Time<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn div(self, rhs: &Time<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() / rhs.s.clone()}
	}
}

// Amount * Frequency -> CatalyticActivity
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> std::ops::Mul<Frequency<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol * rhs.Hz}
	}
}
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> std::ops::Mul<Frequency<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() * rhs.Hz}
	}
}
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> std::ops::Mul<&Frequency<T>> for Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol * rhs.Hz.clone()}
	}
}
/// Multiplying a Amount by a Frequency returns a value of type CatalyticActivity
impl<T> std::ops::Mul<&Frequency<T>> for &Amount<T> where T: NumLike {
	type Output = CatalyticActivity<T>;
	fn mul(self, rhs: &Frequency<T>) -> Self::Output {
		CatalyticActivity{molps: self.mol.clone() * rhs.Hz.clone()}
	}
}

// Amount / Volume -> Concentration
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> std::ops::Div<Volume<T>> for Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol / rhs.m3}
	}
}
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> std::ops::Div<Volume<T>> for &Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol.clone() / rhs.m3}
	}
}
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> std::ops::Div<&Volume<T>> for Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol / rhs.m3.clone()}
	}
}
/// Dividing a Amount by a Volume returns a value of type Concentration
impl<T> std::ops::Div<&Volume<T>> for &Amount<T> where T: NumLike {
	type Output = Concentration<T>;
	fn div(self, rhs: &Volume<T>) -> Self::Output {
		Concentration{molpm3: self.mol.clone() / rhs.m3.clone()}
	}
}

// Amount / CatalyticActivity -> Time
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> std::ops::Div<CatalyticActivity<T>> for Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol / rhs.molps}
	}
}
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> std::ops::Div<CatalyticActivity<T>> for &Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol.clone() / rhs.molps}
	}
}
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> std::ops::Div<&CatalyticActivity<T>> for Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol / rhs.molps.clone()}
	}
}
/// Dividing a Amount by a CatalyticActivity returns a value of type Time
impl<T> std::ops::Div<&CatalyticActivity<T>> for &Amount<T> where T: NumLike {
	type Output = Time<T>;
	fn div(self, rhs: &CatalyticActivity<T>) -> Self::Output {
		Time{s: self.mol.clone() / rhs.molps.clone()}
	}
}

// Amount / Concentration -> Volume
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> std::ops::Div<Concentration<T>> for Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Concentration<T>) -> Self::Output {
		Volume{m3: self.mol / rhs.molpm3}
	}
}
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> std::ops::Div<Concentration<T>> for &Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: Concentration<T>) -> Self::Output {
		Volume{m3: self.mol.clone() / rhs.molpm3}
	}
}
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> std::ops::Div<&Concentration<T>> for Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Concentration<T>) -> Self::Output {
		Volume{m3: self.mol / rhs.molpm3.clone()}
	}
}
/// Dividing a Amount by a Concentration returns a value of type Volume
impl<T> std::ops::Div<&Concentration<T>> for &Amount<T> where T: NumLike {
	type Output = Volume<T>;
	fn div(self, rhs: &Concentration<T>) -> Self::Output {
		Volume{m3: self.mol.clone() / rhs.molpm3.clone()}
	}
}

/// The electrical current unit type, defined as amperes in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Current<T: NumLike>{
	/// The value of this Electrical current in amperes
	pub A: T
}

impl<T> Current<T> where T: NumLike {

	/// Returns the standard unit name of electrical current: "amperes"
	pub fn unit_name() -> &'static str {
		return "amperes";
	}
	
	/// Returns the abbreviated name or symbol of electrical current: "A" for amperes
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
	
	/// Returns a copy of this electrical current value in amperes
	pub fn to_amps(self) -> T {
		return self.A.clone() / T::from(1.0_f64);
	}

	/// Returns a new electrical current value from the given number of amperes
	///
	/// # Arguments
	/// * `amps` - Any number-like type, representing a quantity of amperes
	pub fn from_amps(amps: T) -> Self {
		Current{A: amps * T::from(1.0_f64)}
	}

	/// Returns a copy of this electrical current value in milliamps
	pub fn to_mA(self) -> T {
		return self.A.clone() / T::from(0.001_f64);
	}

	/// Returns a new electrical current value from the given number of milliamps
	///
	/// # Arguments
	/// * `mA` - Any number-like type, representing a quantity of milliamps
	pub fn from_mA(mA: T) -> Self {
		Current{A: mA * T::from(0.001_f64)}
	}

	/// Returns a copy of this electrical current value in microamps
	pub fn to_uA(self) -> T {
		return self.A.clone() / T::from(1e-06_f64);
	}

	/// Returns a new electrical current value from the given number of microamps
	///
	/// # Arguments
	/// * `uA` - Any number-like type, representing a quantity of microamps
	pub fn from_uA(uA: T) -> Self {
		Current{A: uA * T::from(1e-06_f64)}
	}

	/// Returns a copy of this electrical current value in nanoamps
	pub fn to_nA(self) -> T {
		return self.A.clone() / T::from(1e-09_f64);
	}

	/// Returns a new electrical current value from the given number of nanoamps
	///
	/// # Arguments
	/// * `nA` - Any number-like type, representing a quantity of nanoamps
	pub fn from_nA(nA: T) -> Self {
		Current{A: nA * T::from(1e-09_f64)}
	}

	/// Returns a copy of this electrical current value in kiloamps
	pub fn to_kA(self) -> T {
		return self.A.clone() / T::from(1000.0_f64);
	}

	/// Returns a new electrical current value from the given number of kiloamps
	///
	/// # Arguments
	/// * `kA` - Any number-like type, representing a quantity of kiloamps
	pub fn from_kA(kA: T) -> Self {
		Current{A: kA * T::from(1000.0_f64)}
	}

	/// Returns a copy of this electrical current value in mega-amps
	pub fn to_MA(self) -> T {
		return self.A.clone() / T::from(1000000.0_f64);
	}

	/// Returns a new electrical current value from the given number of mega-amps
	///
	/// # Arguments
	/// * `MA` - Any number-like type, representing a quantity of mega-amps
	pub fn from_MA(MA: T) -> Self {
		Current{A: MA * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this electrical current value in giga-amps
	pub fn to_GA(self) -> T {
		return self.A.clone() / T::from(1000000000.0_f64);
	}

	/// Returns a new electrical current value from the given number of giga-amps
	///
	/// # Arguments
	/// * `GA` - Any number-like type, representing a quantity of giga-amps
	pub fn from_GA(GA: T) -> Self {
		Current{A: GA * T::from(1000000000.0_f64)}
	}

}

// Current * Time -> Charge
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> std::ops::Mul<Time<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Charge{C: self.A * rhs.s}
	}
}
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> std::ops::Mul<Time<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Charge{C: self.A.clone() * rhs.s}
	}
}
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> std::ops::Mul<&Time<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Charge{C: self.A * rhs.s.clone()}
	}
}
/// Multiplying a Current by a Time returns a value of type Charge
impl<T> std::ops::Mul<&Time<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn mul(self, rhs: &Time<T>) -> Self::Output {
		Charge{C: self.A.clone() * rhs.s.clone()}
	}
}

// Current / Frequency -> Charge
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> std::ops::Div<Frequency<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Charge{C: self.A / rhs.Hz}
	}
}
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> std::ops::Div<Frequency<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: Frequency<T>) -> Self::Output {
		Charge{C: self.A.clone() / rhs.Hz}
	}
}
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> std::ops::Div<&Frequency<T>> for Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Charge{C: self.A / rhs.Hz.clone()}
	}
}
/// Dividing a Current by a Frequency returns a value of type Charge
impl<T> std::ops::Div<&Frequency<T>> for &Current<T> where T: NumLike {
	type Output = Charge<T>;
	fn div(self, rhs: &Frequency<T>) -> Self::Output {
		Charge{C: self.A.clone() / rhs.Hz.clone()}
	}
}

// Current / Charge -> Frequency
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> std::ops::Div<Charge<T>> for Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Frequency{Hz: self.A / rhs.C}
	}
}
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> std::ops::Div<Charge<T>> for &Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: Charge<T>) -> Self::Output {
		Frequency{Hz: self.A.clone() / rhs.C}
	}
}
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> std::ops::Div<&Charge<T>> for Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Frequency{Hz: self.A / rhs.C.clone()}
	}
}
/// Dividing a Current by a Charge returns a value of type Frequency
impl<T> std::ops::Div<&Charge<T>> for &Current<T> where T: NumLike {
	type Output = Frequency<T>;
	fn div(self, rhs: &Charge<T>) -> Self::Output {
		Frequency{Hz: self.A.clone() / rhs.C.clone()}
	}
}

// Current * Voltage -> Power
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> std::ops::Mul<Voltage<T>> for Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Power{W: self.A * rhs.V}
	}
}
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> std::ops::Mul<Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: Voltage<T>) -> Self::Output {
		Power{W: self.A.clone() * rhs.V}
	}
}
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> std::ops::Mul<&Voltage<T>> for Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Power{W: self.A * rhs.V.clone()}
	}
}
/// Multiplying a Current by a Voltage returns a value of type Power
impl<T> std::ops::Mul<&Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Power<T>;
	fn mul(self, rhs: &Voltage<T>) -> Self::Output {
		Power{W: self.A.clone() * rhs.V.clone()}
	}
}

// Current / Voltage -> Conductance
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> std::ops::Div<Voltage<T>> for Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Conductance{S: self.A / rhs.V}
	}
}
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> std::ops::Div<Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: Voltage<T>) -> Self::Output {
		Conductance{S: self.A.clone() / rhs.V}
	}
}
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> std::ops::Div<&Voltage<T>> for Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Conductance{S: self.A / rhs.V.clone()}
	}
}
/// Dividing a Current by a Voltage returns a value of type Conductance
impl<T> std::ops::Div<&Voltage<T>> for &Current<T> where T: NumLike {
	type Output = Conductance<T>;
	fn div(self, rhs: &Voltage<T>) -> Self::Output {
		Conductance{S: self.A.clone() / rhs.V.clone()}
	}
}

// Current * Resistance -> Voltage
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> std::ops::Mul<Resistance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Voltage{V: self.A * rhs.Ohm}
	}
}
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> std::ops::Mul<Resistance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: Resistance<T>) -> Self::Output {
		Voltage{V: self.A.clone() * rhs.Ohm}
	}
}
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> std::ops::Mul<&Resistance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Voltage{V: self.A * rhs.Ohm.clone()}
	}
}
/// Multiplying a Current by a Resistance returns a value of type Voltage
impl<T> std::ops::Mul<&Resistance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn mul(self, rhs: &Resistance<T>) -> Self::Output {
		Voltage{V: self.A.clone() * rhs.Ohm.clone()}
	}
}

// Current / Conductance -> Voltage
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> std::ops::Div<Conductance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Voltage{V: self.A / rhs.S}
	}
}
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> std::ops::Div<Conductance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: Conductance<T>) -> Self::Output {
		Voltage{V: self.A.clone() / rhs.S}
	}
}
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> std::ops::Div<&Conductance<T>> for Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Voltage{V: self.A / rhs.S.clone()}
	}
}
/// Dividing a Current by a Conductance returns a value of type Voltage
impl<T> std::ops::Div<&Conductance<T>> for &Current<T> where T: NumLike {
	type Output = Voltage<T>;
	fn div(self, rhs: &Conductance<T>) -> Self::Output {
		Voltage{V: self.A.clone() / rhs.S.clone()}
	}
}

// Current * Inductance -> MagneticFlux
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> std::ops::Mul<Inductance<T>> for Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A * rhs.H}
	}
}
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> std::ops::Mul<Inductance<T>> for &Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A.clone() * rhs.H}
	}
}
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Inductance<T>> for Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A * rhs.H.clone()}
	}
}
/// Multiplying a Current by a Inductance returns a value of type MagneticFlux
impl<T> std::ops::Mul<&Inductance<T>> for &Current<T> where T: NumLike {
	type Output = MagneticFlux<T>;
	fn mul(self, rhs: &Inductance<T>) -> Self::Output {
		MagneticFlux{Wb: self.A.clone() * rhs.H.clone()}
	}
}

// Current * MagneticFlux -> Energy
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> std::ops::Mul<MagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A * rhs.Wb}
	}
}
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> std::ops::Mul<MagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A.clone() * rhs.Wb}
	}
}
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> std::ops::Mul<&MagneticFlux<T>> for Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A * rhs.Wb.clone()}
	}
}
/// Multiplying a Current by a MagneticFlux returns a value of type Energy
impl<T> std::ops::Mul<&MagneticFlux<T>> for &Current<T> where T: NumLike {
	type Output = Energy<T>;
	fn mul(self, rhs: &MagneticFlux<T>) -> Self::Output {
		Energy{J: self.A.clone() * rhs.Wb.clone()}
	}
}

/// The luminosity unit type, defined as candela in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Luminosity<T: NumLike>{
	/// The value of this Luminosity in candela
	pub cd: T
}

impl<T> Luminosity<T> where T: NumLike {

	/// Returns the standard unit name of luminosity: "candela"
	pub fn unit_name() -> &'static str {
		return "candela";
	}
	
	/// Returns the abbreviated name or symbol of luminosity: "cd" for candela
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
	
	/// Returns a copy of this luminosity value in candela
	pub fn to_candela(self) -> T {
		return self.cd.clone() / T::from(1.0_f64);
	}

	/// Returns a new luminosity value from the given number of candela
	///
	/// # Arguments
	/// * `candela` - Any number-like type, representing a quantity of candela
	pub fn from_candela(candela: T) -> Self {
		Luminosity{cd: candela * T::from(1.0_f64)}
	}

	/// Returns a copy of this luminosity value in millicondela
	pub fn to_mcd(self) -> T {
		return self.cd.clone() / T::from(0.001_f64);
	}

	/// Returns a new luminosity value from the given number of millicondela
	///
	/// # Arguments
	/// * `mcd` - Any number-like type, representing a quantity of millicondela
	pub fn from_mcd(mcd: T) -> Self {
		Luminosity{cd: mcd * T::from(0.001_f64)}
	}

	/// Returns a copy of this luminosity value in microcondela
	pub fn to_ucd(self) -> T {
		return self.cd.clone() / T::from(1e-06_f64);
	}

	/// Returns a new luminosity value from the given number of microcondela
	///
	/// # Arguments
	/// * `ucd` - Any number-like type, representing a quantity of microcondela
	pub fn from_ucd(ucd: T) -> Self {
		Luminosity{cd: ucd * T::from(1e-06_f64)}
	}

	/// Returns a copy of this luminosity value in nanocondela
	pub fn to_ncd(self) -> T {
		return self.cd.clone() / T::from(1e-09_f64);
	}

	/// Returns a new luminosity value from the given number of nanocondela
	///
	/// # Arguments
	/// * `ncd` - Any number-like type, representing a quantity of nanocondela
	pub fn from_ncd(ncd: T) -> Self {
		Luminosity{cd: ncd * T::from(1e-09_f64)}
	}

	/// Returns a copy of this luminosity value in kilocandela
	pub fn to_kcd(self) -> T {
		return self.cd.clone() / T::from(1000.0_f64);
	}

	/// Returns a new luminosity value from the given number of kilocandela
	///
	/// # Arguments
	/// * `kcd` - Any number-like type, representing a quantity of kilocandela
	pub fn from_kcd(kcd: T) -> Self {
		Luminosity{cd: kcd * T::from(1000.0_f64)}
	}

	/// Returns a copy of this luminosity value in megacandela
	pub fn to_Mcd(self) -> T {
		return self.cd.clone() / T::from(1000000.0_f64);
	}

	/// Returns a new luminosity value from the given number of megacandela
	///
	/// # Arguments
	/// * `Mcd` - Any number-like type, representing a quantity of megacandela
	pub fn from_Mcd(Mcd: T) -> Self {
		Luminosity{cd: Mcd * T::from(1000000.0_f64)}
	}

	/// Returns a copy of this luminosity value in gigacandela
	pub fn to_Gcd(self) -> T {
		return self.cd.clone() / T::from(1000000000.0_f64);
	}

	/// Returns a new luminosity value from the given number of gigacandela
	///
	/// # Arguments
	/// * `Gcd` - Any number-like type, representing a quantity of gigacandela
	pub fn from_Gcd(Gcd: T) -> Self {
		Luminosity{cd: Gcd * T::from(1000000000.0_f64)}
	}

}

// Luminosity * SolidAngle -> LuminousFlux
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> std::ops::Mul<SolidAngle<T>> for Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd * rhs.sr}
	}
}
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> std::ops::Mul<SolidAngle<T>> for &Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd.clone() * rhs.sr}
	}
}
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> std::ops::Mul<&SolidAngle<T>> for Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd * rhs.sr.clone()}
	}
}
/// Multiplying a Luminosity by a SolidAngle returns a value of type LuminousFlux
impl<T> std::ops::Mul<&SolidAngle<T>> for &Luminosity<T> where T: NumLike {
	type Output = LuminousFlux<T>;
	fn mul(self, rhs: &SolidAngle<T>) -> Self::Output {
		LuminousFlux{lm: self.cd.clone() * rhs.sr.clone()}
	}
}


