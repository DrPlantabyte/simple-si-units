#![warn(missing_docs)]
//! # Simple SI Units
//! This Rust library provides compiler-checked types for the standard set of
//! SI units, as specified by the US [National Institute of
//! Standards and Technology](https://www.nist.gov/pml/owm/metric-si/si-units)
//! (this project is not officially endorsed by NIST).
//!
//! ## What's included?
//! * Official standard SI Units
//! * Common secondary units, such as velocity
//! * Implements operators to automatically convert between units with basic
//!   arithmatic (eg distance / time = velocity)
//! * Units are templated so that you can choose whether to use `f32` or `f64` or other number-like type as your concrete number type.
//! * Optional, limited integration with [uom](https://crates.io/crates/uom)
//!
//! Since these structs use generic type templates for the internal data type, you
//! can use any number-like data type with these structs, including
//! [num_complex::Complex](https://crates.io/crates/num-complex) and
//! [num_bigfloat::BigFloat](https://crates.io/crates/num-bigfloat) (see caveat
//! section below regarding primitive types other than `f64`).
//!
//! ### Units
//! This crate provides types for the following units. Other kinds of
//! quantities not listed below (eg jolt) are beyond the scope of this crate.
//!
//! #### Base SI units (and standard unit of measure):
//! * Distance, aka Length (meters)
//! * Mass (kilogram)
//! * Time (seconds)
//! * Temperature (kelvin)
//! * Amount, aka Quantity (moles)
//! * Current (amperes)
//! * Luminosity (candela)
//!
//! #### Derived units:
//! * Angle (rad)
//! * Solid Angle (sr)
//! * Frequency (1/s, aka Hz)
//! * Area (m^2)
//! * Volume (m^3)
//! * Velocity (m/s)
//! * Acceleration (m/s^2)
//! * Force (kg.m/s^2, aka N)
//! * Pressure (N/m^2, aka Pa)
//! * Energy (kg.m^2/s^2, aka J)
//! * Coulomb (A.s, aka C)
//! * Power, aka Watt (J/s, aka W)
//! * Voltage (W/A, aka V)
//! * Resistance (V/A, aka Ohm)
//! * Conductance (1/ohm, aka S)
//! * Capacitance (C/V)
//! * Inductance (Wb/A, aka H)
//! * Magnetic Flux (V.s, aka Wb)
//! * Magnetic Flux Density (Wb/m^2, aka T)
//! * Catalytic Activity (mol/s)
//! * Concentration (mol/m^3)
//! * Luminous Flux (cd.sr, aka lm)
//! * Illuminance (lm/m^2, aka lux)
//! * Radioactivity (1/s, aka Bq)
//! * Absorbed Dose (J/kg, aka Gy)
//! * Dose Equivalent (J/kg, aka Sv)
//!
//! ## What's NOT included?
//! * Not supporting dimensional analysis
//! * Not providing an exhaustive list of all possible unit types (but you can use
//!   this library to implement them yourself)
//! * Not supporting unusual number types (eg integers)
//! * Not aiming for full integration with [uom](https://crates.io/crates/uom)
//! 

pub use simple_si_units_macros::UnitStruct;
pub use simple_si_units_core::NumLike;


// START OF GENERATED CODE
// END OF GENERATED CODE

/// Unit tests
#[cfg(test)]
mod unit_tests {
	use super::*;
	/// utility function for asserting equality of decimal values with approximations
	fn assert_approx_equal(a: f64, b: f64, sigfigs: i32) {
		if a.is_nan() {
			assert!(b.is_nan());
		} else if a.is_infinite() {
			assert!(b.is_infinite() && a.is_sign_negative() == b.is_sign_positive());
		} else {
			let ypsilon = 10f64.powi(-sigfigs);
			let max_delta = (a.abs() + b.abs()) * 0.5 * ypsilon;
			assert!((a - b).abs() < max_delta);
		}
	}

