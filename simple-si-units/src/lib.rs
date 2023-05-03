#![no_std]
#![allow(non_snake_case)]
#![warn(missing_docs)]
#![ doc = include_str!("../README.md")]

/// This derive macro automatically 
/// derives all of the relevant mathematical operators for the derived struct,
/// so long as that struct contains only a single named field. 
/// 
/// For example:
/// 
/// ```rust
/// use simple_si_units::{UnitStruct, NumLike};
/// 
/// #[derive(UnitStruct, Debug, Clone)]
/// struct HyperVelocity<T: NumLike>{
///   square_meters_per_second: T
/// }
/// 
/// fn weighted_hypervel_sum<T: NumLike>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T>
///   where T:NumLike + From<f64>
/// {
///   return weight*a + (1.-weight)*b;
/// }
/// ```
pub use simple_si_units_macros::UnitStruct;
/// The `NumLike` trait is just a shorthand definition for any "number-like" 
/// type in Rust. "Number-like" means that a type implements the traits for 
/// standard arithmatic (Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, 
/// DivAssign, and Neg), plus Clone, Debug, and Display. Most number types also
/// implement the Copy marker trait, but that is not required (for example, an 
/// arbitrary-precision number type must dynamically allocate memory and thus 
/// cannot implement Copy).
/// 
/// This trait is not meant to be implemented, just for making generic type 
/// templates more ergonomic. E.g.
/// ```rust
/// use simple_si_units::NumLike;
/// 
/// fn delta_squared<T>(a: T, b: T) -> T where T: NumLike {
///   let delta = b - a;
///   return delta.clone() * delta;
/// }
/// ```
pub use simple_si_units_core::NumLike;
// NOTE: test with: RUST_BACKTRACE=full cargo clean && cargo test --all-features

// optional supports
#[cfg(feature="serde")]
extern crate serde;
#[cfg(feature="num-bigfloat")]
extern crate num_bigfloat;
#[cfg(feature="num-complex")]
extern crate num_complex;
#[cfg(feature="num-rational")]
extern crate num_rational;
#[cfg(feature="uom")]
extern crate uom;

pub mod base;
pub mod chemical;
pub mod electromagnetic;
pub mod geometry;
pub mod mechanical;
pub mod nuclear;

#[cfg(test)]
#[macro_use]
extern crate std; // import std lib only in test mode

/// Unit tests
#[cfg(test)]
mod unit_tests {
	use num_traits::Zero;
	use super::base::*;
	use super::chemical::*;
	use super::electromagnetic::*;
	use super::geometry::*;
	use super::mechanical::*;
	use super::nuclear::*;
	/// utility function for asserting equality of decimal values with approximations
	fn assert_approx_equal(a: f64, b: f64, sigfigs: i32) {
		if a.is_nan() {
			assert!(b.is_nan());
		} else if a.is_infinite() {
			assert!(b.is_infinite() && a.is_sign_positive() == b.is_sign_positive());
		} else if a.is_zero() {
			assert!(b.is_zero());
		} else {
			let ypsilon = 10f64.powi(-sigfigs);
			let max_delta = (a.abs() + b.abs()) * 0.5 * ypsilon;
			assert!((a - b).abs() < max_delta, "Error: {} != {} within margin of {}", a, b, max_delta);
		}
	}

	/// Unit test
	#[test]
	fn add_subtract_test() {
		// Note: math operators are implemented by the proc macro, so all units behave identically
		// therefore only need to test one of them to ensure all are compliant
		let d1 = Distance::from_m(2.5);
		let d2 = Distance::from_m(1.0);
		assert_approx_equal((d1+d2).to_m(), 3.5, 9);
		assert_approx_equal((d2+d1).to_m(), 3.5, 9);
		assert_approx_equal((d1-d2).to_m(), 1.5, 9);
		assert_approx_equal((d2-d1).to_m(), -1.5, 9);
		assert_approx_equal((d1-d1).to_m(), 0.0, 9);
	}
	/// Unit test
	#[test]
	fn mul_div_test() {
		let d1 = Distance::from_m(2.5);
		let d2 = Distance::from_m(2.0);
		assert_approx_equal(d1/d2, 1.25, 9);
		assert_approx_equal(d2/d1, 0.8, 9);
		assert_approx_equal((d1*d2).to_m2(), 5.0, 9);
		assert_approx_equal((d2*d1).to_m2(), 5.0, 9);
	}
	/// Unit test
	#[test]
	fn op_assign_test() {
		// +=, -=, *=, /=
		let mut d1 = Distance::from_m(2.5);
		let mut d2 = Distance::from_m(2.0);
		d1 += d2;
		assert_approx_equal(d1.to_m(), 4.5, 9);
		d2 -= d1;
		assert_approx_equal(d2.to_m(), -2.5, 9);
		d1 *= 2.0;
		assert_approx_equal(d1.to_m(), 9.0, 9);
		d2 /= -0.5;
		assert_approx_equal(d2.to_m(), 5.0, 9);
	}

	fn mul_check<
		'y,
		A: std::ops::Mul<B, Output = X>+Clone+'y,
		B: std::ops::Mul<A, Output = X>+Clone+'y,
		X: std::cmp::PartialEq+Clone
	>(a: &'y A, b: &'y B) -> X where
		A: std::ops::Mul<B, Output = X>+std::ops::Mul<&'y B, Output = X>,
		B: std::ops::Mul<A, Output = X>+std::ops::Mul<&'y A, Output = X>,
		&'y A: std::ops::Mul<B, Output = X>, &'y B: std::ops::Mul<A, Output = X>,
		&'y A: std::ops::Mul<&'y B, Output = X>, &'y B: std::ops::Mul<&'y A, Output = X>
	{
		let x1: X = a * b;
		let x2: X = b * a;
		let x3: X = a.clone() * b.clone();
		let x4: X = b.clone() * a.clone();
		let x5: X = a * b.clone();
		let x6: X = b * a.clone();
		let x7: X = a.clone() * b;
		let x8: X = b.clone() * a;
		// let x6: X = b.clone() * &(a.clone());
		assert!((x1.eq(&x2)));
		assert!((x1.eq(&x3)));
		assert!((x1.eq(&x4)));
		assert!((x1.eq(&x5)));
		assert!((x1.eq(&x6)));
		assert!((x1.eq(&x7)));
		assert!((x1.eq(&x8)));
		return x1;
	}

	fn div_check<
		'y,
		A: std::ops::Div<B, Output = X>+Clone+'y,
		B: Clone+'y,
		X: std::cmp::PartialEq+Clone
	>(a: &'y A, b: &'y B) -> X where
		A: std::ops::Div<B, Output = X>+std::ops::Div<&'y B, Output = X>,
		&'y A: std::ops::Div<B, Output = X>+std::ops::Div<&'y B, Output = X>
	{
		let x1: X = a / b;
		let x2: X = a.clone() / b.clone();
		let x3: X = a / b.clone();
		let x4: X = a.clone() / b;
		assert!((x1.eq(&x2)));
		assert!((x1.eq(&x3)));
		assert!((x1.eq(&x4)));
		return x1;
	}

	macro_rules! mul_div_check {
		($a: expr, $b: expr, $c: expr, $d: expr) => {
			assert_eq!(($a) * ($b), ($c));
			assert_eq!(($a) * &($b), ($c));
			assert_eq!(&($a) * ($b), ($c));
			assert_eq!(&($a) * &($b), ($c));
			assert_eq!(($b) * ($a), ($c));
			assert_eq!(($b) * &($a), ($c));
			assert_eq!(&($b) * ($a), ($c));
			assert_eq!(&($b) * &($a), ($c));
			assert_eq!(($a) / ($b), ($d));
			assert_eq!(($a) / &($b), ($d));
			assert_eq!(&($a) / ($b), ($d));
			assert_eq!(&($a) / &($b), ($d));
		}
	}

	macro_rules! check_scalar_mul_div {
		($a: expr, $b: expr) => {
			let x = ($a).clone();
			let y = ($b).clone();
			let xy = x.clone() * y.clone();
			let xovery = x.clone() / y.clone();
			mul_div_check!(Amount{mol: x.clone()}, y.clone(), Amount{mol: xy.clone()}, Amount{mol: xovery.clone()});
			mul_div_check!(Current{A: x.clone()}, y.clone(), Current{A: xy.clone()}, Current{A: xovery.clone()});
			mul_div_check!(Distance{m: x.clone()}, y.clone(), Distance{m: xy.clone()}, Distance{m: xovery.clone()});
			mul_div_check!(Luminosity{cd: x.clone()}, y.clone(), Luminosity{cd: xy.clone()}, Luminosity{cd: xovery.clone()});
			mul_div_check!(Mass{kg: x.clone()}, y.clone(), Mass{kg: xy.clone()}, Mass{kg: xovery.clone()});
			mul_div_check!(Temperature{K: x.clone()}, y.clone(), Temperature{K: xy.clone()}, Temperature{K: xovery.clone()});
			mul_div_check!(Time{s: x.clone()}, y.clone(), Time{s: xy.clone()}, Time{s: xovery.clone()});
			mul_div_check!(CatalyticActivity{molps: x.clone()}, y.clone(), CatalyticActivity{molps: xy.clone()}, CatalyticActivity{molps: xovery.clone()});
			mul_div_check!(Concentration{molpm3: x.clone()}, y.clone(), Concentration{molpm3: xy.clone()}, Concentration{molpm3: xovery.clone()});
			mul_div_check!(Molality{molpkg: x.clone()}, y.clone(), Molality{molpkg: xy.clone()}, Molality{molpkg: xovery.clone()});
			mul_div_check!(MolarMass{kgpmol: x.clone()}, y.clone(), MolarMass{kgpmol: xy.clone()}, MolarMass{kgpmol: xovery.clone()});
			mul_div_check!(SpecificHeatCapacity{J_per_kgK: x.clone()}, y.clone(), SpecificHeatCapacity{J_per_kgK: xy.clone()}, SpecificHeatCapacity{J_per_kgK: xovery.clone()});
			mul_div_check!(Capacitance{F: x.clone()}, y.clone(), Capacitance{F: xy.clone()}, Capacitance{F: xovery.clone()});
			mul_div_check!(Charge{C: x.clone()}, y.clone(), Charge{C: xy.clone()}, Charge{C: xovery.clone()});
			mul_div_check!(Conductance{S: x.clone()}, y.clone(), Conductance{S: xy.clone()}, Conductance{S: xovery.clone()});
			mul_div_check!(Illuminance{lux: x.clone()}, y.clone(), Illuminance{lux: xy.clone()}, Illuminance{lux: xovery.clone()});
			mul_div_check!(Inductance{H: x.clone()}, y.clone(), Inductance{H: xy.clone()}, Inductance{H: xovery.clone()});
			mul_div_check!(LuminousFlux{lm: x.clone()}, y.clone(), LuminousFlux{lm: xy.clone()}, LuminousFlux{lm: xovery.clone()});
			mul_div_check!(MagneticFlux{Wb: x.clone()}, y.clone(), MagneticFlux{Wb: xy.clone()}, MagneticFlux{Wb: xovery.clone()});
			mul_div_check!(MagneticFluxDensity{T: x.clone()}, y.clone(), MagneticFluxDensity{T: xy.clone()}, MagneticFluxDensity{T: xovery.clone()});
			mul_div_check!(Resistance{Ohm: x.clone()}, y.clone(), Resistance{Ohm: xy.clone()}, Resistance{Ohm: xovery.clone()});
			mul_div_check!(Voltage{V: x.clone()}, y.clone(), Voltage{V: xy.clone()}, Voltage{V: xovery.clone()});
			mul_div_check!(Angle{rad: x.clone()}, y.clone(), Angle{rad: xy.clone()}, Angle{rad: xovery.clone()});
			mul_div_check!(Area{m2: x.clone()}, y.clone(), Area{m2: xy.clone()}, Area{m2: xovery.clone()});
			mul_div_check!(SolidAngle{sr: x.clone()}, y.clone(), SolidAngle{sr: xy.clone()}, SolidAngle{sr: xovery.clone()});
			mul_div_check!(Volume{m3: x.clone()}, y.clone(), Volume{m3: xy.clone()}, Volume{m3: xovery.clone()});
			mul_div_check!(Acceleration{mps2: x.clone()}, y.clone(), Acceleration{mps2: xy.clone()}, Acceleration{mps2: xovery.clone()});
			mul_div_check!(AngularAcceleration{radps2: x.clone()}, y.clone(), AngularAcceleration{radps2: xy.clone()}, AngularAcceleration{radps2: xovery.clone()});
			mul_div_check!(AngularMomentum{kgm2radps: x.clone()}, y.clone(), AngularMomentum{kgm2radps: xy.clone()}, AngularMomentum{kgm2radps: xovery.clone()});
			mul_div_check!(AngularVelocity{radps: x.clone()}, y.clone(), AngularVelocity{radps: xy.clone()}, AngularVelocity{radps: xovery.clone()});
			mul_div_check!(AreaDensity{kgpm2: x.clone()}, y.clone(), AreaDensity{kgpm2: xy.clone()}, AreaDensity{kgpm2: xovery.clone()});
			mul_div_check!(Density{kgpm3: x.clone()}, y.clone(), Density{kgpm3: xy.clone()}, Density{kgpm3: xovery.clone()});
			mul_div_check!(Energy{J: x.clone()}, y.clone(), Energy{J: xy.clone()}, Energy{J: xovery.clone()});
			mul_div_check!(Force{N: x.clone()}, y.clone(), Force{N: xy.clone()}, Force{N: xovery.clone()});
			mul_div_check!(Frequency{Hz: x.clone()}, y.clone(), Frequency{Hz: xy.clone()}, Frequency{Hz: xovery.clone()});
			mul_div_check!(MomentOfInertia{kgm2: x.clone()}, y.clone(), MomentOfInertia{kgm2: xy.clone()}, MomentOfInertia{kgm2: xovery.clone()});
			mul_div_check!(Momentum{kgmps: x.clone()}, y.clone(), Momentum{kgmps: xy.clone()}, Momentum{kgmps: xovery.clone()});
			mul_div_check!(Power{W: x.clone()}, y.clone(), Power{W: xy.clone()}, Power{W: xovery.clone()});
			mul_div_check!(Pressure{Pa: x.clone()}, y.clone(), Pressure{Pa: xy.clone()}, Pressure{Pa: xovery.clone()});
			mul_div_check!(Torque{Nm: x.clone()}, y.clone(), Torque{Nm: xy.clone()}, Torque{Nm: xovery.clone()});
			mul_div_check!(Velocity{mps: x.clone()}, y.clone(), Velocity{mps: xy.clone()}, Velocity{mps: xovery.clone()});
			mul_div_check!(AbsorbedDose{Gy: x.clone()}, y.clone(), AbsorbedDose{Gy: xy.clone()}, AbsorbedDose{Gy: xovery.clone()});
			mul_div_check!(DoseEquivalent{Sv: x.clone()}, y.clone(), DoseEquivalent{Sv: xy.clone()}, DoseEquivalent{Sv: xovery.clone()});
			mul_div_check!(Radioactivity{Bq: x.clone()}, y.clone(), Radioactivity{Bq: xy.clone()}, Radioactivity{Bq: xovery.clone()});
	}
}

