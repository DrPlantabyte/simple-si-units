#![allow(non_snake_case)]
use crate::{UnitStruct, NumLike};
use std::fmt::{Display, Formatter};
// TODO: base SI units

/// Placeholder: Work in progress
#[derive(UnitStruct, Debug, Clone)]
pub struct Distance<T: NumLike>{
	pub m: T
}

impl<T> Display for Distance<T> where T: NumLike {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		// TODO: better display
		return write!(f, "{} m", self.m);
	}
}
impl<T> Distance<T> where T: NumLike {
	pub fn from_m(m: T) -> Self{
		Distance{m}
	}
	pub fn to_m(self) -> T{
		return self.m;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	pub fn from_au(au: T) -> Self{
		let m_per_au = T::from(1.495979e11f64);
		Distance{m: m_per_au * au}
	}
	pub fn to_au(self) -> T{
		let au_per_m = T::from(6.684585e-12f64);
		return au_per_m * self.m;
	}
}


#[derive(UnitStruct, Debug, Clone)]
pub struct Mass<T: NumLike>{
	pub kg: T
}
impl<T> Mass<T> where T: NumLike+From<f64> {
	pub fn from_earth_mass(earth_mass: T) -> Self {
		let earth_mass_kg: T = T::from(5.972e24f64);
		Mass{kg: earth_mass_kg * earth_mass}
	}
	pub fn from_solar_mass(sun_mass: T) -> Self {
		let sun_mass_kg: T = T::from(1.989e30f64);
		Mass{kg: sun_mass_kg * sun_mass}
	}
	pub fn from_g(g: T) -> Self {
		let c: T = T::from(1e-3f64);
		Mass{kg: c * g}
	}
}



#[derive(UnitStruct, Debug, Clone)]
pub struct Time<T: NumLike>{
	pub s: T
}
impl<T> Time<T> where T: NumLike+From<f64> {
	pub fn from_days(d: T) -> Self{
		let day_s = T::from(86400f64);
		Time{s: day_s * d}
	}
}