	#[test]
	fn distance_units() {
		assert_approx_equal(
			Distance::from_meters(1.0_f64).to_meters(),
			Distance::from_m(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1.0_f64).to_meters(),
			Distance::from_cm(100.0_f64).to_meters(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1.0_f64).to_meters(),
			Distance::from_mm(1000.0_f64).to_meters(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1.0_f64).to_meters(),
			Distance::from_um(1e6_f64).to_meters(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1.0_f64).to_meters(),
			Distance::from_nm(1e9_f64).to_meters(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1.0_f64).to_meters(),
			Distance::from_pm(1e12_f64).to_meters(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1000.0_f64).to_meters(),
			Distance::from_km(1.0_f64).to_meters(), 9
		);
		assert_approx_equal(
			Distance::from_meters(1.495979e11_f64).to_meters(),
			Distance::from_au(1.0_f64).to_meters(), 6
		);
		assert_approx_equal(
			Distance::from_meters(9.4607e15_f64).to_meters(),
			Distance::from_lyr(1.0_f64).to_meters(), 4
		);
		assert_approx_equal(
			Distance::from_meters(3.0857e16_f64).to_meters(),
			Distance::from_parsec(1.0_f64).to_meters(), 4
		);
	}

	#[test]
	fn mass_units() {
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg(),
			Mass::from_g(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_g(1.0_f64).to_kg(),
			Mass::from_mg(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_mg(1.0_f64).to_kg(),
			Mass::from_ug(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_ug(1.0_f64).to_kg(),
			Mass::from_ng(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_ng(1.0_f64).to_kg(),
			Mass::from_pg(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_tons(1.0_f64).to_kg(),
			Mass::from_kg(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_earth_mass(1.0_f64).to_kg(),
			Mass::from_kg(5.972e24_f64).to_kg(), 3
		);
		assert_approx_equal(
			Mass::from_jupiter_mass(1.0_f64).to_kg(),
			Mass::from_kg(1.898e27_f64).to_kg(), 3
		);
		assert_approx_equal(
			Mass::from_solar_mass(1.0_f64).to_kg(),
			Mass::from_kg(1.988e30_f64).to_kg(), 3
		);
	}
	#[test]
	fn time_units() {
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s(),
			Time::from_seconds(1.0_f64).to_seconds(), 9
		);
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s(),
			Time::from_ms(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_ms(1.0_f64).to_s(),
			Time::from_us(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_us(1.0_f64).to_s(),
			Time::from_ns(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_ns(1.0_f64).to_s(),
			Time::from_ps(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_minutes(1.0_f64).to_s(),
			Time::from_s(60.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_min(1.0_f64).to_s(),
			Time::from_s(60.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_hours(1.0_f64).to_s(),
			Time::from_min(60.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_hr(1.0_f64).to_s(),
			Time::from_min(60.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_days(1.0_f64).to_s(),
			Time::from_hr(24.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_weeks(1.0_f64).to_s(),
			Time::from_days(7.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_years(1.0_f64).to_s(),
			Time::from_days(365.2425_f64).to_s(), 6
		);
		assert_approx_equal(
			Time::from_yr(1.0_f64).to_s(),
			Time::from_days(365.2425_f64).to_s(), 6
		);
		assert_approx_equal(
			Time::from_kyr(1.0_f64).to_s(),
			Time::from_yr(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_Myr(1.0_f64).to_s(),
			Time::from_yr(1e6_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_Gyr(1.0_f64).to_s(),
			Time::from_yr(1e9_f64).to_s(), 9
		);
	}
	#[test]
	fn temperature_units() {
		assert_approx_equal(
			Temperature::from_kelvin(1.0_f64).to_kelvin(),
			Temperature::from_K(1.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_K(272.15_f64).to_K(),
			Temperature::from_C(0.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_K(272.15_f64).to_K(),
			Temperature::from_celcius(0.0_f64).to_K(), 4
		);
		assert_approx_equal(
			Temperature::from_C(0.0_f64).to_K(),
			Temperature::from_F(32_f64).to_K(), 3
		);
		assert_approx_equal(
			Temperature::from_C(0.0_f64).to_K(),
			Temperature::from_fahrenheit(32_f64).to_K(), 3
		);
	}
	#[test]
	fn quantity_units() {
		assert_approx_equal(
			Quantity::from_count(6.0221415e23_f64).to_count(),
			Quantity::from_moles(1.0_f64).to_count(), 7
		);
		assert_approx_equal(
			Quantity::from_moles(1.0_f64).to_count(),
			Quantity::from_mol(1.0_f64).to_count(), 7
		);
		assert_approx_equal(
			Quantity::from_moles(1.0_f64).to_count(),
			Quantity::from_mmol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Quantity::from_mmol(1.0_f64).to_count(),
			Quantity::from_umol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Quantity::from_umol(1.0_f64).to_count(),
			Quantity::from_nmol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Quantity::from_nmol(1.0_f64).to_count(),
			Quantity::from_pmol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Quantity::from_count(12.0_f64).to_count(),
			Quantity::from_dozen(1.0_f64).to_count(), 9
		);
	}
	#[test]
	fn current_units() {
		assert_approx_equal(
			Current::from_amps(1.0_f64).to_amps(),
			Current::from_A(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A(),
			Current::from_mA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_mA(1.0_f64).to_A(),
			Current::from_uA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_uA(1.0_f64).to_A(),
			Current::from_nA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_nA(1.0_f64).to_A(),
			Current::from_pA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1000.0_f64).to_A(),
			Current::from_kA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_kA(1000.0_f64).to_A(),
			Current::from_MA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_MA(1000.0_f64).to_A(),
			Current::from_GA(1.0_f64).to_A(), 9
		);
	}
	#[test]
	fn luminosity_units() {
		assert_approx_equal(
			Luminosity::from_candela(1.0_f64).to_candela(),
			Luminosity::from_cd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd(),
			Luminosity::from_mcd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_mcd(1.0_f64).to_cd(),
			Luminosity::from_ucd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_ucd(1.0_f64).to_cd(),
			Luminosity::from_ncd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_ncd(1.0_f64).to_cd(),
			Luminosity::from_pcd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1000.0_f64).to_cd(),
			Luminosity::from_kcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_kcd(1000.0_f64).to_cd(),
			Luminosity::from_Mcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_Mcd(1000.0_f64).to_cd(),
			Luminosity::from_Gcd(1.0_f64).to_cd(), 9
		);
	}

	#[test]
	fn angle_units() {
		assert_approx_equal(
			Angle::from_radians(1.0_f64).to_radians(),
			Angle::from_rad(1.0_f64).to_rad(), 9
		);
		assert_approx_equal(
			Angle::from_deg(360.0_f64).to_rad(),
			Angle::from_rad(6.283185307179586_f64).to_rad(), 9
		);
		assert_approx_equal(
			Angle::from_degrees(360.0_f64).to_rad(),
			Angle::from_rad(6.283185307179586_f64).to_rad(), 9
		);
	}
	#[test]
	fn solid_angle_units() {
		assert_approx_equal(
			SolidAngle::from_sr(1.0_f64).to_sr(),
			SolidAngle::from_steradians(1.0_f64).to_steradians(), 9
		);
	}
	#[test]
	fn frequency_units() {
		assert_approx_equal(
			Frequency::from_hertz(1.0_f64).to_hertz(),
			Frequency::from_Hz(1.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_kHz(1.0_f64).to_Hz(),
			Frequency::from_Hz(1000.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_MHz(1.0_f64).to_Hz(),
			Frequency::from_kHz(1000.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_GHz(1.0_f64).to_Hz(),
			Frequency::from_mHz(1000.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_THz(1.0_f64).to_Hz(),
			Frequency::from_GHz(1000.0_f64).to_Hz(), 9
		);
	}
	#[test]
	fn area_units() {
		todo!();
	}
	#[test]
	fn volume_units() {
		todo!();
	}
	#[test]
	fn velocity_units() {
		todo!();
	}
	#[test]
	fn acceleration_units() {
		todo!();
	}
	#[test]
	fn force_units() {
		todo!();
	}
	#[test]
	fn pressure_units() {
		todo!();
	}
	#[test]
	fn energy_units() {
		todo!();
	}
	#[test]
	fn coulomb_units() {
		todo!();
	}
	#[test]
	fn power_units() {
		todo!();
	}
	#[test]
	fn voltage_units() {
		todo!();
	}
	#[test]
	fn resistance_units() {
		todo!();
	}
	#[test]
	fn conductance_units() {
		todo!();
	}
	#[test]
	fn capacitance_units() {
		todo!();
	}
	#[test]
	fn inductance_units() {
		todo!();
	}
	#[test]
	fn capacitance_units() {
		todo!();
	}
	#[test]
	fn magnetic_flux_units() {
		todo!();
	}
	#[test]
	fn magnetic_flux_density_units() {
		todo!();
	}
	#[test]
	fn catalytic_activity_units() {
		todo!();
	}
	#[test]
	fn concentration_units() {
		todo!();
	}
	#[test]
	fn luminous_flux_units() {
		todo!();
	}
	#[test]
	fn illuminance_units() {
		todo!();
	}
	#[test]
	fn radioactivity_units() {
		todo!();
	}
	#[test]
	fn absorbed_dose_units() {
		todo!();
	}
	#[test]
	fn dose_equivalent_units() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn distance_add_subtract() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn distance_mul_div() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn distance_op_assign() {
		// +=, -=, *=, /=
		todo!();
	}
}