	#[test]
	#[cfg(feature="num-complex")]
	fn test_complex_scalar_multiply() {
		use num_complex::{Complex32, Complex64};
		let x = Complex64::new(1.2, 3.4);
		let y = Complex64::new(5.6, 7.8);
		check_scalar_mul_div!(x, y);
		let x = Complex32::new(1.2, 3.4);
		let y = Complex32::new(5.6, 7.8);
		check_scalar_mul_div!(x, y);
	}

	#[test]
	#[cfg(feature="num-bigfloat")]
	fn test_bigfloat_scalar_multiply() {
		use num_bigfloat::BigFloat;
		let x = BigFloat::from(4.2);
		let y = BigFloat::from(2.1);
		check_scalar_mul_div!(x, y);
	}

	///// Place generated unit tests below this comment /////

	#[test]
	fn unit_names_and_symbols_test() {
		assert!(Amount::<f64>::unit_name().eq("moles"));
		assert!(Amount::<f64>::unit_symbol().eq("mol"));
		assert!(Current::<f64>::unit_name().eq("amperes"));
		assert!(Current::<f64>::unit_symbol().eq("A"));
		assert!(Distance::<f64>::unit_name().eq("meters"));
		assert!(Distance::<f64>::unit_symbol().eq("m"));
		assert!(Luminosity::<f64>::unit_name().eq("candela"));
		assert!(Luminosity::<f64>::unit_symbol().eq("cd"));
		assert!(Mass::<f64>::unit_name().eq("kilograms"));
		assert!(Mass::<f64>::unit_symbol().eq("kg"));
		assert!(Temperature::<f64>::unit_name().eq("degrees kelvin"));
		assert!(Temperature::<f64>::unit_symbol().eq("K"));
		assert!(Time::<f64>::unit_name().eq("seconds"));
		assert!(Time::<f64>::unit_symbol().eq("s"));
		assert!(CatalyticActivity::<f64>::unit_name().eq("moles per second"));
		assert!(CatalyticActivity::<f64>::unit_symbol().eq("mol/s"));
		assert!(Concentration::<f64>::unit_name().eq("moles per cubic meter"));
		assert!(Concentration::<f64>::unit_symbol().eq("mol/m³"));
		assert!(Molality::<f64>::unit_name().eq("moles per kilogram"));
		assert!(Molality::<f64>::unit_symbol().eq("mol/kg"));
		assert!(MolarMass::<f64>::unit_name().eq("kilograms per mole"));
		assert!(MolarMass::<f64>::unit_symbol().eq("kg/mol"));
		assert!(SpecificHeatCapacity::<f64>::unit_name().eq("joules per kilogram per kelvin"));
		assert!(SpecificHeatCapacity::<f64>::unit_symbol().eq("J/kg·K"));
		assert!(Capacitance::<f64>::unit_name().eq("farads"));
		assert!(Capacitance::<f64>::unit_symbol().eq("F"));
		assert!(Charge::<f64>::unit_name().eq("coulombs"));
		assert!(Charge::<f64>::unit_symbol().eq("C"));
		assert!(Conductance::<f64>::unit_name().eq("siemens"));
		assert!(Conductance::<f64>::unit_symbol().eq("S"));
		assert!(Illuminance::<f64>::unit_name().eq("lux"));
		assert!(Illuminance::<f64>::unit_symbol().eq("lux"));
		assert!(Inductance::<f64>::unit_name().eq("henries"));
		assert!(Inductance::<f64>::unit_symbol().eq("H"));
		assert!(LuminousFlux::<f64>::unit_name().eq("lumens"));
		assert!(LuminousFlux::<f64>::unit_symbol().eq("lm"));
		assert!(MagneticFlux::<f64>::unit_name().eq("webers"));
		assert!(MagneticFlux::<f64>::unit_symbol().eq("Wb"));
		assert!(MagneticFluxDensity::<f64>::unit_name().eq("teslas"));
		assert!(MagneticFluxDensity::<f64>::unit_symbol().eq("T"));
		assert!(Resistance::<f64>::unit_name().eq("ohms"));
		assert!(Resistance::<f64>::unit_symbol().eq("Ohm"));
		assert!(Voltage::<f64>::unit_name().eq("volts"));
		assert!(Voltage::<f64>::unit_symbol().eq("V"));
		assert!(Angle::<f64>::unit_name().eq("radians"));
		assert!(Angle::<f64>::unit_symbol().eq("rad"));
		assert!(Area::<f64>::unit_name().eq("square meters"));
		assert!(Area::<f64>::unit_symbol().eq("m²"));
		assert!(SolidAngle::<f64>::unit_name().eq("steradian"));
		assert!(SolidAngle::<f64>::unit_symbol().eq("sr"));
		assert!(Volume::<f64>::unit_name().eq("cubic meters"));
		assert!(Volume::<f64>::unit_symbol().eq("m³"));
		assert!(Acceleration::<f64>::unit_name().eq("meters per second squared"));
		assert!(Acceleration::<f64>::unit_symbol().eq("m/s²"));
		assert!(AngularAcceleration::<f64>::unit_name().eq("radians per second squared"));
		assert!(AngularAcceleration::<f64>::unit_symbol().eq("rad/s²"));
		assert!(AngularMomentum::<f64>::unit_name().eq("kilogram meters squared radians per second"));
		assert!(AngularMomentum::<f64>::unit_symbol().eq("kg·m²·rad/s"));
		assert!(AngularVelocity::<f64>::unit_name().eq("radians per second"));
		assert!(AngularVelocity::<f64>::unit_symbol().eq("rad/s"));
		assert!(AreaDensity::<f64>::unit_name().eq("kilograms per square meter"));
		assert!(AreaDensity::<f64>::unit_symbol().eq("kg/m²"));
		assert!(Density::<f64>::unit_name().eq("kilograms per cubic meter"));
		assert!(Density::<f64>::unit_symbol().eq("kg/m³"));
		assert!(Energy::<f64>::unit_name().eq("joules"));
		assert!(Energy::<f64>::unit_symbol().eq("J"));
		assert!(Force::<f64>::unit_name().eq("newtons"));
		assert!(Force::<f64>::unit_symbol().eq("N"));
		assert!(Frequency::<f64>::unit_name().eq("hertz"));
		assert!(Frequency::<f64>::unit_symbol().eq("Hz"));
		assert!(MomentOfInertia::<f64>::unit_name().eq("kilogram meters squared"));
		assert!(MomentOfInertia::<f64>::unit_symbol().eq("kg·m²"));
		assert!(Momentum::<f64>::unit_name().eq("kilogram meters per second"));
		assert!(Momentum::<f64>::unit_symbol().eq("kg·m/s"));
		assert!(Power::<f64>::unit_name().eq("watts"));
		assert!(Power::<f64>::unit_symbol().eq("W"));
		assert!(Pressure::<f64>::unit_name().eq("pascals"));
		assert!(Pressure::<f64>::unit_symbol().eq("Pa"));
		assert!(Torque::<f64>::unit_name().eq("newton meters"));
		assert!(Torque::<f64>::unit_symbol().eq("Nm"));
		assert!(Velocity::<f64>::unit_name().eq("meters per second"));
		assert!(Velocity::<f64>::unit_symbol().eq("m/s"));
		assert!(AbsorbedDose::<f64>::unit_name().eq("grays"));
		assert!(AbsorbedDose::<f64>::unit_symbol().eq("Gy"));
		assert!(DoseEquivalent::<f64>::unit_name().eq("sieverts"));
		assert!(DoseEquivalent::<f64>::unit_symbol().eq("Sv"));
		assert!(Radioactivity::<f64>::unit_name().eq("becquerels"));
		assert!(Radioactivity::<f64>::unit_symbol().eq("Bq"));
	}

	#[test]
	fn unit_print_display_test() {
		println!("{}", Amount{mol: 1});
		println!("{}", Current{A: 1});
		println!("{}", Distance{m: 1});
		println!("{}", Luminosity{cd: 1});
		println!("{}", Mass{kg: 1});
		println!("{}", Temperature{K: 1});
		println!("{}", Time{s: 1});
		println!("{}", CatalyticActivity{molps: 1});
		println!("{}", Concentration{molpm3: 1});
		println!("{}", Molality{molpkg: 1});
		println!("{}", MolarMass{kgpmol: 1});
		println!("{}", SpecificHeatCapacity{J_per_kgK: 1});
		println!("{}", Capacitance{F: 1});
		println!("{}", Charge{C: 1});
		println!("{}", Conductance{S: 1});
		println!("{}", Illuminance{lux: 1});
		println!("{}", Inductance{H: 1});
		println!("{}", LuminousFlux{lm: 1});
		println!("{}", MagneticFlux{Wb: 1});
		println!("{}", MagneticFluxDensity{T: 1});
		println!("{}", Resistance{Ohm: 1});
		println!("{}", Voltage{V: 1});
		println!("{}", Angle{rad: 1});
		println!("{}", Area{m2: 1});
		println!("{}", SolidAngle{sr: 1});
		println!("{}", Volume{m3: 1});
		println!("{}", Acceleration{mps2: 1});
		println!("{}", AngularAcceleration{radps2: 1});
		println!("{}", AngularMomentum{kgm2radps: 1});
		println!("{}", AngularVelocity{radps: 1});
		println!("{}", AreaDensity{kgpm2: 1});
		println!("{}", Density{kgpm3: 1});
		println!("{}", Energy{J: 1});
		println!("{}", Force{N: 1});
		println!("{}", Frequency{Hz: 1});
		println!("{}", MomentOfInertia{kgm2: 1});
		println!("{}", Momentum{kgmps: 1});
		println!("{}", Power{W: 1});
		println!("{}", Pressure{Pa: 1});
		println!("{}", Torque{Nm: 1});
		println!("{}", Velocity{mps: 1});
		println!("{}", AbsorbedDose{Gy: 1});
		println!("{}", DoseEquivalent{Sv: 1});
		println!("{}", Radioactivity{Bq: 1});
	}

