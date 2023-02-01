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
		let _ = Distance::from_m(1.0_f64).to_m();
		let _ = Distance::from_m(1.0_f64).to_cm();
		let _ = Distance::from_m(1.0_f64).to_mm();
		let _ = Distance::from_m(1.0_f64).to_um();
		let _ = Distance::from_m(1.0_f64).to_nm();
		let _ = Distance::from_m(1.0_f64).to_pm();
		let _ = Distance::from_m(1.0_f64).to_km();
		let _ = Distance::from_m(1.0_f64).to_au();
		let _ = Distance::from_m(1.0_f64).to_lyr();
		let _ = Distance::from_m(1.0_f64).to_parsec();
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
			Mass::from_ton(1.0_f64).to_kg(),
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
		let _ = Mass::from_kg(1.0_f64).to_kg();
		let _ = Mass::from_kg(1.0_f64).to_g();
		let _ = Mass::from_kg(1.0_f64).to_mg();
		let _ = Mass::from_kg(1.0_f64).to_ug();
		let _ = Mass::from_kg(1.0_f64).to_ng();
		let _ = Mass::from_kg(1.0_f64).to_pg();
		let _ = Mass::from_kg(1.0_f64).to_ton();
		let _ = Mass::from_kg(1.0_f64).to_earth_mass();
		let _ = Mass::from_kg(1.0_f64).to_jupiter_mass();
		let _ = Mass::from_kg(1.0_f64).to_solar_mass();
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
			Time::from_min(1.0_f64).to_s(),
			Time::from_s(60.0_f64).to_s(), 9
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
		let _ = Time::from_s(1.0_f64).to_s();
		let _ = Time::from_s(1.0_f64).to_ms();
		let _ = Time::from_s(1.0_f64).to_us();
		let _ = Time::from_s(1.0_f64).to_ns();
		let _ = Time::from_s(1.0_f64).to_ps();
		let _ = Time::from_s(1.0_f64).to_min();
		let _ = Time::from_s(1.0_f64).to_hr();
		let _ = Time::from_s(1.0_f64).to_days();
		let _ = Time::from_s(1.0_f64).to_weeks();
		let _ = Time::from_s(1.0_f64).to_yr();
		let _ = Time::from_s(1.0_f64).to_kyr();
		let _ = Time::from_s(1.0_f64).to_Myr();
		let _ = Time::from_s(1.0_f64).to_Gyr();
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
		let _ = Temperature::from_K(300_f64).to_K();
		let _ = Temperature::from_K(300_f64).to_C();
		let _ = Temperature::from_K(300_f64).to_F();
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
		let _ = Quantity::from_mol(1.0_f64).to_mol();
		let _ = Quantity::from_mol(1.0_f64).to_mmol();
		let _ = Quantity::from_mol(1.0_f64).to_umol();
		let _ = Quantity::from_mol(1.0_f64).to_nmol();
		let _ = Quantity::from_mol(1.0_f64).to_pmol();
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
		let _ = Current::from_A(1.0_f64).to_A();
		let _ = Current::from_A(1.0_f64).to_mA();
		let _ = Current::from_A(1.0_f64).to_uA();
		let _ = Current::from_A(1.0_f64).to_nA();
		let _ = Current::from_A(1.0_f64).to_kA();
		let _ = Current::from_A(1.0_f64).to_MA();
		let _ = Current::from_A(1.0_f64).to_GA();
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
		let _ = Luminosity::from_cd(1.0_f64).to_cd();
		let _ = Luminosity::from_cd(1.0_f64).to_mcd();
		let _ = Luminosity::from_cd(1.0_f64).to_ucd();
		let _ = Luminosity::from_cd(1.0_f64).to_ncd();
		let _ = Luminosity::from_cd(1.0_f64).to_kcd();
		let _ = Luminosity::from_cd(1.0_f64).to_Mcd();
		let _ = Luminosity::from_cd(1.0_f64).to_Gcd();
	}

	#[test]
	fn angle_units() {
		assert_approx_equal(
			Angle::from_deg(360.0_f64).to_rad(),
			Angle::from_rad(6.283185307179586_f64).to_rad(), 9
		);
		let _ = Angle::from_deg(360.0_f64).to_rad();
		let _ = Angle::from_deg(360.0_f64).to_deg();
	}
	#[test]
	fn solid_angle_units() {
		assert_approx_equal(
			SolidAngle::from_sr(1.0_f64).to_sr(),
			SolidAngle::from_sr(1.0_f64).to_sr(), 9
		);
	}
	#[test]
	fn angular_velocity_units() {
		assert_approx_equal(
			AnglularVelocity::from_radps(6.283185307179586_f64 / 60.0_f64).to_radps(),
			AnglularVelocity::from_rpm(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AnglularVelocity::from_radps(6.283185307179586_f64).to_radps(),
			AnglularVelocity::from_rps(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AnglularVelocity::from_degps(360.0_f64 / 60.0_f64).to_radps(),
			AnglularVelocity::from_rpm(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AnglularVelocity::from_degps(360.0_f64).to_radps(),
			AnglularVelocity::from_rps(1.0_f64).to_radps(), 9
		);
		let _ = AnglularVelocity::from_radps(1.0_f64).to_radps();
		let _ = AnglularVelocity::from_radps(1.0_f64).to_degps();
		let _ = AnglularVelocity::from_radps(1.0_f64).to_rpm();
		let _ = AnglularVelocity::from_radps(1.0_f64).to_rps();
	}
	#[test]
	fn angular_acceleration_units() {
		assert_approx_equal(
			AnglularAcceleration::from_radps2(6.283185307179586_f64 / 60.0_f64 / 60.0_f64).to_radps2(),
			AnglularAcceleration::from_rpm2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AnglularAcceleration::from_radps2(6.283185307179586_f64).to_radps2(),
			AnglularAcceleration::from_rps2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AnglularAcceleration::from_degps2(360.0_f64 / 60.0_f64 / 60.0_f64).to_radps2(),
			AnglularAcceleration::from_rpm2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AnglularAcceleration::from_degps2(360.0_f64).to_radps2(),
			AnglularAcceleration::from_rps2(1.0_f64).to_radps2(), 9
		);
		let _ = AnglularAcceleration::from_radps2(1.0_f64).to_radps2();
		let _ = AnglularAcceleration::from_radps2(1.0_f64).to_degps2();
		let _ = AnglularAcceleration::from_radps2(1.0_f64).to_rpm2();
		let _ = AnglularAcceleration::from_radps2(1.0_f64).to_rps2();}
	#[test]
	fn moment_of_inertia_units() {
		assert_approx_equal(
			MomentOfInertia::from_kgm2(1.0_f64).to_kgm2(),
			MomentOfInertia::from_gm2(1000.0_f64).to_kgm2(), 9
		);
		assert_approx_equal(
			MomentOfInertia::from_kgm2(1.0_f64).to_kgm2(),
			MomentOfInertia::from_gcm2(0.1_f64).to_kgm2(), 9
		);
		let _ = MomentOfInertia::from_kgm2(1.0_f64).to_kgm2();
		let _ = MomentOfInertia::from_kgm2(1.0_f64).to_gm2();
		let _ = MomentOfInertia::from_kgm2(1.0_f64).to_gcm2();
	}
	#[test]
	fn torque_units() {
		assert_approx_equal(
			Torque::from_Nm(1.356_f64).to_Nm(),
			Torque::from_ftlb(1.0_f64).to_ftlb(), 3
		);
		let _ = Torque::from_Nm(1.0_f64).to_Nm();
		let _ = Torque::from_Nm(1.0_f64).to_ftlb();
	}
	#[test]
	fn momentum_units() {
		assert_approx_equal(
			Momentum::from_kgmps(1.0_f64).to_kgmps(),
			Momentum::from_gcmps(10.0_f64).to_kgmps(), 9
		);
		let _ = Momentum::from_kgmps(1.0_f64).to_kgmps();
		let _ = Momentum::from_kgmps(1.0_f64).to_gcmps();
	}
	#[test]
	fn angular_momentum_units() {
		// only one unit of measure supported
		let _ = AngularMomentum::from_kgm2radps(1.0_f64).to_kgm2radps();
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
	fn area_density_units() {
		assert_approx_equal(
			AreaDensity::from_kgm2(1.0_f64).to_kgm2(),
			AreaDensity::from_gm2(1000.0_f64).to_kgm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgm2(1.0_f64).to_kgm2(),
			AreaDensity::from_gcm2(0.1_f64).to_kgm2(), 9
		);
		let _ = AreaDensity::from_kgm2(1.0_f64).to_kgm2();
		let _ = AreaDensity::from_kgm2(1.0_f64).to_gm2();
		let _ = AreaDensity::from_kgm2(1.0_f64).to_gcm2();
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
		assert_approx_equal(
			Acceleration::from_mps2(1.0_f64).to_mps2(),
			Acceleration::from_mmps2(1000.0_f64).to_mps2(), 9
		);
		assert_approx_equal(
			Acceleration::from_kmph2(1.0_f64).to_mps2(),
			Acceleration::from_mps2(1000.0_f64 / (3600.0_f64 * 3600.0_f64)).to_mps2(), 9
		);
	}
	#[test]
	fn force_units() {
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N(),
			Force::from_mN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_kgG(1.0_f64).to_N(),
			Force::from_N(9.80665_f64).to_N(), 4
		);
		assert_approx_equal(
			Force::from_kgG(1.0_f64).to_N(),
			Force::from_lb(2.2_f64).to_N(), 4
		);
		assert_approx_equal(
			Force::from_mN(1.0_f64).to_N(),
			Force::from_uN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_uN(1.0_f64).to_N(),
			Force::from_nN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_kN(1.0_f64).to_N(),
			Force::from_N(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_MN(1.0_f64).to_N(),
			Force::from_kN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_GN(1.0_f64).to_N(),
			Force::from_MN(1000.0_f64).to_N(), 9
		);
	}
	#[test]
	fn pressure_units() {
		assert_approx_equal(
			Force::from_Pa(1000.0_f64).to_Pa(),
			Force::from_kPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Force::from_hPa(10.0_f64).to_Pa(),
			Force::from_kPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Force::from_bar(1.0_f64).to_Pa(),
			Force::from_kPa(100.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Force::from_atm(1.0_f64).to_Pa(),
			Force::from_kPa(101.325_f64).to_Pa(), 3
		);
		assert_approx_equal(
			Force::from_atm(1.0_f64).to_Pa(),
			Force::from_mmHg(760_f64).to_Pa(), 3
		);
		assert_approx_equal(
			Force::from_psi(1.0_f64).to_Pa(),
			Force::from_Pa(6894.757_f64).to_Pa(), 5
		);
	}
	#[test]
	fn energy_units() {
		assert_approx_equal(
			Energy::from_J(1000.0_f64).to_J(),
			Energy::from_kJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_kcal(1.0_f64).to_J(),
			Energy::from_kJ(4.184_f64).to_J(), 3
		);
		assert_approx_equal(
			Energy::from_BTU(1.0_f64).to_J(),
			Energy::from_kJ(1.055_f64).to_J(), 3
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J(),
			Energy::from_mJ(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_mJ(1.0_f64).to_J(),
			Energy::from_uJ(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_uJ(1.0_f64).to_J(),
			Energy::from_nJ(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_kJ(1000.0_f64).to_J(),
			Energy::from_MJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_MJ(1000.0_f64).to_J(),
			Energy::from_GJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_eV(1.0_f64).to_J(),
			Energy::from_J(1.602177e-19_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_Whr(1.0_f64).to_J(),
			Energy::from_J(3600.0_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_Whr(1.0_f64).to_J(),
			Energy::from_mWhr(1000.0_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_kWhr(1.0_f64).to_J(),
			Energy::from_Whr(1000.0_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_MWhr(1.0_f64).to_J(),
			Energy::from_kWhr(1000.0_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_tonTNT(1.0_f64).to_J(),
			Energy::from_J(4.19e9_f64).to_J(), 2
		);
		assert_approx_equal(
			Energy::from_ktonTNT(1.0_f64).to_J(),
			Energy::from_tonTNT(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_MtonTNT(1.0_f64).to_J(),
			Energy::from_ktonTNT(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_GtonTNT(1.0_f64).to_J(),
			Energy::from_MtonTNT(1000.0_f64).to_J(), 9
		);
	}
	#[test]
	fn coulomb_units() {
		assert_approx_equal(
			Charge::from_C(-1.60217646_f64).to_C(),
			Charge::from_e(1.0_f64).to_C(), 7
		);
		assert_approx_equal(
			Charge::from_C(1.60217646_f64).to_C(),
			Charge::from_p(1.0_f64).to_C(), 7
		);
	}
	#[test]
	fn power_units() {
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W(),
			Power::from_mW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_mW(1.0_f64).to_W(),
			Power::from_uW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_uW(1.0_f64).to_W(),
			Power::from_nW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_kW(1.0_f64).to_W(),
			Power::from_W(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_MW(1.0_f64).to_W(),
			Power::from_kW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_GW(1.0_f64).to_W(),
			Power::from_MW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_horsepower(1.0_f64).to_W(),
			Power::from_W(745.70_f64).to_W(), 4
		);
	}
	#[test]
	fn voltage_units() {
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V(),
			Voltage::from_mV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_mV(1.0_f64).to_V(),
			Voltage::from_uV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_uV(1.0_f64).to_V(),
			Voltage::from_nV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_kV(1.0_f64).to_V(),
			Voltage::from_V(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_MV(1.0_f64).to_V(),
			Voltage::from_kV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_GV(1.0_f64).to_V(),
			Voltage::from_MV(1000.0_f64).to_V(), 9
		);
	}
	#[test]
	fn resistance_units() {
		assert_approx_equal(
			Resistance::from_ohm(1.0_f64).to_ohm(),
			Resistance::from_mohm(1000.0_f64).to_ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_mohm(1.0_f64).to_ohm(),
			Resistance::from_uohm(1000.0_f64).to_ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_uohm(1.0_f64).to_ohm(),
			Resistance::from_nohm(1000.0_f64).to_ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_kohm(1.0_f64).to_ohm(),
			Resistance::from_ohm(1000.0_f64).to_ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Mohm(1.0_f64).to_ohm(),
			Resistance::from_kohm(1000.0_f64).to_ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_Gohm(1.0_f64).to_ohm(),
			Resistance::from_Mohm(1000.0_f64).to_ohm(), 9
		);
	}
	#[test]
	fn conductance_units() {
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S(),
			Conductance::from_mS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_mS(1.0_f64).to_S(),
			Conductance::from_uS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_uS(1.0_f64).to_S(),
			Conductance::from_nS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_kS(1.0_f64).to_S(),
			Conductance::from_S(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_MS(1.0_f64).to_S(),
			Conductance::from_kS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_GS(1.0_f64).to_S(),
			Conductance::from_MS(1000.0_f64).to_S(), 9
		);
	}
	#[test]
	fn capacitance_units() {
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F(),
			Capacitance::from_mF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_mF(1.0_f64).to_F(),
			Capacitance::from_uF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_uF(1.0_f64).to_F(),
			Capacitance::from_nF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_kF(1.0_f64).to_F(),
			Capacitance::from_F(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_MF(1.0_f64).to_F(),
			Capacitance::from_kF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_GF(1.0_f64).to_F(),
			Capacitance::from_MF(1000.0_f64).to_F(), 9
		);
	}
	#[test]
	fn inductance_units() {
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H(),
			Inductance::from_mH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_mH(1.0_f64).to_H(),
			Inductance::from_uH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_uH(1.0_f64).to_H(),
			Inductance::from_nH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_kH(1.0_f64).to_H(),
			Inductance::from_H(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_MH(1.0_f64).to_H(),
			Inductance::from_kH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_GH(1.0_f64).to_H(),
			Inductance::from_MH(1000.0_f64).to_H(), 9
		);
	}
	#[test]
	fn magnetic_flux_units() {
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb(),
			MagneticFlux::from_mWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_mWb(1.0_f64).to_Wb(),
			MagneticFlux::from_uWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_uWb(1.0_f64).to_Wb(),
			MagneticFlux::from_nWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_kWb(1.0_f64).to_Wb(),
			MagneticFlux::from_Wb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_MWb(1.0_f64).to_Wb(),
			MagneticFlux::from_kWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_GWb(1.0_f64).to_Wb(),
			MagneticFlux::from_MWb(1000.0_f64).to_Wb(), 9
		);
	}
	#[test]
	fn magnetic_flux_density_units() {
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T(),
			MagneticFluxDensity::from_mT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_mT(1.0_f64).to_T(),
			MagneticFluxDensity::from_uT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_uT(1.0_f64).to_T(),
			MagneticFluxDensity::from_nT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_kT(1.0_f64).to_T(),
			MagneticFluxDensity::from_T(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_MT(1.0_f64).to_T(),
			MagneticFluxDensity::from_kT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_GT(1.0_f64).to_T(),
			MagneticFluxDensity::from_MT(1000.0_f64).to_T(), 9
		);
	}
	#[test]
	fn catalytic_activity_units() {
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps(),
			CatalyticActivity::from_mmolps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_mmolps(1.0_f64).to_molps(),
			CatalyticActivity::from_umolps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_umolps(1.0_f64).to_molps(),
			CatalyticActivity::from_nmolps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_kmolps(1.0_f64).to_molps(),
			CatalyticActivity::from_molps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_Mmolps(1.0_f64).to_molps(),
			CatalyticActivity::from_kmolps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_Gmolps(1.0_f64).to_molps(),
			CatalyticActivity::from_Mmolps(1000.0_f64).to_molps(), 9
		);
	}
	#[test]
	fn concentration_units() {
		assert_approx_equal(
			Concentration::from_M(1.0_f64).to_M(),
			Concentration::from_mM(1000.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_mM(1.0_f64).to_M(),
			Concentration::from_uM(1000.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_uM(1.0_f64).to_M(),
			Concentration::from_nM(1000.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_count_per_L(6.02214e23_f64).to_M(),
			Concentration::from_M(1.0_f64).to_M(), 5
		);
		assert_approx_equal(
			Concentration::from_count_per_m3(6.02214e23_f64).to_M(),
			Concentration::from_mM(1.0_f64).to_M(), 5
		);
		assert_approx_equal(
			Concentration::from_count_per_cc(6.02214e23_f64).to_M(),
			Concentration::from_M(1000.0_f64).to_M(), 5
		);
	}
	#[test]
	fn luminous_flux_units() {
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm(),
			LuminousFlux::from_mlm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_mlm(1.0_f64).to_lm(),
			LuminousFlux::from_ulm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_ulm(1.0_f64).to_lm(),
			LuminousFlux::from_nlm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_klm(1.0_f64).to_lm(),
			LuminousFlux::from_lm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_Mlm(1.0_f64).to_lm(),
			LuminousFlux::from_klm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_Glm(1.0_f64).to_lm(),
			LuminousFlux::from_Mlm(1000.0_f64).to_lm(), 9
		);
	}
	#[test]
	fn illuminance_units() {
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux(),
			Illuminance::from_mlux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_mlux(1.0_f64).to_lux(),
			Illuminance::from_ulux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_ulux(1.0_f64).to_lux(),
			Illuminance::from_nlux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_klux(1.0_f64).to_lux(),
			Illuminance::from_lux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_Mlux(1.0_f64).to_lux(),
			Illuminance::from_klux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_Glux(1.0_f64).to_lux(),
			Illuminance::from_Mlux(1000.0_f64).to_lux(), 9
		);
	}
	#[test]
	fn radioactivity_units() {
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq(),
			Radioactivity::from_mBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_mBq(1.0_f64).to_Bq(),
			Radioactivity::from_uBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_uBq(1.0_f64).to_Bq(),
			Radioactivity::from_nBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_kBq(1.0_f64).to_Bq(),
			Radioactivity::from_Bq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_MBq(1.0_f64).to_Bq(),
			Radioactivity::from_kBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_GBq(1.0_f64).to_Bq(),
			Radioactivity::from_MBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Ci(1.0_f64).to_Bq(),
			Radioactivity::from_GBq(37.0_f64).to_Bq(), 2
		);
		assert_approx_equal(
			Radioactivity::from_Rd(1.0_f64).to_Bq(),
			Radioactivity::from_MBq(1.0_f64).to_Bq(), 9
		);
	}
	#[test]
	fn absorbed_dose_units() {
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy(),
			AbsorbedDose::from_mGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_mGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_uGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_uGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_nGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_kGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_Gy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_MGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_kGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_GGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_MGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(0.01_f64).to_Gy(),
			AbsorbedDose::from_rad(1.0_f64).to_Gy(), 3
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1e-4_f64).to_Gy(),
			AbsorbedDose::from_erg(1.0_f64).to_Gy(), 3
		);
		assert_approx_equal(
			AbsorbedDose::from_rad(1.0_f64).to_rad(),
			AbsorbedDose::from_mrad(1000.0_f64).to_rad(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_krad(1.0_f64).to_rad(),
			AbsorbedDose::from_rad(1000.0_f64).to_rad(), 9
		);
	}
	#[test]
	fn dose_equivalent_units() {
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv(),
			DoseEquivalent::from_mSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_mSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_uSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_uSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_nSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_kSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_Sv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_MSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_kSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_GSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_MSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_rem(1.0_f64).to_Sv(),
			DoseEquivalent::from_Sv(0.01_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_rem(1.0_f64).to_Sv(),
			DoseEquivalent::from_mrem(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_krem(1.0_f64).to_Sv(),
			DoseEquivalent::from_rem(1000.0_f64).to_Sv(), 9
		);
	}
	#[test]
	fn unit_conversion_test(){
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn add_subtract_test() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn mul_div_test() {
		todo!();
	}
	/// Placeholder: Work in progress
	#[test]
	fn op_assign_test() {
		// +=, -=, *=, /=
		todo!();
	}
}
