use std::io::Write;
use std::time::Duration;
use simple_si_units::{UnitStruct, NumLike};

#[test]
pub fn placeholder_test() {
	//  placeholder to ensure we fail the testing phase until tests are done
	assert_eq!(1, 2)
}

// ===== ASTROPHYSICS SIMULATION ===== //
#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Distance<T: NumLike>{ // TODO: remove
	pub m: T
}
impl<T> Distance<T> where T: NumLike {
	pub fn from_meters(m: T) -> Self{
		Distance{m}
	}
	pub fn to_meters(self) -> T{
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
#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Mass<T: NumLike>{ // TODO: remove
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


#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Area<T: NumLike>{ // TODO: remove
	pub m2: T
}

impl<T> Area<T> where T: NumLike+From<f64>+Into<f64> {
	pub fn sqrt(self) -> Distance<T> {
		return Distance{m: T::from(f64::sqrt(self.m2.into()))};
	}
}

#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Time<T: NumLike>{ // TODO: remove
	pub s: T
}
impl<T> Time<T> where T: NumLike+From<f64> {
	fn from_days(d: T) -> Self{
		let day_s = T::from(86400f64);
		Time{s: day_s * d}
	}
}

#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Velocity<T: NumLike>{ // TODO: remove
	pub mps: T
}

#[derive(UnitStruct, Debug, Copy, Clone)]
pub struct Acceleration<T: NumLike>{ // TODO: remove
	pub mps2: T
}

impl<T> std::ops::Mul<Distance<T>> for Distance<T> where T: NumLike {
	type Output = Area<T>;
	fn mul(self, rhs: Distance<T>) -> Self::Output {
		Area{m2: self.m * rhs.m}
	}
}
impl<T> std::ops::Div<Time<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;

	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.m / rhs.s}
	}
}

impl<T> std::ops::Div<Time<T>> for Velocity<T> where T: NumLike {
	type Output = Acceleration<T>;

	fn div(self, rhs: Time<T>) -> Self::Output {
		Acceleration{mps2: self.mps / rhs.s}
	}
}

impl<T> std::ops::Mul<Time<T>> for Velocity<T> where T: NumLike {
	type Output = Distance<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Distance{m: self.mps * rhs.s}
	}
}
impl<T> std::ops::Mul<Time<T>> for Acceleration<T> where T: NumLike {
	type Output = Velocity<T>;
	fn mul(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.mps2 * rhs.s}
	}
}

#[derive(Debug, Clone)]
struct MassPoint {
	mass: Mass<f64>,
	pos: [Distance<f64>; 2],
	vel: [Velocity<f64>; 2],
	accel: [Acceleration<f64>; 2],
}

struct LCGRand {
	seed: u64
}
impl LCGRand {
	fn rand_f64(&mut self) -> f64 {
		self.seed = (self.seed.overflowing_mul(0x5DEECE66Du64).0.overflowing_add(0xBu64).0) & 0xFFFFFFFFFFFFu64;
		return (self.seed & 0xFFFFFFFFu64) as f64 / 0xFFFFFFFFu64 as f64;
	}
}

fn populate_system() -> Vec<MassPoint> {
	let mut prng = LCGRand{seed: 12348768};
	//
	let mut system: Vec<MassPoint> = Vec::new();
	system.push(MassPoint{
		mass: Mass::from_solar_mass(prng.rand_f64() + 0.5),
		pos: [Distance{m: 0.}, Distance{m: 0.}],
		vel: [Velocity{mps: 0.}, Velocity{mps: 0.}],
		accel: [Acceleration{mps2: 0.}, Acceleration{mps2: 0.}]
	});
	for _ in 0..12 {
		system.push(MassPoint{
			mass: Mass::from_earth_mass(prng.rand_f64() * 10.),
			pos: [Distance::from_au(prng.rand_f64() * 20. - 10.),
				Distance::from_au(prng.rand_f64() * 20. - 10.)],
			vel: [Velocity{mps: 0.}, Velocity{mps: 0.}],
			accel: [Acceleration{mps2: 0.}, Acceleration{mps2: 0.}]
		});
	}
	return system;
}

fn calc_gravity_at(mass: &MassPoint, masses: &[MassPoint]) -> [Acceleration<f64>; 2] {
	use std::ptr;
	const G: f64 = 6.67408e-11; // m3 kg-1 s-2
	let mut net_accel = [Acceleration{mps2: 0f64}, Acceleration{mps2: 0f64}];
	for mp in masses {
		if ptr::eq(mass, mp) {
			continue;
		}
		let pos = mass.pos;
		let mut dsqr = Area{m2: 0.};
		for i in 0..2 {
			let di = pos[i] - mp.pos[i];
			dsqr += di * di;
		}
		let d = dsqr.sqrt();
		let nvec = [(mp.pos[0] - pos[0]).m / d.m, (mp.pos[1] - pos[1]).m / d.m];
		let A = Acceleration{mps2: G * mp.mass.kg / dsqr.m2};
		for i in 0..2 {
			net_accel[i] = net_accel[i] + nvec[i] * A;
		}
	}
	return net_accel;
}

fn print_system(masses: &[MassPoint]) {
	let mut rows: Vec<Vec<char>> = vec![vec!['.'; 20]; 20];
	let mut i = 0;
	for mp in masses {
		let textx = 10 + (mp.pos[0].to_au()) as i32;
		let texty = 10 + (mp.pos[1].to_au()) as i32;
		if textx > 0 && textx < 20 && texty > 0 && texty < 20 {
			rows[(19-texty) as usize][textx as usize] = match i { 0 => '*', _ => 'o'};
		}
		i += 1;
	}
	for row in rows {
		let row_str = std::string::String::from_iter(row.iter());
		println!("{}", row_str);
	}
	println!();
}

#[test]
pub fn test_gravity_sim() {
	// use std::thread;
	let timestep = Time::from_days(1.);
	let num_iters = 300;
	let mut system = populate_system();
	// print visualization
	println!("Before:");
	print_system(&system);
	for _ in 0..num_iters {
		// set accel
		for n in 0..system.len() {
			system[n].accel = calc_gravity_at(&system[n], &system);
		}
		// set velocity
		for n in 0..system.len() {
			for i in 0..2 {
				system[n].vel[i] = system[n].vel[i] + system[n].accel[i] * timestep;
			}
		}
		//print_system(&system);
		// set position
		for n in 0..system.len() {
			for i in 0..2 {
				system[n].pos[i] = system[n].pos[i] + system[n].vel[i] * timestep;
			}
		}
		//print_system(&system);
		// thread::sleep(Duration::from_millis(100));

	}
	// print visualization
	println!("");
	println!("After:");
	print_system(&system);
}
// ===== end of ASTROPHYSICS SIMULATION ===== //
