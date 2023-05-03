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
		mul_div_check!(InverseAmount{per_mol: x.clone()}, y.clone(), InverseAmount{per_mol: xy.clone()}, InverseAmount{per_mol: xovery.clone()});
		mul_div_check!(InverseCurrent{per_A: x.clone()}, y.clone(), InverseCurrent{per_A: xy.clone()}, InverseCurrent{per_A: xovery.clone()});
		mul_div_check!(InverseDistance{per_m: x.clone()}, y.clone(), InverseDistance{per_m: xy.clone()}, InverseDistance{per_m: xovery.clone()});
		mul_div_check!(InverseLuminosity{per_cd: x.clone()}, y.clone(), InverseLuminosity{per_cd: xy.clone()}, InverseLuminosity{per_cd: xovery.clone()});
		mul_div_check!(InverseMass{per_kg: x.clone()}, y.clone(), InverseMass{per_kg: xy.clone()}, InverseMass{per_kg: xovery.clone()});
		mul_div_check!(InverseTemperature{per_K: x.clone()}, y.clone(), InverseTemperature{per_K: xy.clone()}, InverseTemperature{per_K: xovery.clone()});
		mul_div_check!(InverseCatalyticActivity{s_per_mol: x.clone()}, y.clone(), InverseCatalyticActivity{s_per_mol: xy.clone()}, InverseCatalyticActivity{s_per_mol: xovery.clone()});
		mul_div_check!(InverseSpecificHeatCapacity{kgK_per_J: x.clone()}, y.clone(), InverseSpecificHeatCapacity{kgK_per_J: xy.clone()}, InverseSpecificHeatCapacity{kgK_per_J: xovery.clone()});
		mul_div_check!(MolarVolume{m3_per_mol: x.clone()}, y.clone(), MolarVolume{m3_per_mol: xy.clone()}, MolarVolume{m3_per_mol: xovery.clone()});
		mul_div_check!(AreaPerLumen{m2_per_lm: x.clone()}, y.clone(), AreaPerLumen{m2_per_lm: xy.clone()}, AreaPerLumen{m2_per_lm: xovery.clone()});
		mul_div_check!(Elastance{per_F: x.clone()}, y.clone(), Elastance{per_F: xy.clone()}, Elastance{per_F: xovery.clone()});
		mul_div_check!(InverseCharge{per_C: x.clone()}, y.clone(), InverseCharge{per_C: xy.clone()}, InverseCharge{per_C: xovery.clone()});
		mul_div_check!(InverseInductance{per_H: x.clone()}, y.clone(), InverseInductance{per_H: xy.clone()}, InverseInductance{per_H: xovery.clone()});
		mul_div_check!(InverseLuminousFlux{per_lm: x.clone()}, y.clone(), InverseLuminousFlux{per_lm: xy.clone()}, InverseLuminousFlux{per_lm: xovery.clone()});
		mul_div_check!(InverseMagneticFlux{per_Wb: x.clone()}, y.clone(), InverseMagneticFlux{per_Wb: xy.clone()}, InverseMagneticFlux{per_Wb: xovery.clone()});
		mul_div_check!(InverseMagneticFluxDensity{m2_per_Wb: x.clone()}, y.clone(), InverseMagneticFluxDensity{m2_per_Wb: xy.clone()}, InverseMagneticFluxDensity{m2_per_Wb: xovery.clone()});
		mul_div_check!(InverseVoltage{per_V: x.clone()}, y.clone(), InverseVoltage{per_V: xy.clone()}, InverseVoltage{per_V: xovery.clone()});
		mul_div_check!(InverseAngle{per_rad: x.clone()}, y.clone(), InverseAngle{per_rad: xy.clone()}, InverseAngle{per_rad: xovery.clone()});
		mul_div_check!(InverseArea{per_m2: x.clone()}, y.clone(), InverseArea{per_m2: xy.clone()}, InverseArea{per_m2: xovery.clone()});
		mul_div_check!(InverseSolidAngle{per_sr: x.clone()}, y.clone(), InverseSolidAngle{per_sr: xy.clone()}, InverseSolidAngle{per_sr: xovery.clone()});
		mul_div_check!(InverseVolume{per_m3: x.clone()}, y.clone(), InverseVolume{per_m3: xy.clone()}, InverseVolume{per_m3: xovery.clone()});
		mul_div_check!(AreaPerMass{m2_per_kg: x.clone()}, y.clone(), AreaPerMass{m2_per_kg: xy.clone()}, AreaPerMass{m2_per_kg: xovery.clone()});
		mul_div_check!(InverseAcceleration{s2pm: x.clone()}, y.clone(), InverseAcceleration{s2pm: xy.clone()}, InverseAcceleration{s2pm: xovery.clone()});
		mul_div_check!(InverseAngularAcceleration{s2prad: x.clone()}, y.clone(), InverseAngularAcceleration{s2prad: xy.clone()}, InverseAngularAcceleration{s2prad: xovery.clone()});
		mul_div_check!(InverseAngularMomentum{s_per_kgm2rad: x.clone()}, y.clone(), InverseAngularMomentum{s_per_kgm2rad: xy.clone()}, InverseAngularMomentum{s_per_kgm2rad: xovery.clone()});
		mul_div_check!(InverseAngularVelocity{s_per_rad: x.clone()}, y.clone(), InverseAngularVelocity{s_per_rad: xy.clone()}, InverseAngularVelocity{s_per_rad: xovery.clone()});
		mul_div_check!(InverseEnergy{per_J: x.clone()}, y.clone(), InverseEnergy{per_J: xy.clone()}, InverseEnergy{per_J: xovery.clone()});
		mul_div_check!(InverseForce{per_N: x.clone()}, y.clone(), InverseForce{per_N: xy.clone()}, InverseForce{per_N: xovery.clone()});
		mul_div_check!(InverseMomentOfInertia{per_kgm2: x.clone()}, y.clone(), InverseMomentOfInertia{per_kgm2: xy.clone()}, InverseMomentOfInertia{per_kgm2: xovery.clone()});
		mul_div_check!(InverseMomentum{s_per_kgm: x.clone()}, y.clone(), InverseMomentum{s_per_kgm: xy.clone()}, InverseMomentum{s_per_kgm: xovery.clone()});
		mul_div_check!(InversePower{per_W: x.clone()}, y.clone(), InversePower{per_W: xy.clone()}, InversePower{per_W: xovery.clone()});
		mul_div_check!(InversePressure{per_Pa: x.clone()}, y.clone(), InversePressure{per_Pa: xy.clone()}, InversePressure{per_Pa: xovery.clone()});
		mul_div_check!(InverseTorque{per_Nm: x.clone()}, y.clone(), InverseTorque{per_Nm: xy.clone()}, InverseTorque{per_Nm: xovery.clone()});
		mul_div_check!(TimePerDistance{spm: x.clone()}, y.clone(), TimePerDistance{spm: xy.clone()}, TimePerDistance{spm: xovery.clone()});
		mul_div_check!(VolumePerMass{m3_per_kg: x.clone()}, y.clone(), VolumePerMass{m3_per_kg: xy.clone()}, VolumePerMass{m3_per_kg: xovery.clone()});
		mul_div_check!(InverseAbsorbedDose{per_Gy: x.clone()}, y.clone(), InverseAbsorbedDose{per_Gy: xy.clone()}, InverseAbsorbedDose{per_Gy: xovery.clone()});
		mul_div_check!(InverseDoseEquivalent{per_Sv: x.clone()}, y.clone(), InverseDoseEquivalent{per_Sv: xy.clone()}, InverseDoseEquivalent{per_Sv: xovery.clone()});
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
		assert!(InverseAmount::<f64>::unit_name().eq("inverse moles"));
		assert!(InverseAmount::<f64>::unit_symbol().eq("1/mol"));
		assert!(InverseCurrent::<f64>::unit_name().eq("inverse amperes"));
		assert!(InverseCurrent::<f64>::unit_symbol().eq("1/A"));
		assert!(InverseDistance::<f64>::unit_name().eq("inverse meters"));
		assert!(InverseDistance::<f64>::unit_symbol().eq("1/m"));
		assert!(InverseLuminosity::<f64>::unit_name().eq("inverse candela"));
		assert!(InverseLuminosity::<f64>::unit_symbol().eq("1/cd"));
		assert!(InverseMass::<f64>::unit_name().eq("inverse kilograms"));
		assert!(InverseMass::<f64>::unit_symbol().eq("1/kg"));
		assert!(InverseTemperature::<f64>::unit_name().eq("inverse degrees kelvin"));
		assert!(InverseTemperature::<f64>::unit_symbol().eq("1/K"));
		assert!(InverseCatalyticActivity::<f64>::unit_name().eq("seconds per mole"));
		assert!(InverseCatalyticActivity::<f64>::unit_symbol().eq("s/mol"));
		assert!(InverseSpecificHeatCapacity::<f64>::unit_name().eq("kilogram per kelvin per joules"));
		assert!(InverseSpecificHeatCapacity::<f64>::unit_symbol().eq("kg·K/J"));
		assert!(MolarVolume::<f64>::unit_name().eq("cubic meters per mole"));
		assert!(MolarVolume::<f64>::unit_symbol().eq("m³/mol"));
		assert!(AreaPerLumen::<f64>::unit_name().eq("square meters per lumen"));
		assert!(AreaPerLumen::<f64>::unit_symbol().eq("m²/lm"));
		assert!(Elastance::<f64>::unit_name().eq("inverse farads"));
		assert!(Elastance::<f64>::unit_symbol().eq("1/F"));
		assert!(InverseCharge::<f64>::unit_name().eq("inverse coulombs"));
		assert!(InverseCharge::<f64>::unit_symbol().eq("1/C"));
		assert!(InverseInductance::<f64>::unit_name().eq("inverse henries"));
		assert!(InverseInductance::<f64>::unit_symbol().eq("1/H"));
		assert!(InverseLuminousFlux::<f64>::unit_name().eq("inverse lumens"));
		assert!(InverseLuminousFlux::<f64>::unit_symbol().eq("1/lm"));
		assert!(InverseMagneticFlux::<f64>::unit_name().eq("inverse webers"));
		assert!(InverseMagneticFlux::<f64>::unit_symbol().eq("1/Wb"));
		assert!(InverseMagneticFluxDensity::<f64>::unit_name().eq("square meters per weber"));
		assert!(InverseMagneticFluxDensity::<f64>::unit_symbol().eq("m²/Wb"));
		assert!(InverseVoltage::<f64>::unit_name().eq("inverse volts"));
		assert!(InverseVoltage::<f64>::unit_symbol().eq("1/V"));
		assert!(InverseAngle::<f64>::unit_name().eq("inverse radians"));
		assert!(InverseAngle::<f64>::unit_symbol().eq("1/rad"));
		assert!(InverseArea::<f64>::unit_name().eq("inverse square meters"));
		assert!(InverseArea::<f64>::unit_symbol().eq("1/m²"));
		assert!(InverseSolidAngle::<f64>::unit_name().eq("inverse steradian"));
		assert!(InverseSolidAngle::<f64>::unit_symbol().eq("1/sr"));
		assert!(InverseVolume::<f64>::unit_name().eq("inverse cubic meters"));
		assert!(InverseVolume::<f64>::unit_symbol().eq("1/m³"));
		assert!(AreaPerMass::<f64>::unit_name().eq("square meters per kilogram"));
		assert!(AreaPerMass::<f64>::unit_symbol().eq("m²/kg"));
		assert!(InverseAcceleration::<f64>::unit_name().eq("seconds squared per meter"));
		assert!(InverseAcceleration::<f64>::unit_symbol().eq("s²/m"));
		assert!(InverseAngularAcceleration::<f64>::unit_name().eq("seconds squared per radian"));
		assert!(InverseAngularAcceleration::<f64>::unit_symbol().eq("s²/rad"));
		assert!(InverseAngularMomentum::<f64>::unit_name().eq("seconds per kilogram meters squared radian"));
		assert!(InverseAngularMomentum::<f64>::unit_symbol().eq("s/kg·m²·rad"));
		assert!(InverseAngularVelocity::<f64>::unit_name().eq("seconds per radian"));
		assert!(InverseAngularVelocity::<f64>::unit_symbol().eq("s/rad"));
		assert!(InverseEnergy::<f64>::unit_name().eq("inverse joules"));
		assert!(InverseEnergy::<f64>::unit_symbol().eq("1/J"));
		assert!(InverseForce::<f64>::unit_name().eq("inverse newtons"));
		assert!(InverseForce::<f64>::unit_symbol().eq("1/N"));
		assert!(InverseMomentOfInertia::<f64>::unit_name().eq("inverse kilogram meters squared"));
		assert!(InverseMomentOfInertia::<f64>::unit_symbol().eq("1/kg·m²"));
		assert!(InverseMomentum::<f64>::unit_name().eq("seconds per kilogram meter"));
		assert!(InverseMomentum::<f64>::unit_symbol().eq("s/kg·m"));
		assert!(InversePower::<f64>::unit_name().eq("inverse watts"));
		assert!(InversePower::<f64>::unit_symbol().eq("1/W"));
		assert!(InversePressure::<f64>::unit_name().eq("inverse pascals"));
		assert!(InversePressure::<f64>::unit_symbol().eq("1/Pa"));
		assert!(InverseTorque::<f64>::unit_name().eq("inverse newton meters"));
		assert!(InverseTorque::<f64>::unit_symbol().eq("1/Nm"));
		assert!(TimePerDistance::<f64>::unit_name().eq("seconds per meter"));
		assert!(TimePerDistance::<f64>::unit_symbol().eq("s/m"));
		assert!(VolumePerMass::<f64>::unit_name().eq("cubic meters per kilogram"));
		assert!(VolumePerMass::<f64>::unit_symbol().eq("m³/kg"));
		assert!(InverseAbsorbedDose::<f64>::unit_name().eq("inverse grays"));
		assert!(InverseAbsorbedDose::<f64>::unit_symbol().eq("1/Gy"));
		assert!(InverseDoseEquivalent::<f64>::unit_name().eq("inverse sieverts"));
		assert!(InverseDoseEquivalent::<f64>::unit_symbol().eq("1/Sv"));
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
		println!("{}", InverseAmount{per_mol: 1});
		println!("{}", InverseCurrent{per_A: 1});
		println!("{}", InverseDistance{per_m: 1});
		println!("{}", InverseLuminosity{per_cd: 1});
		println!("{}", InverseMass{per_kg: 1});
		println!("{}", InverseTemperature{per_K: 1});
		println!("{}", InverseCatalyticActivity{s_per_mol: 1});
		println!("{}", InverseSpecificHeatCapacity{kgK_per_J: 1});
		println!("{}", MolarVolume{m3_per_mol: 1});
		println!("{}", AreaPerLumen{m2_per_lm: 1});
		println!("{}", Elastance{per_F: 1});
		println!("{}", InverseCharge{per_C: 1});
		println!("{}", InverseInductance{per_H: 1});
		println!("{}", InverseLuminousFlux{per_lm: 1});
		println!("{}", InverseMagneticFlux{per_Wb: 1});
		println!("{}", InverseMagneticFluxDensity{m2_per_Wb: 1});
		println!("{}", InverseVoltage{per_V: 1});
		println!("{}", InverseAngle{per_rad: 1});
		println!("{}", InverseArea{per_m2: 1});
		println!("{}", InverseSolidAngle{per_sr: 1});
		println!("{}", InverseVolume{per_m3: 1});
		println!("{}", AreaPerMass{m2_per_kg: 1});
		println!("{}", InverseAcceleration{s2pm: 1});
		println!("{}", InverseAngularAcceleration{s2prad: 1});
		println!("{}", InverseAngularMomentum{s_per_kgm2rad: 1});
		println!("{}", InverseAngularVelocity{s_per_rad: 1});
		println!("{}", InverseEnergy{per_J: 1});
		println!("{}", InverseForce{per_N: 1});
		println!("{}", InverseMomentOfInertia{per_kgm2: 1});
		println!("{}", InverseMomentum{s_per_kgm: 1});
		println!("{}", InversePower{per_W: 1});
		println!("{}", InversePressure{per_Pa: 1});
		println!("{}", InverseTorque{per_Nm: 1});
		println!("{}", TimePerDistance{spm: 1});
		println!("{}", VolumePerMass{m3_per_kg: 1});
		println!("{}", InverseAbsorbedDose{per_Gy: 1});
		println!("{}", InverseDoseEquivalent{per_Sv: 1});
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
		assert_eq!(mul_check(&Amount{mol: x}, &InverseMass{per_kg: y}), Molality{molpkg: x*y});
		assert_eq!(mul_check(&Amount{mol: x}, &InverseCatalyticActivity{s_per_mol: y}), Time{s: x*y});
		assert_eq!(mul_check(&Amount{mol: x}, &MolarVolume{m3_per_mol: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&Amount{mol: x}, &InverseVolume{per_m3: y}), Concentration{molpm3: x*y});
		assert_eq!(mul_check(&Current{A: x}, &InverseCharge{per_C: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&Current{A: x}, &InverseInductance{per_H: y}), MagneticFlux{Wb: x/y});
		assert_eq!(mul_check(&Current{A: x}, &InverseMagneticFlux{per_Wb: y}), InverseInductance{per_H: x*y});
		assert_eq!(div_check(&Current{A: x}, &InverseMagneticFlux{per_Wb: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Current{A: x}, &InverseVoltage{per_V: y}), Conductance{S: x*y});
		assert_eq!(div_check(&Current{A: x}, &InverseVoltage{per_V: y}), Power{W: x/y});
		assert_eq!(div_check(&Current{A: x}, &MagneticFlux{Wb: y}), InverseInductance{per_H: x/y});
		assert_eq!(div_check(&Current{A: x}, &Energy{J: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&Current{A: x}, &Torque{Nm: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(mul_check(&Current{A: x}, &InverseEnergy{per_J: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&Current{A: x}, &InverseTorque{per_Nm: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&Current{A: x}, &InversePower{per_W: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&Current{A: x}, &Power{W: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&Distance{m: x}, &InverseDistance{per_m: y}), Area{m2: x/y});
		assert_eq!(div_check(&Distance{m: x}, &Area{m2: y}), InverseDistance{per_m: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &InverseArea{per_m2: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&Distance{m: x}, &InverseArea{per_m2: y}), Volume{m3: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &InverseVolume{per_m3: y}), InverseArea{per_m2: x*y});
		assert_eq!(div_check(&Distance{m: x}, &Volume{m3: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&Distance{m: x}, &AreaDensity{kgpm2: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &AreaPerMass{m2_per_kg: y}), VolumePerMass{m3_per_kg: x*y});
		assert_eq!(div_check(&Distance{m: x}, &Energy{J: y}), InverseForce{per_N: x/y});
		assert_eq!(div_check(&Distance{m: x}, &Torque{Nm: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &InverseEnergy{per_J: y}), InverseForce{per_N: x*y});
		assert_eq!(mul_check(&Distance{m: x}, &InverseTorque{per_Nm: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&Distance{m: x}, &InverseForce{per_N: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &TimePerDistance{spm: y}), Time{s: x*y});
		assert_eq!(div_check(&Distance{m: x}, &VolumePerMass{m3_per_kg: y}), AreaDensity{kgpm2: x/y});
		assert_eq!(mul_check(&Distance{m: x}, &InverseAbsorbedDose{per_Gy: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(mul_check(&Distance{m: x}, &InverseDoseEquivalent{per_Sv: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&InverseAmount{per_mol: x}, &InverseMass{per_kg: y}), MolarMass{kgpmol: x/y});
		assert_eq!(mul_check(&InverseAmount{per_mol: x}, &Mass{kg: y}), MolarMass{kgpmol: x*y});
		assert_eq!(mul_check(&InverseAmount{per_mol: x}, &Time{s: y}), InverseCatalyticActivity{s_per_mol: x*y});
		assert_eq!(mul_check(&InverseAmount{per_mol: x}, &CatalyticActivity{molps: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&InverseAmount{per_mol: x}, &Concentration{molpm3: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&InverseAmount{per_mol: x}, &InverseCatalyticActivity{s_per_mol: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&InverseAmount{per_mol: x}, &Molality{molpkg: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&InverseAmount{per_mol: x}, &MolarMass{kgpmol: y}), InverseMass{per_kg: x/y});
		assert_eq!(div_check(&InverseAmount{per_mol: x}, &MolarVolume{m3_per_mol: y}), InverseVolume{per_m3: x/y});
		assert_eq!(div_check(&InverseAmount{per_mol: x}, &InverseVolume{per_m3: y}), MolarVolume{m3_per_mol: x/y});
		assert_eq!(mul_check(&InverseAmount{per_mol: x}, &Volume{m3: y}), MolarVolume{m3_per_mol: x*y});
		assert_eq!(div_check(&InverseAmount{per_mol: x}, &Frequency{Hz: y}), InverseCatalyticActivity{s_per_mol: x/y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &Time{s: y}), InverseCharge{per_C: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Charge{C: y}), Time{s: x*y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Conductance{S: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &Inductance{H: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &InverseCharge{per_C: y}), Time{s: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &InverseInductance{per_H: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &InverseMagneticFlux{per_Wb: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &InverseMagneticFlux{per_Wb: y}), Inductance{H: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &InverseVoltage{per_V: y}), InversePower{per_W: x*y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &InverseVoltage{per_V: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &MagneticFlux{Wb: y}), Inductance{H: x*y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &MagneticFlux{Wb: y}), InverseEnergy{per_J: x/y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &Resistance{Ohm: y}), InverseVoltage{per_V: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Voltage{V: y}), Resistance{Ohm: x*y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &Voltage{V: y}), InversePower{per_W: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Energy{J: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Torque{Nm: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Frequency{Hz: y}), InverseCharge{per_C: x*y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &InverseEnergy{per_J: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &InverseTorque{per_Nm: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&InverseCurrent{per_A: x}, &InversePower{per_W: y}), Voltage{V: x/y});
		assert_eq!(mul_check(&InverseCurrent{per_A: x}, &Power{W: y}), Voltage{V: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &Distance{m: y}), InverseArea{per_m2: x/y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &InverseDistance{per_m: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &Time{s: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &Area{m2: y}), Distance{m: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &Area{m2: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &InverseArea{per_m2: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &InverseArea{per_m2: y}), Distance{m: x/y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &InverseVolume{per_m3: y}), Area{m2: x/y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &Volume{m3: y}), Area{m2: x*y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &AreaDensity{kgpm2: y}), Density{kgpm3: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &AreaPerMass{m2_per_kg: y}), Density{kgpm3: x/y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &Density{kgpm3: y}), AreaPerMass{m2_per_kg: x/y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &Energy{J: y}), Force{N: x*y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &Torque{Nm: y}), Force{N: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &Force{N: y}), InverseEnergy{per_J: x/y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &Frequency{Hz: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &InverseEnergy{per_J: y}), Force{N: x/y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &InverseTorque{per_Nm: y}), Force{N: x/y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &InverseForce{per_N: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &TimePerDistance{spm: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &Velocity{mps: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&InverseDistance{per_m: x}, &VolumePerMass{m3_per_kg: y}), AreaPerMass{m2_per_kg: x*y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &InverseAbsorbedDose{per_Gy: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&InverseDistance{per_m: x}, &InverseDoseEquivalent{per_Sv: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&InverseLuminosity{per_cd: x}, &InverseLuminousFlux{per_lm: y}), SolidAngle{sr: x/y});
		assert_eq!(mul_check(&InverseLuminosity{per_cd: x}, &LuminousFlux{lm: y}), SolidAngle{sr: x*y});
		assert_eq!(mul_check(&InverseLuminosity{per_cd: x}, &InverseSolidAngle{per_sr: y}), InverseLuminousFlux{per_lm: x*y});
		assert_eq!(div_check(&InverseLuminosity{per_cd: x}, &SolidAngle{sr: y}), InverseLuminousFlux{per_lm: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &Amount{mol: y}), Molality{molpkg: x*y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &InverseAmount{per_mol: y}), Molality{molpkg: x/y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &Molality{molpkg: y}), InverseAmount{per_mol: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &MolarMass{kgpmol: y}), InverseAmount{per_mol: x*y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &Area{m2: y}), AreaPerMass{m2_per_kg: x*y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &InverseArea{per_m2: y}), AreaPerMass{m2_per_kg: x/y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &InverseVolume{per_m3: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &Volume{m3: y}), VolumePerMass{m3_per_kg: x*y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &Acceleration{mps2: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &AreaDensity{kgpm2: y}), InverseArea{per_m2: x*y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &AreaPerMass{m2_per_kg: y}), InverseArea{per_m2: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &Density{kgpm3: y}), InverseVolume{per_m3: x*y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &Force{N: y}), Acceleration{mps2: x*y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &InverseAcceleration{s2pm: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &InverseForce{per_N: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &InverseMomentOfInertia{per_kgm2: y}), Area{m2: x/y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &InverseMomentum{s_per_kgm: y}), Velocity{mps: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &MomentOfInertia{kgm2: y}), Area{m2: x*y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &Momentum{kgmps: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &TimePerDistance{spm: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &Velocity{mps: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(div_check(&InverseMass{per_kg: x}, &VolumePerMass{m3_per_kg: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &InverseAbsorbedDose{per_Gy: y}), InverseEnergy{per_J: x*y});
		assert_eq!(mul_check(&InverseMass{per_kg: x}, &InverseDoseEquivalent{per_Sv: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseTemperature{per_K: x}, &InverseAbsorbedDose{per_Gy: y}), SpecificHeatCapacity{J_per_kgK: x/y});
		assert_eq!(div_check(&InverseTemperature{per_K: x}, &InverseDoseEquivalent{per_Sv: y}), SpecificHeatCapacity{J_per_kgK: x/y});
		assert_eq!(mul_check(&Luminosity{cd: x}, &InverseLuminousFlux{per_lm: y}), InverseSolidAngle{per_sr: x*y});
		assert_eq!(div_check(&Luminosity{cd: x}, &LuminousFlux{lm: y}), InverseSolidAngle{per_sr: x/y});
		assert_eq!(div_check(&Luminosity{cd: x}, &InverseSolidAngle{per_sr: y}), LuminousFlux{lm: x/y});
		assert_eq!(mul_check(&Mass{kg: x}, &InverseAmount{per_mol: y}), MolarMass{kgpmol: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &InverseArea{per_m2: y}), AreaDensity{kgpm2: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &InverseVolume{per_m3: y}), Density{kgpm3: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &AreaPerMass{m2_per_kg: y}), Area{m2: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &Force{N: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(div_check(&Mass{kg: x}, &InverseAcceleration{s2pm: y}), Force{N: x/y});
		assert_eq!(mul_check(&Mass{kg: x}, &InverseForce{per_N: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &InverseMomentOfInertia{per_kgm2: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&Mass{kg: x}, &InverseMomentum{s_per_kgm: y}), TimePerDistance{spm: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &MomentOfInertia{kgm2: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&Mass{kg: x}, &Momentum{kgmps: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&Mass{kg: x}, &TimePerDistance{spm: y}), Momentum{kgmps: x/y});
		assert_eq!(mul_check(&Mass{kg: x}, &VolumePerMass{m3_per_kg: y}), Volume{m3: x*y});
		assert_eq!(div_check(&Mass{kg: x}, &InverseAbsorbedDose{per_Gy: y}), Energy{J: x/y});
		assert_eq!(div_check(&Mass{kg: x}, &InverseDoseEquivalent{per_Sv: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Temperature{K: x}, &InverseAbsorbedDose{per_Gy: y}), InverseSpecificHeatCapacity{kgK_per_J: x*y});
		assert_eq!(mul_check(&Temperature{K: x}, &InverseDoseEquivalent{per_Sv: y}), InverseSpecificHeatCapacity{kgK_per_J: x*y});
		assert_eq!(div_check(&Time{s: x}, &Amount{mol: y}), InverseCatalyticActivity{s_per_mol: x/y});
		assert_eq!(div_check(&Time{s: x}, &Distance{m: y}), TimePerDistance{spm: x/y});
		assert_eq!(mul_check(&Time{s: x}, &InverseAmount{per_mol: y}), InverseCatalyticActivity{s_per_mol: x*y});
		assert_eq!(div_check(&Time{s: x}, &InverseCurrent{per_A: y}), Charge{C: x/y});
		assert_eq!(mul_check(&Time{s: x}, &InverseDistance{per_m: y}), TimePerDistance{spm: x*y});
		assert_eq!(div_check(&Time{s: x}, &InverseCatalyticActivity{s_per_mol: y}), Amount{mol: x/y});
		assert_eq!(div_check(&Time{s: x}, &Charge{C: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&Time{s: x}, &Elastance{per_F: y}), Resistance{Ohm: x*y});
		assert_eq!(mul_check(&Time{s: x}, &InverseCharge{per_C: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&Time{s: x}, &InverseInductance{per_H: y}), Conductance{S: x*y});
		assert_eq!(mul_check(&Time{s: x}, &InverseMagneticFlux{per_Wb: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&Time{s: x}, &InverseVoltage{per_V: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Time{s: x}, &MagneticFlux{Wb: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&Time{s: x}, &Angle{rad: y}), InverseAngularVelocity{s_per_rad: x/y});
		assert_eq!(mul_check(&Time{s: x}, &InverseAngle{per_rad: y}), InverseAngularVelocity{s_per_rad: x*y});
		assert_eq!(div_check(&Time{s: x}, &AngularVelocity{radps: y}), InverseAngularAcceleration{s2prad: x/y});
		assert_eq!(div_check(&Time{s: x}, &Energy{J: y}), InversePower{per_W: x/y});
		assert_eq!(div_check(&Time{s: x}, &Torque{Nm: y}), InversePower{per_W: x/y});
		assert_eq!(div_check(&Time{s: x}, &InverseAcceleration{s2pm: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&Time{s: x}, &InverseAngularAcceleration{s2prad: y}), AngularVelocity{radps: x/y});
		assert_eq!(mul_check(&Time{s: x}, &InverseAngularVelocity{s_per_rad: y}), InverseAngularAcceleration{s2prad: x*y});
		assert_eq!(div_check(&Time{s: x}, &InverseAngularVelocity{s_per_rad: y}), Angle{rad: x/y});
		assert_eq!(mul_check(&Time{s: x}, &InverseEnergy{per_J: y}), InversePower{per_W: x*y});
		assert_eq!(mul_check(&Time{s: x}, &InverseTorque{per_Nm: y}), InversePower{per_W: x*y});
		assert_eq!(div_check(&Time{s: x}, &InverseForce{per_N: y}), Momentum{kgmps: x/y});
		assert_eq!(mul_check(&Time{s: x}, &InverseMomentum{s_per_kgm: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&Time{s: x}, &InversePower{per_W: y}), Energy{J: x/y});
		assert_eq!(div_check(&Time{s: x}, &Momentum{kgmps: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&Time{s: x}, &TimePerDistance{spm: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&Time{s: x}, &TimePerDistance{spm: y}), Distance{m: x/y});
		assert_eq!(div_check(&Time{s: x}, &Velocity{mps: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&CatalyticActivity{molps: x}, &InverseAmount{per_mol: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&Concentration{molpm3: x}, &Amount{mol: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&Concentration{molpm3: x}, &InverseAmount{per_mol: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&Concentration{molpm3: x}, &InverseVolume{per_m3: y}), Amount{mol: x/y});
		assert_eq!(mul_check(&Concentration{molpm3: x}, &VolumePerMass{m3_per_kg: y}), Molality{molpkg: x*y});
		assert_eq!(mul_check(&InverseCatalyticActivity{s_per_mol: x}, &Amount{mol: y}), Time{s: x*y});
		assert_eq!(div_check(&InverseCatalyticActivity{s_per_mol: x}, &InverseAmount{per_mol: y}), Time{s: x/y});
		assert_eq!(div_check(&InverseCatalyticActivity{s_per_mol: x}, &Time{s: y}), InverseAmount{per_mol: x/y});
		assert_eq!(mul_check(&InverseCatalyticActivity{s_per_mol: x}, &Frequency{Hz: y}), InverseAmount{per_mol: x*y});
		assert_eq!(div_check(&InverseSpecificHeatCapacity{kgK_per_J: x}, &InverseAbsorbedDose{per_Gy: y}), Temperature{K: x/y});
		assert_eq!(div_check(&InverseSpecificHeatCapacity{kgK_per_J: x}, &InverseDoseEquivalent{per_Sv: y}), Temperature{K: x/y});
		assert_eq!(div_check(&Molality{molpkg: x}, &Amount{mol: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&Molality{molpkg: x}, &InverseAmount{per_mol: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&Molality{molpkg: x}, &InverseMass{per_kg: y}), Amount{mol: x/y});
		assert_eq!(div_check(&Molality{molpkg: x}, &Concentration{molpm3: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(mul_check(&Molality{molpkg: x}, &MolarVolume{m3_per_mol: y}), VolumePerMass{m3_per_kg: x*y});
		assert_eq!(div_check(&Molality{molpkg: x}, &VolumePerMass{m3_per_kg: y}), Concentration{molpm3: x/y});
		assert_eq!(div_check(&MolarMass{kgpmol: x}, &InverseAmount{per_mol: y}), Mass{kg: x/y});
		assert_eq!(mul_check(&MolarMass{kgpmol: x}, &InverseMass{per_kg: y}), InverseAmount{per_mol: x*y});
		assert_eq!(div_check(&MolarMass{kgpmol: x}, &Mass{kg: y}), InverseAmount{per_mol: x/y});
		assert_eq!(div_check(&MolarMass{kgpmol: x}, &MolarVolume{m3_per_mol: y}), Density{kgpm3: x/y});
		assert_eq!(div_check(&MolarMass{kgpmol: x}, &Density{kgpm3: y}), MolarVolume{m3_per_mol: x/y});
		assert_eq!(mul_check(&MolarMass{kgpmol: x}, &VolumePerMass{m3_per_kg: y}), MolarVolume{m3_per_mol: x*y});
		assert_eq!(mul_check(&MolarVolume{m3_per_mol: x}, &Amount{mol: y}), Volume{m3: x*y});
		assert_eq!(div_check(&MolarVolume{m3_per_mol: x}, &InverseAmount{per_mol: y}), Volume{m3: x/y});
		assert_eq!(mul_check(&MolarVolume{m3_per_mol: x}, &Molality{molpkg: y}), VolumePerMass{m3_per_kg: x*y});
		assert_eq!(div_check(&MolarVolume{m3_per_mol: x}, &MolarMass{kgpmol: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(mul_check(&MolarVolume{m3_per_mol: x}, &InverseVolume{per_m3: y}), InverseAmount{per_mol: x*y});
		assert_eq!(div_check(&MolarVolume{m3_per_mol: x}, &Volume{m3: y}), InverseAmount{per_mol: x/y});
		assert_eq!(mul_check(&MolarVolume{m3_per_mol: x}, &Density{kgpm3: y}), MolarMass{kgpmol: x*y});
		assert_eq!(div_check(&MolarVolume{m3_per_mol: x}, &VolumePerMass{m3_per_kg: y}), MolarMass{kgpmol: x/y});
		assert_eq!(mul_check(&SpecificHeatCapacity{J_per_kgK: x}, &InverseAbsorbedDose{per_Gy: y}), InverseTemperature{per_K: x*y});
		assert_eq!(mul_check(&SpecificHeatCapacity{J_per_kgK: x}, &InverseDoseEquivalent{per_Sv: y}), InverseTemperature{per_K: x*y});
		assert_eq!(div_check(&AreaPerLumen{m2_per_lm: x}, &InverseLuminousFlux{per_lm: y}), Area{m2: x/y});
		assert_eq!(mul_check(&AreaPerLumen{m2_per_lm: x}, &LuminousFlux{lm: y}), Area{m2: x*y});
		assert_eq!(div_check(&AreaPerLumen{m2_per_lm: x}, &Area{m2: y}), InverseLuminousFlux{per_lm: x/y});
		assert_eq!(mul_check(&AreaPerLumen{m2_per_lm: x}, &InverseArea{per_m2: y}), InverseLuminousFlux{per_lm: x*y});
		assert_eq!(div_check(&Capacitance{F: x}, &Charge{C: y}), InverseVoltage{per_V: x/y});
		assert_eq!(mul_check(&Capacitance{F: x}, &InverseCharge{per_C: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&Capacitance{F: x}, &InverseVoltage{per_V: y}), Charge{C: x/y});
		assert_eq!(mul_check(&Charge{C: x}, &InverseCurrent{per_A: y}), Time{s: x*y});
		assert_eq!(mul_check(&Charge{C: x}, &Elastance{per_F: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Charge{C: x}, &InverseMagneticFlux{per_Wb: y}), Conductance{S: x*y});
		assert_eq!(mul_check(&Charge{C: x}, &InverseVoltage{per_V: y}), Capacitance{F: x*y});
		assert_eq!(div_check(&Charge{C: x}, &InverseVoltage{per_V: y}), Energy{J: x/y});
		assert_eq!(div_check(&Charge{C: x}, &Energy{J: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&Charge{C: x}, &Torque{Nm: y}), InverseVoltage{per_V: x/y});
		assert_eq!(mul_check(&Charge{C: x}, &InverseEnergy{per_J: y}), InverseVoltage{per_V: x*y});
		assert_eq!(mul_check(&Charge{C: x}, &InverseTorque{per_Nm: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&Conductance{S: x}, &Current{A: y}), InverseVoltage{per_V: x/y});
		assert_eq!(mul_check(&Conductance{S: x}, &InverseCurrent{per_A: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&Conductance{S: x}, &Time{s: y}), InverseInductance{per_H: x/y});
		assert_eq!(div_check(&Conductance{S: x}, &Charge{C: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(mul_check(&Conductance{S: x}, &Elastance{per_F: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&Conductance{S: x}, &InverseCharge{per_C: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(div_check(&Conductance{S: x}, &InverseInductance{per_H: y}), Time{s: x/y});
		assert_eq!(div_check(&Conductance{S: x}, &InverseMagneticFlux{per_Wb: y}), Charge{C: x/y});
		assert_eq!(div_check(&Conductance{S: x}, &InverseVoltage{per_V: y}), Current{A: x/y});
		assert_eq!(mul_check(&Conductance{S: x}, &Frequency{Hz: y}), InverseInductance{per_H: x*y});
		assert_eq!(mul_check(&Elastance{per_F: x}, &Time{s: y}), Resistance{Ohm: x*y});
		assert_eq!(mul_check(&Elastance{per_F: x}, &Charge{C: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Elastance{per_F: x}, &Conductance{S: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&Elastance{per_F: x}, &InverseCharge{per_C: y}), Voltage{V: x/y});
		assert_eq!(mul_check(&Elastance{per_F: x}, &InverseVoltage{per_V: y}), InverseCharge{per_C: x*y});
		assert_eq!(div_check(&Elastance{per_F: x}, &Resistance{Ohm: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&Elastance{per_F: x}, &Voltage{V: y}), InverseCharge{per_C: x/y});
		assert_eq!(div_check(&Elastance{per_F: x}, &Frequency{Hz: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&Illuminance{lux: x}, &InverseLuminousFlux{per_lm: y}), InverseArea{per_m2: x*y});
		assert_eq!(div_check(&Illuminance{lux: x}, &LuminousFlux{lm: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&Illuminance{lux: x}, &InverseArea{per_m2: y}), LuminousFlux{lm: x/y});
		assert_eq!(div_check(&Inductance{H: x}, &InverseCurrent{per_A: y}), MagneticFlux{Wb: x/y});
		assert_eq!(mul_check(&Inductance{H: x}, &InverseMagneticFlux{per_Wb: y}), InverseCurrent{per_A: x*y});
		assert_eq!(div_check(&Inductance{H: x}, &MagneticFlux{Wb: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Current{A: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &InverseCurrent{per_A: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Time{s: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Capacitance{F: y}), InverseVoltage{per_V: x*y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Conductance{S: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &Elastance{per_F: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &InverseMagneticFlux{per_Wb: y}), Resistance{Ohm: x/y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &InverseVoltage{per_V: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &InverseVoltage{per_V: y}), Elastance{per_F: x/y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &MagneticFlux{Wb: y}), Resistance{Ohm: x*y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &Resistance{Ohm: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Voltage{V: y}), Elastance{per_F: x*y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &Voltage{V: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Energy{J: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&InverseCharge{per_C: x}, &Torque{Nm: y}), Voltage{V: x*y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &Frequency{Hz: y}), InverseCurrent{per_A: x/y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &InverseEnergy{per_J: y}), Voltage{V: x/y});
		assert_eq!(div_check(&InverseCharge{per_C: x}, &InverseTorque{per_Nm: y}), Voltage{V: x/y});
		assert_eq!(div_check(&InverseInductance{per_H: x}, &Current{A: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(mul_check(&InverseInductance{per_H: x}, &InverseCurrent{per_A: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&InverseInductance{per_H: x}, &Time{s: y}), Conductance{S: x*y});
		assert_eq!(div_check(&InverseInductance{per_H: x}, &Conductance{S: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&InverseInductance{per_H: x}, &InverseMagneticFlux{per_Wb: y}), Current{A: x/y});
		assert_eq!(mul_check(&InverseInductance{per_H: x}, &MagneticFlux{Wb: y}), Current{A: x*y});
		assert_eq!(mul_check(&InverseInductance{per_H: x}, &Resistance{Ohm: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&InverseInductance{per_H: x}, &Frequency{Hz: y}), Conductance{S: x/y});
		assert_eq!(div_check(&InverseLuminousFlux{per_lm: x}, &InverseLuminosity{per_cd: y}), InverseSolidAngle{per_sr: x/y});
		assert_eq!(mul_check(&InverseLuminousFlux{per_lm: x}, &Luminosity{cd: y}), InverseSolidAngle{per_sr: x*y});
		assert_eq!(div_check(&InverseLuminousFlux{per_lm: x}, &AreaPerLumen{m2_per_lm: y}), InverseArea{per_m2: x/y});
		assert_eq!(mul_check(&InverseLuminousFlux{per_lm: x}, &Illuminance{lux: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&InverseLuminousFlux{per_lm: x}, &Area{m2: y}), AreaPerLumen{m2_per_lm: x*y});
		assert_eq!(div_check(&InverseLuminousFlux{per_lm: x}, &InverseArea{per_m2: y}), AreaPerLumen{m2_per_lm: x/y});
		assert_eq!(div_check(&InverseLuminousFlux{per_lm: x}, &InverseSolidAngle{per_sr: y}), InverseLuminosity{per_cd: x/y});
		assert_eq!(mul_check(&InverseLuminousFlux{per_lm: x}, &SolidAngle{sr: y}), InverseLuminosity{per_cd: x*y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Current{A: y}), InverseInductance{per_H: x*y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &Current{A: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &InverseCurrent{per_A: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseCurrent{per_A: y}), InverseInductance{per_H: x/y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Time{s: y}), InverseVoltage{per_V: x*y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Charge{C: y}), Conductance{S: x*y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &Conductance{S: y}), InverseCharge{per_C: x/y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Inductance{H: y}), InverseCurrent{per_A: x*y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseCharge{per_C: y}), Conductance{S: x/y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseInductance{per_H: y}), InverseCurrent{per_A: x/y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseMagneticFluxDensity{m2_per_Wb: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseVoltage{per_V: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &MagneticFluxDensity{T: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Resistance{Ohm: y}), InverseCharge{per_C: x*y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Voltage{V: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Area{m2: y}), InverseMagneticFluxDensity{m2_per_Wb: x*y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseArea{per_m2: y}), InverseMagneticFluxDensity{m2_per_Wb: x/y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Energy{J: y}), Current{A: x*y});
		assert_eq!(mul_check(&InverseMagneticFlux{per_Wb: x}, &Torque{Nm: y}), Current{A: x*y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &Frequency{Hz: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseEnergy{per_J: y}), Current{A: x/y});
		assert_eq!(div_check(&InverseMagneticFlux{per_Wb: x}, &InverseTorque{per_Nm: y}), Current{A: x/y});
		assert_eq!(div_check(&InverseMagneticFluxDensity{m2_per_Wb: x}, &InverseMagneticFlux{per_Wb: y}), Area{m2: x/y});
		assert_eq!(mul_check(&InverseMagneticFluxDensity{m2_per_Wb: x}, &MagneticFlux{Wb: y}), Area{m2: x*y});
		assert_eq!(div_check(&InverseMagneticFluxDensity{m2_per_Wb: x}, &Area{m2: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(mul_check(&InverseMagneticFluxDensity{m2_per_Wb: x}, &InverseArea{per_m2: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Current{A: y}), Conductance{S: x*y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &Current{A: y}), InversePower{per_W: x/y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &InverseCurrent{per_A: y}), InversePower{per_W: x*y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &InverseCurrent{per_A: y}), Conductance{S: x/y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &Time{s: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &Capacitance{F: y}), InverseCharge{per_C: x/y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Charge{C: y}), Capacitance{F: x*y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &Charge{C: y}), InverseEnergy{per_J: x/y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &Conductance{S: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Elastance{per_F: y}), InverseCharge{per_C: x*y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &InverseCharge{per_C: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &InverseCharge{per_C: y}), Capacitance{F: x/y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &InverseMagneticFlux{per_Wb: y}), Time{s: x/y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &MagneticFlux{Wb: y}), Time{s: x*y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Resistance{Ohm: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Energy{J: y}), Charge{C: x*y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Torque{Nm: y}), Charge{C: x*y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Frequency{Hz: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &InverseEnergy{per_J: y}), Charge{C: x/y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &InverseTorque{per_Nm: y}), Charge{C: x/y});
		assert_eq!(div_check(&InverseVoltage{per_V: x}, &InversePower{per_W: y}), Current{A: x/y});
		assert_eq!(mul_check(&InverseVoltage{per_V: x}, &Power{W: y}), Current{A: x*y});
		assert_eq!(mul_check(&LuminousFlux{lm: x}, &InverseLuminosity{per_cd: y}), SolidAngle{sr: x*y});
		assert_eq!(mul_check(&LuminousFlux{lm: x}, &AreaPerLumen{m2_per_lm: y}), Area{m2: x*y});
		assert_eq!(mul_check(&LuminousFlux{lm: x}, &InverseArea{per_m2: y}), Illuminance{lux: x*y});
		assert_eq!(mul_check(&LuminousFlux{lm: x}, &InverseSolidAngle{per_sr: y}), Luminosity{cd: x*y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseCurrent{per_A: y}), Inductance{H: x*y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &InverseCurrent{per_A: y}), Energy{J: x/y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseCharge{per_C: y}), Resistance{Ohm: x*y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseInductance{per_H: y}), Current{A: x*y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseMagneticFluxDensity{m2_per_Wb: y}), Area{m2: x*y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseVoltage{per_V: y}), Time{s: x*y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseArea{per_m2: y}), MagneticFluxDensity{T: x*y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Energy{J: y}), InverseCurrent{per_A: x/y});
		assert_eq!(div_check(&MagneticFlux{Wb: x}, &Torque{Nm: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseEnergy{per_J: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&MagneticFlux{Wb: x}, &InverseTorque{per_Nm: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&MagneticFluxDensity{T: x}, &InverseMagneticFlux{per_Wb: y}), InverseArea{per_m2: x*y});
		assert_eq!(div_check(&MagneticFluxDensity{T: x}, &MagneticFlux{Wb: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&MagneticFluxDensity{T: x}, &InverseArea{per_m2: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &InverseCurrent{per_A: y}), Voltage{V: x/y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &Time{s: y}), Elastance{per_F: x/y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &Elastance{per_F: y}), Time{s: x/y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &InverseCharge{per_C: y}), MagneticFlux{Wb: x/y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &InverseInductance{per_H: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &InverseMagneticFlux{per_Wb: y}), InverseCharge{per_C: x*y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &InverseVoltage{per_V: y}), InverseCurrent{per_A: x*y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &MagneticFlux{Wb: y}), InverseCharge{per_C: x/y});
		assert_eq!(div_check(&Resistance{Ohm: x}, &Voltage{V: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&Resistance{Ohm: x}, &Frequency{Hz: y}), Elastance{per_F: x*y});
		assert_eq!(mul_check(&Voltage{V: x}, &InverseCurrent{per_A: y}), Resistance{Ohm: x*y});
		assert_eq!(div_check(&Voltage{V: x}, &InverseCurrent{per_A: y}), Power{W: x/y});
		assert_eq!(div_check(&Voltage{V: x}, &Charge{C: y}), Elastance{per_F: x/y});
		assert_eq!(div_check(&Voltage{V: x}, &Elastance{per_F: y}), Charge{C: x/y});
		assert_eq!(mul_check(&Voltage{V: x}, &InverseCharge{per_C: y}), Elastance{per_F: x*y});
		assert_eq!(div_check(&Voltage{V: x}, &InverseCharge{per_C: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Voltage{V: x}, &InverseMagneticFlux{per_Wb: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&Voltage{V: x}, &Energy{J: y}), InverseCharge{per_C: x/y});
		assert_eq!(div_check(&Voltage{V: x}, &Torque{Nm: y}), InverseCharge{per_C: x/y});
		assert_eq!(mul_check(&Voltage{V: x}, &InverseEnergy{per_J: y}), InverseCharge{per_C: x*y});
		assert_eq!(mul_check(&Voltage{V: x}, &InverseTorque{per_Nm: y}), InverseCharge{per_C: x*y});
		assert_eq!(mul_check(&Voltage{V: x}, &InversePower{per_W: y}), InverseCurrent{per_A: x*y});
		assert_eq!(div_check(&Voltage{V: x}, &Power{W: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&Angle{rad: x}, &Angle{rad: y}), SolidAngle{sr: x*y});
		assert_eq!(div_check(&Angle{rad: x}, &InverseAngle{per_rad: y}), SolidAngle{sr: x/y});
		assert_eq!(mul_check(&Angle{rad: x}, &InverseSolidAngle{per_sr: y}), InverseAngle{per_rad: x*y});
		assert_eq!(div_check(&Angle{rad: x}, &SolidAngle{sr: y}), InverseAngle{per_rad: x/y});
		assert_eq!(mul_check(&Angle{rad: x}, &InverseAngularVelocity{s_per_rad: y}), Time{s: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseDistance{per_m: y}), Distance{m: x*y});
		assert_eq!(div_check(&Area{m2: x}, &InverseDistance{per_m: y}), Volume{m3: x/y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseMass{per_kg: y}), AreaPerMass{m2_per_kg: x*y});
		assert_eq!(div_check(&Area{m2: x}, &Mass{kg: y}), AreaPerMass{m2_per_kg: x/y});
		assert_eq!(div_check(&Area{m2: x}, &AreaPerLumen{m2_per_lm: y}), LuminousFlux{lm: x/y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseLuminousFlux{per_lm: y}), AreaPerLumen{m2_per_lm: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseMagneticFlux{per_Wb: y}), InverseMagneticFluxDensity{m2_per_Wb: x*y});
		assert_eq!(div_check(&Area{m2: x}, &InverseMagneticFluxDensity{m2_per_Wb: y}), MagneticFlux{Wb: x/y});
		assert_eq!(div_check(&Area{m2: x}, &LuminousFlux{lm: y}), AreaPerLumen{m2_per_lm: x/y});
		assert_eq!(div_check(&Area{m2: x}, &MagneticFlux{Wb: y}), InverseMagneticFluxDensity{m2_per_Wb: x/y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseVolume{per_m3: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&Area{m2: x}, &Volume{m3: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&Area{m2: x}, &AreaPerMass{m2_per_kg: y}), Mass{kg: x/y});
		assert_eq!(div_check(&Area{m2: x}, &Force{N: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseForce{per_N: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&Area{m2: x}, &InverseMomentOfInertia{per_kgm2: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&Area{m2: x}, &InversePressure{per_Pa: y}), Force{N: x/y});
		assert_eq!(div_check(&Area{m2: x}, &MomentOfInertia{kgm2: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&InverseAngle{per_rad: x}, &Time{s: y}), InverseAngularVelocity{s_per_rad: x*y});
		assert_eq!(div_check(&InverseAngle{per_rad: x}, &Angle{rad: y}), InverseSolidAngle{per_sr: x/y});
		assert_eq!(mul_check(&InverseAngle{per_rad: x}, &InverseAngle{per_rad: y}), InverseSolidAngle{per_sr: x*y});
		assert_eq!(div_check(&InverseAngle{per_rad: x}, &InverseSolidAngle{per_sr: y}), Angle{rad: x/y});
		assert_eq!(mul_check(&InverseAngle{per_rad: x}, &SolidAngle{sr: y}), Angle{rad: x*y});
		assert_eq!(mul_check(&InverseAngle{per_rad: x}, &AngularVelocity{radps: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&InverseAngle{per_rad: x}, &Frequency{Hz: y}), InverseAngularVelocity{s_per_rad: x/y});
		assert_eq!(div_check(&InverseAngle{per_rad: x}, &InverseAngularVelocity{s_per_rad: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &Distance{m: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &Distance{m: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &InverseDistance{per_m: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseDistance{per_m: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseMass{per_kg: y}), AreaDensity{kgpm2: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &Mass{kg: y}), AreaDensity{kgpm2: x*y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &AreaPerLumen{m2_per_lm: y}), InverseLuminousFlux{per_lm: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &Illuminance{lux: y}), InverseLuminousFlux{per_lm: x/y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseLuminousFlux{per_lm: y}), Illuminance{lux: x/y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseMagneticFlux{per_Wb: y}), MagneticFluxDensity{T: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &InverseMagneticFluxDensity{m2_per_Wb: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &LuminousFlux{lm: y}), Illuminance{lux: x*y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &MagneticFlux{Wb: y}), MagneticFluxDensity{T: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &MagneticFluxDensity{T: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseVolume{per_m3: y}), Distance{m: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &Volume{m3: y}), Distance{m: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &AreaDensity{kgpm2: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &AreaPerMass{m2_per_kg: y}), InverseMass{per_kg: x*y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &Force{N: y}), Pressure{Pa: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseForce{per_N: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &InverseMomentOfInertia{per_kgm2: y}), Mass{kg: x/y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &InversePressure{per_Pa: y}), InverseForce{per_N: x*y});
		assert_eq!(mul_check(&InverseArea{per_m2: x}, &MomentOfInertia{kgm2: y}), Mass{kg: x*y});
		assert_eq!(div_check(&InverseArea{per_m2: x}, &Pressure{Pa: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InverseSolidAngle{per_sr: x}, &InverseLuminosity{per_cd: y}), InverseLuminousFlux{per_lm: x*y});
		assert_eq!(div_check(&InverseSolidAngle{per_sr: x}, &Luminosity{cd: y}), InverseLuminousFlux{per_lm: x/y});
		assert_eq!(div_check(&InverseSolidAngle{per_sr: x}, &InverseLuminousFlux{per_lm: y}), Luminosity{cd: x/y});
		assert_eq!(mul_check(&InverseSolidAngle{per_sr: x}, &LuminousFlux{lm: y}), Luminosity{cd: x*y});
		assert_eq!(mul_check(&InverseSolidAngle{per_sr: x}, &Angle{rad: y}), InverseAngle{per_rad: x*y});
		assert_eq!(div_check(&InverseSolidAngle{per_sr: x}, &InverseAngle{per_rad: y}), InverseAngle{per_rad: x/y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &Amount{mol: y}), Concentration{molpm3: x*y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &Distance{m: y}), InverseArea{per_m2: x*y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &InverseAmount{per_mol: y}), Concentration{molpm3: x/y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &InverseDistance{per_m: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &InverseMass{per_kg: y}), Density{kgpm3: x/y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &Mass{kg: y}), Density{kgpm3: x*y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &Concentration{molpm3: y}), InverseAmount{per_mol: x/y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &MolarVolume{m3_per_mol: y}), InverseAmount{per_mol: x*y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &Area{m2: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &InverseArea{per_m2: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &Density{kgpm3: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &Energy{J: y}), Pressure{Pa: x*y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &Torque{Nm: y}), Pressure{Pa: x*y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &InverseEnergy{per_J: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &InverseTorque{per_Nm: y}), Pressure{Pa: x/y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &InversePressure{per_Pa: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseVolume{per_m3: x}, &Pressure{Pa: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InverseVolume{per_m3: x}, &VolumePerMass{m3_per_kg: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&SolidAngle{sr: x}, &InverseLuminosity{per_cd: y}), LuminousFlux{lm: x/y});
		assert_eq!(mul_check(&SolidAngle{sr: x}, &InverseLuminousFlux{per_lm: y}), InverseLuminosity{per_cd: x*y});
		assert_eq!(div_check(&SolidAngle{sr: x}, &LuminousFlux{lm: y}), InverseLuminosity{per_cd: x/y});
		assert_eq!(div_check(&SolidAngle{sr: x}, &Angle{rad: y}), Angle{rad: x/y});
		assert_eq!(mul_check(&SolidAngle{sr: x}, &InverseAngle{per_rad: y}), Angle{rad: x*y});
		assert_eq!(div_check(&Volume{m3: x}, &Amount{mol: y}), MolarVolume{m3_per_mol: x/y});
		assert_eq!(mul_check(&Volume{m3: x}, &InverseAmount{per_mol: y}), MolarVolume{m3_per_mol: x*y});
		assert_eq!(mul_check(&Volume{m3: x}, &InverseDistance{per_m: y}), Area{m2: x*y});
		assert_eq!(mul_check(&Volume{m3: x}, &InverseMass{per_kg: y}), VolumePerMass{m3_per_kg: x*y});
		assert_eq!(div_check(&Volume{m3: x}, &Mass{kg: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(div_check(&Volume{m3: x}, &MolarVolume{m3_per_mol: y}), Amount{mol: x/y});
		assert_eq!(mul_check(&Volume{m3: x}, &InverseArea{per_m2: y}), Distance{m: x*y});
		assert_eq!(div_check(&Volume{m3: x}, &Energy{J: y}), InversePressure{per_Pa: x/y});
		assert_eq!(div_check(&Volume{m3: x}, &Torque{Nm: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&Volume{m3: x}, &InverseEnergy{per_J: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&Volume{m3: x}, &InverseTorque{per_Nm: y}), InversePressure{per_Pa: x*y});
		assert_eq!(div_check(&Volume{m3: x}, &InversePressure{per_Pa: y}), Energy{J: x/y});
		assert_eq!(div_check(&Volume{m3: x}, &VolumePerMass{m3_per_kg: y}), Mass{kg: x/y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &InverseMass{per_kg: y}), Force{N: x/y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &AreaPerMass{m2_per_kg: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &Force{N: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &InverseForce{per_N: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &InverseMomentum{s_per_kgm: y}), Power{W: x/y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &InversePower{per_W: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &InversePressure{per_Pa: y}), AreaPerMass{m2_per_kg: x*y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &Power{W: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(div_check(&Acceleration{mps2: x}, &Pressure{Pa: y}), AreaPerMass{m2_per_kg: x/y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &TimePerDistance{spm: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &InverseAbsorbedDose{per_Gy: y}), InverseDistance{per_m: x*y});
		assert_eq!(mul_check(&Acceleration{mps2: x}, &InverseDoseEquivalent{per_Sv: y}), InverseDistance{per_m: x*y});
		assert_eq!(mul_check(&AngularAcceleration{radps2: x}, &InverseAngularVelocity{s_per_rad: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&AngularMomentum{kgm2radps: x}, &InverseMomentOfInertia{per_kgm2: y}), AngularVelocity{radps: x*y});
		assert_eq!(mul_check(&AngularVelocity{radps: x}, &InverseAngle{per_rad: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&AngularVelocity{radps: x}, &InverseAngularAcceleration{s2prad: y}), Time{s: x*y});
		assert_eq!(div_check(&AngularVelocity{radps: x}, &InverseMomentOfInertia{per_kgm2: y}), AngularMomentum{kgm2radps: x/y});
		assert_eq!(mul_check(&AreaDensity{kgpm2: x}, &InverseDistance{per_m: y}), Density{kgpm3: x*y});
		assert_eq!(mul_check(&AreaDensity{kgpm2: x}, &InverseMass{per_kg: y}), InverseArea{per_m2: x*y});
		assert_eq!(div_check(&AreaDensity{kgpm2: x}, &Mass{kg: y}), InverseArea{per_m2: x/y});
		assert_eq!(div_check(&AreaDensity{kgpm2: x}, &InverseArea{per_m2: y}), Mass{kg: x/y});
		assert_eq!(div_check(&AreaDensity{kgpm2: x}, &InverseAcceleration{s2pm: y}), Pressure{Pa: x/y});
		assert_eq!(mul_check(&AreaDensity{kgpm2: x}, &InversePressure{per_Pa: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&AreaDensity{kgpm2: x}, &Pressure{Pa: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&AreaDensity{kgpm2: x}, &VolumePerMass{m3_per_kg: y}), Distance{m: x*y});
		assert_eq!(mul_check(&AreaPerMass{m2_per_kg: x}, &Distance{m: y}), VolumePerMass{m3_per_kg: x*y});
		assert_eq!(div_check(&AreaPerMass{m2_per_kg: x}, &InverseDistance{per_m: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(div_check(&AreaPerMass{m2_per_kg: x}, &InverseMass{per_kg: y}), Area{m2: x/y});
		assert_eq!(mul_check(&AreaPerMass{m2_per_kg: x}, &Mass{kg: y}), Area{m2: x*y});
		assert_eq!(div_check(&AreaPerMass{m2_per_kg: x}, &Area{m2: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&AreaPerMass{m2_per_kg: x}, &InverseArea{per_m2: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&AreaPerMass{m2_per_kg: x}, &Acceleration{mps2: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&AreaPerMass{m2_per_kg: x}, &Density{kgpm3: y}), InverseDistance{per_m: x*y});
		assert_eq!(mul_check(&AreaPerMass{m2_per_kg: x}, &InverseAcceleration{s2pm: y}), InversePressure{per_Pa: x*y});
		assert_eq!(div_check(&AreaPerMass{m2_per_kg: x}, &InversePressure{per_Pa: y}), Acceleration{mps2: x/y});
		assert_eq!(mul_check(&AreaPerMass{m2_per_kg: x}, &Pressure{Pa: y}), Acceleration{mps2: x*y});
		assert_eq!(div_check(&AreaPerMass{m2_per_kg: x}, &VolumePerMass{m3_per_kg: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&Density{kgpm3: x}, &InverseDistance{per_m: y}), AreaDensity{kgpm2: x/y});
		assert_eq!(mul_check(&Density{kgpm3: x}, &InverseMass{per_kg: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&Density{kgpm3: x}, &Mass{kg: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&Density{kgpm3: x}, &MolarVolume{m3_per_mol: y}), MolarMass{kgpmol: x*y});
		assert_eq!(div_check(&Density{kgpm3: x}, &InverseVolume{per_m3: y}), Mass{kg: x/y});
		assert_eq!(div_check(&Density{kgpm3: x}, &AreaDensity{kgpm2: y}), InverseDistance{per_m: x/y});
		assert_eq!(mul_check(&Density{kgpm3: x}, &AreaPerMass{m2_per_kg: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&Density{kgpm3: x}, &InverseAbsorbedDose{per_Gy: y}), Pressure{Pa: x/y});
		assert_eq!(div_check(&Density{kgpm3: x}, &InverseDoseEquivalent{per_Sv: y}), Pressure{Pa: x/y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseCurrent{per_A: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseDistance{per_m: y}), Force{N: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseCharge{per_C: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseMagneticFlux{per_Wb: y}), Current{A: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseVoltage{per_V: y}), Charge{C: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseVolume{per_m3: y}), Pressure{Pa: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseForce{per_N: y}), Distance{m: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseMomentum{s_per_kgm: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InversePower{per_W: y}), Time{s: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InversePressure{per_Pa: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &TimePerDistance{spm: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseAbsorbedDose{per_Gy: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&Energy{J: x}, &InverseDoseEquivalent{per_Sv: y}), Mass{kg: x*y});
		assert_eq!(div_check(&Force{N: x}, &InverseDistance{per_m: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Force{N: x}, &InverseMass{per_kg: y}), Acceleration{mps2: x*y});
		assert_eq!(mul_check(&Force{N: x}, &InverseArea{per_m2: y}), Pressure{Pa: x*y});
		assert_eq!(div_check(&Force{N: x}, &Energy{J: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&Force{N: x}, &Torque{Nm: y}), InverseDistance{per_m: x/y});
		assert_eq!(mul_check(&Force{N: x}, &InverseAcceleration{s2pm: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&Force{N: x}, &InverseEnergy{per_J: y}), InverseDistance{per_m: x*y});
		assert_eq!(mul_check(&Force{N: x}, &InverseTorque{per_Nm: y}), InverseDistance{per_m: x*y});
		assert_eq!(mul_check(&Force{N: x}, &InverseMomentum{s_per_kgm: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&Force{N: x}, &InversePower{per_W: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&Force{N: x}, &InversePressure{per_Pa: y}), Area{m2: x*y});
		assert_eq!(div_check(&Force{N: x}, &Power{W: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&Force{N: x}, &TimePerDistance{spm: y}), Power{W: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Current{A: y}), InverseCharge{per_C: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseAmount{per_mol: y}), CatalyticActivity{molps: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseCurrent{per_A: y}), InverseCharge{per_C: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseDistance{per_m: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &CatalyticActivity{molps: y}), InverseAmount{per_mol: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseCatalyticActivity{s_per_mol: y}), InverseAmount{per_mol: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Conductance{S: y}), InverseInductance{per_H: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Conductance{S: y}), Elastance{per_F: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Elastance{per_F: y}), Conductance{S: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseCharge{per_C: y}), Current{A: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseInductance{per_H: y}), Resistance{Ohm: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseMagneticFlux{per_Wb: y}), Voltage{V: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseVoltage{per_V: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &Resistance{Ohm: y}), Elastance{per_F: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Resistance{Ohm: y}), InverseInductance{per_H: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Voltage{V: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseAngle{per_rad: y}), AngularVelocity{radps: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Acceleration{mps2: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &AngularAcceleration{radps2: y}), InverseAngularVelocity{s_per_rad: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &AngularVelocity{radps: y}), InverseAngle{per_rad: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Force{N: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseAcceleration{s2pm: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseAngularAcceleration{s2prad: y}), InverseAngularVelocity{s_per_rad: x*y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseAngularVelocity{s_per_rad: y}), InverseAngle{per_rad: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseAngularVelocity{s_per_rad: y}), AngularAcceleration{radps2: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseEnergy{per_J: y}), Power{W: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseTorque{per_Nm: y}), Power{W: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InverseForce{per_N: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &InverseMomentum{s_per_kgm: y}), Force{N: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &InversePower{per_W: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Power{W: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&Frequency{Hz: x}, &TimePerDistance{spm: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&Frequency{Hz: x}, &TimePerDistance{spm: y}), Acceleration{mps2: x/y});
		assert_eq!(div_check(&Frequency{Hz: x}, &Velocity{mps: y}), InverseDistance{per_m: x/y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &InverseMass{per_kg: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &Mass{kg: y}), InverseForce{per_N: x/y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &Time{s: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &AreaDensity{kgpm2: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &AreaPerMass{m2_per_kg: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &Force{N: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &Frequency{Hz: y}), TimePerDistance{spm: x*y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &InverseForce{per_N: y}), Mass{kg: x/y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &InverseMomentum{s_per_kgm: y}), InversePower{per_W: x*y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &InversePower{per_W: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &InversePressure{per_Pa: y}), AreaDensity{kgpm2: x/y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &Momentum{kgmps: y}), InversePower{per_W: x/y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &Power{W: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &Pressure{Pa: y}), AreaDensity{kgpm2: x*y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &TimePerDistance{spm: y}), Time{s: x/y});
		assert_eq!(mul_check(&InverseAcceleration{s2pm: x}, &Velocity{mps: y}), Time{s: x*y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &InverseAbsorbedDose{per_Gy: y}), Distance{m: x/y});
		assert_eq!(div_check(&InverseAcceleration{s2pm: x}, &InverseDoseEquivalent{per_Sv: y}), Distance{m: x/y});
		assert_eq!(div_check(&InverseAngularAcceleration{s2prad: x}, &Time{s: y}), InverseAngularVelocity{s_per_rad: x/y});
		assert_eq!(mul_check(&InverseAngularAcceleration{s2prad: x}, &AngularVelocity{radps: y}), Time{s: x*y});
		assert_eq!(mul_check(&InverseAngularAcceleration{s2prad: x}, &Frequency{Hz: y}), InverseAngularVelocity{s_per_rad: x*y});
		assert_eq!(div_check(&InverseAngularAcceleration{s2prad: x}, &InverseAngularVelocity{s_per_rad: y}), Time{s: x/y});
		assert_eq!(div_check(&InverseAngularMomentum{s_per_kgm2rad: x}, &InverseMomentOfInertia{per_kgm2: y}), InverseAngularVelocity{s_per_rad: x/y});
		assert_eq!(mul_check(&InverseAngularMomentum{s_per_kgm2rad: x}, &MomentOfInertia{kgm2: y}), InverseAngularVelocity{s_per_rad: x*y});
		assert_eq!(mul_check(&InverseAngularVelocity{s_per_rad: x}, &Time{s: y}), InverseAngularAcceleration{s2prad: x*y});
		assert_eq!(div_check(&InverseAngularVelocity{s_per_rad: x}, &Time{s: y}), InverseAngle{per_rad: x/y});
		assert_eq!(mul_check(&InverseAngularVelocity{s_per_rad: x}, &Angle{rad: y}), Time{s: x*y});
		assert_eq!(div_check(&InverseAngularVelocity{s_per_rad: x}, &InverseAngle{per_rad: y}), Time{s: x/y});
		assert_eq!(mul_check(&InverseAngularVelocity{s_per_rad: x}, &AngularAcceleration{radps2: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&InverseAngularVelocity{s_per_rad: x}, &Frequency{Hz: y}), InverseAngle{per_rad: x*y});
		assert_eq!(div_check(&InverseAngularVelocity{s_per_rad: x}, &Frequency{Hz: y}), InverseAngularAcceleration{s2prad: x/y});
		assert_eq!(div_check(&InverseAngularVelocity{s_per_rad: x}, &InverseAngularAcceleration{s2prad: y}), Frequency{Hz: x/y});
		assert_eq!(mul_check(&InverseAngularVelocity{s_per_rad: x}, &InverseMomentOfInertia{per_kgm2: y}), InverseAngularMomentum{s_per_kgm2rad: x*y});
		assert_eq!(div_check(&InverseAngularVelocity{s_per_rad: x}, &MomentOfInertia{kgm2: y}), InverseAngularMomentum{s_per_kgm2rad: x/y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Current{A: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Distance{m: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseCurrent{per_A: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseDistance{per_m: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Time{s: y}), InversePower{per_W: x*y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Charge{C: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseCharge{per_C: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseMagneticFlux{per_Wb: y}), InverseCurrent{per_A: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseVoltage{per_V: y}), InverseCharge{per_C: x/y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &MagneticFlux{Wb: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Voltage{V: y}), InverseCharge{per_C: x*y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseVolume{per_m3: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Volume{m3: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Force{N: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &Frequency{Hz: y}), InversePower{per_W: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseForce{per_N: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseMomentum{s_per_kgm: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InversePower{per_W: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InversePressure{per_Pa: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Momentum{kgmps: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Power{W: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Pressure{Pa: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &TimePerDistance{spm: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(mul_check(&InverseEnergy{per_J: x}, &Velocity{mps: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseAbsorbedDose{per_Gy: y}), InverseMass{per_kg: x/y});
		assert_eq!(div_check(&InverseEnergy{per_J: x}, &InverseDoseEquivalent{per_Sv: y}), InverseMass{per_kg: x/y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &Distance{m: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &InverseDistance{per_m: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InverseMass{per_kg: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Mass{kg: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &Time{s: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Area{m2: y}), InversePressure{per_Pa: x*y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InverseArea{per_m2: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Acceleration{mps2: y}), InverseMass{per_kg: x*y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Energy{J: y}), Distance{m: x*y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Torque{Nm: y}), Distance{m: x*y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Frequency{Hz: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InverseAcceleration{s2pm: y}), InverseMass{per_kg: x/y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InverseEnergy{per_J: y}), Distance{m: x/y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InverseTorque{per_Nm: y}), Distance{m: x/y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InverseMomentum{s_per_kgm: y}), Time{s: x/y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InversePower{per_W: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &InversePressure{per_Pa: y}), InverseArea{per_m2: x/y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Momentum{kgmps: y}), Time{s: x*y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Power{W: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &Pressure{Pa: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&InverseForce{per_N: x}, &TimePerDistance{spm: y}), InversePower{per_W: x*y});
		assert_eq!(div_check(&InverseForce{per_N: x}, &Velocity{mps: y}), InversePower{per_W: x/y});
		assert_eq!(div_check(&InverseMomentOfInertia{per_kgm2: x}, &InverseMass{per_kg: y}), InverseArea{per_m2: x/y});
		assert_eq!(mul_check(&InverseMomentOfInertia{per_kgm2: x}, &Mass{kg: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&InverseMomentOfInertia{per_kgm2: x}, &Area{m2: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&InverseMomentOfInertia{per_kgm2: x}, &InverseArea{per_m2: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&InverseMomentOfInertia{per_kgm2: x}, &AngularMomentum{kgm2radps: y}), AngularVelocity{radps: x*y});
		assert_eq!(div_check(&InverseMomentOfInertia{per_kgm2: x}, &AngularVelocity{radps: y}), InverseAngularMomentum{s_per_kgm2rad: x/y});
		assert_eq!(div_check(&InverseMomentOfInertia{per_kgm2: x}, &InverseAngularMomentum{s_per_kgm2rad: y}), AngularVelocity{radps: x/y});
		assert_eq!(mul_check(&InverseMomentOfInertia{per_kgm2: x}, &InverseAngularVelocity{s_per_rad: y}), InverseAngularMomentum{s_per_kgm2rad: x*y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &InverseMass{per_kg: y}), TimePerDistance{spm: x/y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Mass{kg: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Time{s: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &Acceleration{mps2: y}), InversePower{per_W: x/y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Energy{J: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Torque{Nm: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Force{N: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &Frequency{Hz: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &InverseAcceleration{s2pm: y}), InversePower{per_W: x*y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &InverseEnergy{per_J: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &InverseTorque{per_Nm: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &InverseForce{per_N: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &InversePower{per_W: y}), Acceleration{mps2: x/y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Power{W: y}), Acceleration{mps2: x*y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &TimePerDistance{spm: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &TimePerDistance{spm: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&InverseMomentum{s_per_kgm: x}, &Velocity{mps: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&InverseMomentum{s_per_kgm: x}, &Velocity{mps: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Current{A: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseCurrent{per_A: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&InversePower{per_W: x}, &Time{s: y}), InverseEnergy{per_J: x/y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseVoltage{per_V: y}), InverseCurrent{per_A: x/y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Voltage{V: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Acceleration{mps2: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Energy{J: y}), Time{s: x*y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Torque{Nm: y}), Time{s: x*y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Force{N: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Frequency{Hz: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseAcceleration{s2pm: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseEnergy{per_J: y}), Time{s: x/y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseTorque{per_Nm: y}), Time{s: x/y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseForce{per_N: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&InversePower{per_W: x}, &InverseMomentum{s_per_kgm: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Momentum{kgmps: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&InversePower{per_W: x}, &TimePerDistance{spm: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InversePower{per_W: x}, &Velocity{mps: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &Area{m2: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &InverseArea{per_m2: y}), InverseForce{per_N: x*y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &InverseVolume{per_m3: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &Volume{m3: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &Acceleration{mps2: y}), AreaPerMass{m2_per_kg: x*y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &AreaDensity{kgpm2: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &AreaPerMass{m2_per_kg: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &Energy{J: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &Torque{Nm: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&InversePressure{per_Pa: x}, &Force{N: y}), Area{m2: x*y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &InverseAcceleration{s2pm: y}), AreaPerMass{m2_per_kg: x/y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &InverseEnergy{per_J: y}), Volume{m3: x/y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &InverseTorque{per_Nm: y}), Volume{m3: x/y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &InverseForce{per_N: y}), Area{m2: x/y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &InverseAbsorbedDose{per_Gy: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(div_check(&InversePressure{per_Pa: x}, &InverseDoseEquivalent{per_Sv: y}), VolumePerMass{m3_per_kg: x/y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Current{A: y}), InverseMagneticFlux{per_Wb: x*y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Distance{m: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseCurrent{per_A: y}), InverseMagneticFlux{per_Wb: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseDistance{per_m: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Time{s: y}), InversePower{per_W: x*y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Charge{C: y}), InverseVoltage{per_V: x*y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseCharge{per_C: y}), InverseVoltage{per_V: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseMagneticFlux{per_Wb: y}), InverseCurrent{per_A: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseVoltage{per_V: y}), InverseCharge{per_C: x/y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &MagneticFlux{Wb: y}), InverseCurrent{per_A: x*y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Voltage{V: y}), InverseCharge{per_C: x*y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseVolume{per_m3: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Volume{m3: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Force{N: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &Frequency{Hz: y}), InversePower{per_W: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseForce{per_N: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseMomentum{s_per_kgm: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InversePower{per_W: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InversePressure{per_Pa: y}), InverseVolume{per_m3: x/y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Momentum{kgmps: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Power{W: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Pressure{Pa: y}), InverseVolume{per_m3: x*y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &TimePerDistance{spm: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(mul_check(&InverseTorque{per_Nm: x}, &Velocity{mps: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseAbsorbedDose{per_Gy: y}), InverseMass{per_kg: x/y});
		assert_eq!(div_check(&InverseTorque{per_Nm: x}, &InverseDoseEquivalent{per_Sv: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&MomentOfInertia{kgm2: x}, &InverseMass{per_kg: y}), Area{m2: x*y});
		assert_eq!(mul_check(&MomentOfInertia{kgm2: x}, &InverseArea{per_m2: y}), Mass{kg: x*y});
		assert_eq!(div_check(&MomentOfInertia{kgm2: x}, &AngularMomentum{kgm2radps: y}), InverseAngularVelocity{s_per_rad: x/y});
		assert_eq!(mul_check(&MomentOfInertia{kgm2: x}, &InverseAngularMomentum{s_per_kgm2rad: y}), InverseAngularVelocity{s_per_rad: x*y});
		assert_eq!(div_check(&MomentOfInertia{kgm2: x}, &InverseAngularVelocity{s_per_rad: y}), AngularMomentum{kgm2radps: x/y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &InverseMass{per_kg: y}), Velocity{mps: x*y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Energy{J: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Torque{Nm: y}), TimePerDistance{spm: x/y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &InverseAcceleration{s2pm: y}), Power{W: x/y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &InverseEnergy{per_J: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &InverseTorque{per_Nm: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &InverseForce{per_N: y}), Time{s: x*y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &InversePower{per_W: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &Power{W: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&Momentum{kgmps: x}, &TimePerDistance{spm: y}), Mass{kg: x*y});
		assert_eq!(div_check(&Momentum{kgmps: x}, &TimePerDistance{spm: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Power{W: x}, &InverseCurrent{per_A: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Power{W: x}, &InverseVoltage{per_V: y}), Current{A: x*y});
		assert_eq!(mul_check(&Power{W: x}, &InverseAcceleration{s2pm: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&Power{W: x}, &InverseEnergy{per_J: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&Power{W: x}, &InverseTorque{per_Nm: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&Power{W: x}, &InverseForce{per_N: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&Power{W: x}, &InverseMomentum{s_per_kgm: y}), Acceleration{mps2: x*y});
		assert_eq!(mul_check(&Power{W: x}, &TimePerDistance{spm: y}), Force{N: x*y});
		assert_eq!(div_check(&Pressure{Pa: x}, &InverseArea{per_m2: y}), Force{N: x/y});
		assert_eq!(div_check(&Pressure{Pa: x}, &InverseVolume{per_m3: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &AreaPerMass{m2_per_kg: y}), Acceleration{mps2: x*y});
		assert_eq!(div_check(&Pressure{Pa: x}, &Energy{J: y}), InverseVolume{per_m3: x/y});
		assert_eq!(div_check(&Pressure{Pa: x}, &Torque{Nm: y}), InverseVolume{per_m3: x/y});
		assert_eq!(div_check(&Pressure{Pa: x}, &Force{N: y}), InverseArea{per_m2: x/y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &InverseAcceleration{s2pm: y}), AreaDensity{kgpm2: x*y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &InverseEnergy{per_J: y}), InverseVolume{per_m3: x*y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &InverseTorque{per_Nm: y}), InverseVolume{per_m3: x*y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &InverseForce{per_N: y}), InverseArea{per_m2: x*y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &InverseAbsorbedDose{per_Gy: y}), Density{kgpm3: x*y});
		assert_eq!(mul_check(&Pressure{Pa: x}, &InverseDoseEquivalent{per_Sv: y}), Density{kgpm3: x*y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Distance{m: y}), Time{s: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseDistance{per_m: y}), Time{s: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &InverseMass{per_kg: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &Mass{kg: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Time{s: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &Time{s: y}), InverseDistance{per_m: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Acceleration{mps2: y}), Frequency{Hz: x*y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Energy{J: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Torque{Nm: y}), Momentum{kgmps: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &Force{N: y}), InversePower{per_W: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Frequency{Hz: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &Frequency{Hz: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseAcceleration{s2pm: y}), Frequency{Hz: x/y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseEnergy{per_J: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseTorque{per_Nm: y}), Momentum{kgmps: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &InverseForce{per_N: y}), InversePower{per_W: x*y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &InverseMomentum{s_per_kgm: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseMomentum{s_per_kgm: y}), Mass{kg: x/y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InversePower{per_W: y}), Force{N: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Momentum{kgmps: y}), Mass{kg: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &Momentum{kgmps: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&TimePerDistance{spm: x}, &Power{W: y}), Force{N: x*y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseAbsorbedDose{per_Gy: y}), Velocity{mps: x/y});
		assert_eq!(div_check(&TimePerDistance{spm: x}, &InverseDoseEquivalent{per_Sv: y}), Velocity{mps: x/y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseCurrent{per_A: y}), MagneticFlux{Wb: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseDistance{per_m: y}), Force{N: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseCharge{per_C: y}), Voltage{V: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseMagneticFlux{per_Wb: y}), Current{A: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseVoltage{per_V: y}), Charge{C: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseVolume{per_m3: y}), Pressure{Pa: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseForce{per_N: y}), Distance{m: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseMomentum{s_per_kgm: y}), Velocity{mps: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InversePower{per_W: y}), Time{s: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InversePressure{per_Pa: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &TimePerDistance{spm: y}), Momentum{kgmps: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseAbsorbedDose{per_Gy: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&Torque{Nm: x}, &InverseDoseEquivalent{per_Sv: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseDistance{per_m: y}), Frequency{Hz: x*y});
		assert_eq!(div_check(&Velocity{mps: x}, &InverseMass{per_kg: y}), Momentum{kgmps: x/y});
		assert_eq!(div_check(&Velocity{mps: x}, &Energy{J: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(div_check(&Velocity{mps: x}, &Torque{Nm: y}), InverseMomentum{s_per_kgm: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseAcceleration{s2pm: y}), Time{s: x*y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseEnergy{per_J: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseTorque{per_Nm: y}), InverseMomentum{s_per_kgm: x*y});
		assert_eq!(div_check(&Velocity{mps: x}, &InverseForce{per_N: y}), Power{W: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseMomentum{s_per_kgm: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&Velocity{mps: x}, &InverseMomentum{s_per_kgm: y}), Energy{J: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InversePower{per_W: y}), InverseForce{per_N: x*y});
		assert_eq!(div_check(&Velocity{mps: x}, &Momentum{kgmps: y}), InverseMass{per_kg: x/y});
		assert_eq!(div_check(&Velocity{mps: x}, &Power{W: y}), InverseForce{per_N: x/y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseAbsorbedDose{per_Gy: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&Velocity{mps: x}, &InverseDoseEquivalent{per_Sv: y}), TimePerDistance{spm: x*y});
		assert_eq!(div_check(&VolumePerMass{m3_per_kg: x}, &Distance{m: y}), AreaPerMass{m2_per_kg: x/y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &InverseDistance{per_m: y}), AreaPerMass{m2_per_kg: x*y});
		assert_eq!(div_check(&VolumePerMass{m3_per_kg: x}, &InverseMass{per_kg: y}), Volume{m3: x/y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &Mass{kg: y}), Volume{m3: x*y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &Concentration{molpm3: y}), Molality{molpkg: x*y});
		assert_eq!(div_check(&VolumePerMass{m3_per_kg: x}, &Molality{molpkg: y}), MolarVolume{m3_per_mol: x/y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &MolarMass{kgpmol: y}), MolarVolume{m3_per_mol: x*y});
		assert_eq!(div_check(&VolumePerMass{m3_per_kg: x}, &MolarVolume{m3_per_mol: y}), Molality{molpkg: x/y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &InverseVolume{per_m3: y}), InverseMass{per_kg: x*y});
		assert_eq!(div_check(&VolumePerMass{m3_per_kg: x}, &Volume{m3: y}), InverseMass{per_kg: x/y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &AreaDensity{kgpm2: y}), Distance{m: x*y});
		assert_eq!(div_check(&VolumePerMass{m3_per_kg: x}, &AreaPerMass{m2_per_kg: y}), Distance{m: x/y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &InverseAbsorbedDose{per_Gy: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&VolumePerMass{m3_per_kg: x}, &InverseDoseEquivalent{per_Sv: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Distance{m: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InverseDistance{per_m: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &InverseMass{per_kg: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InverseTemperature{per_K: y}), InverseSpecificHeatCapacity{kgK_per_J: x/y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &Mass{kg: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Temperature{K: y}), InverseSpecificHeatCapacity{kgK_per_J: x*y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InverseSpecificHeatCapacity{kgK_per_J: y}), InverseTemperature{per_K: x/y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &SpecificHeatCapacity{J_per_kgK: y}), InverseTemperature{per_K: x*y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Acceleration{mps2: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &Density{kgpm3: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Energy{J: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Torque{Nm: y}), Mass{kg: x*y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InverseAcceleration{s2pm: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InverseEnergy{per_J: y}), Mass{kg: x/y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InverseTorque{per_Nm: y}), Mass{kg: x/y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &InversePressure{per_Pa: y}), Density{kgpm3: x/y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Pressure{Pa: y}), Density{kgpm3: x*y});
		assert_eq!(div_check(&InverseAbsorbedDose{per_Gy: x}, &TimePerDistance{spm: y}), TimePerDistance{spm: x/y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &Velocity{mps: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InverseAbsorbedDose{per_Gy: x}, &VolumePerMass{m3_per_kg: y}), InversePressure{per_Pa: x*y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Distance{m: y}), InverseAcceleration{s2pm: x*y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InverseDistance{per_m: y}), InverseAcceleration{s2pm: x/y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &InverseMass{per_kg: y}), InverseEnergy{per_J: x*y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InverseTemperature{per_K: y}), InverseSpecificHeatCapacity{kgK_per_J: x/y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &Mass{kg: y}), InverseEnergy{per_J: x/y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Temperature{K: y}), InverseSpecificHeatCapacity{kgK_per_J: x*y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InverseSpecificHeatCapacity{kgK_per_J: y}), InverseTemperature{per_K: x/y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &SpecificHeatCapacity{J_per_kgK: y}), InverseTemperature{per_K: x*y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Acceleration{mps2: y}), InverseDistance{per_m: x*y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &Density{kgpm3: y}), InversePressure{per_Pa: x/y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Energy{J: y}), Mass{kg: x*y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Torque{Nm: y}), Mass{kg: x*y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InverseAcceleration{s2pm: y}), InverseDistance{per_m: x/y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InverseEnergy{per_J: y}), Mass{kg: x/y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InverseTorque{per_Nm: y}), Mass{kg: x/y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &InversePressure{per_Pa: y}), Density{kgpm3: x/y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Pressure{Pa: y}), Density{kgpm3: x*y});
		assert_eq!(div_check(&InverseDoseEquivalent{per_Sv: x}, &TimePerDistance{spm: y}), TimePerDistance{spm: x/y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &Velocity{mps: y}), TimePerDistance{spm: x*y});
		assert_eq!(mul_check(&InverseDoseEquivalent{per_Sv: x}, &VolumePerMass{m3_per_kg: y}), InversePressure{per_Pa: x*y});
		assert_eq!(div_check(&(x as f64), &Amount{mol: y as f64}), InverseAmount{per_mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Amount{mol: y as f32}), InverseAmount{per_mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Amount{mol: y as i64}), InverseAmount{per_mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Amount{mol: y as i32}), InverseAmount{per_mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Current{A: y as f64}), InverseCurrent{per_A: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Current{A: y as f32}), InverseCurrent{per_A: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Current{A: y as i64}), InverseCurrent{per_A: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Current{A: y as i32}), InverseCurrent{per_A: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Distance{m: y as f64}), InverseDistance{per_m: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Distance{m: y as f32}), InverseDistance{per_m: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Distance{m: y as i64}), InverseDistance{per_m: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Distance{m: y as i32}), InverseDistance{per_m: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAmount{per_mol: y as f64}), Amount{mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAmount{per_mol: y as f32}), Amount{mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAmount{per_mol: y as i64}), Amount{mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAmount{per_mol: y as i32}), Amount{mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseCurrent{per_A: y as f64}), Current{A: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseCurrent{per_A: y as f32}), Current{A: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseCurrent{per_A: y as i64}), Current{A: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseCurrent{per_A: y as i32}), Current{A: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseDistance{per_m: y as f64}), Distance{m: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseDistance{per_m: y as f32}), Distance{m: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseDistance{per_m: y as i64}), Distance{m: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseDistance{per_m: y as i32}), Distance{m: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseLuminosity{per_cd: y as f64}), Luminosity{cd: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseLuminosity{per_cd: y as f32}), Luminosity{cd: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseLuminosity{per_cd: y as i64}), Luminosity{cd: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseLuminosity{per_cd: y as i32}), Luminosity{cd: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMass{per_kg: y as f64}), Mass{kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMass{per_kg: y as f32}), Mass{kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMass{per_kg: y as i64}), Mass{kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMass{per_kg: y as i32}), Mass{kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseTemperature{per_K: y as f64}), Temperature{K: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseTemperature{per_K: y as f32}), Temperature{K: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseTemperature{per_K: y as i64}), Temperature{K: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseTemperature{per_K: y as i32}), Temperature{K: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Luminosity{cd: y as f64}), InverseLuminosity{per_cd: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Luminosity{cd: y as f32}), InverseLuminosity{per_cd: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Luminosity{cd: y as i64}), InverseLuminosity{per_cd: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Luminosity{cd: y as i32}), InverseLuminosity{per_cd: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Mass{kg: y as f64}), InverseMass{per_kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Mass{kg: y as f32}), InverseMass{per_kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Mass{kg: y as i64}), InverseMass{per_kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Mass{kg: y as i32}), InverseMass{per_kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Temperature{K: y as f64}), InverseTemperature{per_K: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Temperature{K: y as f32}), InverseTemperature{per_K: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Temperature{K: y as i64}), InverseTemperature{per_K: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Temperature{K: y as i32}), InverseTemperature{per_K: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Amount{mol: y as f64}), InverseAmount{per_mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Amount{mol: y as f32}), InverseAmount{per_mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Amount{mol: y as i64}), InverseAmount{per_mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Amount{mol: y as i32}), InverseAmount{per_mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Current{A: y as f64}), InverseCurrent{per_A: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Current{A: y as f32}), InverseCurrent{per_A: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Current{A: y as i64}), InverseCurrent{per_A: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Current{A: y as i32}), InverseCurrent{per_A: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Distance{m: y as f64}), InverseDistance{per_m: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Distance{m: y as f32}), InverseDistance{per_m: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Distance{m: y as i64}), InverseDistance{per_m: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Distance{m: y as i32}), InverseDistance{per_m: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAmount{per_mol: y as f64}), Amount{mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAmount{per_mol: y as f32}), Amount{mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAmount{per_mol: y as i64}), Amount{mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAmount{per_mol: y as i32}), Amount{mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseCurrent{per_A: y as f64}), Current{A: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseCurrent{per_A: y as f32}), Current{A: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseCurrent{per_A: y as i64}), Current{A: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseCurrent{per_A: y as i32}), Current{A: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseDistance{per_m: y as f64}), Distance{m: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseDistance{per_m: y as f32}), Distance{m: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseDistance{per_m: y as i64}), Distance{m: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseDistance{per_m: y as i32}), Distance{m: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseLuminosity{per_cd: y as f64}), Luminosity{cd: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseLuminosity{per_cd: y as f32}), Luminosity{cd: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseLuminosity{per_cd: y as i64}), Luminosity{cd: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseLuminosity{per_cd: y as i32}), Luminosity{cd: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMass{per_kg: y as f64}), Mass{kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMass{per_kg: y as f32}), Mass{kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMass{per_kg: y as i64}), Mass{kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMass{per_kg: y as i32}), Mass{kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseTemperature{per_K: y as f64}), Temperature{K: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseTemperature{per_K: y as f32}), Temperature{K: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseTemperature{per_K: y as i64}), Temperature{K: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseTemperature{per_K: y as i32}), Temperature{K: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Luminosity{cd: y as f64}), InverseLuminosity{per_cd: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Luminosity{cd: y as f32}), InverseLuminosity{per_cd: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Luminosity{cd: y as i64}), InverseLuminosity{per_cd: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Luminosity{cd: y as i32}), InverseLuminosity{per_cd: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Mass{kg: y as f64}), InverseMass{per_kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Mass{kg: y as f32}), InverseMass{per_kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Mass{kg: y as i64}), InverseMass{per_kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Mass{kg: y as i32}), InverseMass{per_kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Temperature{K: y as f64}), InverseTemperature{per_K: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Temperature{K: y as f32}), InverseTemperature{per_K: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Temperature{K: y as i64}), InverseTemperature{per_K: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Temperature{K: y as i32}), InverseTemperature{per_K: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &CatalyticActivity{molps: y as f64}), InverseCatalyticActivity{s_per_mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &CatalyticActivity{molps: y as f32}), InverseCatalyticActivity{s_per_mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &CatalyticActivity{molps: y as i64}), InverseCatalyticActivity{s_per_mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &CatalyticActivity{molps: y as i32}), InverseCatalyticActivity{s_per_mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Concentration{molpm3: y as f64}), MolarVolume{m3_per_mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Concentration{molpm3: y as f32}), MolarVolume{m3_per_mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Concentration{molpm3: y as i64}), MolarVolume{m3_per_mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Concentration{molpm3: y as i32}), MolarVolume{m3_per_mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseCatalyticActivity{s_per_mol: y as f64}), CatalyticActivity{molps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseCatalyticActivity{s_per_mol: y as f32}), CatalyticActivity{molps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseCatalyticActivity{s_per_mol: y as i64}), CatalyticActivity{molps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseCatalyticActivity{s_per_mol: y as i32}), CatalyticActivity{molps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseSpecificHeatCapacity{kgK_per_J: y as f64}), SpecificHeatCapacity{J_per_kgK: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseSpecificHeatCapacity{kgK_per_J: y as f32}), SpecificHeatCapacity{J_per_kgK: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseSpecificHeatCapacity{kgK_per_J: y as i64}), SpecificHeatCapacity{J_per_kgK: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseSpecificHeatCapacity{kgK_per_J: y as i32}), SpecificHeatCapacity{J_per_kgK: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MolarVolume{m3_per_mol: y as f64}), Concentration{molpm3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MolarVolume{m3_per_mol: y as f32}), Concentration{molpm3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MolarVolume{m3_per_mol: y as i64}), Concentration{molpm3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MolarVolume{m3_per_mol: y as i32}), Concentration{molpm3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &SpecificHeatCapacity{J_per_kgK: y as f64}), InverseSpecificHeatCapacity{kgK_per_J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &SpecificHeatCapacity{J_per_kgK: y as f32}), InverseSpecificHeatCapacity{kgK_per_J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &SpecificHeatCapacity{J_per_kgK: y as i64}), InverseSpecificHeatCapacity{kgK_per_J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &SpecificHeatCapacity{J_per_kgK: y as i32}), InverseSpecificHeatCapacity{kgK_per_J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &CatalyticActivity{molps: y as f64}), InverseCatalyticActivity{s_per_mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &CatalyticActivity{molps: y as f32}), InverseCatalyticActivity{s_per_mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &CatalyticActivity{molps: y as i64}), InverseCatalyticActivity{s_per_mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &CatalyticActivity{molps: y as i32}), InverseCatalyticActivity{s_per_mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Concentration{molpm3: y as f64}), MolarVolume{m3_per_mol: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Concentration{molpm3: y as f32}), MolarVolume{m3_per_mol: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Concentration{molpm3: y as i64}), MolarVolume{m3_per_mol: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Concentration{molpm3: y as i32}), MolarVolume{m3_per_mol: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseCatalyticActivity{s_per_mol: y as f64}), CatalyticActivity{molps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseCatalyticActivity{s_per_mol: y as f32}), CatalyticActivity{molps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseCatalyticActivity{s_per_mol: y as i64}), CatalyticActivity{molps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseCatalyticActivity{s_per_mol: y as i32}), CatalyticActivity{molps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseSpecificHeatCapacity{kgK_per_J: y as f64}), SpecificHeatCapacity{J_per_kgK: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseSpecificHeatCapacity{kgK_per_J: y as f32}), SpecificHeatCapacity{J_per_kgK: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseSpecificHeatCapacity{kgK_per_J: y as i64}), SpecificHeatCapacity{J_per_kgK: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseSpecificHeatCapacity{kgK_per_J: y as i32}), SpecificHeatCapacity{J_per_kgK: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MolarVolume{m3_per_mol: y as f64}), Concentration{molpm3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MolarVolume{m3_per_mol: y as f32}), Concentration{molpm3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MolarVolume{m3_per_mol: y as i64}), Concentration{molpm3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MolarVolume{m3_per_mol: y as i32}), Concentration{molpm3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &SpecificHeatCapacity{J_per_kgK: y as f64}), InverseSpecificHeatCapacity{kgK_per_J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &SpecificHeatCapacity{J_per_kgK: y as f32}), InverseSpecificHeatCapacity{kgK_per_J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &SpecificHeatCapacity{J_per_kgK: y as i64}), InverseSpecificHeatCapacity{kgK_per_J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &SpecificHeatCapacity{J_per_kgK: y as i32}), InverseSpecificHeatCapacity{kgK_per_J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AreaPerLumen{m2_per_lm: y as f64}), Illuminance{lux: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AreaPerLumen{m2_per_lm: y as f32}), Illuminance{lux: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AreaPerLumen{m2_per_lm: y as i64}), Illuminance{lux: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AreaPerLumen{m2_per_lm: y as i32}), Illuminance{lux: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Capacitance{F: y as f64}), Elastance{per_F: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Capacitance{F: y as f32}), Elastance{per_F: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Capacitance{F: y as i64}), Elastance{per_F: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Capacitance{F: y as i32}), Elastance{per_F: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Charge{C: y as f64}), InverseCharge{per_C: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Charge{C: y as f32}), InverseCharge{per_C: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Charge{C: y as i64}), InverseCharge{per_C: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Charge{C: y as i32}), InverseCharge{per_C: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Elastance{per_F: y as f64}), Capacitance{F: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Elastance{per_F: y as f32}), Capacitance{F: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Elastance{per_F: y as i64}), Capacitance{F: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Elastance{per_F: y as i32}), Capacitance{F: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Illuminance{lux: y as f64}), AreaPerLumen{m2_per_lm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Illuminance{lux: y as f32}), AreaPerLumen{m2_per_lm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Illuminance{lux: y as i64}), AreaPerLumen{m2_per_lm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Illuminance{lux: y as i32}), AreaPerLumen{m2_per_lm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Inductance{H: y as f64}), InverseInductance{per_H: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Inductance{H: y as f32}), InverseInductance{per_H: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Inductance{H: y as i64}), InverseInductance{per_H: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Inductance{H: y as i32}), InverseInductance{per_H: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseCharge{per_C: y as f64}), Charge{C: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseCharge{per_C: y as f32}), Charge{C: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseCharge{per_C: y as i64}), Charge{C: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseCharge{per_C: y as i32}), Charge{C: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseInductance{per_H: y as f64}), Inductance{H: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseInductance{per_H: y as f32}), Inductance{H: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseInductance{per_H: y as i64}), Inductance{H: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseInductance{per_H: y as i32}), Inductance{H: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseLuminousFlux{per_lm: y as f64}), LuminousFlux{lm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseLuminousFlux{per_lm: y as f32}), LuminousFlux{lm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseLuminousFlux{per_lm: y as i64}), LuminousFlux{lm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseLuminousFlux{per_lm: y as i32}), LuminousFlux{lm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMagneticFlux{per_Wb: y as f64}), MagneticFlux{Wb: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMagneticFlux{per_Wb: y as f32}), MagneticFlux{Wb: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMagneticFlux{per_Wb: y as i64}), MagneticFlux{Wb: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMagneticFlux{per_Wb: y as i32}), MagneticFlux{Wb: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMagneticFluxDensity{m2_per_Wb: y as f64}), MagneticFluxDensity{T: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMagneticFluxDensity{m2_per_Wb: y as f32}), MagneticFluxDensity{T: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMagneticFluxDensity{m2_per_Wb: y as i64}), MagneticFluxDensity{T: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMagneticFluxDensity{m2_per_Wb: y as i32}), MagneticFluxDensity{T: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseVoltage{per_V: y as f64}), Voltage{V: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseVoltage{per_V: y as f32}), Voltage{V: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseVoltage{per_V: y as i64}), Voltage{V: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseVoltage{per_V: y as i32}), Voltage{V: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &LuminousFlux{lm: y as f64}), InverseLuminousFlux{per_lm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &LuminousFlux{lm: y as f32}), InverseLuminousFlux{per_lm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &LuminousFlux{lm: y as i64}), InverseLuminousFlux{per_lm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &LuminousFlux{lm: y as i32}), InverseLuminousFlux{per_lm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MagneticFlux{Wb: y as f64}), InverseMagneticFlux{per_Wb: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MagneticFlux{Wb: y as f32}), InverseMagneticFlux{per_Wb: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MagneticFlux{Wb: y as i64}), InverseMagneticFlux{per_Wb: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MagneticFlux{Wb: y as i32}), InverseMagneticFlux{per_Wb: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MagneticFluxDensity{T: y as f64}), InverseMagneticFluxDensity{m2_per_Wb: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MagneticFluxDensity{T: y as f32}), InverseMagneticFluxDensity{m2_per_Wb: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MagneticFluxDensity{T: y as i64}), InverseMagneticFluxDensity{m2_per_Wb: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MagneticFluxDensity{T: y as i32}), InverseMagneticFluxDensity{m2_per_Wb: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Voltage{V: y as f64}), InverseVoltage{per_V: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Voltage{V: y as f32}), InverseVoltage{per_V: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Voltage{V: y as i64}), InverseVoltage{per_V: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Voltage{V: y as i32}), InverseVoltage{per_V: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AreaPerLumen{m2_per_lm: y as f64}), Illuminance{lux: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AreaPerLumen{m2_per_lm: y as f32}), Illuminance{lux: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AreaPerLumen{m2_per_lm: y as i64}), Illuminance{lux: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AreaPerLumen{m2_per_lm: y as i32}), Illuminance{lux: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Capacitance{F: y as f64}), Elastance{per_F: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Capacitance{F: y as f32}), Elastance{per_F: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Capacitance{F: y as i64}), Elastance{per_F: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Capacitance{F: y as i32}), Elastance{per_F: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Charge{C: y as f64}), InverseCharge{per_C: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Charge{C: y as f32}), InverseCharge{per_C: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Charge{C: y as i64}), InverseCharge{per_C: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Charge{C: y as i32}), InverseCharge{per_C: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Elastance{per_F: y as f64}), Capacitance{F: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Elastance{per_F: y as f32}), Capacitance{F: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Elastance{per_F: y as i64}), Capacitance{F: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Elastance{per_F: y as i32}), Capacitance{F: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Illuminance{lux: y as f64}), AreaPerLumen{m2_per_lm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Illuminance{lux: y as f32}), AreaPerLumen{m2_per_lm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Illuminance{lux: y as i64}), AreaPerLumen{m2_per_lm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Illuminance{lux: y as i32}), AreaPerLumen{m2_per_lm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Inductance{H: y as f64}), InverseInductance{per_H: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Inductance{H: y as f32}), InverseInductance{per_H: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Inductance{H: y as i64}), InverseInductance{per_H: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Inductance{H: y as i32}), InverseInductance{per_H: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseCharge{per_C: y as f64}), Charge{C: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseCharge{per_C: y as f32}), Charge{C: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseCharge{per_C: y as i64}), Charge{C: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseCharge{per_C: y as i32}), Charge{C: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseInductance{per_H: y as f64}), Inductance{H: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseInductance{per_H: y as f32}), Inductance{H: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseInductance{per_H: y as i64}), Inductance{H: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseInductance{per_H: y as i32}), Inductance{H: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseLuminousFlux{per_lm: y as f64}), LuminousFlux{lm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseLuminousFlux{per_lm: y as f32}), LuminousFlux{lm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseLuminousFlux{per_lm: y as i64}), LuminousFlux{lm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseLuminousFlux{per_lm: y as i32}), LuminousFlux{lm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMagneticFlux{per_Wb: y as f64}), MagneticFlux{Wb: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMagneticFlux{per_Wb: y as f32}), MagneticFlux{Wb: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMagneticFlux{per_Wb: y as i64}), MagneticFlux{Wb: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMagneticFlux{per_Wb: y as i32}), MagneticFlux{Wb: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMagneticFluxDensity{m2_per_Wb: y as f64}), MagneticFluxDensity{T: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMagneticFluxDensity{m2_per_Wb: y as f32}), MagneticFluxDensity{T: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMagneticFluxDensity{m2_per_Wb: y as i64}), MagneticFluxDensity{T: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMagneticFluxDensity{m2_per_Wb: y as i32}), MagneticFluxDensity{T: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseVoltage{per_V: y as f64}), Voltage{V: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseVoltage{per_V: y as f32}), Voltage{V: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseVoltage{per_V: y as i64}), Voltage{V: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseVoltage{per_V: y as i32}), Voltage{V: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &LuminousFlux{lm: y as f64}), InverseLuminousFlux{per_lm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &LuminousFlux{lm: y as f32}), InverseLuminousFlux{per_lm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &LuminousFlux{lm: y as i64}), InverseLuminousFlux{per_lm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &LuminousFlux{lm: y as i32}), InverseLuminousFlux{per_lm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MagneticFlux{Wb: y as f64}), InverseMagneticFlux{per_Wb: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MagneticFlux{Wb: y as f32}), InverseMagneticFlux{per_Wb: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MagneticFlux{Wb: y as i64}), InverseMagneticFlux{per_Wb: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MagneticFlux{Wb: y as i32}), InverseMagneticFlux{per_Wb: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &MagneticFluxDensity{T: y as f64}), InverseMagneticFluxDensity{m2_per_Wb: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &MagneticFluxDensity{T: y as f32}), InverseMagneticFluxDensity{m2_per_Wb: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &MagneticFluxDensity{T: y as i64}), InverseMagneticFluxDensity{m2_per_Wb: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &MagneticFluxDensity{T: y as i32}), InverseMagneticFluxDensity{m2_per_Wb: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Voltage{V: y as f64}), InverseVoltage{per_V: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Voltage{V: y as f32}), InverseVoltage{per_V: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Voltage{V: y as i64}), InverseVoltage{per_V: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Voltage{V: y as i32}), InverseVoltage{per_V: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Angle{rad: y as f64}), InverseAngle{per_rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Angle{rad: y as f32}), InverseAngle{per_rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Angle{rad: y as i64}), InverseAngle{per_rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Angle{rad: y as i32}), InverseAngle{per_rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Area{m2: y as f64}), InverseArea{per_m2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Area{m2: y as f32}), InverseArea{per_m2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Area{m2: y as i64}), InverseArea{per_m2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Area{m2: y as i32}), InverseArea{per_m2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngle{per_rad: y as f64}), Angle{rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngle{per_rad: y as f32}), Angle{rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngle{per_rad: y as i64}), Angle{rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngle{per_rad: y as i32}), Angle{rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseArea{per_m2: y as f64}), Area{m2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseArea{per_m2: y as f32}), Area{m2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseArea{per_m2: y as i64}), Area{m2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseArea{per_m2: y as i32}), Area{m2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseSolidAngle{per_sr: y as f64}), SolidAngle{sr: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseSolidAngle{per_sr: y as f32}), SolidAngle{sr: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseSolidAngle{per_sr: y as i64}), SolidAngle{sr: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseSolidAngle{per_sr: y as i32}), SolidAngle{sr: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseVolume{per_m3: y as f64}), Volume{m3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseVolume{per_m3: y as f32}), Volume{m3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseVolume{per_m3: y as i64}), Volume{m3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseVolume{per_m3: y as i32}), Volume{m3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &SolidAngle{sr: y as f64}), InverseSolidAngle{per_sr: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &SolidAngle{sr: y as f32}), InverseSolidAngle{per_sr: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &SolidAngle{sr: y as i64}), InverseSolidAngle{per_sr: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &SolidAngle{sr: y as i32}), InverseSolidAngle{per_sr: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Volume{m3: y as f64}), InverseVolume{per_m3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Volume{m3: y as f32}), InverseVolume{per_m3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Volume{m3: y as i64}), InverseVolume{per_m3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Volume{m3: y as i32}), InverseVolume{per_m3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Angle{rad: y as f64}), InverseAngle{per_rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Angle{rad: y as f32}), InverseAngle{per_rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Angle{rad: y as i64}), InverseAngle{per_rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Angle{rad: y as i32}), InverseAngle{per_rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Area{m2: y as f64}), InverseArea{per_m2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Area{m2: y as f32}), InverseArea{per_m2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Area{m2: y as i64}), InverseArea{per_m2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Area{m2: y as i32}), InverseArea{per_m2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngle{per_rad: y as f64}), Angle{rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngle{per_rad: y as f32}), Angle{rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngle{per_rad: y as i64}), Angle{rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngle{per_rad: y as i32}), Angle{rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseArea{per_m2: y as f64}), Area{m2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseArea{per_m2: y as f32}), Area{m2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseArea{per_m2: y as i64}), Area{m2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseArea{per_m2: y as i32}), Area{m2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseSolidAngle{per_sr: y as f64}), SolidAngle{sr: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseSolidAngle{per_sr: y as f32}), SolidAngle{sr: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseSolidAngle{per_sr: y as i64}), SolidAngle{sr: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseSolidAngle{per_sr: y as i32}), SolidAngle{sr: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseVolume{per_m3: y as f64}), Volume{m3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseVolume{per_m3: y as f32}), Volume{m3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseVolume{per_m3: y as i64}), Volume{m3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseVolume{per_m3: y as i32}), Volume{m3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &SolidAngle{sr: y as f64}), InverseSolidAngle{per_sr: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &SolidAngle{sr: y as f32}), InverseSolidAngle{per_sr: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &SolidAngle{sr: y as i64}), InverseSolidAngle{per_sr: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &SolidAngle{sr: y as i32}), InverseSolidAngle{per_sr: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Volume{m3: y as f64}), InverseVolume{per_m3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Volume{m3: y as f32}), InverseVolume{per_m3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Volume{m3: y as i64}), InverseVolume{per_m3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Volume{m3: y as i32}), InverseVolume{per_m3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Acceleration{mps2: y as f64}), InverseAcceleration{s2pm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Acceleration{mps2: y as f32}), InverseAcceleration{s2pm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Acceleration{mps2: y as i64}), InverseAcceleration{s2pm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Acceleration{mps2: y as i32}), InverseAcceleration{s2pm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AngularAcceleration{radps2: y as f64}), InverseAngularAcceleration{s2prad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AngularAcceleration{radps2: y as f32}), InverseAngularAcceleration{s2prad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AngularAcceleration{radps2: y as i64}), InverseAngularAcceleration{s2prad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AngularAcceleration{radps2: y as i32}), InverseAngularAcceleration{s2prad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AngularMomentum{kgm2radps: y as f64}), InverseAngularMomentum{s_per_kgm2rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AngularMomentum{kgm2radps: y as f32}), InverseAngularMomentum{s_per_kgm2rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AngularMomentum{kgm2radps: y as i64}), InverseAngularMomentum{s_per_kgm2rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AngularMomentum{kgm2radps: y as i32}), InverseAngularMomentum{s_per_kgm2rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AngularVelocity{radps: y as f64}), InverseAngularVelocity{s_per_rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AngularVelocity{radps: y as f32}), InverseAngularVelocity{s_per_rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AngularVelocity{radps: y as i64}), InverseAngularVelocity{s_per_rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AngularVelocity{radps: y as i32}), InverseAngularVelocity{s_per_rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AreaDensity{kgpm2: y as f64}), AreaPerMass{m2_per_kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AreaDensity{kgpm2: y as f32}), AreaPerMass{m2_per_kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AreaDensity{kgpm2: y as i64}), AreaPerMass{m2_per_kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AreaDensity{kgpm2: y as i32}), AreaPerMass{m2_per_kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AreaPerMass{m2_per_kg: y as f64}), AreaDensity{kgpm2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AreaPerMass{m2_per_kg: y as f32}), AreaDensity{kgpm2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AreaPerMass{m2_per_kg: y as i64}), AreaDensity{kgpm2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AreaPerMass{m2_per_kg: y as i32}), AreaDensity{kgpm2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Density{kgpm3: y as f64}), VolumePerMass{m3_per_kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Density{kgpm3: y as f32}), VolumePerMass{m3_per_kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Density{kgpm3: y as i64}), VolumePerMass{m3_per_kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Density{kgpm3: y as i32}), VolumePerMass{m3_per_kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Energy{J: y as f64}), InverseEnergy{per_J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Energy{J: y as f32}), InverseEnergy{per_J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Energy{J: y as i64}), InverseEnergy{per_J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Energy{J: y as i32}), InverseEnergy{per_J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Force{N: y as f64}), InverseForce{per_N: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Force{N: y as f32}), InverseForce{per_N: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Force{N: y as i64}), InverseForce{per_N: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Force{N: y as i32}), InverseForce{per_N: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAcceleration{s2pm: y as f64}), Acceleration{mps2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAcceleration{s2pm: y as f32}), Acceleration{mps2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAcceleration{s2pm: y as i64}), Acceleration{mps2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAcceleration{s2pm: y as i32}), Acceleration{mps2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngularAcceleration{s2prad: y as f64}), AngularAcceleration{radps2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngularAcceleration{s2prad: y as f32}), AngularAcceleration{radps2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngularAcceleration{s2prad: y as i64}), AngularAcceleration{radps2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngularAcceleration{s2prad: y as i32}), AngularAcceleration{radps2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngularMomentum{s_per_kgm2rad: y as f64}), AngularMomentum{kgm2radps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngularMomentum{s_per_kgm2rad: y as f32}), AngularMomentum{kgm2radps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngularMomentum{s_per_kgm2rad: y as i64}), AngularMomentum{kgm2radps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngularMomentum{s_per_kgm2rad: y as i32}), AngularMomentum{kgm2radps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngularVelocity{s_per_rad: y as f64}), AngularVelocity{radps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngularVelocity{s_per_rad: y as f32}), AngularVelocity{radps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngularVelocity{s_per_rad: y as i64}), AngularVelocity{radps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngularVelocity{s_per_rad: y as i32}), AngularVelocity{radps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseEnergy{per_J: y as f64}), Energy{J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseEnergy{per_J: y as f32}), Energy{J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseEnergy{per_J: y as i64}), Energy{J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseEnergy{per_J: y as i32}), Energy{J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseForce{per_N: y as f64}), Force{N: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseForce{per_N: y as f32}), Force{N: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseForce{per_N: y as i64}), Force{N: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseForce{per_N: y as i32}), Force{N: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMomentum{s_per_kgm: y as f64}), Momentum{kgmps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMomentum{s_per_kgm: y as f32}), Momentum{kgmps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMomentum{s_per_kgm: y as i64}), Momentum{kgmps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMomentum{s_per_kgm: y as i32}), Momentum{kgmps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InversePower{per_W: y as f64}), Power{W: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InversePower{per_W: y as f32}), Power{W: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InversePower{per_W: y as i64}), Power{W: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InversePower{per_W: y as i32}), Power{W: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InversePressure{per_Pa: y as f64}), Pressure{Pa: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InversePressure{per_Pa: y as f32}), Pressure{Pa: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InversePressure{per_Pa: y as i64}), Pressure{Pa: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InversePressure{per_Pa: y as i32}), Pressure{Pa: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseTorque{per_Nm: y as f64}), Energy{J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseTorque{per_Nm: y as f32}), Energy{J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseTorque{per_Nm: y as i64}), Energy{J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseTorque{per_Nm: y as i32}), Energy{J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Momentum{kgmps: y as f64}), InverseMomentum{s_per_kgm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Momentum{kgmps: y as f32}), InverseMomentum{s_per_kgm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Momentum{kgmps: y as i64}), InverseMomentum{s_per_kgm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Momentum{kgmps: y as i32}), InverseMomentum{s_per_kgm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Power{W: y as f64}), InversePower{per_W: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Power{W: y as f32}), InversePower{per_W: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Power{W: y as i64}), InversePower{per_W: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Power{W: y as i32}), InversePower{per_W: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Pressure{Pa: y as f64}), InversePressure{per_Pa: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Pressure{Pa: y as f32}), InversePressure{per_Pa: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Pressure{Pa: y as i64}), InversePressure{per_Pa: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Pressure{Pa: y as i32}), InversePressure{per_Pa: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &TimePerDistance{spm: y as f64}), Velocity{mps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &TimePerDistance{spm: y as f32}), Velocity{mps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &TimePerDistance{spm: y as i64}), Velocity{mps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &TimePerDistance{spm: y as i32}), Velocity{mps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Torque{Nm: y as f64}), InverseEnergy{per_J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Torque{Nm: y as f32}), InverseEnergy{per_J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Torque{Nm: y as i64}), InverseEnergy{per_J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Torque{Nm: y as i32}), InverseEnergy{per_J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Velocity{mps: y as f64}), TimePerDistance{spm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Velocity{mps: y as f32}), TimePerDistance{spm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Velocity{mps: y as i64}), TimePerDistance{spm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Velocity{mps: y as i32}), TimePerDistance{spm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &VolumePerMass{m3_per_kg: y as f64}), Density{kgpm3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &VolumePerMass{m3_per_kg: y as f32}), Density{kgpm3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &VolumePerMass{m3_per_kg: y as i64}), Density{kgpm3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &VolumePerMass{m3_per_kg: y as i32}), Density{kgpm3: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Acceleration{mps2: y as f64}), InverseAcceleration{s2pm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Acceleration{mps2: y as f32}), InverseAcceleration{s2pm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Acceleration{mps2: y as i64}), InverseAcceleration{s2pm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Acceleration{mps2: y as i32}), InverseAcceleration{s2pm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AngularAcceleration{radps2: y as f64}), InverseAngularAcceleration{s2prad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AngularAcceleration{radps2: y as f32}), InverseAngularAcceleration{s2prad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AngularAcceleration{radps2: y as i64}), InverseAngularAcceleration{s2prad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AngularAcceleration{radps2: y as i32}), InverseAngularAcceleration{s2prad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AngularMomentum{kgm2radps: y as f64}), InverseAngularMomentum{s_per_kgm2rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AngularMomentum{kgm2radps: y as f32}), InverseAngularMomentum{s_per_kgm2rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AngularMomentum{kgm2radps: y as i64}), InverseAngularMomentum{s_per_kgm2rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AngularMomentum{kgm2radps: y as i32}), InverseAngularMomentum{s_per_kgm2rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AngularVelocity{radps: y as f64}), InverseAngularVelocity{s_per_rad: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AngularVelocity{radps: y as f32}), InverseAngularVelocity{s_per_rad: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AngularVelocity{radps: y as i64}), InverseAngularVelocity{s_per_rad: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AngularVelocity{radps: y as i32}), InverseAngularVelocity{s_per_rad: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AreaDensity{kgpm2: y as f64}), AreaPerMass{m2_per_kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AreaDensity{kgpm2: y as f32}), AreaPerMass{m2_per_kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AreaDensity{kgpm2: y as i64}), AreaPerMass{m2_per_kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AreaDensity{kgpm2: y as i32}), AreaPerMass{m2_per_kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &AreaPerMass{m2_per_kg: y as f64}), AreaDensity{kgpm2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &AreaPerMass{m2_per_kg: y as f32}), AreaDensity{kgpm2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &AreaPerMass{m2_per_kg: y as i64}), AreaDensity{kgpm2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &AreaPerMass{m2_per_kg: y as i32}), AreaDensity{kgpm2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Density{kgpm3: y as f64}), VolumePerMass{m3_per_kg: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Density{kgpm3: y as f32}), VolumePerMass{m3_per_kg: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Density{kgpm3: y as i64}), VolumePerMass{m3_per_kg: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Density{kgpm3: y as i32}), VolumePerMass{m3_per_kg: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Energy{J: y as f64}), InverseEnergy{per_J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Energy{J: y as f32}), InverseEnergy{per_J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Energy{J: y as i64}), InverseEnergy{per_J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Energy{J: y as i32}), InverseEnergy{per_J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Force{N: y as f64}), InverseForce{per_N: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Force{N: y as f32}), InverseForce{per_N: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Force{N: y as i64}), InverseForce{per_N: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Force{N: y as i32}), InverseForce{per_N: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAcceleration{s2pm: y as f64}), Acceleration{mps2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAcceleration{s2pm: y as f32}), Acceleration{mps2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAcceleration{s2pm: y as i64}), Acceleration{mps2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAcceleration{s2pm: y as i32}), Acceleration{mps2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngularAcceleration{s2prad: y as f64}), AngularAcceleration{radps2: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngularAcceleration{s2prad: y as f32}), AngularAcceleration{radps2: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngularAcceleration{s2prad: y as i64}), AngularAcceleration{radps2: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngularAcceleration{s2prad: y as i32}), AngularAcceleration{radps2: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngularMomentum{s_per_kgm2rad: y as f64}), AngularMomentum{kgm2radps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngularMomentum{s_per_kgm2rad: y as f32}), AngularMomentum{kgm2radps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngularMomentum{s_per_kgm2rad: y as i64}), AngularMomentum{kgm2radps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngularMomentum{s_per_kgm2rad: y as i32}), AngularMomentum{kgm2radps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseAngularVelocity{s_per_rad: y as f64}), AngularVelocity{radps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseAngularVelocity{s_per_rad: y as f32}), AngularVelocity{radps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseAngularVelocity{s_per_rad: y as i64}), AngularVelocity{radps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseAngularVelocity{s_per_rad: y as i32}), AngularVelocity{radps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseEnergy{per_J: y as f64}), Energy{J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseEnergy{per_J: y as f32}), Energy{J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseEnergy{per_J: y as i64}), Energy{J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseEnergy{per_J: y as i32}), Energy{J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseForce{per_N: y as f64}), Force{N: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseForce{per_N: y as f32}), Force{N: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseForce{per_N: y as i64}), Force{N: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseForce{per_N: y as i32}), Force{N: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseMomentum{s_per_kgm: y as f64}), Momentum{kgmps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseMomentum{s_per_kgm: y as f32}), Momentum{kgmps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseMomentum{s_per_kgm: y as i64}), Momentum{kgmps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseMomentum{s_per_kgm: y as i32}), Momentum{kgmps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InversePower{per_W: y as f64}), Power{W: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InversePower{per_W: y as f32}), Power{W: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InversePower{per_W: y as i64}), Power{W: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InversePower{per_W: y as i32}), Power{W: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InversePressure{per_Pa: y as f64}), Pressure{Pa: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InversePressure{per_Pa: y as f32}), Pressure{Pa: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InversePressure{per_Pa: y as i64}), Pressure{Pa: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InversePressure{per_Pa: y as i32}), Pressure{Pa: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &InverseTorque{per_Nm: y as f64}), Energy{J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &InverseTorque{per_Nm: y as f32}), Energy{J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &InverseTorque{per_Nm: y as i64}), Energy{J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &InverseTorque{per_Nm: y as i32}), Energy{J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Momentum{kgmps: y as f64}), InverseMomentum{s_per_kgm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Momentum{kgmps: y as f32}), InverseMomentum{s_per_kgm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Momentum{kgmps: y as i64}), InverseMomentum{s_per_kgm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Momentum{kgmps: y as i32}), InverseMomentum{s_per_kgm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Power{W: y as f64}), InversePower{per_W: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Power{W: y as f32}), InversePower{per_W: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Power{W: y as i64}), InversePower{per_W: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Power{W: y as i32}), InversePower{per_W: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Pressure{Pa: y as f64}), InversePressure{per_Pa: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Pressure{Pa: y as f32}), InversePressure{per_Pa: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Pressure{Pa: y as i64}), InversePressure{per_Pa: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Pressure{Pa: y as i32}), InversePressure{per_Pa: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &TimePerDistance{spm: y as f64}), Velocity{mps: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &TimePerDistance{spm: y as f32}), Velocity{mps: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &TimePerDistance{spm: y as i64}), Velocity{mps: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &TimePerDistance{spm: y as i32}), Velocity{mps: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Torque{Nm: y as f64}), InverseEnergy{per_J: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Torque{Nm: y as f32}), InverseEnergy{per_J: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Torque{Nm: y as i64}), InverseEnergy{per_J: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Torque{Nm: y as i32}), InverseEnergy{per_J: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &Velocity{mps: y as f64}), TimePerDistance{spm: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &Velocity{mps: y as f32}), TimePerDistance{spm: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &Velocity{mps: y as i64}), TimePerDistance{spm: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &Velocity{mps: y as i32}), TimePerDistance{spm: x as i32/y as i32});
		assert_eq!(div_check(&(x as f64), &VolumePerMass{m3_per_kg: y as f64}), Density{kgpm3: x as f64/y as f64});
		assert_eq!(div_check(&(x as f32), &VolumePerMass{m3_per_kg: y as f32}), Density{kgpm3: x as f32/y as f32});
		assert_eq!(div_check(&(x as i64), &VolumePerMass{m3_per_kg: y as i64}), Density{kgpm3: x as i64/y as i64});
		assert_eq!(div_check(&(x as i32), &VolumePerMass{m3_per_kg: y as i32}), Density{kgpm3: x as i32/y as i32});
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
		assert_eq!(div_check(
			&BigFloat::from(x), &Amount{mol: BigFloat::from(y)}),
				   InverseAmount{per_mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Current{A: BigFloat::from(y)}),
				   InverseCurrent{per_A: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Distance{m: BigFloat::from(y)}),
				   InverseDistance{per_m: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAmount{per_mol: BigFloat::from(y)}),
				   Amount{mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseCurrent{per_A: BigFloat::from(y)}),
				   Current{A: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseDistance{per_m: BigFloat::from(y)}),
				   Distance{m: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseLuminosity{per_cd: BigFloat::from(y)}),
				   Luminosity{cd: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMass{per_kg: BigFloat::from(y)}),
				   Mass{kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseTemperature{per_K: BigFloat::from(y)}),
				   Temperature{K: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Luminosity{cd: BigFloat::from(y)}),
				   InverseLuminosity{per_cd: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Mass{kg: BigFloat::from(y)}),
				   InverseMass{per_kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Temperature{K: BigFloat::from(y)}),
				   InverseTemperature{per_K: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Amount{mol: BigFloat::from(y)}),
				   InverseAmount{per_mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Current{A: BigFloat::from(y)}),
				   InverseCurrent{per_A: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Distance{m: BigFloat::from(y)}),
				   InverseDistance{per_m: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAmount{per_mol: BigFloat::from(y)}),
				   Amount{mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseCurrent{per_A: BigFloat::from(y)}),
				   Current{A: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseDistance{per_m: BigFloat::from(y)}),
				   Distance{m: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseLuminosity{per_cd: BigFloat::from(y)}),
				   Luminosity{cd: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMass{per_kg: BigFloat::from(y)}),
				   Mass{kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseTemperature{per_K: BigFloat::from(y)}),
				   Temperature{K: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Luminosity{cd: BigFloat::from(y)}),
				   InverseLuminosity{per_cd: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Mass{kg: BigFloat::from(y)}),
				   InverseMass{per_kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Temperature{K: BigFloat::from(y)}),
				   InverseTemperature{per_K: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &CatalyticActivity{molps: BigFloat::from(y)}),
				   InverseCatalyticActivity{s_per_mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Concentration{molpm3: BigFloat::from(y)}),
				   MolarVolume{m3_per_mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseCatalyticActivity{s_per_mol: BigFloat::from(y)}),
				   CatalyticActivity{molps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseSpecificHeatCapacity{kgK_per_J: BigFloat::from(y)}),
				   SpecificHeatCapacity{J_per_kgK: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MolarVolume{m3_per_mol: BigFloat::from(y)}),
				   Concentration{molpm3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &SpecificHeatCapacity{J_per_kgK: BigFloat::from(y)}),
				   InverseSpecificHeatCapacity{kgK_per_J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &CatalyticActivity{molps: BigFloat::from(y)}),
				   InverseCatalyticActivity{s_per_mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Concentration{molpm3: BigFloat::from(y)}),
				   MolarVolume{m3_per_mol: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseCatalyticActivity{s_per_mol: BigFloat::from(y)}),
				   CatalyticActivity{molps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseSpecificHeatCapacity{kgK_per_J: BigFloat::from(y)}),
				   SpecificHeatCapacity{J_per_kgK: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MolarVolume{m3_per_mol: BigFloat::from(y)}),
				   Concentration{molpm3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &SpecificHeatCapacity{J_per_kgK: BigFloat::from(y)}),
				   InverseSpecificHeatCapacity{kgK_per_J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AreaPerLumen{m2_per_lm: BigFloat::from(y)}),
				   Illuminance{lux: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Capacitance{F: BigFloat::from(y)}),
				   Elastance{per_F: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Charge{C: BigFloat::from(y)}),
				   InverseCharge{per_C: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Elastance{per_F: BigFloat::from(y)}),
				   Capacitance{F: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Illuminance{lux: BigFloat::from(y)}),
				   AreaPerLumen{m2_per_lm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Inductance{H: BigFloat::from(y)}),
				   InverseInductance{per_H: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseCharge{per_C: BigFloat::from(y)}),
				   Charge{C: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseInductance{per_H: BigFloat::from(y)}),
				   Inductance{H: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseLuminousFlux{per_lm: BigFloat::from(y)}),
				   LuminousFlux{lm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMagneticFlux{per_Wb: BigFloat::from(y)}),
				   MagneticFlux{Wb: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMagneticFluxDensity{m2_per_Wb: BigFloat::from(y)}),
				   MagneticFluxDensity{T: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseVoltage{per_V: BigFloat::from(y)}),
				   Voltage{V: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &LuminousFlux{lm: BigFloat::from(y)}),
				   InverseLuminousFlux{per_lm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MagneticFlux{Wb: BigFloat::from(y)}),
				   InverseMagneticFlux{per_Wb: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MagneticFluxDensity{T: BigFloat::from(y)}),
				   InverseMagneticFluxDensity{m2_per_Wb: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Voltage{V: BigFloat::from(y)}),
				   InverseVoltage{per_V: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AreaPerLumen{m2_per_lm: BigFloat::from(y)}),
				   Illuminance{lux: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Capacitance{F: BigFloat::from(y)}),
				   Elastance{per_F: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Charge{C: BigFloat::from(y)}),
				   InverseCharge{per_C: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Elastance{per_F: BigFloat::from(y)}),
				   Capacitance{F: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Illuminance{lux: BigFloat::from(y)}),
				   AreaPerLumen{m2_per_lm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Inductance{H: BigFloat::from(y)}),
				   InverseInductance{per_H: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseCharge{per_C: BigFloat::from(y)}),
				   Charge{C: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseInductance{per_H: BigFloat::from(y)}),
				   Inductance{H: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseLuminousFlux{per_lm: BigFloat::from(y)}),
				   LuminousFlux{lm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMagneticFlux{per_Wb: BigFloat::from(y)}),
				   MagneticFlux{Wb: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMagneticFluxDensity{m2_per_Wb: BigFloat::from(y)}),
				   MagneticFluxDensity{T: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseVoltage{per_V: BigFloat::from(y)}),
				   Voltage{V: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &LuminousFlux{lm: BigFloat::from(y)}),
				   InverseLuminousFlux{per_lm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MagneticFlux{Wb: BigFloat::from(y)}),
				   InverseMagneticFlux{per_Wb: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &MagneticFluxDensity{T: BigFloat::from(y)}),
				   InverseMagneticFluxDensity{m2_per_Wb: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Voltage{V: BigFloat::from(y)}),
				   InverseVoltage{per_V: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Angle{rad: BigFloat::from(y)}),
				   InverseAngle{per_rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Area{m2: BigFloat::from(y)}),
				   InverseArea{per_m2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngle{per_rad: BigFloat::from(y)}),
				   Angle{rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseArea{per_m2: BigFloat::from(y)}),
				   Area{m2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseSolidAngle{per_sr: BigFloat::from(y)}),
				   SolidAngle{sr: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseVolume{per_m3: BigFloat::from(y)}),
				   Volume{m3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &SolidAngle{sr: BigFloat::from(y)}),
				   InverseSolidAngle{per_sr: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Volume{m3: BigFloat::from(y)}),
				   InverseVolume{per_m3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Angle{rad: BigFloat::from(y)}),
				   InverseAngle{per_rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Area{m2: BigFloat::from(y)}),
				   InverseArea{per_m2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngle{per_rad: BigFloat::from(y)}),
				   Angle{rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseArea{per_m2: BigFloat::from(y)}),
				   Area{m2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseSolidAngle{per_sr: BigFloat::from(y)}),
				   SolidAngle{sr: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseVolume{per_m3: BigFloat::from(y)}),
				   Volume{m3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &SolidAngle{sr: BigFloat::from(y)}),
				   InverseSolidAngle{per_sr: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Volume{m3: BigFloat::from(y)}),
				   InverseVolume{per_m3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Acceleration{mps2: BigFloat::from(y)}),
				   InverseAcceleration{s2pm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AngularAcceleration{radps2: BigFloat::from(y)}),
				   InverseAngularAcceleration{s2prad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AngularMomentum{kgm2radps: BigFloat::from(y)}),
				   InverseAngularMomentum{s_per_kgm2rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AngularVelocity{radps: BigFloat::from(y)}),
				   InverseAngularVelocity{s_per_rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AreaDensity{kgpm2: BigFloat::from(y)}),
				   AreaPerMass{m2_per_kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AreaPerMass{m2_per_kg: BigFloat::from(y)}),
				   AreaDensity{kgpm2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Density{kgpm3: BigFloat::from(y)}),
				   VolumePerMass{m3_per_kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Energy{J: BigFloat::from(y)}),
				   InverseEnergy{per_J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Force{N: BigFloat::from(y)}),
				   InverseForce{per_N: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAcceleration{s2pm: BigFloat::from(y)}),
				   Acceleration{mps2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngularAcceleration{s2prad: BigFloat::from(y)}),
				   AngularAcceleration{radps2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngularMomentum{s_per_kgm2rad: BigFloat::from(y)}),
				   AngularMomentum{kgm2radps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngularVelocity{s_per_rad: BigFloat::from(y)}),
				   AngularVelocity{radps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseEnergy{per_J: BigFloat::from(y)}),
				   Energy{J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseForce{per_N: BigFloat::from(y)}),
				   Force{N: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMomentum{s_per_kgm: BigFloat::from(y)}),
				   Momentum{kgmps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InversePower{per_W: BigFloat::from(y)}),
				   Power{W: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InversePressure{per_Pa: BigFloat::from(y)}),
				   Pressure{Pa: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseTorque{per_Nm: BigFloat::from(y)}),
				   Energy{J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Momentum{kgmps: BigFloat::from(y)}),
				   InverseMomentum{s_per_kgm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Power{W: BigFloat::from(y)}),
				   InversePower{per_W: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Pressure{Pa: BigFloat::from(y)}),
				   InversePressure{per_Pa: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &TimePerDistance{spm: BigFloat::from(y)}),
				   Velocity{mps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Torque{Nm: BigFloat::from(y)}),
				   InverseEnergy{per_J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Velocity{mps: BigFloat::from(y)}),
				   TimePerDistance{spm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &VolumePerMass{m3_per_kg: BigFloat::from(y)}),
				   Density{kgpm3: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Acceleration{mps2: BigFloat::from(y)}),
				   InverseAcceleration{s2pm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AngularAcceleration{radps2: BigFloat::from(y)}),
				   InverseAngularAcceleration{s2prad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AngularMomentum{kgm2radps: BigFloat::from(y)}),
				   InverseAngularMomentum{s_per_kgm2rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AngularVelocity{radps: BigFloat::from(y)}),
				   InverseAngularVelocity{s_per_rad: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AreaDensity{kgpm2: BigFloat::from(y)}),
				   AreaPerMass{m2_per_kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &AreaPerMass{m2_per_kg: BigFloat::from(y)}),
				   AreaDensity{kgpm2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Density{kgpm3: BigFloat::from(y)}),
				   VolumePerMass{m3_per_kg: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Energy{J: BigFloat::from(y)}),
				   InverseEnergy{per_J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Force{N: BigFloat::from(y)}),
				   InverseForce{per_N: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAcceleration{s2pm: BigFloat::from(y)}),
				   Acceleration{mps2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngularAcceleration{s2prad: BigFloat::from(y)}),
				   AngularAcceleration{radps2: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngularMomentum{s_per_kgm2rad: BigFloat::from(y)}),
				   AngularMomentum{kgm2radps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseAngularVelocity{s_per_rad: BigFloat::from(y)}),
				   AngularVelocity{radps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseEnergy{per_J: BigFloat::from(y)}),
				   Energy{J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseForce{per_N: BigFloat::from(y)}),
				   Force{N: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseMomentum{s_per_kgm: BigFloat::from(y)}),
				   Momentum{kgmps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InversePower{per_W: BigFloat::from(y)}),
				   Power{W: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InversePressure{per_Pa: BigFloat::from(y)}),
				   Pressure{Pa: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &InverseTorque{per_Nm: BigFloat::from(y)}),
				   Energy{J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Momentum{kgmps: BigFloat::from(y)}),
				   InverseMomentum{s_per_kgm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Power{W: BigFloat::from(y)}),
				   InversePower{per_W: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Pressure{Pa: BigFloat::from(y)}),
				   InversePressure{per_Pa: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &TimePerDistance{spm: BigFloat::from(y)}),
				   Velocity{mps: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Torque{Nm: BigFloat::from(y)}),
				   InverseEnergy{per_J: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &Velocity{mps: BigFloat::from(y)}),
				   TimePerDistance{spm: BigFloat::from(x)/BigFloat::from(y)}
		);
		assert_eq!(div_check(
			&BigFloat::from(x), &VolumePerMass{m3_per_kg: BigFloat::from(y)}),
				   Density{kgpm3: BigFloat::from(x)/BigFloat::from(y)}
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
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Amount{mol: Complex32::from(y as f32)}),
				   InverseAmount{per_mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Amount{mol: Complex64::from(y)}),
				   InverseAmount{per_mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Current{A: Complex32::from(y as f32)}),
				   InverseCurrent{per_A: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Current{A: Complex64::from(y)}),
				   InverseCurrent{per_A: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Distance{m: Complex32::from(y as f32)}),
				   InverseDistance{per_m: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Distance{m: Complex64::from(y)}),
				   InverseDistance{per_m: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAmount{per_mol: Complex32::from(y as f32)}),
				   Amount{mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAmount{per_mol: Complex64::from(y)}),
				   Amount{mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseCurrent{per_A: Complex32::from(y as f32)}),
				   Current{A: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseCurrent{per_A: Complex64::from(y)}),
				   Current{A: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseDistance{per_m: Complex32::from(y as f32)}),
				   Distance{m: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseDistance{per_m: Complex64::from(y)}),
				   Distance{m: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseLuminosity{per_cd: Complex32::from(y as f32)}),
				   Luminosity{cd: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseLuminosity{per_cd: Complex64::from(y)}),
				   Luminosity{cd: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMass{per_kg: Complex32::from(y as f32)}),
				   Mass{kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMass{per_kg: Complex64::from(y)}),
				   Mass{kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseTemperature{per_K: Complex32::from(y as f32)}),
				   Temperature{K: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseTemperature{per_K: Complex64::from(y)}),
				   Temperature{K: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Luminosity{cd: Complex32::from(y as f32)}),
				   InverseLuminosity{per_cd: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Luminosity{cd: Complex64::from(y)}),
				   InverseLuminosity{per_cd: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Mass{kg: Complex32::from(y as f32)}),
				   InverseMass{per_kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Mass{kg: Complex64::from(y)}),
				   InverseMass{per_kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Temperature{K: Complex32::from(y as f32)}),
				   InverseTemperature{per_K: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Temperature{K: Complex64::from(y)}),
				   InverseTemperature{per_K: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Amount{mol: Complex32::from(y as f32)}),
				   InverseAmount{per_mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Amount{mol: Complex64::from(y)}),
				   InverseAmount{per_mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Current{A: Complex32::from(y as f32)}),
				   InverseCurrent{per_A: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Current{A: Complex64::from(y)}),
				   InverseCurrent{per_A: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Distance{m: Complex32::from(y as f32)}),
				   InverseDistance{per_m: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Distance{m: Complex64::from(y)}),
				   InverseDistance{per_m: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAmount{per_mol: Complex32::from(y as f32)}),
				   Amount{mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAmount{per_mol: Complex64::from(y)}),
				   Amount{mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseCurrent{per_A: Complex32::from(y as f32)}),
				   Current{A: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseCurrent{per_A: Complex64::from(y)}),
				   Current{A: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseDistance{per_m: Complex32::from(y as f32)}),
				   Distance{m: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseDistance{per_m: Complex64::from(y)}),
				   Distance{m: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseLuminosity{per_cd: Complex32::from(y as f32)}),
				   Luminosity{cd: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseLuminosity{per_cd: Complex64::from(y)}),
				   Luminosity{cd: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMass{per_kg: Complex32::from(y as f32)}),
				   Mass{kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMass{per_kg: Complex64::from(y)}),
				   Mass{kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseTemperature{per_K: Complex32::from(y as f32)}),
				   Temperature{K: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseTemperature{per_K: Complex64::from(y)}),
				   Temperature{K: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Luminosity{cd: Complex32::from(y as f32)}),
				   InverseLuminosity{per_cd: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Luminosity{cd: Complex64::from(y)}),
				   InverseLuminosity{per_cd: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Mass{kg: Complex32::from(y as f32)}),
				   InverseMass{per_kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Mass{kg: Complex64::from(y)}),
				   InverseMass{per_kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Temperature{K: Complex32::from(y as f32)}),
				   InverseTemperature{per_K: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Temperature{K: Complex64::from(y)}),
				   InverseTemperature{per_K: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &CatalyticActivity{molps: Complex32::from(y as f32)}),
				   InverseCatalyticActivity{s_per_mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &CatalyticActivity{molps: Complex64::from(y)}),
				   InverseCatalyticActivity{s_per_mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Concentration{molpm3: Complex32::from(y as f32)}),
				   MolarVolume{m3_per_mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Concentration{molpm3: Complex64::from(y)}),
				   MolarVolume{m3_per_mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseCatalyticActivity{s_per_mol: Complex32::from(y as f32)}),
				   CatalyticActivity{molps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseCatalyticActivity{s_per_mol: Complex64::from(y)}),
				   CatalyticActivity{molps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseSpecificHeatCapacity{kgK_per_J: Complex32::from(y as f32)}),
				   SpecificHeatCapacity{J_per_kgK: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseSpecificHeatCapacity{kgK_per_J: Complex64::from(y)}),
				   SpecificHeatCapacity{J_per_kgK: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MolarVolume{m3_per_mol: Complex32::from(y as f32)}),
				   Concentration{molpm3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MolarVolume{m3_per_mol: Complex64::from(y)}),
				   Concentration{molpm3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &SpecificHeatCapacity{J_per_kgK: Complex32::from(y as f32)}),
				   InverseSpecificHeatCapacity{kgK_per_J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &SpecificHeatCapacity{J_per_kgK: Complex64::from(y)}),
				   InverseSpecificHeatCapacity{kgK_per_J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &CatalyticActivity{molps: Complex32::from(y as f32)}),
				   InverseCatalyticActivity{s_per_mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &CatalyticActivity{molps: Complex64::from(y)}),
				   InverseCatalyticActivity{s_per_mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Concentration{molpm3: Complex32::from(y as f32)}),
				   MolarVolume{m3_per_mol: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Concentration{molpm3: Complex64::from(y)}),
				   MolarVolume{m3_per_mol: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseCatalyticActivity{s_per_mol: Complex32::from(y as f32)}),
				   CatalyticActivity{molps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseCatalyticActivity{s_per_mol: Complex64::from(y)}),
				   CatalyticActivity{molps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseSpecificHeatCapacity{kgK_per_J: Complex32::from(y as f32)}),
				   SpecificHeatCapacity{J_per_kgK: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseSpecificHeatCapacity{kgK_per_J: Complex64::from(y)}),
				   SpecificHeatCapacity{J_per_kgK: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MolarVolume{m3_per_mol: Complex32::from(y as f32)}),
				   Concentration{molpm3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MolarVolume{m3_per_mol: Complex64::from(y)}),
				   Concentration{molpm3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &SpecificHeatCapacity{J_per_kgK: Complex32::from(y as f32)}),
				   InverseSpecificHeatCapacity{kgK_per_J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &SpecificHeatCapacity{J_per_kgK: Complex64::from(y)}),
				   InverseSpecificHeatCapacity{kgK_per_J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AreaPerLumen{m2_per_lm: Complex32::from(y as f32)}),
				   Illuminance{lux: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AreaPerLumen{m2_per_lm: Complex64::from(y)}),
				   Illuminance{lux: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Capacitance{F: Complex32::from(y as f32)}),
				   Elastance{per_F: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Capacitance{F: Complex64::from(y)}),
				   Elastance{per_F: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Charge{C: Complex32::from(y as f32)}),
				   InverseCharge{per_C: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Charge{C: Complex64::from(y)}),
				   InverseCharge{per_C: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Elastance{per_F: Complex32::from(y as f32)}),
				   Capacitance{F: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Elastance{per_F: Complex64::from(y)}),
				   Capacitance{F: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Illuminance{lux: Complex32::from(y as f32)}),
				   AreaPerLumen{m2_per_lm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Illuminance{lux: Complex64::from(y)}),
				   AreaPerLumen{m2_per_lm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Inductance{H: Complex32::from(y as f32)}),
				   InverseInductance{per_H: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Inductance{H: Complex64::from(y)}),
				   InverseInductance{per_H: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseCharge{per_C: Complex32::from(y as f32)}),
				   Charge{C: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseCharge{per_C: Complex64::from(y)}),
				   Charge{C: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseInductance{per_H: Complex32::from(y as f32)}),
				   Inductance{H: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseInductance{per_H: Complex64::from(y)}),
				   Inductance{H: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseLuminousFlux{per_lm: Complex32::from(y as f32)}),
				   LuminousFlux{lm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseLuminousFlux{per_lm: Complex64::from(y)}),
				   LuminousFlux{lm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMagneticFlux{per_Wb: Complex32::from(y as f32)}),
				   MagneticFlux{Wb: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMagneticFlux{per_Wb: Complex64::from(y)}),
				   MagneticFlux{Wb: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMagneticFluxDensity{m2_per_Wb: Complex32::from(y as f32)}),
				   MagneticFluxDensity{T: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMagneticFluxDensity{m2_per_Wb: Complex64::from(y)}),
				   MagneticFluxDensity{T: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseVoltage{per_V: Complex32::from(y as f32)}),
				   Voltage{V: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseVoltage{per_V: Complex64::from(y)}),
				   Voltage{V: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &LuminousFlux{lm: Complex32::from(y as f32)}),
				   InverseLuminousFlux{per_lm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &LuminousFlux{lm: Complex64::from(y)}),
				   InverseLuminousFlux{per_lm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MagneticFlux{Wb: Complex32::from(y as f32)}),
				   InverseMagneticFlux{per_Wb: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MagneticFlux{Wb: Complex64::from(y)}),
				   InverseMagneticFlux{per_Wb: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MagneticFluxDensity{T: Complex32::from(y as f32)}),
				   InverseMagneticFluxDensity{m2_per_Wb: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MagneticFluxDensity{T: Complex64::from(y)}),
				   InverseMagneticFluxDensity{m2_per_Wb: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Voltage{V: Complex32::from(y as f32)}),
				   InverseVoltage{per_V: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Voltage{V: Complex64::from(y)}),
				   InverseVoltage{per_V: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AreaPerLumen{m2_per_lm: Complex32::from(y as f32)}),
				   Illuminance{lux: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AreaPerLumen{m2_per_lm: Complex64::from(y)}),
				   Illuminance{lux: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Capacitance{F: Complex32::from(y as f32)}),
				   Elastance{per_F: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Capacitance{F: Complex64::from(y)}),
				   Elastance{per_F: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Charge{C: Complex32::from(y as f32)}),
				   InverseCharge{per_C: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Charge{C: Complex64::from(y)}),
				   InverseCharge{per_C: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Elastance{per_F: Complex32::from(y as f32)}),
				   Capacitance{F: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Elastance{per_F: Complex64::from(y)}),
				   Capacitance{F: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Illuminance{lux: Complex32::from(y as f32)}),
				   AreaPerLumen{m2_per_lm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Illuminance{lux: Complex64::from(y)}),
				   AreaPerLumen{m2_per_lm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Inductance{H: Complex32::from(y as f32)}),
				   InverseInductance{per_H: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Inductance{H: Complex64::from(y)}),
				   InverseInductance{per_H: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseCharge{per_C: Complex32::from(y as f32)}),
				   Charge{C: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseCharge{per_C: Complex64::from(y)}),
				   Charge{C: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseInductance{per_H: Complex32::from(y as f32)}),
				   Inductance{H: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseInductance{per_H: Complex64::from(y)}),
				   Inductance{H: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseLuminousFlux{per_lm: Complex32::from(y as f32)}),
				   LuminousFlux{lm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseLuminousFlux{per_lm: Complex64::from(y)}),
				   LuminousFlux{lm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMagneticFlux{per_Wb: Complex32::from(y as f32)}),
				   MagneticFlux{Wb: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMagneticFlux{per_Wb: Complex64::from(y)}),
				   MagneticFlux{Wb: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMagneticFluxDensity{m2_per_Wb: Complex32::from(y as f32)}),
				   MagneticFluxDensity{T: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMagneticFluxDensity{m2_per_Wb: Complex64::from(y)}),
				   MagneticFluxDensity{T: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseVoltage{per_V: Complex32::from(y as f32)}),
				   Voltage{V: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseVoltage{per_V: Complex64::from(y)}),
				   Voltage{V: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &LuminousFlux{lm: Complex32::from(y as f32)}),
				   InverseLuminousFlux{per_lm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &LuminousFlux{lm: Complex64::from(y)}),
				   InverseLuminousFlux{per_lm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MagneticFlux{Wb: Complex32::from(y as f32)}),
				   InverseMagneticFlux{per_Wb: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MagneticFlux{Wb: Complex64::from(y)}),
				   InverseMagneticFlux{per_Wb: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &MagneticFluxDensity{T: Complex32::from(y as f32)}),
				   InverseMagneticFluxDensity{m2_per_Wb: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &MagneticFluxDensity{T: Complex64::from(y)}),
				   InverseMagneticFluxDensity{m2_per_Wb: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Voltage{V: Complex32::from(y as f32)}),
				   InverseVoltage{per_V: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Voltage{V: Complex64::from(y)}),
				   InverseVoltage{per_V: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Angle{rad: Complex32::from(y as f32)}),
				   InverseAngle{per_rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Angle{rad: Complex64::from(y)}),
				   InverseAngle{per_rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Area{m2: Complex32::from(y as f32)}),
				   InverseArea{per_m2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Area{m2: Complex64::from(y)}),
				   InverseArea{per_m2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngle{per_rad: Complex32::from(y as f32)}),
				   Angle{rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngle{per_rad: Complex64::from(y)}),
				   Angle{rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseArea{per_m2: Complex32::from(y as f32)}),
				   Area{m2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseArea{per_m2: Complex64::from(y)}),
				   Area{m2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseSolidAngle{per_sr: Complex32::from(y as f32)}),
				   SolidAngle{sr: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseSolidAngle{per_sr: Complex64::from(y)}),
				   SolidAngle{sr: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseVolume{per_m3: Complex32::from(y as f32)}),
				   Volume{m3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseVolume{per_m3: Complex64::from(y)}),
				   Volume{m3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &SolidAngle{sr: Complex32::from(y as f32)}),
				   InverseSolidAngle{per_sr: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &SolidAngle{sr: Complex64::from(y)}),
				   InverseSolidAngle{per_sr: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Volume{m3: Complex32::from(y as f32)}),
				   InverseVolume{per_m3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Volume{m3: Complex64::from(y)}),
				   InverseVolume{per_m3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Angle{rad: Complex32::from(y as f32)}),
				   InverseAngle{per_rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Angle{rad: Complex64::from(y)}),
				   InverseAngle{per_rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Area{m2: Complex32::from(y as f32)}),
				   InverseArea{per_m2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Area{m2: Complex64::from(y)}),
				   InverseArea{per_m2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngle{per_rad: Complex32::from(y as f32)}),
				   Angle{rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngle{per_rad: Complex64::from(y)}),
				   Angle{rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseArea{per_m2: Complex32::from(y as f32)}),
				   Area{m2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseArea{per_m2: Complex64::from(y)}),
				   Area{m2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseSolidAngle{per_sr: Complex32::from(y as f32)}),
				   SolidAngle{sr: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseSolidAngle{per_sr: Complex64::from(y)}),
				   SolidAngle{sr: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseVolume{per_m3: Complex32::from(y as f32)}),
				   Volume{m3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseVolume{per_m3: Complex64::from(y)}),
				   Volume{m3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &SolidAngle{sr: Complex32::from(y as f32)}),
				   InverseSolidAngle{per_sr: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &SolidAngle{sr: Complex64::from(y)}),
				   InverseSolidAngle{per_sr: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Volume{m3: Complex32::from(y as f32)}),
				   InverseVolume{per_m3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Volume{m3: Complex64::from(y)}),
				   InverseVolume{per_m3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Acceleration{mps2: Complex32::from(y as f32)}),
				   InverseAcceleration{s2pm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Acceleration{mps2: Complex64::from(y)}),
				   InverseAcceleration{s2pm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AngularAcceleration{radps2: Complex32::from(y as f32)}),
				   InverseAngularAcceleration{s2prad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AngularAcceleration{radps2: Complex64::from(y)}),
				   InverseAngularAcceleration{s2prad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AngularMomentum{kgm2radps: Complex32::from(y as f32)}),
				   InverseAngularMomentum{s_per_kgm2rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AngularMomentum{kgm2radps: Complex64::from(y)}),
				   InverseAngularMomentum{s_per_kgm2rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AngularVelocity{radps: Complex32::from(y as f32)}),
				   InverseAngularVelocity{s_per_rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AngularVelocity{radps: Complex64::from(y)}),
				   InverseAngularVelocity{s_per_rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AreaDensity{kgpm2: Complex32::from(y as f32)}),
				   AreaPerMass{m2_per_kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AreaDensity{kgpm2: Complex64::from(y)}),
				   AreaPerMass{m2_per_kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AreaPerMass{m2_per_kg: Complex32::from(y as f32)}),
				   AreaDensity{kgpm2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AreaPerMass{m2_per_kg: Complex64::from(y)}),
				   AreaDensity{kgpm2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Density{kgpm3: Complex32::from(y as f32)}),
				   VolumePerMass{m3_per_kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Density{kgpm3: Complex64::from(y)}),
				   VolumePerMass{m3_per_kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Energy{J: Complex32::from(y as f32)}),
				   InverseEnergy{per_J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Energy{J: Complex64::from(y)}),
				   InverseEnergy{per_J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Force{N: Complex32::from(y as f32)}),
				   InverseForce{per_N: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Force{N: Complex64::from(y)}),
				   InverseForce{per_N: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAcceleration{s2pm: Complex32::from(y as f32)}),
				   Acceleration{mps2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAcceleration{s2pm: Complex64::from(y)}),
				   Acceleration{mps2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngularAcceleration{s2prad: Complex32::from(y as f32)}),
				   AngularAcceleration{radps2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngularAcceleration{s2prad: Complex64::from(y)}),
				   AngularAcceleration{radps2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngularMomentum{s_per_kgm2rad: Complex32::from(y as f32)}),
				   AngularMomentum{kgm2radps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngularMomentum{s_per_kgm2rad: Complex64::from(y)}),
				   AngularMomentum{kgm2radps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngularVelocity{s_per_rad: Complex32::from(y as f32)}),
				   AngularVelocity{radps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngularVelocity{s_per_rad: Complex64::from(y)}),
				   AngularVelocity{radps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseEnergy{per_J: Complex32::from(y as f32)}),
				   Energy{J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseEnergy{per_J: Complex64::from(y)}),
				   Energy{J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseForce{per_N: Complex32::from(y as f32)}),
				   Force{N: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseForce{per_N: Complex64::from(y)}),
				   Force{N: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMomentum{s_per_kgm: Complex32::from(y as f32)}),
				   Momentum{kgmps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMomentum{s_per_kgm: Complex64::from(y)}),
				   Momentum{kgmps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InversePower{per_W: Complex32::from(y as f32)}),
				   Power{W: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InversePower{per_W: Complex64::from(y)}),
				   Power{W: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InversePressure{per_Pa: Complex32::from(y as f32)}),
				   Pressure{Pa: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InversePressure{per_Pa: Complex64::from(y)}),
				   Pressure{Pa: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseTorque{per_Nm: Complex32::from(y as f32)}),
				   Energy{J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseTorque{per_Nm: Complex64::from(y)}),
				   Energy{J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Momentum{kgmps: Complex32::from(y as f32)}),
				   InverseMomentum{s_per_kgm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Momentum{kgmps: Complex64::from(y)}),
				   InverseMomentum{s_per_kgm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Power{W: Complex32::from(y as f32)}),
				   InversePower{per_W: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Power{W: Complex64::from(y)}),
				   InversePower{per_W: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Pressure{Pa: Complex32::from(y as f32)}),
				   InversePressure{per_Pa: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Pressure{Pa: Complex64::from(y)}),
				   InversePressure{per_Pa: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &TimePerDistance{spm: Complex32::from(y as f32)}),
				   Velocity{mps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &TimePerDistance{spm: Complex64::from(y)}),
				   Velocity{mps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Torque{Nm: Complex32::from(y as f32)}),
				   InverseEnergy{per_J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Torque{Nm: Complex64::from(y)}),
				   InverseEnergy{per_J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Velocity{mps: Complex32::from(y as f32)}),
				   TimePerDistance{spm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Velocity{mps: Complex64::from(y)}),
				   TimePerDistance{spm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &VolumePerMass{m3_per_kg: Complex32::from(y as f32)}),
				   Density{kgpm3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &VolumePerMass{m3_per_kg: Complex64::from(y)}),
				   Density{kgpm3: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Acceleration{mps2: Complex32::from(y as f32)}),
				   InverseAcceleration{s2pm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Acceleration{mps2: Complex64::from(y)}),
				   InverseAcceleration{s2pm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AngularAcceleration{radps2: Complex32::from(y as f32)}),
				   InverseAngularAcceleration{s2prad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AngularAcceleration{radps2: Complex64::from(y)}),
				   InverseAngularAcceleration{s2prad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AngularMomentum{kgm2radps: Complex32::from(y as f32)}),
				   InverseAngularMomentum{s_per_kgm2rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AngularMomentum{kgm2radps: Complex64::from(y)}),
				   InverseAngularMomentum{s_per_kgm2rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AngularVelocity{radps: Complex32::from(y as f32)}),
				   InverseAngularVelocity{s_per_rad: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AngularVelocity{radps: Complex64::from(y)}),
				   InverseAngularVelocity{s_per_rad: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AreaDensity{kgpm2: Complex32::from(y as f32)}),
				   AreaPerMass{m2_per_kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AreaDensity{kgpm2: Complex64::from(y)}),
				   AreaPerMass{m2_per_kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &AreaPerMass{m2_per_kg: Complex32::from(y as f32)}),
				   AreaDensity{kgpm2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &AreaPerMass{m2_per_kg: Complex64::from(y)}),
				   AreaDensity{kgpm2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Density{kgpm3: Complex32::from(y as f32)}),
				   VolumePerMass{m3_per_kg: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Density{kgpm3: Complex64::from(y)}),
				   VolumePerMass{m3_per_kg: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Energy{J: Complex32::from(y as f32)}),
				   InverseEnergy{per_J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Energy{J: Complex64::from(y)}),
				   InverseEnergy{per_J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Force{N: Complex32::from(y as f32)}),
				   InverseForce{per_N: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Force{N: Complex64::from(y)}),
				   InverseForce{per_N: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAcceleration{s2pm: Complex32::from(y as f32)}),
				   Acceleration{mps2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAcceleration{s2pm: Complex64::from(y)}),
				   Acceleration{mps2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngularAcceleration{s2prad: Complex32::from(y as f32)}),
				   AngularAcceleration{radps2: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngularAcceleration{s2prad: Complex64::from(y)}),
				   AngularAcceleration{radps2: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngularMomentum{s_per_kgm2rad: Complex32::from(y as f32)}),
				   AngularMomentum{kgm2radps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngularMomentum{s_per_kgm2rad: Complex64::from(y)}),
				   AngularMomentum{kgm2radps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseAngularVelocity{s_per_rad: Complex32::from(y as f32)}),
				   AngularVelocity{radps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseAngularVelocity{s_per_rad: Complex64::from(y)}),
				   AngularVelocity{radps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseEnergy{per_J: Complex32::from(y as f32)}),
				   Energy{J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseEnergy{per_J: Complex64::from(y)}),
				   Energy{J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseForce{per_N: Complex32::from(y as f32)}),
				   Force{N: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseForce{per_N: Complex64::from(y)}),
				   Force{N: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseMomentum{s_per_kgm: Complex32::from(y as f32)}),
				   Momentum{kgmps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseMomentum{s_per_kgm: Complex64::from(y)}),
				   Momentum{kgmps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InversePower{per_W: Complex32::from(y as f32)}),
				   Power{W: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InversePower{per_W: Complex64::from(y)}),
				   Power{W: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InversePressure{per_Pa: Complex32::from(y as f32)}),
				   Pressure{Pa: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InversePressure{per_Pa: Complex64::from(y)}),
				   Pressure{Pa: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &InverseTorque{per_Nm: Complex32::from(y as f32)}),
				   Energy{J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &InverseTorque{per_Nm: Complex64::from(y)}),
				   Energy{J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Momentum{kgmps: Complex32::from(y as f32)}),
				   InverseMomentum{s_per_kgm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Momentum{kgmps: Complex64::from(y)}),
				   InverseMomentum{s_per_kgm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Power{W: Complex32::from(y as f32)}),
				   InversePower{per_W: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Power{W: Complex64::from(y)}),
				   InversePower{per_W: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Pressure{Pa: Complex32::from(y as f32)}),
				   InversePressure{per_Pa: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Pressure{Pa: Complex64::from(y)}),
				   InversePressure{per_Pa: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &TimePerDistance{spm: Complex32::from(y as f32)}),
				   Velocity{mps: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &TimePerDistance{spm: Complex64::from(y)}),
				   Velocity{mps: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Torque{Nm: Complex32::from(y as f32)}),
				   InverseEnergy{per_J: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Torque{Nm: Complex64::from(y)}),
				   InverseEnergy{per_J: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &Velocity{mps: Complex32::from(y as f32)}),
				   TimePerDistance{spm: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &Velocity{mps: Complex64::from(y)}),
				   TimePerDistance{spm: Complex64::from(x)/Complex64::from(y)}
		);
		assert_eq!(div_check(
			&Complex32::from(x as f32), &VolumePerMass{m3_per_kg: Complex32::from(y as f32)}),
				   Density{kgpm3: Complex32::from(x as f32)/Complex32::from(y as f32)}
		);
		assert_eq!(div_check(
			&Complex64::from(x), &VolumePerMass{m3_per_kg: Complex64::from(y)}),
				   Density{kgpm3: Complex64::from(x)/Complex64::from(y)}
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


	#[test]
	fn inverse_amount_units() {
		assert_approx_equal(
			InverseAmount::from_per_mol(6.02e+23_f64).to_per_mol(),
			InverseAmount::from_per_count(1.0_f64).to_per_mol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1.0_f64).to_per_mol() * 1.66e-24,
			InverseAmount::from_per_mol(1.0_f64).to_per_count(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1000.0_f64).to_per_mol(),
			InverseAmount::from_per_mmol(1.0_f64).to_per_mol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1.0_f64).to_per_mol() * 0.001,
			InverseAmount::from_per_mol(1.0_f64).to_per_mmol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1000000.0_f64).to_per_mol(),
			InverseAmount::from_per_umol(1.0_f64).to_per_mol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1.0_f64).to_per_mol() * 1e-06,
			InverseAmount::from_per_mol(1.0_f64).to_per_umol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1000000000.0_f64).to_per_mol(),
			InverseAmount::from_per_nmol(1.0_f64).to_per_mol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1.0_f64).to_per_mol() * 1e-09,
			InverseAmount::from_per_mol(1.0_f64).to_per_nmol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1000000000000.0_f64).to_per_mol(),
			InverseAmount::from_per_pmol(1.0_f64).to_per_mol(), 9
		);
		assert_approx_equal(
			InverseAmount::from_per_mol(1.0_f64).to_per_mol() * 1e-12,
			InverseAmount::from_per_mol(1.0_f64).to_per_pmol(), 9
		);
	}

	#[test]
	fn inverse_current_units() {
		assert_approx_equal(
			InverseCurrent::from_per_A(1000.0_f64).to_per_A(),
			InverseCurrent::from_per_mA(1.0_f64).to_per_A(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1.0_f64).to_per_A() * 0.001,
			InverseCurrent::from_per_A(1.0_f64).to_per_mA(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1000000.0_f64).to_per_A(),
			InverseCurrent::from_per_uA(1.0_f64).to_per_A(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1.0_f64).to_per_A() * 1e-06,
			InverseCurrent::from_per_A(1.0_f64).to_per_uA(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1000000000.0_f64).to_per_A(),
			InverseCurrent::from_per_nA(1.0_f64).to_per_A(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1.0_f64).to_per_A() * 1e-09,
			InverseCurrent::from_per_A(1.0_f64).to_per_nA(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(0.001_f64).to_per_A(),
			InverseCurrent::from_per_kA(1.0_f64).to_per_A(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1.0_f64).to_per_A() * 1000.0,
			InverseCurrent::from_per_A(1.0_f64).to_per_kA(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1e-06_f64).to_per_A(),
			InverseCurrent::from_per_MA(1.0_f64).to_per_A(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1.0_f64).to_per_A() * 1000000.0,
			InverseCurrent::from_per_A(1.0_f64).to_per_MA(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1e-09_f64).to_per_A(),
			InverseCurrent::from_per_GA(1.0_f64).to_per_A(), 9
		);
		assert_approx_equal(
			InverseCurrent::from_per_A(1.0_f64).to_per_A() * 1000000000.0,
			InverseCurrent::from_per_A(1.0_f64).to_per_GA(), 9
		);
	}

	#[test]
	fn inverse_distance_units() {
		assert_approx_equal(
			InverseDistance::from_per_m(100.0_f64).to_per_m(),
			InverseDistance::from_per_cm(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 0.01,
			InverseDistance::from_per_m(1.0_f64).to_per_cm(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1000.0_f64).to_per_m(),
			InverseDistance::from_per_mm(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 0.001,
			InverseDistance::from_per_m(1.0_f64).to_per_mm(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1000000.0_f64).to_per_m(),
			InverseDistance::from_per_um(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 1e-06,
			InverseDistance::from_per_m(1.0_f64).to_per_um(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1000000000.0_f64).to_per_m(),
			InverseDistance::from_per_nm(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 1e-09,
			InverseDistance::from_per_m(1.0_f64).to_per_nm(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1000000000000.0_f64).to_per_m(),
			InverseDistance::from_per_pm(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 1e-12,
			InverseDistance::from_per_m(1.0_f64).to_per_pm(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(0.001_f64).to_per_m(),
			InverseDistance::from_per_km(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 1000.0,
			InverseDistance::from_per_m(1.0_f64).to_per_km(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(6.68e-12_f64).to_per_m(),
			InverseDistance::from_per_au(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 149597870700.0,
			InverseDistance::from_per_m(1.0_f64).to_per_au(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(3.24e-17_f64).to_per_m(),
			InverseDistance::from_per_parsec(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 3.09e+16,
			InverseDistance::from_per_m(1.0_f64).to_per_parsec(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.06e-16_f64).to_per_m(),
			InverseDistance::from_per_lyr(1.0_f64).to_per_m(), 9
		);
		assert_approx_equal(
			InverseDistance::from_per_m(1.0_f64).to_per_m() * 9460528169656200.0,
			InverseDistance::from_per_m(1.0_f64).to_per_lyr(), 9
		);
	}

	#[test]
	fn inverse_luminosity_units() {
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1000.0_f64).to_per_cd(),
			InverseLuminosity::from_per_mcd(1.0_f64).to_per_cd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1.0_f64).to_per_cd() * 0.001,
			InverseLuminosity::from_per_cd(1.0_f64).to_per_mcd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1000000.0_f64).to_per_cd(),
			InverseLuminosity::from_per_ucd(1.0_f64).to_per_cd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1.0_f64).to_per_cd() * 1e-06,
			InverseLuminosity::from_per_cd(1.0_f64).to_per_ucd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1000000000.0_f64).to_per_cd(),
			InverseLuminosity::from_per_ncd(1.0_f64).to_per_cd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1.0_f64).to_per_cd() * 1e-09,
			InverseLuminosity::from_per_cd(1.0_f64).to_per_ncd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(0.001_f64).to_per_cd(),
			InverseLuminosity::from_per_kcd(1.0_f64).to_per_cd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1.0_f64).to_per_cd() * 1000.0,
			InverseLuminosity::from_per_cd(1.0_f64).to_per_kcd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1e-06_f64).to_per_cd(),
			InverseLuminosity::from_per_Mcd(1.0_f64).to_per_cd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1.0_f64).to_per_cd() * 1000000.0,
			InverseLuminosity::from_per_cd(1.0_f64).to_per_Mcd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1e-09_f64).to_per_cd(),
			InverseLuminosity::from_per_Gcd(1.0_f64).to_per_cd(), 9
		);
		assert_approx_equal(
			InverseLuminosity::from_per_cd(1.0_f64).to_per_cd() * 1000000000.0,
			InverseLuminosity::from_per_cd(1.0_f64).to_per_Gcd(), 9
		);
	}

	#[test]
	fn inverse_mass_units() {
		assert_approx_equal(
			InverseMass::from_per_kg(1000.0_f64).to_per_kg(),
			InverseMass::from_per_g(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 0.001,
			InverseMass::from_per_kg(1.0_f64).to_per_g(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1000000.0_f64).to_per_kg(),
			InverseMass::from_per_mg(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1e-06,
			InverseMass::from_per_kg(1.0_f64).to_per_mg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1000000000.0_f64).to_per_kg(),
			InverseMass::from_per_ug(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1e-09,
			InverseMass::from_per_kg(1.0_f64).to_per_ug(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1000000000000.0_f64).to_per_kg(),
			InverseMass::from_per_ng(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1e-12,
			InverseMass::from_per_kg(1.0_f64).to_per_ng(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1000000000000000.0_f64).to_per_kg(),
			InverseMass::from_per_pg(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1e-15,
			InverseMass::from_per_kg(1.0_f64).to_per_pg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(0.001_f64).to_per_kg(),
			InverseMass::from_per_tons(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1000.0,
			InverseMass::from_per_kg(1.0_f64).to_per_tons(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.6699999999999999e-25_f64).to_per_kg(),
			InverseMass::from_per_earth_mass(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 5.97e+24,
			InverseMass::from_per_kg(1.0_f64).to_per_earth_mass(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(5.27e-28_f64).to_per_kg(),
			InverseMass::from_per_jupiter_mass(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1.9e+27,
			InverseMass::from_per_kg(1.0_f64).to_per_jupiter_mass(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(5.03e-31_f64).to_per_kg(),
			InverseMass::from_per_solar_mass(1.0_f64).to_per_kg(), 9
		);
		assert_approx_equal(
			InverseMass::from_per_kg(1.0_f64).to_per_kg() * 1.99e+30,
			InverseMass::from_per_kg(1.0_f64).to_per_solar_mass(), 9
		);
	}

	#[test]
	fn inverse_catalytic_activity_units() {
		assert_approx_equal(
			InverseCatalyticActivity::from_s_per_mol(60.0_f64).to_s_per_mol(),
			InverseCatalyticActivity::from_minutes_per_mole(1.0_f64).to_s_per_mol(), 9
		);
		assert_approx_equal(
			InverseCatalyticActivity::from_s_per_mol(1.0_f64).to_s_per_mol() * 0.0166666666666667,
			InverseCatalyticActivity::from_s_per_mol(1.0_f64).to_minutes_per_mole(), 9
		);
		assert_approx_equal(
			InverseCatalyticActivity::from_s_per_mol(3600.0_f64).to_s_per_mol(),
			InverseCatalyticActivity::from_hours_per_mole(1.0_f64).to_s_per_mol(), 9
		);
		assert_approx_equal(
			InverseCatalyticActivity::from_s_per_mol(1.0_f64).to_s_per_mol() * 0.000277777777777778,
			InverseCatalyticActivity::from_s_per_mol(1.0_f64).to_hours_per_mole(), 9
		);
	}

	#[test]
	fn molar_volume_units() {
		assert_approx_equal(
			MolarVolume::from_m3_per_mol(0.001_f64).to_m3_per_mol(),
			MolarVolume::from_L_per_mol(1.0_f64).to_m3_per_mol(), 9
		);
		assert_approx_equal(
			MolarVolume::from_m3_per_mol(1.0_f64).to_m3_per_mol() * 1000.0,
			MolarVolume::from_m3_per_mol(1.0_f64).to_L_per_mol(), 9
		);
		assert_approx_equal(
			MolarVolume::from_m3_per_mol(0.001_f64).to_m3_per_mol(),
			MolarVolume::from_liters_per_mole(1.0_f64).to_m3_per_mol(), 9
		);
		assert_approx_equal(
			MolarVolume::from_m3_per_mol(1.0_f64).to_m3_per_mol() * 1000.0,
			MolarVolume::from_m3_per_mol(1.0_f64).to_liters_per_mole(), 9
		);
	}

	#[test]
	fn specific_heat_capacity_units() {
		assert_approx_equal(
			SpecificHeatCapacity::from_J_per_kgK(1000.0_f64).to_J_per_kgK(),
			SpecificHeatCapacity::from_joules_per_gram_kelvin(1.0_f64).to_J_per_kgK(), 9
		);
		assert_approx_equal(
			SpecificHeatCapacity::from_J_per_kgK(1.0_f64).to_J_per_kgK() * 0.001,
			SpecificHeatCapacity::from_J_per_kgK(1.0_f64).to_joules_per_gram_kelvin(), 9
		);
		assert_approx_equal(
			SpecificHeatCapacity::from_J_per_kgK(1000.0_f64).to_J_per_kgK(),
			SpecificHeatCapacity::from_J_per_gK(1.0_f64).to_J_per_kgK(), 9
		);
		assert_approx_equal(
			SpecificHeatCapacity::from_J_per_kgK(1.0_f64).to_J_per_kgK() * 0.001,
			SpecificHeatCapacity::from_J_per_kgK(1.0_f64).to_J_per_gK(), 9
		);
	}

	#[test]
	fn elastance_units() {
		assert_approx_equal(
			Elastance::from_per_F(1000.0_f64).to_per_F(),
			Elastance::from_per_mF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 0.001,
			Elastance::from_per_F(1.0_f64).to_per_mF(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1000000.0_f64).to_per_F(),
			Elastance::from_per_uF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 1e-06,
			Elastance::from_per_F(1.0_f64).to_per_uF(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1000000000.0_f64).to_per_F(),
			Elastance::from_per_nF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 1e-09,
			Elastance::from_per_F(1.0_f64).to_per_nF(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1000000000000.0_f64).to_per_F(),
			Elastance::from_per_pF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 1e-12,
			Elastance::from_per_F(1.0_f64).to_per_pF(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(0.001_f64).to_per_F(),
			Elastance::from_per_kF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 1000.0,
			Elastance::from_per_F(1.0_f64).to_per_kF(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1e-06_f64).to_per_F(),
			Elastance::from_per_MF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 1000000.0,
			Elastance::from_per_F(1.0_f64).to_per_MF(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1e-09_f64).to_per_F(),
			Elastance::from_per_GF(1.0_f64).to_per_F(), 9
		);
		assert_approx_equal(
			Elastance::from_per_F(1.0_f64).to_per_F() * 1000000000.0,
			Elastance::from_per_F(1.0_f64).to_per_GF(), 9
		);
	}

	#[test]
	fn inverse_charge_units() {
		assert_approx_equal(
			InverseCharge::from_per_C(1000.0_f64).to_per_C(),
			InverseCharge::from_per_mC(1.0_f64).to_per_C(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1.0_f64).to_per_C() * 0.001,
			InverseCharge::from_per_C(1.0_f64).to_per_mC(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1000000.0_f64).to_per_C(),
			InverseCharge::from_per_uC(1.0_f64).to_per_C(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1.0_f64).to_per_C() * 1e-06,
			InverseCharge::from_per_C(1.0_f64).to_per_uC(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1000000000.0_f64).to_per_C(),
			InverseCharge::from_per_nC(1.0_f64).to_per_C(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1.0_f64).to_per_C() * 1e-09,
			InverseCharge::from_per_C(1.0_f64).to_per_nC(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(0.001_f64).to_per_C(),
			InverseCharge::from_per_kC(1.0_f64).to_per_C(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1.0_f64).to_per_C() * 1000.0,
			InverseCharge::from_per_C(1.0_f64).to_per_kC(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1e-06_f64).to_per_C(),
			InverseCharge::from_per_MC(1.0_f64).to_per_C(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1.0_f64).to_per_C() * 1000000.0,
			InverseCharge::from_per_C(1.0_f64).to_per_MC(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1e-09_f64).to_per_C(),
			InverseCharge::from_per_GC(1.0_f64).to_per_C(), 9
		);
		assert_approx_equal(
			InverseCharge::from_per_C(1.0_f64).to_per_C() * 1000000000.0,
			InverseCharge::from_per_C(1.0_f64).to_per_GC(), 9
		);
	}

	#[test]
	fn inverse_inductance_units() {
		assert_approx_equal(
			InverseInductance::from_per_H(1000.0_f64).to_per_H(),
			InverseInductance::from_per_mH(1.0_f64).to_per_H(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1.0_f64).to_per_H() * 0.001,
			InverseInductance::from_per_H(1.0_f64).to_per_mH(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1000000.0_f64).to_per_H(),
			InverseInductance::from_per_uH(1.0_f64).to_per_H(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1.0_f64).to_per_H() * 1e-06,
			InverseInductance::from_per_H(1.0_f64).to_per_uH(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1000000000.0_f64).to_per_H(),
			InverseInductance::from_per_nH(1.0_f64).to_per_H(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1.0_f64).to_per_H() * 1e-09,
			InverseInductance::from_per_H(1.0_f64).to_per_nH(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(0.001_f64).to_per_H(),
			InverseInductance::from_per_kH(1.0_f64).to_per_H(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1.0_f64).to_per_H() * 1000.0,
			InverseInductance::from_per_H(1.0_f64).to_per_kH(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1e-06_f64).to_per_H(),
			InverseInductance::from_per_MH(1.0_f64).to_per_H(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1.0_f64).to_per_H() * 1000000.0,
			InverseInductance::from_per_H(1.0_f64).to_per_MH(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1e-09_f64).to_per_H(),
			InverseInductance::from_per_GH(1.0_f64).to_per_H(), 9
		);
		assert_approx_equal(
			InverseInductance::from_per_H(1.0_f64).to_per_H() * 1000000000.0,
			InverseInductance::from_per_H(1.0_f64).to_per_GH(), 9
		);
	}

	#[test]
	fn inverse_luminous_flux_units() {
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1000.0_f64).to_per_lm(),
			InverseLuminousFlux::from_per_mlm(1.0_f64).to_per_lm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_lm() * 0.001,
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_mlm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1000000.0_f64).to_per_lm(),
			InverseLuminousFlux::from_per_ulm(1.0_f64).to_per_lm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_lm() * 1e-06,
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_ulm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1000000000.0_f64).to_per_lm(),
			InverseLuminousFlux::from_per_nlm(1.0_f64).to_per_lm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_lm() * 1e-09,
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_nlm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(0.001_f64).to_per_lm(),
			InverseLuminousFlux::from_per_klm(1.0_f64).to_per_lm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_lm() * 1000.0,
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_klm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1e-06_f64).to_per_lm(),
			InverseLuminousFlux::from_per_Mlm(1.0_f64).to_per_lm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_lm() * 1000000.0,
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_Mlm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1e-09_f64).to_per_lm(),
			InverseLuminousFlux::from_per_Glm(1.0_f64).to_per_lm(), 9
		);
		assert_approx_equal(
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_lm() * 1000000000.0,
			InverseLuminousFlux::from_per_lm(1.0_f64).to_per_Glm(), 9
		);
	}

	#[test]
	fn inverse_magnetic_flux_units() {
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1000.0_f64).to_per_Wb(),
			InverseMagneticFlux::from_per_mWb(1.0_f64).to_per_Wb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_Wb() * 0.001,
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_mWb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1000000.0_f64).to_per_Wb(),
			InverseMagneticFlux::from_per_uWb(1.0_f64).to_per_Wb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_Wb() * 1e-06,
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_uWb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1000000000.0_f64).to_per_Wb(),
			InverseMagneticFlux::from_per_nWb(1.0_f64).to_per_Wb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_Wb() * 1e-09,
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_nWb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(0.001_f64).to_per_Wb(),
			InverseMagneticFlux::from_per_kWb(1.0_f64).to_per_Wb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_Wb() * 1000.0,
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_kWb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1e-06_f64).to_per_Wb(),
			InverseMagneticFlux::from_per_MWb(1.0_f64).to_per_Wb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_Wb() * 1000000.0,
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_MWb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1e-09_f64).to_per_Wb(),
			InverseMagneticFlux::from_per_GWb(1.0_f64).to_per_Wb(), 9
		);
		assert_approx_equal(
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_Wb() * 1000000000.0,
			InverseMagneticFlux::from_per_Wb(1.0_f64).to_per_GWb(), 9
		);
	}

	#[test]
	fn inverse_voltage_units() {
		assert_approx_equal(
			InverseVoltage::from_per_V(1000.0_f64).to_per_V(),
			InverseVoltage::from_per_mV(1.0_f64).to_per_V(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1.0_f64).to_per_V() * 0.001,
			InverseVoltage::from_per_V(1.0_f64).to_per_mV(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1000000.0_f64).to_per_V(),
			InverseVoltage::from_per_uV(1.0_f64).to_per_V(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1.0_f64).to_per_V() * 1e-06,
			InverseVoltage::from_per_V(1.0_f64).to_per_uV(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1000000000.0_f64).to_per_V(),
			InverseVoltage::from_per_nV(1.0_f64).to_per_V(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1.0_f64).to_per_V() * 1e-09,
			InverseVoltage::from_per_V(1.0_f64).to_per_nV(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(0.001_f64).to_per_V(),
			InverseVoltage::from_per_kV(1.0_f64).to_per_V(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1.0_f64).to_per_V() * 1000.0,
			InverseVoltage::from_per_V(1.0_f64).to_per_kV(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1e-06_f64).to_per_V(),
			InverseVoltage::from_per_MV(1.0_f64).to_per_V(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1.0_f64).to_per_V() * 1000000.0,
			InverseVoltage::from_per_V(1.0_f64).to_per_MV(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1e-09_f64).to_per_V(),
			InverseVoltage::from_per_GV(1.0_f64).to_per_V(), 9
		);
		assert_approx_equal(
			InverseVoltage::from_per_V(1.0_f64).to_per_V() * 1000000000.0,
			InverseVoltage::from_per_V(1.0_f64).to_per_GV(), 9
		);
	}

	#[test]
	fn inverse_angle_units() {
		assert_approx_equal(
			InverseAngle::from_per_rad(57.2957795130823_f64).to_per_rad(),
			InverseAngle::from_per_degrees(1.0_f64).to_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngle::from_per_rad(1.0_f64).to_per_rad() * 0.0174532925199433,
			InverseAngle::from_per_rad(1.0_f64).to_per_degrees(), 9
		);
		assert_approx_equal(
			InverseAngle::from_per_rad(57.2957795130823_f64).to_per_rad(),
			InverseAngle::from_per_deg(1.0_f64).to_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngle::from_per_rad(1.0_f64).to_per_rad() * 0.0174532925199433,
			InverseAngle::from_per_rad(1.0_f64).to_per_deg(), 9
		);
	}

	#[test]
	fn inverse_area_units() {
		assert_approx_equal(
			InverseArea::from_per_m2(10000.0_f64).to_per_m2(),
			InverseArea::from_per_cm2(1.0_f64).to_per_m2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1.0_f64).to_per_m2() * 0.0001,
			InverseArea::from_per_m2(1.0_f64).to_per_cm2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(10000.0_f64).to_per_m2(),
			InverseArea::from_per_square_cm(1.0_f64).to_per_m2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1.0_f64).to_per_m2() * 0.0001,
			InverseArea::from_per_m2(1.0_f64).to_per_square_cm(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1000000.0_f64).to_per_m2(),
			InverseArea::from_per_mm2(1.0_f64).to_per_m2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1.0_f64).to_per_m2() * 1e-06,
			InverseArea::from_per_m2(1.0_f64).to_per_mm2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1000000000000.0_f64).to_per_m2(),
			InverseArea::from_per_um2(1.0_f64).to_per_m2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1.0_f64).to_per_m2() * 1e-12,
			InverseArea::from_per_m2(1.0_f64).to_per_um2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1e+18_f64).to_per_m2(),
			InverseArea::from_per_nm2(1.0_f64).to_per_m2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1.0_f64).to_per_m2() * 1e-18,
			InverseArea::from_per_m2(1.0_f64).to_per_nm2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1e-06_f64).to_per_m2(),
			InverseArea::from_per_km2(1.0_f64).to_per_m2(), 9
		);
		assert_approx_equal(
			InverseArea::from_per_m2(1.0_f64).to_per_m2() * 1000000.0,
			InverseArea::from_per_m2(1.0_f64).to_per_km2(), 9
		);
	}

	#[test]
	fn inverse_volume_units() {
		assert_approx_equal(
			InverseVolume::from_per_m3(1000000.0_f64).to_per_m3(),
			InverseVolume::from_per_cc(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1e-06,
			InverseVolume::from_per_m3(1.0_f64).to_per_cc(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1000.0_f64).to_per_m3(),
			InverseVolume::from_per_L(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 0.001,
			InverseVolume::from_per_m3(1.0_f64).to_per_L(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1000.0_f64).to_per_m3(),
			InverseVolume::from_per_liters(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 0.001,
			InverseVolume::from_per_m3(1.0_f64).to_per_liters(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1000000.0_f64).to_per_m3(),
			InverseVolume::from_per_mL(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1e-06,
			InverseVolume::from_per_m3(1.0_f64).to_per_mL(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1000000000.0_f64).to_per_m3(),
			InverseVolume::from_per_uL(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1e-09,
			InverseVolume::from_per_m3(1.0_f64).to_per_uL(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1000000000000.0_f64).to_per_m3(),
			InverseVolume::from_per_nL(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1e-12,
			InverseVolume::from_per_m3(1.0_f64).to_per_nL(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1000000000000000.0_f64).to_per_m3(),
			InverseVolume::from_per_pL(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1e-15,
			InverseVolume::from_per_m3(1.0_f64).to_per_pL(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(0.001_f64).to_per_m3(),
			InverseVolume::from_per_ML(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1000.0,
			InverseVolume::from_per_m3(1.0_f64).to_per_ML(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1e-06_f64).to_per_m3(),
			InverseVolume::from_per_GL(1.0_f64).to_per_m3(), 9
		);
		assert_approx_equal(
			InverseVolume::from_per_m3(1.0_f64).to_per_m3() * 1000000.0,
			InverseVolume::from_per_m3(1.0_f64).to_per_GL(), 9
		);
	}

	#[test]
	fn area_per_mass_units() {
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(1000.0_f64).to_m2_per_kg(),
			AreaPerMass::from_m2_per_g(1.0_f64).to_m2_per_kg(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(1.0_f64).to_m2_per_kg() * 0.001,
			AreaPerMass::from_m2_per_kg(1.0_f64).to_m2_per_g(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(1000.0_f64).to_m2_per_kg(),
			AreaPerMass::from_square_meters_per_gram(1.0_f64).to_m2_per_kg(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(1.0_f64).to_m2_per_kg() * 0.001,
			AreaPerMass::from_m2_per_kg(1.0_f64).to_square_meters_per_gram(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(0.1_f64).to_m2_per_kg(),
			AreaPerMass::from_cm2_per_g(1.0_f64).to_m2_per_kg(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(1.0_f64).to_m2_per_kg() * 10.0,
			AreaPerMass::from_m2_per_kg(1.0_f64).to_cm2_per_g(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(0.1_f64).to_m2_per_kg(),
			AreaPerMass::from_square_centimeters_per_gram(1.0_f64).to_m2_per_kg(), 9
		);
		assert_approx_equal(
			AreaPerMass::from_m2_per_kg(1.0_f64).to_m2_per_kg() * 10.0,
			AreaPerMass::from_m2_per_kg(1.0_f64).to_square_centimeters_per_gram(), 9
		);
	}

	#[test]
	fn inverse_acceleration_units() {
		assert_approx_equal(
			InverseAcceleration::from_s2pm(1000.0_f64).to_s2pm(),
			InverseAcceleration::from_s2pmm(1.0_f64).to_s2pm(), 9
		);
		assert_approx_equal(
			InverseAcceleration::from_s2pm(1.0_f64).to_s2pm() * 0.001,
			InverseAcceleration::from_s2pm(1.0_f64).to_s2pmm(), 9
		);
		assert_approx_equal(
			InverseAcceleration::from_s2pm(1000000.0_f64).to_s2pm(),
			InverseAcceleration::from_hours_squared_per_kilometers(1.0_f64).to_s2pm(), 9
		);
		assert_approx_equal(
			InverseAcceleration::from_s2pm(1.0_f64).to_s2pm() * 1e-06,
			InverseAcceleration::from_s2pm(1.0_f64).to_hours_squared_per_kilometers(), 9
		);
		assert_approx_equal(
			InverseAcceleration::from_s2pm(12960.0_f64).to_s2pm(),
			InverseAcceleration::from_hr2_per_km(1.0_f64).to_s2pm(), 9
		);
		assert_approx_equal(
			InverseAcceleration::from_s2pm(1.0_f64).to_s2pm() * 7.72e-05,
			InverseAcceleration::from_s2pm(1.0_f64).to_hr2_per_km(), 9
		);
	}

	#[test]
	fn inverse_angular_acceleration_units() {
		assert_approx_equal(
			InverseAngularAcceleration::from_s2prad(57.2957795130823_f64).to_s2prad(),
			InverseAngularAcceleration::from_seconds_squared_per_degree(1.0_f64).to_s2prad(), 9
		);
		assert_approx_equal(
			InverseAngularAcceleration::from_s2prad(1.0_f64).to_s2prad() * 0.0174532925199433,
			InverseAngularAcceleration::from_s2prad(1.0_f64).to_seconds_squared_per_degree(), 9
		);
	}

	#[test]
	fn inverse_angular_momentum_units() {
		assert_approx_equal(
			InverseAngularMomentum::from_s_per_kgm2rad(10000000.0_f64).to_s_per_kgm2rad(),
			InverseAngularMomentum::from_s_per_gcm2rad(1.0_f64).to_s_per_kgm2rad(), 9
		);
		assert_approx_equal(
			InverseAngularMomentum::from_s_per_kgm2rad(1.0_f64).to_s_per_kgm2rad() * 1e-07,
			InverseAngularMomentum::from_s_per_kgm2rad(1.0_f64).to_s_per_gcm2rad(), 9
		);
	}

	#[test]
	fn inverse_angular_velocity_units() {
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(57.2957795130823_f64).to_s_per_rad(),
			InverseAngularVelocity::from_seconds_per_degree(1.0_f64).to_s_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_s_per_rad() * 0.0174532925199433,
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_seconds_per_degree(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(57.2957795130823_f64).to_s_per_rad(),
			InverseAngularVelocity::from_s_per_deg(1.0_f64).to_s_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_s_per_rad() * 0.0174532925199433,
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_s_per_deg(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(0.159154943091895_f64).to_s_per_rad(),
			InverseAngularVelocity::from_spr(1.0_f64).to_s_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_s_per_rad() * 6.28318530717959,
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_spr(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(9.54929658551372_f64).to_s_per_rad(),
			InverseAngularVelocity::from_mpr(1.0_f64).to_s_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_s_per_rad() * 0.10471975511966,
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_mpr(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(572.957795130823_f64).to_s_per_rad(),
			InverseAngularVelocity::from_hpr(1.0_f64).to_s_per_rad(), 9
		);
		assert_approx_equal(
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_s_per_rad() * 0.0017453292519943,
			InverseAngularVelocity::from_s_per_rad(1.0_f64).to_hpr(), 9
		);
	}

	#[test]
	fn inverse_energy_units() {
		assert_approx_equal(
			InverseEnergy::from_per_J(1000.0_f64).to_per_J(),
			InverseEnergy::from_per_mJ(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 0.001,
			InverseEnergy::from_per_J(1.0_f64).to_per_mJ(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1000000.0_f64).to_per_J(),
			InverseEnergy::from_per_uJ(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1e-06,
			InverseEnergy::from_per_J(1.0_f64).to_per_uJ(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1000000000.0_f64).to_per_J(),
			InverseEnergy::from_per_nJ(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1e-09,
			InverseEnergy::from_per_J(1.0_f64).to_per_nJ(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(0.001_f64).to_per_J(),
			InverseEnergy::from_per_kJ(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1000.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_kJ(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1e-06_f64).to_per_J(),
			InverseEnergy::from_per_MJ(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1000000.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_MJ(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1e-09_f64).to_per_J(),
			InverseEnergy::from_per_GJ(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1000000000.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_GJ(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(0.239005736137667_f64).to_per_J(),
			InverseEnergy::from_per_cal(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 4.184,
			InverseEnergy::from_per_J(1.0_f64).to_per_cal(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(0.0002390057361376_f64).to_per_J(),
			InverseEnergy::from_per_kcal(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 4184.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_kcal(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(0.0002777777777777_f64).to_per_J(),
			InverseEnergy::from_per_Whr(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 3600.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_Whr(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(2.78e-07_f64).to_per_J(),
			InverseEnergy::from_per_kWhr(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 3600000.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_kWhr(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(6.24e+18_f64).to_per_J(),
			InverseEnergy::from_per_eV(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1.6e-19,
			InverseEnergy::from_per_J(1.0_f64).to_per_eV(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(0.0009478672985781_f64).to_per_J(),
			InverseEnergy::from_per_BTU(1.0_f64).to_per_J(), 9
		);
		assert_approx_equal(
			InverseEnergy::from_per_J(1.0_f64).to_per_J() * 1055.0,
			InverseEnergy::from_per_J(1.0_f64).to_per_BTU(), 9
		);
	}

	#[test]
	fn inverse_force_units() {
		assert_approx_equal(
			InverseForce::from_per_N(0.224337566199999_f64).to_per_N(),
			InverseForce::from_per_lb(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 4.45756819483586,
			InverseForce::from_per_N(1.0_f64).to_per_lb(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(0.101971620999999_f64).to_per_N(),
			InverseForce::from_per_kgG(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 9.8066500286389,
			InverseForce::from_per_N(1.0_f64).to_per_kgG(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1000.0_f64).to_per_N(),
			InverseForce::from_per_mN(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 0.001,
			InverseForce::from_per_N(1.0_f64).to_per_mN(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1000000.0_f64).to_per_N(),
			InverseForce::from_per_uN(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 1e-06,
			InverseForce::from_per_N(1.0_f64).to_per_uN(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1000000000.0_f64).to_per_N(),
			InverseForce::from_per_nN(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 1e-09,
			InverseForce::from_per_N(1.0_f64).to_per_nN(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(0.001_f64).to_per_N(),
			InverseForce::from_per_kN(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 1000.0,
			InverseForce::from_per_N(1.0_f64).to_per_kN(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1e-06_f64).to_per_N(),
			InverseForce::from_per_MN(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 1000000.0,
			InverseForce::from_per_N(1.0_f64).to_per_MN(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1e-09_f64).to_per_N(),
			InverseForce::from_per_GN(1.0_f64).to_per_N(), 9
		);
		assert_approx_equal(
			InverseForce::from_per_N(1.0_f64).to_per_N() * 1000000000.0,
			InverseForce::from_per_N(1.0_f64).to_per_GN(), 9
		);
	}

	#[test]
	fn inverse_moment_of_inertia_units() {
		assert_approx_equal(
			InverseMomentOfInertia::from_per_kgm2(0.1_f64).to_per_kgm2(),
			InverseMomentOfInertia::from_per_gcm2(1.0_f64).to_per_kgm2(), 9
		);
		assert_approx_equal(
			InverseMomentOfInertia::from_per_kgm2(1.0_f64).to_per_kgm2() * 10.0,
			InverseMomentOfInertia::from_per_kgm2(1.0_f64).to_per_gcm2(), 9
		);
		assert_approx_equal(
			InverseMomentOfInertia::from_per_kgm2(1000.0_f64).to_per_kgm2(),
			InverseMomentOfInertia::from_per_gm2(1.0_f64).to_per_kgm2(), 9
		);
		assert_approx_equal(
			InverseMomentOfInertia::from_per_kgm2(1.0_f64).to_per_kgm2() * 0.001,
			InverseMomentOfInertia::from_per_kgm2(1.0_f64).to_per_gm2(), 9
		);
	}

	#[test]
	fn inverse_momentum_units() {
		assert_approx_equal(
			InverseMomentum::from_s_per_kgm(100000.0_f64).to_s_per_kgm(),
			InverseMomentum::from_s_per_gcm(1.0_f64).to_s_per_kgm(), 9
		);
		assert_approx_equal(
			InverseMomentum::from_s_per_kgm(1.0_f64).to_s_per_kgm() * 1e-05,
			InverseMomentum::from_s_per_kgm(1.0_f64).to_s_per_gcm(), 9
		);
		assert_approx_equal(
			InverseMomentum::from_s_per_kgm(100000.0_f64).to_s_per_kgm(),
			InverseMomentum::from_seconds_per_gram_centimeter(1.0_f64).to_s_per_kgm(), 9
		);
		assert_approx_equal(
			InverseMomentum::from_s_per_kgm(1.0_f64).to_s_per_kgm() * 1e-05,
			InverseMomentum::from_s_per_kgm(1.0_f64).to_seconds_per_gram_centimeter(), 9
		);
	}

	#[test]
	fn inverse_power_units() {
		assert_approx_equal(
			InversePower::from_per_W(1000.0_f64).to_per_W(),
			InversePower::from_per_mW(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 0.001,
			InversePower::from_per_W(1.0_f64).to_per_mW(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1000000.0_f64).to_per_W(),
			InversePower::from_per_uW(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 1e-06,
			InversePower::from_per_W(1.0_f64).to_per_uW(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1000000000.0_f64).to_per_W(),
			InversePower::from_per_nW(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 1e-09,
			InversePower::from_per_W(1.0_f64).to_per_nW(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(0.001_f64).to_per_W(),
			InversePower::from_per_kW(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 1000.0,
			InversePower::from_per_W(1.0_f64).to_per_kW(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1e-06_f64).to_per_W(),
			InversePower::from_per_MW(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 1000000.0,
			InversePower::from_per_W(1.0_f64).to_per_MW(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1e-09_f64).to_per_W(),
			InversePower::from_per_GW(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 1000000000.0,
			InversePower::from_per_W(1.0_f64).to_per_GW(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(0.0013410218586563_f64).to_per_W(),
			InversePower::from_per_horsepower(1.0_f64).to_per_W(), 9
		);
		assert_approx_equal(
			InversePower::from_per_W(1.0_f64).to_per_W() * 745.7,
			InversePower::from_per_W(1.0_f64).to_per_horsepower(), 9
		);
	}

	#[test]
	fn inverse_pressure_units() {
		assert_approx_equal(
			InversePressure::from_per_Pa(0.00014503773773_f64).to_per_Pa(),
			InversePressure::from_per_psi(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 6894.7572931783,
			InversePressure::from_per_Pa(1.0_f64).to_per_psi(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1000.0_f64).to_per_Pa(),
			InversePressure::from_per_mPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 0.001,
			InversePressure::from_per_Pa(1.0_f64).to_per_mPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1000000.0_f64).to_per_Pa(),
			InversePressure::from_per_uPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 1e-06,
			InversePressure::from_per_Pa(1.0_f64).to_per_uPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1000000000.0_f64).to_per_Pa(),
			InversePressure::from_per_nPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 1e-09,
			InversePressure::from_per_Pa(1.0_f64).to_per_nPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(0.001_f64).to_per_Pa(),
			InversePressure::from_per_kPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 1000.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_kPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1e-06_f64).to_per_Pa(),
			InversePressure::from_per_MPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 1000000.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_MPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1e-09_f64).to_per_Pa(),
			InversePressure::from_per_GPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 1000000000.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_GPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(0.01_f64).to_per_Pa(),
			InversePressure::from_per_hPa(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 100.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_hPa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1e-05_f64).to_per_Pa(),
			InversePressure::from_per_bar(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 100000.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_bar(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(0.01_f64).to_per_Pa(),
			InversePressure::from_per_mbar(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 100.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_mbar(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(9.87e-06_f64).to_per_Pa(),
			InversePressure::from_per_atm(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 101325.0,
			InversePressure::from_per_Pa(1.0_f64).to_per_atm(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(0.007500616827039_f64).to_per_Pa(),
			InversePressure::from_per_torr(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 133.3223684211,
			InversePressure::from_per_Pa(1.0_f64).to_per_torr(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(0.007500616827039_f64).to_per_Pa(),
			InversePressure::from_per_mmHg(1.0_f64).to_per_Pa(), 9
		);
		assert_approx_equal(
			InversePressure::from_per_Pa(1.0_f64).to_per_Pa() * 133.3223684211,
			InversePressure::from_per_Pa(1.0_f64).to_per_mmHg(), 9
		);
	}

	#[test]
	fn inverse_torque_units() {
		assert_approx_equal(
			InverseTorque::from_per_Nm(0.73756214927727_f64).to_per_Nm(),
			InverseTorque::from_per_ftlb(1.0_f64).to_per_Nm(), 9
		);
		assert_approx_equal(
			InverseTorque::from_per_Nm(1.0_f64).to_per_Nm() * 1.35581794833139,
			InverseTorque::from_per_Nm(1.0_f64).to_per_ftlb(), 9
		);
	}

	#[test]
	fn time_per_distance_units() {
		assert_approx_equal(
			TimePerDistance::from_spm(100.0_f64).to_spm(),
			TimePerDistance::from_s_per_cm(1.0_f64).to_spm(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(1.0_f64).to_spm() * 0.01,
			TimePerDistance::from_spm(1.0_f64).to_s_per_cm(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(1000.0_f64).to_spm(),
			TimePerDistance::from_s_per_mm(1.0_f64).to_spm(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(1.0_f64).to_spm() * 0.001,
			TimePerDistance::from_spm(1.0_f64).to_s_per_mm(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(3.6_f64).to_spm(),
			TimePerDistance::from_hr_per_km(1.0_f64).to_spm(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(1.0_f64).to_spm() * 0.277777777777778,
			TimePerDistance::from_spm(1.0_f64).to_hr_per_km(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(2.2369362920544_f64).to_spm(),
			TimePerDistance::from_hr_per_mi(1.0_f64).to_spm(), 9
		);
		assert_approx_equal(
			TimePerDistance::from_spm(1.0_f64).to_spm() * 0.44704,
			TimePerDistance::from_spm(1.0_f64).to_hr_per_mi(), 9
		);
	}

	#[test]
	fn volume_per_mass_units() {
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(0.001_f64).to_m3_per_kg(),
			VolumePerMass::from_L_per_kg(1.0_f64).to_m3_per_kg(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(1.0_f64).to_m3_per_kg() * 1000.0,
			VolumePerMass::from_m3_per_kg(1.0_f64).to_L_per_kg(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(0.001_f64).to_m3_per_kg(),
			VolumePerMass::from_liters_per_kilogram(1.0_f64).to_m3_per_kg(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(1.0_f64).to_m3_per_kg() * 1000.0,
			VolumePerMass::from_m3_per_kg(1.0_f64).to_liters_per_kilogram(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(0.001_f64).to_m3_per_kg(),
			VolumePerMass::from_cc_per_g(1.0_f64).to_m3_per_kg(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(1.0_f64).to_m3_per_kg() * 1000.0,
			VolumePerMass::from_m3_per_kg(1.0_f64).to_cc_per_g(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(0.001_f64).to_m3_per_kg(),
			VolumePerMass::from_cubic_centimeters_per_gram(1.0_f64).to_m3_per_kg(), 9
		);
		assert_approx_equal(
			VolumePerMass::from_m3_per_kg(1.0_f64).to_m3_per_kg() * 1000.0,
			VolumePerMass::from_m3_per_kg(1.0_f64).to_cubic_centimeters_per_gram(), 9
		);
	}

	#[test]
	fn inverse_absorbed_dose_units() {
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1000.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_mGy(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 0.001,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_mGy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1000000.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_uGy(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1e-06,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_uGy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1000000000.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_nGy(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1e-09,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_nGy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(0.001_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_kGy(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1000.0,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_kGy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1e-06_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_MGy(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1000000.0,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_MGy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1e-09_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_GGy(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1000000000.0,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_GGy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(100.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_rad(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 0.01,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_rad(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(0.1_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_krad(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 10.0,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_krad(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(100000.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_mrad(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1e-05,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_mrad(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(100000000.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_urad(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 1e-08,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_urad(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(10000.0_f64).to_per_Gy(),
			InverseAbsorbedDose::from_per_erg(1.0_f64).to_per_Gy(), 9
		);
		assert_approx_equal(
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_Gy() * 0.0001,
			InverseAbsorbedDose::from_per_Gy(1.0_f64).to_per_erg(), 9
		);
	}

	#[test]
	fn inverse_dose_equivalent_units() {
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1000.0_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_mSv(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 0.001,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_mSv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1000000.0_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_uSv(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 1e-06,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_uSv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1000000000.0_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_nSv(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 1e-09,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_nSv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(0.001_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_kSv(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 1000.0,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_kSv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1e-06_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_MSv(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 1000000.0,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_MSv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1e-09_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_GSv(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 1000000000.0,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_GSv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(100.0_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_rem(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 0.01,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_rem(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(100000.0_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_mrem(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 1e-05,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_mrem(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(0.1_f64).to_per_Sv(),
			InverseDoseEquivalent::from_per_krem(1.0_f64).to_per_Sv(), 9
		);
		assert_approx_equal(
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_Sv() * 10.0,
			InverseDoseEquivalent::from_per_Sv(1.0_f64).to_per_krem(), 9
		);
	}
}
