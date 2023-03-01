use simple_si_units::base::{Distance, Mass};
use simple_si_units::mechanical::{Acceleration};
pub fn calc_gravity(mass: Mass<f64>, dist: Distance<f64>) -> Acceleration<f64>{
	const G: f64 = 6.67408e-11; // m3 kg-1 s-2
	let d_squared = dist * dist;
	return Acceleration::from_mps2(G * mass.to_kg() / d_squared.to_m2())
}

#[test]
fn test_earth_orbit_gravity(){
	let a = calc_gravity(Mass::from_solar_mass(1.0), Distance::from_au(1.0));
	println!("Solar gravity at Earth orbital distance: {}", a);
}