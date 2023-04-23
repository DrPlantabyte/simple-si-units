// simple-si-units version
mod simple_si_version {
	use simple_si_units::base::{Distance, Mass};
	use simple_si_units::mechanical::{Acceleration};

	pub fn calc_gravity(mass: Mass<f64>, dist: Distance<f64>) -> Acceleration<f64> {
		const G: f64 = 6.67408e-11; // m3 kg-1 s-2
		let d_squared = dist * dist;
		return Acceleration::from_mps2(G * mass.to_kg() / d_squared.to_m2())
	}

	#[test]
	fn test_gravity1() {
		let radius = Distance::from_km(6378.1);
		let mass = Mass::from_earth_mass(1.0);
		println!("simple-si-units: Earth gravity at sea-level is {}", calc_gravity(mass, radius));
	}
}

// uom version
#[cfg(feature = "uom")]
mod uom_version {
	use uom::si::f64::{Length, Mass, Acceleration};
	use uom::si::length::*;
	use uom::si::mass::*;
	use uom::si::acceleration::*;
	use uom::fmt::DisplayStyle::Abbreviation;

	pub fn calc_gravity(mass: Mass, dist: Length) -> Acceleration {
		const G: f64 = 6.67408e-11; // m3 kg-1 s-2
		let d_squared = dist * dist;
		return Acceleration::new::<meter_per_second_squared>(G * mass.value / d_squared.value)
	}

	#[test]
	fn test_gravity2() {
		let radius = Length::new::<kilometer>(6378.1);
		let mass = Mass::new::<kilogram>(5.972e24);
		println!("uom: Earth gravity at sea-level is {}",
			 calc_gravity(mass, radius).into_format_args(meter_per_second_squared, Abbreviation));
	}
}