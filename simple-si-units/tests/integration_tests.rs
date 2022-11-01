use simple_si_units::{Unit, UnitData};


#[derive(Unit, Debug, Copy, Clone)]
struct Bananas<DT>{
	pub count: DT
}

/// misc experimenting
fn some_math<DT: simple_si_units_core::UnitData>(a: DT, b: DT) -> DT {
	return a + b;
}



#[derive(Unit, Debug, Copy, Clone)]
struct HyperVelocity<T: UnitData>{
	square_meters_per_second: T
}

fn weighted_sum<T: UnitData>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T> where
	T:UnitData + From<f64>
{
	return weight*a + (1.-weight)*b;
}

#[derive(Unit, Debug, Copy, Clone)]
struct Mass<T: UnitData>{
	pub kg: T
}
impl<T> Mass<T> where T: UnitData {
	const EARTH_MASS_KG: T = T::from(5.972e24);
	fn from_earth_mass(earth_mass: T) -> Self {
		Mass{kg: Self::EARTH_MASS_KG * earth_mass}
	}
}

#[derive(Unit, Debug, Copy, Clone)]
struct Distance<T: UnitData>{
	pub meters: T
}

impl<T> Distance<T> where T: UnitData {
	const AU_M: T = T::from(1.495979e11);
	fn from_au(au: T) -> Self{
		Distance{meters: Self::AU_M * au}
	}
}

#[derive(Unit, Debug, Copy, Clone)]
struct Time<T: UnitData>{
	pub seconds: T
}

#[derive(Unit, Debug, Copy, Clone)]
struct Velocity<T: UnitData>{
	pub mps: T
}

#[derive(Unit, Debug, Copy, Clone)]
struct Acceleration<T: UnitData>{
	pub mps2: T
}

impl<T> std::ops::Div<Time<T>> for Distance<T> where T: UnitData {
	type Output = Velocity<T>;

	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.meters / rhs.seconds}
	}
}

impl<T> std::ops::Div<Time<T>> for Velocity<T> where T: UnitData {
	type Output = Acceleration<T>;

	fn div(self, rhs: Time<T>) -> Self::Output {
		Acceleration{mps2: self.mps / rhs.seconds}
	}
}

#[derive(Debug, Clone)]
struct MassPoint {
	mass: Mass<f64>,
	pos: [Distance<f64>; 2],
	vel: [Velocity<f64>; 2],
	accel: [Acceleration<f64>; 2],
}

const G: f64 = 6.67408e-1; // m3 kg-1 s-2

struct LCGRand {
	seed: u64
}
impl LCGRand {
	fn rand_f64(mut self) -> f64 {
		self.seed = (self.seed * 0x5DEECE66Du64 + 0xBu64) & 0xFFFFFFFFFFFFu64;
		return (self.seed & 0xFFFFFFFFu64) as f64 / 0xFFFFFFFFu64 as f64;
	}
}

fn populate_system() -> Vec<MassPoint> {
	let prng = LCGRand{seed: 1234876};
	let mut system: Vec<MassPoint> = Vec::new();
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

/*
// if only this worked:
#[derive(Debug)]
struct PowerUnit< const N: i32> {
	pub value: f64
}
fn mul<const N: i32, const M: i32, const O: i32>(a: PowerUnit<N>, b: PowerUnit<M>) -> PowerUnit<O>
where O: N+M
{
	let c: PowerUnit<O> = PowerUnit{
		value: a.value * b.value,
	};
	return c;
}

 */


#[test]
pub fn test_macro_reexport() {
	let b1 = Bananas{count: 3};
	let b2 = Bananas{count: 1};
	println!("b1 - b2 = {:?}", b1 - b2);
	println!("some_math(2.3, 1.0) = {:?}", some_math(2.3, 1.0));
}
#[test]
pub fn placeholder_test() {
	//  placeholder to ensure we fail the testing phase until tests are done
	assert_eq!(1, 2)
}