	#[test]
	fn test_unit_converions() {
		let x = 4.5f64;
		let y = 2.5f64;
		assert_eq!(div_check(&Amount{mol: x}, &Time{s: y}), CatalyticActivity{molps: x/y});
		assert_eq!(div_check(&Amount{mol: x}, &CatalyticActivity{molps: y}), Time{s: x/y});
		assert_eq!(div_check(&Amount{mol: x}, &Concentration{molpm3: y}), Volume{m3: x/y});
		assert_eq!(div_check(&Amount{mol: x}, &Volume{m3: y}), Concentration{molpm3: x/y});
		assert_eq!(mul_check(&Amount{mol: x}, &Frequency{Hz: y}), CatalyticActivity{molps: x*y});
		assert_eq!(mul_check(&Current{A: x}, &Time{s: y}), Charge{C: x*y});
		assert_eq!(div_check(&Current{A: x}, &Charge{C: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Current{A: x}, &Conductance{S: y}), Voltage{V: x/y});
		assert_eq!(mul_check(&Current{A: x}, &Inductance{H: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Current{A: x}, &MagneticFlux{Wb: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Current{A: x}, &Resistance{Ohm: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Current{A: x}, &Voltage{V: y}), Power{W: x*y});
		assert_eq!(div_check(&Current{A: x}, &Voltage{V: y}), Conductance{S: x/y});
		assert_eq!(div_check(&Current{A: x}, &Frequency{Hz: y}), Charge{C: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &Distance{m: y}), Area{m2: x*y});
		assert_eq!(div_check(&Distance{m: x}, &Time{s: y}), Velocity{mps: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &Area{m2: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&Distance{m: x}, &Density{kgpm3: y}), AreaDensity{kgpm2: x*y});
		assert_eq!(mul_check(&Distance{m: x}, &Force{N: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Distance{m: x}, &Frequency{Hz: y}), Velocity{mps: x*y});
		assert_eq!(div_check(&Distance{m: x}, &Velocity{mps: y}), Time{s: x/y});
		assert_eq!(mul_check(&Luminosity{cd: x}, &SolidAngle{sr: y}), LuminousFlux{lm: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &Area{m2: y}), AreaDensity{kgpm2: x/y});
		assert_eq!(div_check(&Mass{kg: x}, &Volume{m3: y}), Density{kgpm3: x/y});
		assert_eq!(mul_check(&Mass{kg: x}, &Acceleration{mps2: y}), Force{N: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &AreaDensity{kgpm2: y}), Area{m2: x/y});
		assert_eq!(div_check(&Mass{kg: x}, &Density{kgpm3: y}), Volume{m3: x/y});
		assert_eq!(mul_check(&Mass{kg: x}, &Velocity{mps: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &AbsorbedDose{Gy: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &DoseEquivalent{Sv: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Time{s: x}, &Current{A: y}), Charge{C: x*y});
		assert_eq!(mul_check(&Time{s: x}, &CatalyticActivity{molps: y}), Amount{mol: x*y});
		assert_eq!(div_check(&Time{s: x}, &Capacitance{F: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&Time{s: x}, &Conductance{S: y}), Capacitance{F: x*y});
		assert_eq!(div_check(&Time{s: x}, &Conductance{S: y}), Inductance{H: x/y});
		assert_eq!(div_check(&Time{s: x}, &Inductance{H: y}), Conductance{S: x/y});
		assert_eq!(mul_check(&Time{s: x}, &Resistance{Ohm: y}), Inductance{H: x*y});
		assert_eq!(div_check(&Time{s: x}, &Resistance{Ohm: y}), Capacitance{F: x/y});
		assert_eq!(mul_check(&Time{s: x}, &Voltage{V: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Time{s: x}, &Acceleration{mps2: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&Time{s: x}, &AngularAcceleration{radps2: y}), AngularVelocity{radps: x*y});
		assert_eq!(mul_check(&Time{s: x}, &AngularVelocity{radps: y}), Angle{rad: x*y});
		assert_eq!(mul_check(&Time{s: x}, &Force{N: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&Time{s: x}, &Power{W: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Time{s: x}, &Velocity{mps: y}), Distance{m: x*y});
		assert_eq!(div_check(&CatalyticActivity{molps: x}, &Amount{mol: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&CatalyticActivity{molps: x}, &Time{s: y}), Amount{mol: x*y});
		assert_eq!(div_check(&CatalyticActivity{molps: x}, &Frequency{Hz: y}), Amount{mol: x/y});
		assert_eq!(mul_check(&Concentration{molpm3: x}, &Volume{m3: y}), Amount{mol: x*y});
		assert_eq!(div_check(&Capacitance{F: x}, &Time{s: y}), Conductance{S: x/y});
		assert_eq!(div_check(&Capacitance{F: x}, &Conductance{S: y}), Time{s: x/y});
		assert_eq!(mul_check(&Capacitance{F: x}, &Resistance{Ohm: y}), Time{s: x*y});
		assert_eq!(mul_check(&Capacitance{F: x}, &Voltage{V: y}), Charge{C: x*y});
		assert_eq!(mul_check(&Capacitance{F: x}, &Frequency{Hz: y}), Conductance{S: x*y});
		assert_eq!(div_check(&Charge{C: x}, &Current{A: y}), Time{s: x/y});
		assert_eq!(div_check(&Charge{C: x}, &Time{s: y}), Current{A: x/y});
		assert_eq!(div_check(&Charge{C: x}, &Capacitance{F: y}), Voltage{V: x/y});
		assert_eq!(div_check(&Charge{C: x}, &Conductance{S: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Charge{C: x}, &MagneticFlux{Wb: y}), Conductance{S: x/y});
		assert_eq!(mul_check(&Charge{C: x}, &Resistance{Ohm: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Charge{C: x}, &Voltage{V: y}), Energy{J: x*y});
		assert_eq!(div_check(&Charge{C: x}, &Voltage{V: y}), Capacitance{F: x/y});
		assert_eq!(mul_check(&Charge{C: x}, &Frequency{Hz: y}), Current{A: x*y});
		assert_eq!(mul_check(&Conductance{S: x}, &Time{s: y}), Capacitance{F: x*y});
		assert_eq!(div_check(&Conductance{S: x}, &Capacitance{F: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&Conductance{S: x}, &Inductance{H: y}), Time{s: x*y});
		assert_eq!(mul_check(&Conductance{S: x}, &MagneticFlux{Wb: y}), Charge{C: x*y});
		assert_eq!(mul_check(&Conductance{S: x}, &Voltage{V: y}), Current{A: x*y});
		assert_eq!(div_check(&Conductance{S: x}, &Frequency{Hz: y}), Capacitance{F: x/y});
		assert_eq!(mul_check(&Illuminance{lux: x}, &Area{m2: y}), LuminousFlux{lm: x*y});
		assert_eq!(mul_check(&Inductance{H: x}, &Current{A: y}), MagneticFlux{Wb: x*y});
		assert_eq!(div_check(&Inductance{H: x}, &Time{s: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&Inductance{H: x}, &Conductance{S: y}), Time{s: x*y});
		assert_eq!(div_check(&Inductance{H: x}, &Resistance{Ohm: y}), Time{s: x/y});
		assert_eq!(mul_check(&Inductance{H: x}, &Frequency{Hz: y}), Resistance{Ohm: x*y});
		assert_eq!(div_check(&LuminousFlux{lm: x}, &Luminosity{cd: y}), SolidAngle{sr: x/y});
		assert_eq!(div_check(&LuminousFlux{lm: x}, &Illuminance{lux: y}), Area{m2: x/y});
		assert_eq!(div_check(&LuminousFlux{lm: x}, &Area{m2: y}), Illuminance{lux: x/y});
		assert_eq!(div_check(&LuminousFlux{lm: x}, &SolidAngle{sr: y}), Luminosity{cd: x/y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &Current{A: y}), Energy{J: x*y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Current{A: y}), Inductance{H: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Time{s: y}), Voltage{V: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Charge{C: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &Conductance{S: y}), Charge{C: x*y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Inductance{H: y}), Current{A: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &MagneticFluxDensity{T: y}), Area{m2: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Resistance{Ohm: y}), Charge{C: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Voltage{V: y}), Time{s: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Area{m2: y}), MagneticFluxDensity{T: x/y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &Frequency{Hz: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&MagneticFluxDensity{T: x}, &Area{m2: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &Current{A: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &Time{s: y}), Inductance{H: x*y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &Capacitance{F: y}), Time{s: x*y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &Charge{C: y}), MagneticFlux{Wb: x*y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &Inductance{H: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &Frequency{Hz: y}), Inductance{H: x/y});
		assert_eq!(mul_check(&Voltage{V: x}, &Current{A: y}), Power{W: x*y});
		assert_eq!(div_check(&Voltage{V: x}, &Current{A: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&Voltage{V: x}, &Time{s: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Voltage{V: x}, &Capacitance{F: y}), Charge{C: x*y});
		assert_eq!(mul_check(&Voltage{V: x}, &Charge{C: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Voltage{V: x}, &Conductance{S: y}), Current{A: x*y});
		assert_eq!(div_check(&Voltage{V: x}, &MagneticFlux{Wb: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Voltage{V: x}, &Resistance{Ohm: y}), Current{A: x/y});
		assert_eq!(div_check(&Voltage{V: x}, &Frequency{Hz: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Angle{rad: x}, &Time{s: y}), AngularVelocity{radps: x/y});
		assert_eq!(div_check(&Angle{rad: x}, &AngularVelocity{radps: y}), Time{s: x/y});
		assert_eq!(mul_check(&Angle{rad: x}, &Frequency{Hz: y}), AngularVelocity{radps: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &Distance{m: y}), Volume{m3: x*y});
		assert_eq!(div_check(&Area{m2: x}, &Distance{m: y}), Distance{m: x/y});
		assert_eq!(mul_check(&Area{m2: x}, &Illuminance{lux: y}), LuminousFlux{lm: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &MagneticFluxDensity{T: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &AreaDensity{kgpm2: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &Pressure{Pa: y}), Force{N: x*y});
		assert_eq!(mul_check(&SolidAngle{sr: x}, &Luminosity{cd: y}), LuminousFlux{lm: x*y});
		assert_eq!(div_check(&Volume{m3: x}, &Distance{m: y}), Area{m2: x/y});
		assert_eq!(mul_check(&Volume{m3: x}, &Concentration{molpm3: y}), Amount{mol: x*y});
		assert_eq!(div_check(&Volume{m3: x}, &Area{m2: y}), Distance{m: x/y});
		assert_eq!(mul_check(&Volume{m3: x}, &Density{kgpm3: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&Volume{m3: x}, &Pressure{Pa: y}), Energy{J: x*y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &Mass{kg: y}), Force{N: x*y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &Time{s: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &AreaDensity{kgpm2: y}), Pressure{Pa: x*y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &Frequency{Hz: y}), Velocity{mps: x/y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &Momentum{kgmps: y}), Power{W: x*y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &Velocity{mps: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&AngularAcceleration{radps2: x}, &Time{s: y}), AngularVelocity{radps: x*y});
		assert_eq!(div_check(&AngularAcceleration{radps2: x}, &AngularVelocity{radps: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&AngularAcceleration{radps2: x}, &Frequency{Hz: y}), AngularVelocity{radps: x/y});
		assert_eq!(div_check(&AngularMomentum{kgm2radps: x}, &MomentOfInertia{kgm2: y}), AngularVelocity{radps: x/y});
		assert_eq!(mul_check(&AngularVelocity{radps: x}, &Time{s: y}), Angle{rad: x*y});
		assert_eq!(div_check(&AngularVelocity{radps: x}, &Time{s: y}), AngularAcceleration{radps2: x/y});
		assert_eq!(div_check(&AngularVelocity{radps: x}, &Angle{rad: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&AngularVelocity{radps: x}, &AngularAcceleration{radps2: y}), Time{s: x/y});
		assert_eq!(mul_check(&AngularVelocity{radps: x}, &Frequency{Hz: y}), AngularAcceleration{radps2: x*y});
		assert_eq!(div_check(&AngularVelocity{radps: x}, &Frequency{Hz: y}), Angle{rad: x/y});
		assert_eq!(mul_check(&AngularVelocity{radps: x}, &MomentOfInertia{kgm2: y}), AngularMomentum{kgm2radps: x*y});
		assert_eq!(div_check(&AreaDensity{kgpm2: x}, &Distance{m: y}), Density{kgpm3: x/y});
		assert_eq!(mul_check(&AreaDensity{kgpm2: x}, &Area{m2: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&AreaDensity{kgpm2: x}, &Acceleration{mps2: y}), Pressure{Pa: x*y});
		assert_eq!(div_check(&AreaDensity{kgpm2: x}, &Density{kgpm3: y}), Distance{m: x/y});
		assert_eq!(mul_check(&Density{kgpm3: x}, &Distance{m: y}), AreaDensity{kgpm2: x*y});
		assert_eq!(mul_check(&Density{kgpm3: x}, &Volume{m3: y}), Mass{kg: x*y});
		assert_eq!(div_check(&Energy{J: x}, &Current{A: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Distance{m: y}), Force{N: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Time{s: y}), Power{W: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Charge{C: y}), Voltage{V: x/y});
		assert_eq!(div_check(&Energy{J: x}, &MagneticFlux{Wb: y}), Current{A: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Voltage{V: y}), Charge{C: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Volume{m3: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Force{N: y}), Distance{m: x/y});
		assert_eq!(mul_check(&Energy{J: x}, &Frequency{Hz: y}), Power{W: x*y});
		assert_eq!(div_check(&Energy{J: x}, &Momentum{kgmps: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Power{W: y}), Time{s: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Pressure{Pa: y}), Volume{m3: x/y});
		assert_eq!(div_check(&Energy{J: x}, &Velocity{mps: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&Energy{J: x}, &AbsorbedDose{Gy: y}), Mass{kg: x/y});
		assert_eq!(div_check(&Energy{J: x}, &DoseEquivalent{Sv: y}), Mass{kg: x/y});
		assert_eq!(mul_check(&Force{N: x}, &Distance{m: y}), Energy{J: x*y});
		assert_eq!(div_check(&Force{N: x}, &Mass{kg: y}), Acceleration{mps2: x/y});
		assert_eq!(mul_check(&Force{N: x}, &Time{s: y}), Momentum{kgmps: x*y});
		assert_eq!(div_check(&Force{N: x}, &Area{m2: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&Force{N: x}, &Acceleration{mps2: y}), Mass{kg: x/y});
		assert_eq!(div_check(&Force{N: x}, &Frequency{Hz: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&Force{N: x}, &Momentum{kgmps: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Force{N: x}, &Pressure{Pa: y}), Area{m2: x/y});
		assert_eq!(mul_check(&Force{N: x}, &Velocity{mps: y}), Power{W: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Amount{mol: y}), CatalyticActivity{molps: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Distance{m: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Capacitance{F: y}), Conductance{S: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Charge{C: y}), Current{A: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Inductance{H: y}), Resistance{Ohm: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &MagneticFlux{Wb: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Angle{rad: y}), AngularVelocity{radps: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &AngularVelocity{radps: y}), AngularAcceleration{radps2: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Energy{J: y}), Power{W: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Torque{Nm: y}), Power{W: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Momentum{kgmps: y}), Force{N: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Velocity{mps: y}), Acceleration{mps2: x*y});
		assert_eq!(div_check(&MomentOfInertia{kgm2: x}, &Mass{kg: y}), Area{m2: x/y});
		assert_eq!(div_check(&MomentOfInertia{kgm2: x}, &Area{m2: y}), Mass{kg: x/y});
		assert_eq!(mul_check(&MomentOfInertia{kgm2: x}, &AngularVelocity{radps: y}), AngularMomentum{kgm2radps: x*y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Mass{kg: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Time{s: y}), Force{N: x/y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &Acceleration{mps2: y}), Power{W: x*y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Force{N: y}), Time{s: x/y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &Frequency{Hz: y}), Force{N: x*y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &Velocity{mps: y}), Energy{J: x*y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Velocity{mps: y}), Mass{kg: x/y});
		assert_eq!(div_check(&Power{W: x}, &Current{A: y}), Voltage{V: x/y});
		assert_eq!(mul_check(&Power{W: x}, &Time{s: y}), Energy{J: x*y});
		assert_eq!(div_check(&Power{W: x}, &Voltage{V: y}), Current{A: x/y});
		assert_eq!(div_check(&Power{W: x}, &Acceleration{mps2: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&Power{W: x}, &Energy{J: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Power{W: x}, &Torque{Nm: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Power{W: x}, &Force{N: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&Power{W: x}, &Frequency{Hz: y}), Energy{J: x/y});
		assert_eq!(div_check(&Power{W: x}, &Momentum{kgmps: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&Power{W: x}, &Velocity{mps: y}), Force{N: x/y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &Area{m2: y}), Force{N: x*y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &Volume{m3: y}), Energy{J: x*y});
		assert_eq!(div_check(&Pressure{Pa: x}, &Acceleration{mps2: y}), AreaDensity{kgpm2: x/y});
		assert_eq!(div_check(&Pressure{Pa: x}, &AreaDensity{kgpm2: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Current{A: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Distance{m: y}), Force{N: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Time{s: y}), Power{W: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Charge{C: y}), Voltage{V: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &MagneticFlux{Wb: y}), Current{A: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Voltage{V: y}), Charge{C: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Volume{m3: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Force{N: y}), Distance{m: x/y});
		assert_eq!(mul_check(&Torque{Nm: x}, &Frequency{Hz: y}), Power{W: x*y});
		assert_eq!(div_check(&Torque{Nm: x}, &Momentum{kgmps: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Power{W: y}), Time{s: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Pressure{Pa: y}), Volume{m3: x/y});
		assert_eq!(div_check(&Torque{Nm: x}, &Velocity{mps: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&Velocity{mps: x}, &Distance{m: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &Mass{kg: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&Velocity{mps: x}, &Time{s: y}), Distance{m: x*y});
		assert_eq!(div_check(&Velocity{mps: x}, &Time{s: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&Velocity{mps: x}, &Acceleration{mps2: y}), Time{s: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &Force{N: y}), Power{W: x*y});
		assert_eq!(mul_check(&Velocity{mps: x}, &Frequency{Hz: y}), Acceleration{mps2: x*y});
		assert_eq!(div_check(&Velocity{mps: x}, &Frequency{Hz: y}), Distance{m: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &Momentum{kgmps: y}), Energy{J: x*y});
		assert_eq!(mul_check(&AbsorbedDose{Gy: x}, &Mass{kg: y}), Energy{J: x*y});
		assert_eq!(mul_check(&DoseEquivalent{Sv: x}, &Mass{kg: y}), Energy{J: x*y});
		assert_eq!(div_check(&(x as f64), &Time{s: y as f64}), Frequency{Hz: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Time{s: y as f32}), Frequency{Hz: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Time{s: y as i64}), Frequency{Hz: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Time{s: y as i32}), Frequency{Hz: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Time{s: y as f64}), Frequency{Hz: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Time{s: y as f32}), Frequency{Hz: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Time{s: y as i64}), Frequency{Hz: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Time{s: y as i32}), Frequency{Hz: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Conductance{S: y as f64}), Resistance{Ohm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Conductance{S: y as f32}), Resistance{Ohm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Conductance{S: y as i64}), Resistance{Ohm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Conductance{S: y as i32}), Resistance{Ohm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Resistance{Ohm: y as f64}), Conductance{S: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Resistance{Ohm: y as f32}), Conductance{S: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Resistance{Ohm: y as i64}), Conductance{S: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Resistance{Ohm: y as i32}), Conductance{S: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Conductance{S: y as f64}), Resistance{Ohm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Conductance{S: y as f32}), Resistance{Ohm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Conductance{S: y as i64}), Resistance{Ohm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Conductance{S: y as i32}), Resistance{Ohm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Resistance{Ohm: y as f64}), Conductance{S: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Resistance{Ohm: y as f32}), Conductance{S: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Resistance{Ohm: y as i64}), Conductance{S: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Resistance{Ohm: y as i32}), Conductance{S: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Frequency{Hz: y as f64}), Time{s: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Frequency{Hz: y as f32}), Time{s: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Frequency{Hz: y as i64}), Time{s: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Frequency{Hz: y as i32}), Time{s: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Frequency{Hz: y as f64}), Time{s: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Frequency{Hz: y as f32}), Time{s: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Frequency{Hz: y as i64}), Time{s: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Frequency{Hz: y as i32}), Time{s: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Radioactivity{Bq: y as f64}), Time{s: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Radioactivity{Bq: y as f32}), Time{s: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Radioactivity{Bq: y as i64}), Time{s: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Radioactivity{Bq: y as i32}), Time{s: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Radioactivity{Bq: y as f64}), Time{s: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Radioactivity{Bq: y as f32}), Time{s: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Radioactivity{Bq: y as i64}), Time{s: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Radioactivity{Bq: y as i32}), Time{s: x as i32/y as i32});
		assert_eq!(div_check(&Amount{mol: x}, &Mass{kg: y}), Molality{molpkg: x/y});
		assert_eq!(div_check(&Amount{mol: x}, &Molality{molpkg: y}), Mass{kg: x/y});
		assert_eq!(mul_check(&Amount{mol: x}, &MolarMass{kgpmol: y}), Mass{kg: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &Amount{mol: y}), MolarMass{kgpmol: x/y});
		assert_eq!(mul_check(&Mass{kg: x}, &Molality{molpkg: y}), Amount{mol: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &MolarMass{kgpmol: y}), Amount{mol: x/y});
		assert_eq!(div_check(&Concentration{molpm3: x}, &Molality{molpkg: y}), Density{kgpm3: x/y});
		assert_eq!(mul_check(&Concentration{molpm3: x}, &MolarMass{kgpmol: y}), Density{kgpm3: x*y});
		assert_eq!(div_check(&Concentration{molpm3: x}, &Density{kgpm3: y}), Molality{molpkg: x/y});
		assert_eq!(mul_check(&Molality{molpkg: x}, &Mass{kg: y}), Amount{mol: x*y});
		assert_eq!(mul_check(&Molality{molpkg: x}, &Density{kgpm3: y}), Concentration{molpm3: x*y});
		assert_eq!(mul_check(&MolarMass{kgpmol: x}, &Amount{mol: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&MolarMass{kgpmol: x}, &Concentration{molpm3: y}), Density{kgpm3: x*y});
		assert_eq!(div_check(&Density{kgpm3: x}, &Concentration{molpm3: y}), MolarMass{kgpmol: x/y});
		assert_eq!(mul_check(&Density{kgpm3: x}, &Molality{molpkg: y}), Concentration{molpm3: x*y});
		assert_eq!(div_check(&Density{kgpm3: x}, &MolarMass{kgpmol: y}), Concentration{molpm3: x/y});
		assert_eq!(div_check(&(x as f64), &Molality{molpkg: y as f64}), MolarMass{kgpmol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Molality{molpkg: y as f32}), MolarMass{kgpmol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Molality{molpkg: y as i64}), MolarMass{kgpmol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Molality{molpkg: y as i32}), MolarMass{kgpmol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MolarMass{kgpmol: y as f64}), Molality{molpkg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MolarMass{kgpmol: y as f32}), Molality{molpkg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MolarMass{kgpmol: y as i64}), Molality{molpkg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MolarMass{kgpmol: y as i32}), Molality{molpkg: x as i32/y as i32});
	}

	#[test]
	fn test_bigfloat_unit_conversions() {
		use num_bigfloat::BigFloat;
		let x = 4.5f64;
		let y = 2.5f64;
		assert_eq!(div_check(
			&BigFloat::from(x), &Time{s: BigFloat::from(y)}),
				   Frequency{Hz: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Time{s: BigFloat::from(y)}),
				   Frequency{Hz: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Conductance{S: BigFloat::from(y)}),
				   Resistance{Ohm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Resistance{Ohm: BigFloat::from(y)}),
				   Conductance{S: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Conductance{S: BigFloat::from(y)}),
				   Resistance{Ohm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Resistance{Ohm: BigFloat::from(y)}),
				   Conductance{S: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Frequency{Hz: BigFloat::from(y)}),
				   Time{s: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Frequency{Hz: BigFloat::from(y)}),
				   Time{s: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Radioactivity{Bq: BigFloat::from(y)}),
				   Time{s: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Radioactivity{Bq: BigFloat::from(y)}),
				   Time{s: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Molality{molpkg: BigFloat::from(y)}),
				   MolarMass{kgpmol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MolarMass{kgpmol: BigFloat::from(y)}),
				   Molality{molpkg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Molality{molpkg: BigFloat::from(y)}),
				   MolarMass{kgpmol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MolarMass{kgpmol: BigFloat::from(y)}),
				   Molality{molpkg: BigFloat::from(x)/BigFloat::from(y)}
		);
	}

	#[test]
	fn test_complex_unit_conversions() {
		use num_complex::{Complex32, Complex64};
		let x = 4.5f64;
		let y = 2.5f64;
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Time{s: Complex32::from(y as f32)}),
				   Frequency{Hz: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Time{s: Complex64::from(y)}),
				   Frequency{Hz: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Time{s: Complex32::from(y as f32)}),
				   Frequency{Hz: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Time{s: Complex64::from(y)}),
				   Frequency{Hz: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Conductance{S: Complex32::from(y as f32)}),
				   Resistance{Ohm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Conductance{S: Complex64::from(y)}),
				   Resistance{Ohm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Resistance{Ohm: Complex32::from(y as f32)}),
				   Conductance{S: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Resistance{Ohm: Complex64::from(y)}),
				   Conductance{S: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Conductance{S: Complex32::from(y as f32)}),
				   Resistance{Ohm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Conductance{S: Complex64::from(y)}),
				   Resistance{Ohm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Resistance{Ohm: Complex32::from(y as f32)}),
				   Conductance{S: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Resistance{Ohm: Complex64::from(y)}),
				   Conductance{S: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Frequency{Hz: Complex32::from(y as f32)}),
				   Time{s: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Frequency{Hz: Complex64::from(y)}),
				   Time{s: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Frequency{Hz: Complex32::from(y as f32)}),
				   Time{s: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Frequency{Hz: Complex64::from(y)}),
				   Time{s: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Radioactivity{Bq: Complex32::from(y as f32)}),
				   Time{s: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Radioactivity{Bq: Complex64::from(y)}),
				   Time{s: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Radioactivity{Bq: Complex32::from(y as f32)}),
				   Time{s: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Radioactivity{Bq: Complex64::from(y)}),
				   Time{s: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Molality{molpkg: Complex32::from(y as f32)}),
				   MolarMass{kgpmol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Molality{molpkg: Complex64::from(y)}),
				   MolarMass{kgpmol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MolarMass{kgpmol: Complex32::from(y as f32)}),
				   Molality{molpkg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MolarMass{kgpmol: Complex64::from(y)}),
				   Molality{molpkg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Molality{molpkg: Complex32::from(y as f32)}),
				   MolarMass{kgpmol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Molality{molpkg: Complex64::from(y)}),
				   MolarMass{kgpmol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MolarMass{kgpmol: Complex32::from(y as f32)}),
				   Molality{molpkg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MolarMass{kgpmol: Complex64::from(y)}),
				   Molality{molpkg: Complex64::from(x)/Complex64::from(y)}
		);
	}

	#[test]
	fn amount_units() {
		assert_approx_equal(
			Amount::from_mol(1.66053906717385e-24_f64).to_mol(),
			Amount::from_count(1.0_f64).to_mol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1.0_f64).to_mol() * 6.02214076e+23,
			Amount::from_mol(1.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Amount::from_mol(0.001_f64).to_mol(),
			Amount::from_mmol(1.0_f64).to_mol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1.0_f64).to_mol() * 1000.0,
			Amount::from_mol(1.0_f64).to_mmol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1e-06_f64).to_mol(),
			Amount::from_umol(1.0_f64).to_mol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1.0_f64).to_mol() * 1000000.0,
			Amount::from_mol(1.0_f64).to_umol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1e-09_f64).to_mol(),
			Amount::from_nmol(1.0_f64).to_mol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1.0_f64).to_mol() * 1000000000.0,
			Amount::from_mol(1.0_f64).to_nmol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1e-12_f64).to_mol(),
			Amount::from_pmol(1.0_f64).to_mol(), 9
		);
		assert_approx_equal(
			Amount::from_mol(1.0_f64).to_mol() * 1000000000000.0,
			Amount::from_mol(1.0_f64).to_pmol(), 9
		);
	}

	#[test]
	fn current_units() {
		assert_approx_equal(
			Current::from_A(0.001_f64).to_A(),
			Current::from_mA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A() * 1000.0,
			Current::from_A(1.0_f64).to_mA(), 9
		);
		assert_approx_equal(
			Current::from_A(1e-06_f64).to_A(),
			Current::from_uA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A() * 1000000.0,
			Current::from_A(1.0_f64).to_uA(), 9
		);
		assert_approx_equal(
			Current::from_A(1e-09_f64).to_A(),
			Current::from_nA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A() * 1000000000.0,
			Current::from_A(1.0_f64).to_nA(), 9
		);
		assert_approx_equal(
			Current::from_A(1000.0_f64).to_A(),
			Current::from_kA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A() * 0.001,
			Current::from_A(1.0_f64).to_kA(), 9
		);
		assert_approx_equal(
			Current::from_A(1000000.0_f64).to_A(),
			Current::from_MA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A() * 1e-06,
			Current::from_A(1.0_f64).to_MA(), 9
		);
		assert_approx_equal(
			Current::from_A(1000000000.0_f64).to_A(),
			Current::from_GA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A() * 1e-09,
			Current::from_A(1.0_f64).to_GA(), 9
		);
	}

	#[test]
	fn distance_units() {
		assert_approx_equal(
			Distance::from_m(0.01_f64).to_m(),
			Distance::from_cm(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 100.0,
			Distance::from_m(1.0_f64).to_cm(), 9
		);
		assert_approx_equal(
			Distance::from_m(0.001_f64).to_m(),
			Distance::from_mm(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 1000.0,
			Distance::from_m(1.0_f64).to_mm(), 9
		);
		assert_approx_equal(
			Distance::from_m(1e-06_f64).to_m(),
			Distance::from_um(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 1000000.0,
			Distance::from_m(1.0_f64).to_um(), 9
		);
		assert_approx_equal(
			Distance::from_m(1e-09_f64).to_m(),
			Distance::from_nm(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 1000000000.0,
			Distance::from_m(1.0_f64).to_nm(), 9
		);
		assert_approx_equal(
			Distance::from_m(1e-12_f64).to_m(),
			Distance::from_pm(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 1000000000000.0,
			Distance::from_m(1.0_f64).to_pm(), 9
		);
		assert_approx_equal(
			Distance::from_m(1000.0_f64).to_m(),
			Distance::from_km(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 0.001,
			Distance::from_m(1.0_f64).to_km(), 9
		);
		assert_approx_equal(
			Distance::from_m(149597870700.0_f64).to_m(),
			Distance::from_au(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 6.68458712226845e-12,
			Distance::from_m(1.0_f64).to_au(), 9
		);
		assert_approx_equal(
			Distance::from_m(3.08568047999355e+16_f64).to_m(),
			Distance::from_parsec(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 3.24077624525171e-17,
			Distance::from_m(1.0_f64).to_parsec(), 9
		);
		assert_approx_equal(
			Distance::from_m(9460528169656200.0_f64).to_m(),
			Distance::from_lyr(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m() * 1.05702343681763e-16,
			Distance::from_m(1.0_f64).to_lyr(), 9
		);
	}

	#[test]
	fn luminosity_units() {
		assert_approx_equal(
			Luminosity::from_cd(0.001_f64).to_cd(),
			Luminosity::from_mcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd() * 1000.0,
			Luminosity::from_cd(1.0_f64).to_mcd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1e-06_f64).to_cd(),
			Luminosity::from_ucd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd() * 1000000.0,
			Luminosity::from_cd(1.0_f64).to_ucd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1e-09_f64).to_cd(),
			Luminosity::from_ncd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd() * 1000000000.0,
			Luminosity::from_cd(1.0_f64).to_ncd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1000.0_f64).to_cd(),
			Luminosity::from_kcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd() * 0.001,
			Luminosity::from_cd(1.0_f64).to_kcd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1000000.0_f64).to_cd(),
			Luminosity::from_Mcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd() * 1e-06,
			Luminosity::from_cd(1.0_f64).to_Mcd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1000000000.0_f64).to_cd(),
			Luminosity::from_Gcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd() * 1e-09,
			Luminosity::from_cd(1.0_f64).to_Gcd(), 9
		);
	}

	#[test]
	fn mass_units() {
		assert_approx_equal(
			Mass::from_kg(0.001_f64).to_kg(),
			Mass::from_g(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 1000.0,
			Mass::from_kg(1.0_f64).to_g(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1e-06_f64).to_kg(),
			Mass::from_mg(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 1000000.0,
			Mass::from_kg(1.0_f64).to_mg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1e-09_f64).to_kg(),
			Mass::from_ug(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 1000000000.0,
			Mass::from_kg(1.0_f64).to_ug(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1e-12_f64).to_kg(),
			Mass::from_ng(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 1000000000000.0,
			Mass::from_kg(1.0_f64).to_ng(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1e-15_f64).to_kg(),
			Mass::from_pg(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 1000000000000000.0,
			Mass::from_kg(1.0_f64).to_pg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1000.0_f64).to_kg(),
			Mass::from_tons(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 0.001,
			Mass::from_kg(1.0_f64).to_tons(), 9
		);
		assert_approx_equal(
			Mass::from_kg(5.9722e+24_f64).to_kg(),
			Mass::from_earth_mass(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 1.6744248350691502e-25,
			Mass::from_kg(1.0_f64).to_earth_mass(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.8986e+27_f64).to_kg(),
			Mass::from_jupiter_mass(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 5.26703887074687e-28,
			Mass::from_kg(1.0_f64).to_jupiter_mass(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.9885500000000002e+30_f64).to_kg(),
			Mass::from_solar_mass(1.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg() * 5.0287898217294e-31,
			Mass::from_kg(1.0_f64).to_solar_mass(), 9
		);
	}

	#[test]
	fn temperature_units() {
		assert_approx_equal(
			Temperature::from_K(274.1500000000000_f64).to_K(),
			Temperature::from_C(1.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_K(1.0_f64).to_K() * -272.1500000000000,
			Temperature::from_K(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Temperature::from_K(274.1500000000000_f64).to_K(),
			Temperature::from_celsius(1.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_K(1.0_f64).to_K() * -272.1500000000000,
			Temperature::from_K(1.0_f64).to_celsius(), 9
		);
		assert_approx_equal(
			Temperature::from_K(255.9277777777780_f64).to_K(),
			Temperature::from_F(1.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_K(1.0_f64).to_K() * -457.8700000000000,
			Temperature::from_K(1.0_f64).to_F(), 9
		);
	}

	#[test]
	fn time_units() {
		assert_approx_equal(
			Time::from_s(0.001_f64).to_s(),
			Time::from_ms(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 1000.0,
			Time::from_s(1.0_f64).to_ms(), 9
		);
		assert_approx_equal(
			Time::from_s(1e-06_f64).to_s(),
			Time::from_us(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 1000000.0,
			Time::from_s(1.0_f64).to_us(), 9
		);
		assert_approx_equal(
			Time::from_s(1e-09_f64).to_s(),
			Time::from_ns(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 1000000000.0,
			Time::from_s(1.0_f64).to_ns(), 9
		);
		assert_approx_equal(
			Time::from_s(1e-12_f64).to_s(),
			Time::from_ps(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 1000000000000.0,
			Time::from_s(1.0_f64).to_ps(), 9
		);
		assert_approx_equal(
			Time::from_s(60.0_f64).to_s(),
			Time::from_min(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 0.0166666666666667,
			Time::from_s(1.0_f64).to_min(), 9
		);
		assert_approx_equal(
			Time::from_s(3600.0_f64).to_s(),
			Time::from_hr(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 0.0002777777777777,
			Time::from_s(1.0_f64).to_hr(), 9
		);
		assert_approx_equal(
			Time::from_s(86400.0_f64).to_s(),
			Time::from_days(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 1.15740740740741e-05,
			Time::from_s(1.0_f64).to_days(), 9
		);
		assert_approx_equal(
			Time::from_s(604800.0_f64).to_s(),
			Time::from_weeks(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 1.65343915343915e-06,
			Time::from_s(1.0_f64).to_weeks(), 9
		);
		assert_approx_equal(
			Time::from_s(31556925.19008_f64).to_s(),
			Time::from_yr(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 3.16887654287165e-08,
			Time::from_s(1.0_f64).to_yr(), 9
		);
		assert_approx_equal(
			Time::from_s(31556925190.08_f64).to_s(),
			Time::from_kyr(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 3.16887654287165e-11,
			Time::from_s(1.0_f64).to_kyr(), 9
		);
		assert_approx_equal(
			Time::from_s(31556925190080.0_f64).to_s(),
			Time::from_Myr(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 3.16887654287165e-14,
			Time::from_s(1.0_f64).to_Myr(), 9
		);
		assert_approx_equal(
			Time::from_s(3.155692519008e+16_f64).to_s(),
			Time::from_Gyr(1.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s() * 3.16887654287165e-17,
			Time::from_s(1.0_f64).to_Gyr(), 9
		);
	}

	#[test]
	fn catalytic_activity_units() {
		assert_approx_equal(
			CatalyticActivity::from_molps(1.66053906717385e-24_f64).to_molps(),
			CatalyticActivity::from_Nps(1.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps() * 6.02214076e+23,
			CatalyticActivity::from_molps(1.0_f64).to_Nps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(0.001_f64).to_molps(),
			CatalyticActivity::from_mmolps(1.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps() * 1000.0,
			CatalyticActivity::from_molps(1.0_f64).to_mmolps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1e-06_f64).to_molps(),
			CatalyticActivity::from_umolps(1.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps() * 1000000.0,
			CatalyticActivity::from_molps(1.0_f64).to_umolps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1e-09_f64).to_molps(),
			CatalyticActivity::from_nmolps(1.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps() * 1000000000.0,
			CatalyticActivity::from_molps(1.0_f64).to_nmolps(), 9
		);
	}

	#[test]
	fn concentration_units() {
		assert_approx_equal(
			Concentration::from_molpm3(1.66053906717385e-24_f64).to_molpm3(),
			Concentration::from_Npm3(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 6.02214076e+23,
			Concentration::from_molpm3(1.0_f64).to_Npm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.66053906717385e-24_f64).to_molpm3(),
			Concentration::from_count_per_cubic_meter(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 6.02214076e+23,
			Concentration::from_molpm3(1.0_f64).to_count_per_cubic_meter(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.66053906717385e-21_f64).to_molpm3(),
			Concentration::from_NpL(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 6.02214076e+26,
			Concentration::from_molpm3(1.0_f64).to_NpL(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.66053906717385e-21_f64).to_molpm3(),
			Concentration::from_count_per_L(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 6.02214076e+26,
			Concentration::from_molpm3(1.0_f64).to_count_per_L(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.66053906717385e-18_f64).to_molpm3(),
			Concentration::from_Npcc(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 6.02214076e+29,
			Concentration::from_molpm3(1.0_f64).to_Npcc(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.66053906717385e-18_f64).to_molpm3(),
			Concentration::from_count_per_cc(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 6.02214076e+29,
			Concentration::from_molpm3(1.0_f64).to_count_per_cc(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1000.0_f64).to_molpm3(),
			Concentration::from_M(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 0.001,
			Concentration::from_molpm3(1.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1000.0_f64).to_molpm3(),
			Concentration::from_molarity(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 0.001,
			Concentration::from_molpm3(1.0_f64).to_molarity(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(0.001_f64).to_molpm3(),
			Concentration::from_uM(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 1000.0,
			Concentration::from_molpm3(1.0_f64).to_uM(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1e-06_f64).to_molpm3(),
			Concentration::from_nM(1.0_f64).to_molpm3(), 9
		);
		assert_approx_equal(
			Concentration::from_molpm3(1.0_f64).to_molpm3() * 1000000.0,
			Concentration::from_molpm3(1.0_f64).to_nM(), 9
		);
	}

	#[test]
	fn molality_units() {
		assert_approx_equal(
			Molality::from_molpkg(0.001_f64).to_molpkg(),
			Molality::from_mmolpkg(1.0_f64).to_molpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1.0_f64).to_molpkg() * 1000.0,
			Molality::from_molpkg(1.0_f64).to_mmolpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1e-06_f64).to_molpkg(),
			Molality::from_umolpkg(1.0_f64).to_molpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1.0_f64).to_molpkg() * 1000000.0,
			Molality::from_molpkg(1.0_f64).to_umolpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1e-09_f64).to_molpkg(),
			Molality::from_nmolpkg(1.0_f64).to_molpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1.0_f64).to_molpkg() * 1000000000.0,
			Molality::from_molpkg(1.0_f64).to_nmolpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(0.001_f64).to_molpkg(),
			Molality::from_umolpg(1.0_f64).to_molpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1.0_f64).to_molpkg() * 1000.0,
			Molality::from_molpkg(1.0_f64).to_umolpg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1e-06_f64).to_molpkg(),
			Molality::from_nmolpg(1.0_f64).to_molpkg(), 9
		);
		assert_approx_equal(
			Molality::from_molpkg(1.0_f64).to_molpkg() * 1000000.0,
			Molality::from_molpkg(1.0_f64).to_nmolpg(), 9
		);
	}

	#[test]
	fn molar_mass_units() {
		assert_approx_equal(
			MolarMass::from_kgpmol(0.001_f64).to_kgpmol(),
			MolarMass::from_gpmol(1.0_f64).to_kgpmol(), 9
		);
		assert_approx_equal(
			MolarMass::from_kgpmol(1.0_f64).to_kgpmol() * 1000.0,
			MolarMass::from_kgpmol(1.0_f64).to_gpmol(), 9
		);
		assert_approx_equal(
			MolarMass::from_kgpmol(0.001_f64).to_kgpmol(),
			MolarMass::from_grams_per_mole(1.0_f64).to_kgpmol(), 9
		);
		assert_approx_equal(
			MolarMass::from_kgpmol(1.0_f64).to_kgpmol() * 1000.0,
			MolarMass::from_kgpmol(1.0_f64).to_grams_per_mole(), 9
		);
	}

	#[test]
	fn inverse_specific_heat_capacity_units() {
		assert_approx_equal(
			InverseSpecificHeatCapacity::from_kgK_per_J(0.001_f64).to_kgK_per_J(),
			InverseSpecificHeatCapacity::from_grams_kelvin_per_joule(1.0_f64).to_kgK_per_J(), 9
		);
		assert_approx_equal(
			InverseSpecificHeatCapacity::from_kgK_per_J(1.0_f64).to_kgK_per_J() * 1000.0,
			InverseSpecificHeatCapacity::from_kgK_per_J(1.0_f64).to_grams_kelvin_per_joule(), 9
		);
		assert_approx_equal(
			InverseSpecificHeatCapacity::from_kgK_per_J(0.001_f64).to_kgK_per_J(),
			InverseSpecificHeatCapacity::from_gK_per_J(1.0_f64).to_kgK_per_J(), 9
		);
		assert_approx_equal(
			InverseSpecificHeatCapacity::from_kgK_per_J(1.0_f64).to_kgK_per_J() * 1000.0,
			InverseSpecificHeatCapacity::from_kgK_per_J(1.0_f64).to_gK_per_J(), 9
		);
	}

	#[test]
	fn capacitance_units() {
		assert_approx_equal(
			Capacitance::from_F(0.001_f64).to_F(),
			Capacitance::from_mF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 1000.0,
			Capacitance::from_F(1.0_f64).to_mF(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1e-06_f64).to_F(),
			Capacitance::from_uF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 1000000.0,
			Capacitance::from_F(1.0_f64).to_uF(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1e-09_f64).to_F(),
			Capacitance::from_nF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 1000000000.0,
			Capacitance::from_F(1.0_f64).to_nF(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1e-12_f64).to_F(),
			Capacitance::from_pF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 1000000000000.0,
			Capacitance::from_F(1.0_f64).to_pF(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1000.0_f64).to_F(),
			Capacitance::from_kF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 0.001,
			Capacitance::from_F(1.0_f64).to_kF(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1000000.0_f64).to_F(),
			Capacitance::from_MF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 1e-06,
			Capacitance::from_F(1.0_f64).to_MF(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1000000000.0_f64).to_F(),
			Capacitance::from_GF(1.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F() * 1e-09,
			Capacitance::from_F(1.0_f64).to_GF(), 9
		);
	}

	#[test]
	fn charge_units() {
		assert_approx_equal(
			Charge::from_C(0.001_f64).to_C(),
			Charge::from_mC(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 1000.0,
			Charge::from_C(1.0_f64).to_mC(), 9
		);
		assert_approx_equal(
			Charge::from_C(1e-06_f64).to_C(),
			Charge::from_uC(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 1000000.0,
			Charge::from_C(1.0_f64).to_uC(), 9
		);
		assert_approx_equal(
			Charge::from_C(1e-09_f64).to_C(),
			Charge::from_nC(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 1000000000.0,
			Charge::from_C(1.0_f64).to_nC(), 9
		);
		assert_approx_equal(
			Charge::from_C(1000.0_f64).to_C(),
			Charge::from_kC(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 0.001,
			Charge::from_C(1.0_f64).to_kC(), 9
		);
		assert_approx_equal(
			Charge::from_C(1000000.0_f64).to_C(),
			Charge::from_MC(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 1e-06,
			Charge::from_C(1.0_f64).to_MC(), 9
		);
		assert_approx_equal(
			Charge::from_C(1000000000.0_f64).to_C(),
			Charge::from_GC(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 1e-09,
			Charge::from_C(1.0_f64).to_GC(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.6021766340000001e-19_f64).to_C(),
			Charge::from_p(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * 6.24150907446076e+18,
			Charge::from_C(1.0_f64).to_p(), 9
		);
		assert_approx_equal(
			Charge::from_C(-1.6021766340000001e-19_f64).to_C(),
			Charge::from_e(1.0_f64).to_C(), 9
		);
		assert_approx_equal(
			Charge::from_C(1.0_f64).to_C() * -6.24150907446076e+18,
			Charge::from_C(1.0_f64).to_e(), 9
		);
	}

	#[test]
	fn conductance_units() {
		assert_approx_equal(
			Conductance::from_S(0.001_f64).to_S(),
			Conductance::from_mS(1.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S() * 1000.0,
			Conductance::from_S(1.0_f64).to_mS(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1e-06_f64).to_S(),
			Conductance::from_uS(1.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S() * 1000000.0,
			Conductance::from_S(1.0_f64).to_uS(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1e-09_f64).to_S(),
			Conductance::from_nS(1.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S() * 1000000000.0,
			Conductance::from_S(1.0_f64).to_nS(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1000.0_f64).to_S(),
			Conductance::from_kS(1.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S() * 0.001,
			Conductance::from_S(1.0_f64).to_kS(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1000000.0_f64).to_S(),
			Conductance::from_MS(1.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S() * 1e-06,
			Conductance::from_S(1.0_f64).to_MS(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1000000000.0_f64).to_S(),
			Conductance::from_GS(1.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S() * 1e-09,
			Conductance::from_S(1.0_f64).to_GS(), 9
		);
	}

	#[test]
	fn illuminance_units() {
		assert_approx_equal(
			Illuminance::from_lux(0.001_f64).to_lux(),
			Illuminance::from_mlux(1.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux() * 1000.0,
			Illuminance::from_lux(1.0_f64).to_mlux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1e-06_f64).to_lux(),
			Illuminance::from_ulux(1.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux() * 1000000.0,
			Illuminance::from_lux(1.0_f64).to_ulux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1e-09_f64).to_lux(),
			Illuminance::from_nlux(1.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux() * 1000000000.0,
			Illuminance::from_lux(1.0_f64).to_nlux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1000.0_f64).to_lux(),
			Illuminance::from_klux(1.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux() * 0.001,
			Illuminance::from_lux(1.0_f64).to_klux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1000000.0_f64).to_lux(),
			Illuminance::from_Mlux(1.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux() * 1e-06,
			Illuminance::from_lux(1.0_f64).to_Mlux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1000000000.0_f64).to_lux(),
			Illuminance::from_Glux(1.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux() * 1e-09,
			Illuminance::from_lux(1.0_f64).to_Glux(), 9
		);
	}

	#[test]
	fn inductance_units() {
		assert_approx_equal(
			Inductance::from_H(0.001_f64).to_H(),
			Inductance::from_mH(1.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H() * 1000.0,
			Inductance::from_H(1.0_f64).to_mH(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1e-06_f64).to_H(),
			Inductance::from_uH(1.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H() * 1000000.0,
			Inductance::from_H(1.0_f64).to_uH(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1e-09_f64).to_H(),
			Inductance::from_nH(1.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H() * 1000000000.0,
			Inductance::from_H(1.0_f64).to_nH(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1000.0_f64).to_H(),
			Inductance::from_kH(1.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H() * 0.001,
			Inductance::from_H(1.0_f64).to_kH(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1000000.0_f64).to_H(),
			Inductance::from_MH(1.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H() * 1e-06,
			Inductance::from_H(1.0_f64).to_MH(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1000000000.0_f64).to_H(),
			Inductance::from_GH(1.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H() * 1e-09,
			Inductance::from_H(1.0_f64).to_GH(), 9
		);
	}

	#[test]
	fn luminous_flux_units() {
		assert_approx_equal(
			LuminousFlux::from_lm(0.001_f64).to_lm(),
			LuminousFlux::from_mlm(1.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm() * 1000.0,
			LuminousFlux::from_lm(1.0_f64).to_mlm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1e-06_f64).to_lm(),
			LuminousFlux::from_ulm(1.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm() * 1000000.0,
			LuminousFlux::from_lm(1.0_f64).to_ulm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1e-09_f64).to_lm(),
			LuminousFlux::from_nlm(1.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm() * 1000000000.0,
			LuminousFlux::from_lm(1.0_f64).to_nlm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1000.0_f64).to_lm(),
			LuminousFlux::from_klm(1.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm() * 0.001,
			LuminousFlux::from_lm(1.0_f64).to_klm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1000000.0_f64).to_lm(),
			LuminousFlux::from_Mlm(1.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm() * 1e-06,
			LuminousFlux::from_lm(1.0_f64).to_Mlm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1000000000.0_f64).to_lm(),
			LuminousFlux::from_Glm(1.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm() * 1e-09,
			LuminousFlux::from_lm(1.0_f64).to_Glm(), 9
		);
	}

	#[test]
	fn magnetic_flux_units() {
		assert_approx_equal(
			MagneticFlux::from_Wb(0.001_f64).to_Wb(),
			MagneticFlux::from_mWb(1.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb() * 1000.0,
			MagneticFlux::from_Wb(1.0_f64).to_mWb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1e-06_f64).to_Wb(),
			MagneticFlux::from_uWb(1.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb() * 1000000.0,
			MagneticFlux::from_Wb(1.0_f64).to_uWb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1e-09_f64).to_Wb(),
			MagneticFlux::from_nWb(1.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb() * 1000000000.0,
			MagneticFlux::from_Wb(1.0_f64).to_nWb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1000.0_f64).to_Wb(),
			MagneticFlux::from_kWb(1.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb() * 0.001,
			MagneticFlux::from_Wb(1.0_f64).to_kWb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1000000.0_f64).to_Wb(),
			MagneticFlux::from_MWb(1.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb() * 1e-06,
			MagneticFlux::from_Wb(1.0_f64).to_MWb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1000000000.0_f64).to_Wb(),
			MagneticFlux::from_GWb(1.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb() * 1e-09,
			MagneticFlux::from_Wb(1.0_f64).to_GWb(), 9
		);
	}

	#[test]
	fn magnetic_flux_density_units() {
		assert_approx_equal(
			MagneticFluxDensity::from_T(0.001_f64).to_T(),
			MagneticFluxDensity::from_mT(1.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T() * 1000.0,
			MagneticFluxDensity::from_T(1.0_f64).to_mT(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1e-06_f64).to_T(),
			MagneticFluxDensity::from_uT(1.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T() * 1000000.0,
			MagneticFluxDensity::from_T(1.0_f64).to_uT(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1e-09_f64).to_T(),
			MagneticFluxDensity::from_nT(1.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T() * 1000000000.0,
			MagneticFluxDensity::from_T(1.0_f64).to_nT(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1000.0_f64).to_T(),
			MagneticFluxDensity::from_kT(1.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T() * 0.001,
			MagneticFluxDensity::from_T(1.0_f64).to_kT(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1000000.0_f64).to_T(),
			MagneticFluxDensity::from_MT(1.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T() * 1e-06,
			MagneticFluxDensity::from_T(1.0_f64).to_MT(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1000000000.0_f64).to_T(),
			MagneticFluxDensity::from_GT(1.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T() * 1e-09,
			MagneticFluxDensity::from_T(1.0_f64).to_GT(), 9
		);
	}

	#[test]
	fn resistance_units() {
		assert_approx_equal(
			Resistance::from_Ohm(0.001_f64).to_Ohm(),
			Resistance::from_mOhm(1.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm() * 1000.0,
			Resistance::from_Ohm(1.0_f64).to_mOhm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1e-06_f64).to_Ohm(),
			Resistance::from_uOhm(1.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm() * 1000000.0,
			Resistance::from_Ohm(1.0_f64).to_uOhm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1e-09_f64).to_Ohm(),
			Resistance::from_nOhm(1.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm() * 1000000000.0,
			Resistance::from_Ohm(1.0_f64).to_nOhm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1000.0_f64).to_Ohm(),
			Resistance::from_kOhm(1.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm() * 0.001,
			Resistance::from_Ohm(1.0_f64).to_kOhm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1000000.0_f64).to_Ohm(),
			Resistance::from_MOhm(1.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm() * 1e-06,
			Resistance::from_Ohm(1.0_f64).to_MOhm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1000000000.0_f64).to_Ohm(),
			Resistance::from_GOhm(1.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm() * 1e-09,
			Resistance::from_Ohm(1.0_f64).to_GOhm(), 9
		);
	}

	#[test]
	fn voltage_units() {
		assert_approx_equal(
			Voltage::from_V(0.001_f64).to_V(),
			Voltage::from_mV(1.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V() * 1000.0,
			Voltage::from_V(1.0_f64).to_mV(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1e-06_f64).to_V(),
			Voltage::from_uV(1.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V() * 1000000.0,
			Voltage::from_V(1.0_f64).to_uV(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1e-09_f64).to_V(),
			Voltage::from_nV(1.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V() * 1000000000.0,
			Voltage::from_V(1.0_f64).to_nV(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1000.0_f64).to_V(),
			Voltage::from_kV(1.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V() * 0.001,
			Voltage::from_V(1.0_f64).to_kV(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1000000.0_f64).to_V(),
			Voltage::from_MV(1.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V() * 1e-06,
			Voltage::from_V(1.0_f64).to_MV(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1000000000.0_f64).to_V(),
			Voltage::from_GV(1.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V() * 1e-09,
			Voltage::from_V(1.0_f64).to_GV(), 9
		);
	}

	#[test]
	fn angle_units() {
		assert_approx_equal(
			Angle::from_rad(0.0174532925199433_f64).to_rad(),
			Angle::from_degrees(1.0_f64).to_rad(), 9
		);
		assert_approx_equal(
			Angle::from_rad(1.0_f64).to_rad() * 57.2957795130823,
			Angle::from_rad(1.0_f64).to_degrees(), 9
		);
		assert_approx_equal(
			Angle::from_rad(0.0174532925199433_f64).to_rad(),
			Angle::from_deg(1.0_f64).to_rad(), 9
		);
		assert_approx_equal(
			Angle::from_rad(1.0_f64).to_rad() * 57.2957795130823,
			Angle::from_rad(1.0_f64).to_deg(), 9
		);
	}

	#[test]
	fn area_units() {
		assert_approx_equal(
			Area::from_m2(0.0001_f64).to_m2(),
			Area::from_cm2(1.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2() * 10000.0,
			Area::from_m2(1.0_f64).to_cm2(), 9
		);
		assert_approx_equal(
			Area::from_m2(0.0001_f64).to_m2(),
			Area::from_square_cm(1.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2() * 10000.0,
			Area::from_m2(1.0_f64).to_square_cm(), 9
		);
		assert_approx_equal(
			Area::from_m2(1e-06_f64).to_m2(),
			Area::from_mm2(1.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2() * 1000000.0,
			Area::from_m2(1.0_f64).to_mm2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1e-12_f64).to_m2(),
			Area::from_um2(1.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2() * 1000000000000.0,
			Area::from_m2(1.0_f64).to_um2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1e-18_f64).to_m2(),
			Area::from_nm2(1.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2() * 1e+18,
			Area::from_m2(1.0_f64).to_nm2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1000000.0_f64).to_m2(),
			Area::from_km2(1.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2() * 1e-06,
			Area::from_m2(1.0_f64).to_km2(), 9
		);
	}

	#[test]
	fn volume_units() {
		assert_approx_equal(
			Volume::from_m3(1e-06_f64).to_m3(),
			Volume::from_cc(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000000.0,
			Volume::from_m3(1.0_f64).to_cc(), 9
		);
		assert_approx_equal(
			Volume::from_m3(0.001_f64).to_m3(),
			Volume::from_L(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000.0,
			Volume::from_m3(1.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_m3(0.001_f64).to_m3(),
			Volume::from_liters(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000.0,
			Volume::from_m3(1.0_f64).to_liters(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1e-06_f64).to_m3(),
			Volume::from_mL(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000000.0,
			Volume::from_m3(1.0_f64).to_mL(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1e-09_f64).to_m3(),
			Volume::from_uL(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000000000.0,
			Volume::from_m3(1.0_f64).to_uL(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1e-12_f64).to_m3(),
			Volume::from_nL(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000000000000.0,
			Volume::from_m3(1.0_f64).to_nL(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1e-15_f64).to_m3(),
			Volume::from_pL(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1000000000000000.0,
			Volume::from_m3(1.0_f64).to_pL(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1000.0_f64).to_m3(),
			Volume::from_ML(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 0.001,
			Volume::from_m3(1.0_f64).to_ML(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1000000.0_f64).to_m3(),
			Volume::from_GL(1.0_f64).to_m3(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_m3() * 1e-06,
			Volume::from_m3(1.0_f64).to_GL(), 9
		);
	}

	#[test]
	fn acceleration_units() {
		assert_approx_equal(
			Acceleration::from_mps2(0.001_f64).to_mps2(),
			Acceleration::from_mmps2(1.0_f64).to_mps2(), 9
		);
		assert_approx_equal(
			Acceleration::from_mps2(1.0_f64).to_mps2() * 1000.0,
			Acceleration::from_mps2(1.0_f64).to_mmps2(), 9
		);
		assert_approx_equal(
			Acceleration::from_mps2(1e-06_f64).to_mps2(),
			Acceleration::from_kilometers_per_hour_squared(1.0_f64).to_mps2(), 9
		);
		assert_approx_equal(
			Acceleration::from_mps2(1.0_f64).to_mps2() * 1000000.0,
			Acceleration::from_mps2(1.0_f64).to_kilometers_per_hour_squared(), 9
		);
		assert_approx_equal(
			Acceleration::from_mps2(7.71604938271605e-05_f64).to_mps2(),
			Acceleration::from_kph2(1.0_f64).to_mps2(), 9
		);
		assert_approx_equal(
			Acceleration::from_mps2(1.0_f64).to_mps2() * 12960.0,
			Acceleration::from_mps2(1.0_f64).to_kph2(), 9
		);
	}

	#[test]
	fn angular_acceleration_units() {
		assert_approx_equal(
			AngularAcceleration::from_radps2(0.0174532925199433_f64).to_radps2(),
			AngularAcceleration::from_degrees_per_second_squared(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(1.0_f64).to_radps2() * 57.2957795130823,
			AngularAcceleration::from_radps2(1.0_f64).to_degrees_per_second_squared(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(6.28318530717959_f64).to_radps2(),
			AngularAcceleration::from_rps2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(1.0_f64).to_radps2() * 0.159154943091895,
			AngularAcceleration::from_radps2(1.0_f64).to_rps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(0.0017453292519943_f64).to_radps2(),
			AngularAcceleration::from_rpm2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(1.0_f64).to_radps2() * 572.957795130823,
			AngularAcceleration::from_radps2(1.0_f64).to_rpm2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(0.0174532925199433_f64).to_radps2(),
			AngularAcceleration::from_degps2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(1.0_f64).to_radps2() * 57.2957795130823,
			AngularAcceleration::from_radps2(1.0_f64).to_degps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(4.84813681109536e-07_f64).to_radps2(),
			AngularAcceleration::from_rph2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(1.0_f64).to_radps2() * 2062648.06247096,
			AngularAcceleration::from_radps2(1.0_f64).to_rph2(), 9
		);
	}

	#[test]
	fn angular_momentum_units() {
		assert_approx_equal(
			AngularMomentum::from_kgm2radps(1e-07_f64).to_kgm2radps(),
			AngularMomentum::from_gcm2radps(1.0_f64).to_kgm2radps(), 9
		);
		assert_approx_equal(
			AngularMomentum::from_kgm2radps(1.0_f64).to_kgm2radps() * 10000000.0,
			AngularMomentum::from_kgm2radps(1.0_f64).to_gcm2radps(), 9
		);
	}

	#[test]
	fn angular_velocity_units() {
		assert_approx_equal(
			AngularVelocity::from_radps(0.0174532925199433_f64).to_radps(),
			AngularVelocity::from_degrees_per_second(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(1.0_f64).to_radps() * 57.2957795130823,
			AngularVelocity::from_radps(1.0_f64).to_degrees_per_second(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(0.0174532925199433_f64).to_radps(),
			AngularVelocity::from_degps(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(1.0_f64).to_radps() * 57.2957795130823,
			AngularVelocity::from_radps(1.0_f64).to_degps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(6.28318530717959_f64).to_radps(),
			AngularVelocity::from_rps(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(1.0_f64).to_radps() * 0.159154943091895,
			AngularVelocity::from_radps(1.0_f64).to_rps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(0.10471975511966_f64).to_radps(),
			AngularVelocity::from_rpm(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(1.0_f64).to_radps() * 9.54929658551372,
			AngularVelocity::from_radps(1.0_f64).to_rpm(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(0.0017453292519943_f64).to_radps(),
			AngularVelocity::from_rph(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(1.0_f64).to_radps() * 572.957795130823,
			AngularVelocity::from_radps(1.0_f64).to_rph(), 9
		);
	}

	#[test]
	fn area_density_units() {
		assert_approx_equal(
			AreaDensity::from_kgpm2(0.001_f64).to_kgpm2(),
			AreaDensity::from_gpm2(1.0_f64).to_kgpm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(1.0_f64).to_kgpm2() * 1000.0,
			AreaDensity::from_kgpm2(1.0_f64).to_gpm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(0.001_f64).to_kgpm2(),
			AreaDensity::from_grams_per_square_meter(1.0_f64).to_kgpm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(1.0_f64).to_kgpm2() * 1000.0,
			AreaDensity::from_kgpm2(1.0_f64).to_grams_per_square_meter(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(10.0_f64).to_kgpm2(),
			AreaDensity::from_gpcm2(1.0_f64).to_kgpm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(1.0_f64).to_kgpm2() * 0.1,
			AreaDensity::from_kgpm2(1.0_f64).to_gpcm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(10.0_f64).to_kgpm2(),
			AreaDensity::from_grams_per_square_cm(1.0_f64).to_kgpm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(1.0_f64).to_kgpm2() * 0.1,
			AreaDensity::from_kgpm2(1.0_f64).to_grams_per_square_cm(), 9
		);
	}

	#[test]
	fn density_units() {
		assert_approx_equal(
			Density::from_kgpm3(1000.0_f64).to_kgpm3(),
			Density::from_kgpL(1.0_f64).to_kgpm3(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1.0_f64).to_kgpm3() * 0.001,
			Density::from_kgpm3(1.0_f64).to_kgpL(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1000.0_f64).to_kgpm3(),
			Density::from_kilograms_per_liter(1.0_f64).to_kgpm3(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1.0_f64).to_kgpm3() * 0.001,
			Density::from_kgpm3(1.0_f64).to_kilograms_per_liter(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1000.0_f64).to_kgpm3(),
			Density::from_gpcc(1.0_f64).to_kgpm3(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1.0_f64).to_kgpm3() * 0.001,
			Density::from_kgpm3(1.0_f64).to_gpcc(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1000.0_f64).to_kgpm3(),
			Density::from_grams_per_cubic_centimeter(1.0_f64).to_kgpm3(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1.0_f64).to_kgpm3() * 0.001,
			Density::from_kgpm3(1.0_f64).to_grams_per_cubic_centimeter(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(0.001_f64).to_kgpm3(),
			Density::from_gpm3(1.0_f64).to_kgpm3(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1.0_f64).to_kgpm3() * 1000.0,
			Density::from_kgpm3(1.0_f64).to_gpm3(), 9
		);
	}

	#[test]
	fn energy_units() {
		assert_approx_equal(
			Energy::from_J(0.001_f64).to_J(),
			Energy::from_mJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 1000.0,
			Energy::from_J(1.0_f64).to_mJ(), 9
		);
		assert_approx_equal(
			Energy::from_J(1e-06_f64).to_J(),
			Energy::from_uJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 1000000.0,
			Energy::from_J(1.0_f64).to_uJ(), 9
		);
		assert_approx_equal(
			Energy::from_J(1e-09_f64).to_J(),
			Energy::from_nJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 1000000000.0,
			Energy::from_J(1.0_f64).to_nJ(), 9
		);
		assert_approx_equal(
			Energy::from_J(1000.0_f64).to_J(),
			Energy::from_kJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 0.001,
			Energy::from_J(1.0_f64).to_kJ(), 9
		);
		assert_approx_equal(
			Energy::from_J(1000000.0_f64).to_J(),
			Energy::from_MJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 1e-06,
			Energy::from_J(1.0_f64).to_MJ(), 9
		);
		assert_approx_equal(
			Energy::from_J(1000000000.0_f64).to_J(),
			Energy::from_GJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 1e-09,
			Energy::from_J(1.0_f64).to_GJ(), 9
		);
		assert_approx_equal(
			Energy::from_J(4.184_f64).to_J(),
			Energy::from_cal(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 0.239005736137667,
			Energy::from_J(1.0_f64).to_cal(), 9
		);
		assert_approx_equal(
			Energy::from_J(4184.0_f64).to_J(),
			Energy::from_kcal(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 0.0002390057361376,
			Energy::from_J(1.0_f64).to_kcal(), 9
		);
		assert_approx_equal(
			Energy::from_J(3600.0_f64).to_J(),
			Energy::from_Whr(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 0.0002777777777777,
			Energy::from_J(1.0_f64).to_Whr(), 9
		);
		assert_approx_equal(
			Energy::from_J(3600000.0_f64).to_J(),
			Energy::from_kWhr(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 2.77777777777778e-07,
			Energy::from_J(1.0_f64).to_kWhr(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.6021766340000001e-19_f64).to_J(),
			Energy::from_eV(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 6.24150907446076e+18,
			Energy::from_J(1.0_f64).to_eV(), 9
		);
		assert_approx_equal(
			Energy::from_J(1055.0_f64).to_J(),
			Energy::from_BTU(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J() * 0.0009478672985781,
			Energy::from_J(1.0_f64).to_BTU(), 9
		);
	}

	#[test]
	fn force_units() {
		assert_approx_equal(
			Force::from_N(4.45756819483586_f64).to_N(),
			Force::from_lb(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 0.224337566199999,
			Force::from_N(1.0_f64).to_lb(), 9
		);
		assert_approx_equal(
			Force::from_N(9.8066500286389_f64).to_N(),
			Force::from_kgG(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 0.101971620999999,
			Force::from_N(1.0_f64).to_kgG(), 9
		);
		assert_approx_equal(
			Force::from_N(0.001_f64).to_N(),
			Force::from_mN(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 1000.0,
			Force::from_N(1.0_f64).to_mN(), 9
		);
		assert_approx_equal(
			Force::from_N(1e-06_f64).to_N(),
			Force::from_uN(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 1000000.0,
			Force::from_N(1.0_f64).to_uN(), 9
		);
		assert_approx_equal(
			Force::from_N(1e-09_f64).to_N(),
			Force::from_nN(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 1000000000.0,
			Force::from_N(1.0_f64).to_nN(), 9
		);
		assert_approx_equal(
			Force::from_N(1000.0_f64).to_N(),
			Force::from_kN(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 0.001,
			Force::from_N(1.0_f64).to_kN(), 9
		);
		assert_approx_equal(
			Force::from_N(1000000.0_f64).to_N(),
			Force::from_MN(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 1e-06,
			Force::from_N(1.0_f64).to_MN(), 9
		);
		assert_approx_equal(
			Force::from_N(1000000000.0_f64).to_N(),
			Force::from_GN(1.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N() * 1e-09,
			Force::from_N(1.0_f64).to_GN(), 9
		);
	}

	#[test]
	fn frequency_units() {
		assert_approx_equal(
			Frequency::from_Hz(1000.0_f64).to_Hz(),
			Frequency::from_kHz(1.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1.0_f64).to_Hz() * 0.001,
			Frequency::from_Hz(1.0_f64).to_kHz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1000000.0_f64).to_Hz(),
			Frequency::from_MHz(1.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1.0_f64).to_Hz() * 1e-06,
			Frequency::from_Hz(1.0_f64).to_MHz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1000000000.0_f64).to_Hz(),
			Frequency::from_GHz(1.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1.0_f64).to_Hz() * 1e-09,
			Frequency::from_Hz(1.0_f64).to_GHz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1000000000000.0_f64).to_Hz(),
			Frequency::from_THz(1.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_Hz(1.0_f64).to_Hz() * 1e-12,
			Frequency::from_Hz(1.0_f64).to_THz(), 9
		);
	}

	#[test]
	fn moment_of_inertia_units() {
		assert_approx_equal(
			MomentOfInertia::from_kgm2(10.0_f64).to_kgm2(),
			MomentOfInertia::from_gcm2(1.0_f64).to_kgm2(), 9
		);
		assert_approx_equal(
			MomentOfInertia::from_kgm2(1.0_f64).to_kgm2() * 0.1,
			MomentOfInertia::from_kgm2(1.0_f64).to_gcm2(), 9
		);
		assert_approx_equal(
			MomentOfInertia::from_kgm2(0.001_f64).to_kgm2(),
			MomentOfInertia::from_gm2(1.0_f64).to_kgm2(), 9
		);
		assert_approx_equal(
			MomentOfInertia::from_kgm2(1.0_f64).to_kgm2() * 1000.0,
			MomentOfInertia::from_kgm2(1.0_f64).to_gm2(), 9
		);
	}

	#[test]
	fn momentum_units() {
		assert_approx_equal(
			Momentum::from_kgmps(1e-05_f64).to_kgmps(),
			Momentum::from_gram_centimeters_per_second(1.0_f64).to_kgmps(), 9
		);
		assert_approx_equal(
			Momentum::from_kgmps(1.0_f64).to_kgmps() * 100000.0,
			Momentum::from_kgmps(1.0_f64).to_gram_centimeters_per_second(), 9
		);
		assert_approx_equal(
			Momentum::from_kgmps(1e-05_f64).to_kgmps(),
			Momentum::from_gcmps(1.0_f64).to_kgmps(), 9
		);
		assert_approx_equal(
			Momentum::from_kgmps(1.0_f64).to_kgmps() * 100000.0,
			Momentum::from_kgmps(1.0_f64).to_gcmps(), 9
		);
	}

	#[test]
	fn power_units() {
		assert_approx_equal(
			Power::from_W(0.001_f64).to_W(),
			Power::from_mW(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 1000.0,
			Power::from_W(1.0_f64).to_mW(), 9
		);
		assert_approx_equal(
			Power::from_W(1e-06_f64).to_W(),
			Power::from_uW(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 1000000.0,
			Power::from_W(1.0_f64).to_uW(), 9
		);
		assert_approx_equal(
			Power::from_W(1e-09_f64).to_W(),
			Power::from_nW(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 1000000000.0,
			Power::from_W(1.0_f64).to_nW(), 9
		);
		assert_approx_equal(
			Power::from_W(1000.0_f64).to_W(),
			Power::from_kW(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 0.001,
			Power::from_W(1.0_f64).to_kW(), 9
		);
		assert_approx_equal(
			Power::from_W(1000000.0_f64).to_W(),
			Power::from_MW(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 1e-06,
			Power::from_W(1.0_f64).to_MW(), 9
		);
		assert_approx_equal(
			Power::from_W(1000000000.0_f64).to_W(),
			Power::from_GW(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 1e-09,
			Power::from_W(1.0_f64).to_GW(), 9
		);
		assert_approx_equal(
			Power::from_W(745.7_f64).to_W(),
			Power::from_horsepower(1.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W() * 0.0013410218586563,
			Power::from_W(1.0_f64).to_horsepower(), 9
		);
	}

	#[test]
	fn pressure_units() {
		assert_approx_equal(
			Pressure::from_Pa(6894.7572931783_f64).to_Pa(),
			Pressure::from_psi(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 0.00014503773773,
			Pressure::from_Pa(1.0_f64).to_psi(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(0.001_f64).to_Pa(),
			Pressure::from_mPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 1000.0,
			Pressure::from_Pa(1.0_f64).to_mPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1e-06_f64).to_Pa(),
			Pressure::from_uPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 1000000.0,
			Pressure::from_Pa(1.0_f64).to_uPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1e-09_f64).to_Pa(),
			Pressure::from_nPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 1000000000.0,
			Pressure::from_Pa(1.0_f64).to_nPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1000.0_f64).to_Pa(),
			Pressure::from_kPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 0.001,
			Pressure::from_Pa(1.0_f64).to_kPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1000000.0_f64).to_Pa(),
			Pressure::from_MPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 1e-06,
			Pressure::from_Pa(1.0_f64).to_MPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1000000000.0_f64).to_Pa(),
			Pressure::from_GPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 1e-09,
			Pressure::from_Pa(1.0_f64).to_GPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(100.0_f64).to_Pa(),
			Pressure::from_hPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 0.01,
			Pressure::from_Pa(1.0_f64).to_hPa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(100000.0_f64).to_Pa(),
			Pressure::from_bar(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 1e-05,
			Pressure::from_Pa(1.0_f64).to_bar(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(100.0_f64).to_Pa(),
			Pressure::from_mbar(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 0.01,
			Pressure::from_Pa(1.0_f64).to_mbar(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(101325.0_f64).to_Pa(),
			Pressure::from_atm(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 9.86923266716013e-06,
			Pressure::from_Pa(1.0_f64).to_atm(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(133.3223684211_f64).to_Pa(),
			Pressure::from_torr(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 0.007500616827039,
			Pressure::from_Pa(1.0_f64).to_torr(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(133.3223684211_f64).to_Pa(),
			Pressure::from_mmHg(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_Pa(1.0_f64).to_Pa() * 0.007500616827039,
			Pressure::from_Pa(1.0_f64).to_mmHg(), 9
		);
	}

	#[test]
	fn torque_units() {
		assert_approx_equal(
			Torque::from_Nm(1.35581794833139_f64).to_Nm(),
			Torque::from_ftlb(1.0_f64).to_Nm(), 9
		);
		assert_approx_equal(
			Torque::from_Nm(1.0_f64).to_Nm() * 0.73756214927727,
			Torque::from_Nm(1.0_f64).to_ftlb(), 9
		);
	}

	#[test]
	fn velocity_units() {
		assert_approx_equal(
			Velocity::from_mps(0.01_f64).to_mps(),
			Velocity::from_cmps(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 100.0,
			Velocity::from_mps(1.0_f64).to_cmps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(0.001_f64).to_mps(),
			Velocity::from_mmps(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 1000.0,
			Velocity::from_mps(1.0_f64).to_mmps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(2.77777777777778e-07_f64).to_mps(),
			Velocity::from_mmph(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 3600000.0,
			Velocity::from_mps(1.0_f64).to_mmph(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(0.277777777777778_f64).to_mps(),
			Velocity::from_kph(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 3.6,
			Velocity::from_mps(1.0_f64).to_kph(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(0.44704_f64).to_mps(),
			Velocity::from_mph(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 2.2369362920544,
			Velocity::from_mps(1.0_f64).to_mph(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1000.0_f64).to_mps(),
			Velocity::from_kmps(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 0.001,
			Velocity::from_mps(1.0_f64).to_kmps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(299792458.0_f64).to_mps(),
			Velocity::from_c(1.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps() * 3.3356409519815204e-09,
			Velocity::from_mps(1.0_f64).to_c(), 9
		);
	}

	#[test]
	fn absorbed_dose_units() {
		assert_approx_equal(
			AbsorbedDose::from_Gy(0.001_f64).to_Gy(),
			AbsorbedDose::from_mGy(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 1000.0,
			AbsorbedDose::from_Gy(1.0_f64).to_mGy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1e-06_f64).to_Gy(),
			AbsorbedDose::from_uGy(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 1000000.0,
			AbsorbedDose::from_Gy(1.0_f64).to_uGy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1e-09_f64).to_Gy(),
			AbsorbedDose::from_nGy(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 1000000000.0,
			AbsorbedDose::from_Gy(1.0_f64).to_nGy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1000.0_f64).to_Gy(),
			AbsorbedDose::from_kGy(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 0.001,
			AbsorbedDose::from_Gy(1.0_f64).to_kGy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1000000.0_f64).to_Gy(),
			AbsorbedDose::from_MGy(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 1e-06,
			AbsorbedDose::from_Gy(1.0_f64).to_MGy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1000000000.0_f64).to_Gy(),
			AbsorbedDose::from_GGy(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 1e-09,
			AbsorbedDose::from_Gy(1.0_f64).to_GGy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(0.01_f64).to_Gy(),
			AbsorbedDose::from_rad(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 100.0,
			AbsorbedDose::from_Gy(1.0_f64).to_rad(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(10.0_f64).to_Gy(),
			AbsorbedDose::from_krad(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 0.1,
			AbsorbedDose::from_Gy(1.0_f64).to_krad(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1e-05_f64).to_Gy(),
			AbsorbedDose::from_mrad(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 100000.0,
			AbsorbedDose::from_Gy(1.0_f64).to_mrad(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1e-08_f64).to_Gy(),
			AbsorbedDose::from_urad(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 100000000.0,
			AbsorbedDose::from_Gy(1.0_f64).to_urad(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(0.0001_f64).to_Gy(),
			AbsorbedDose::from_erg(1.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy() * 10000.0,
			AbsorbedDose::from_Gy(1.0_f64).to_erg(), 9
		);
	}

	#[test]
	fn dose_equivalent_units() {
		assert_approx_equal(
			DoseEquivalent::from_Sv(0.001_f64).to_Sv(),
			DoseEquivalent::from_mSv(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 1000.0,
			DoseEquivalent::from_Sv(1.0_f64).to_mSv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1e-06_f64).to_Sv(),
			DoseEquivalent::from_uSv(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 1000000.0,
			DoseEquivalent::from_Sv(1.0_f64).to_uSv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1e-09_f64).to_Sv(),
			DoseEquivalent::from_nSv(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 1000000000.0,
			DoseEquivalent::from_Sv(1.0_f64).to_nSv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1000.0_f64).to_Sv(),
			DoseEquivalent::from_kSv(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 0.001,
			DoseEquivalent::from_Sv(1.0_f64).to_kSv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1000000.0_f64).to_Sv(),
			DoseEquivalent::from_MSv(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 1e-06,
			DoseEquivalent::from_Sv(1.0_f64).to_MSv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1000000000.0_f64).to_Sv(),
			DoseEquivalent::from_GSv(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 1e-09,
			DoseEquivalent::from_Sv(1.0_f64).to_GSv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(0.01_f64).to_Sv(),
			DoseEquivalent::from_rem(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 100.0,
			DoseEquivalent::from_Sv(1.0_f64).to_rem(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1e-05_f64).to_Sv(),
			DoseEquivalent::from_mrem(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 100000.0,
			DoseEquivalent::from_Sv(1.0_f64).to_mrem(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(10.0_f64).to_Sv(),
			DoseEquivalent::from_krem(1.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv() * 0.1,
			DoseEquivalent::from_Sv(1.0_f64).to_krem(), 9
		);
	}

	#[test]
	fn radioactivity_units() {
		assert_approx_equal(
			Radioactivity::from_Bq(0.001_f64).to_Bq(),
			Radioactivity::from_mBq(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 1000.0,
			Radioactivity::from_Bq(1.0_f64).to_mBq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1e-06_f64).to_Bq(),
			Radioactivity::from_uBq(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 1000000.0,
			Radioactivity::from_Bq(1.0_f64).to_uBq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1e-09_f64).to_Bq(),
			Radioactivity::from_nBq(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 1000000000.0,
			Radioactivity::from_Bq(1.0_f64).to_nBq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1000.0_f64).to_Bq(),
			Radioactivity::from_kBq(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 0.001,
			Radioactivity::from_Bq(1.0_f64).to_kBq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1000000.0_f64).to_Bq(),
			Radioactivity::from_MBq(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 1e-06,
			Radioactivity::from_Bq(1.0_f64).to_MBq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1000000000.0_f64).to_Bq(),
			Radioactivity::from_GBq(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 1e-09,
			Radioactivity::from_Bq(1.0_f64).to_GBq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(37000000000.0_f64).to_Bq(),
			Radioactivity::from_Ci(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 2.7027027027027e-11,
			Radioactivity::from_Bq(1.0_f64).to_Ci(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(37000000.0_f64).to_Bq(),
			Radioactivity::from_mCi(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 2.7027027027027e-08,
			Radioactivity::from_Bq(1.0_f64).to_mCi(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(37000.0_f64).to_Bq(),
			Radioactivity::from_uCi(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 2.7027027027027e-05,
			Radioactivity::from_Bq(1.0_f64).to_uCi(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(37.0_f64).to_Bq(),
			Radioactivity::from_nCi(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 0.027027027027027,
			Radioactivity::from_Bq(1.0_f64).to_nCi(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(0.037_f64).to_Bq(),
			Radioactivity::from_pCi(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 27.027027027027,
			Radioactivity::from_Bq(1.0_f64).to_pCi(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1000000.0_f64).to_Bq(),
			Radioactivity::from_Rd(1.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq() * 1e-06,
			Radioactivity::from_Bq(1.0_f64).to_Rd(), 9
		);
	}


}
