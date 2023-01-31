//! # Simple SI Units
//! Work in progress...

use std::fmt::{Display, Formatter};
pub use simple_si_units_macros::UnitStruct;
pub use simple_si_units_core::NumLike;



// TODO: implement display for to-string representation (and have pretty version with size-aware
// unit suffixes)
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

/// Placeholder: Work in progress
#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Volume<T: NumLike>{
	pub m3: T
}
impl<T> Volume<T> where T: NumLike {
	pub fn from_cubic_meters(m3: T) -> Self{
		Volume{m3: m3}
	}
	pub fn to_cubic_meters(self) -> T{
		return self.m3;
	}
}

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
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_cm(100.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_mm(1000.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_um(1e6_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_nm(1e9_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_pm(1e12_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1000.0_f64).to_m(),
			Distance::from_km(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.495979e11_f64).to_m(),
			Distance::from_au(1.0_f64).to_m(), 6
		);
		assert_approx_equal(
			Distance::from_m(9.4607e15_f64).to_m(),
			Distance::from_lyr(1.0_f64).to_m(), 4
		);
		assert_approx_equal(
			Distance::from_m(3.0857e16_f64).to_m(),
			Distance::from_parsec(1.0_f64).to_m(), 4
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
			Temperature::from_K(272.15_f64).to_K(),
			Temperature::from_C(0.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_C(0.0_f64).to_K(),
			Temperature::from_F(32_f64).to_K(), 3
		);
	}
	#[test]
	fn quantity_units() {
		assert_approx_equal(
			Quantity::from_count(6.0221415e23_f64).to_count(),
			Quantity::from_mol(1.0_f64).to_count(), 7
		);
		assert_approx_equal(
			Quantity::from_mol(1.0_f64).to_count(),
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
	}
	#[test]
	fn current_units() {
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
			Angle::from_deg(360.0_f64).to_rad(),
			Angle::from_rad(6.283185307179586_f64).to_rad(), 9
		);
	}
	#[test]
	fn solid_angle_units() {
		assert_approx_equal(
			SolidAngle::from_sr(1.0_f64).to_sr(),
			SolidAngle::from_sr(1.0_f64).to_sr(), 9
		);
	}
	#[test]
	fn frequency_units() {
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
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2(),
			Area::from_cm2(10000.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_cm2(1.0_f64).to_m2(),
			Area::from_mm2(100.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_mm2(1.0_f64).to_m2(),
			Area::from_um2(1e6_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_um2(1.0_f64).to_m2(),
			Area::from_nm2(1e6_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_km2(1.0_f64).to_m2(),
			Area::from_m2(1e6_f64).to_m2(), 9
		);
	}
	#[test]
	fn volume_units() {
		assert_approx_equal(
			Volume::from_L(1.0_f64).to_L(),
			Volume::from_mL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_mL(1.0_f64).to_L(),
			Volume::from_uL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_uL(1.0_f64).to_L(),
			Volume::from_nL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_nL(1.0_f64).to_L(),
			Volume::from_pL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_L(1.0_f64).to_L(),
			Volume::from_cc(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_L(1.0_f64).to_L(),
			Volume::from_cm3(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_L(),
			Volume::from_L(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_kL(1.0_f64).to_L(),
			Volume::from_L(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_ML(1.0_f64).to_L(),
			Volume::from_L(1e6_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_GL(1.0_f64).to_L(),
			Volume::from_L(1e9_f64).to_L(), 9
		);
	}
	#[test]
	fn velocity_units() {
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_cmps(100.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_mmps(1000.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_mph(3600.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_mmph(1000.0_f64 * 3600.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_kph(1.0_f64).to_mps(),
			Velocity::from_mps(1000.0_f64 / 3600.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_c(1.0_f64).to_mps(),
			Velocity::from_mps(299792458_f64).to_mps(), 8
		);
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
