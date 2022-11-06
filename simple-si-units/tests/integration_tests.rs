use simple_si_units::{UnitStruct, NumLike};

/*
How it will work:
unit type structs will template <T: number-like>, but not impose and from<> restrictions
but the implemented methods which require multiplying by a coefficient (eg 1000*m for km)
will add +From<f64> to the where clause

Users who want f32 or int implementations will still be able to use the structs, but
will need to use the num crate or implement their own wrapper type that implements
From<f64>
*/

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Bananas<DT>{
	pub count: DT
}

/// misc experimenting
fn some_math<DT: simple_si_units_core::NumLike>(a: DT, b: DT) -> DT {
	return a + b;
}



#[derive(UnitStruct, Debug, Copy, Clone)]
struct HyperVelocity<T: NumLike>{
	square_meters_per_second: T
}

fn weighted_sum<T: NumLike>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T> where
	T:NumLike + From<f64>
{
	return weight*a + (1.-weight)*b;
}

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Mass<T: NumLike>{
	pub kg: T
}
impl<T> Mass<T> where T: NumLike+From<f64> {
	fn from_earth_mass(earth_mass: T) -> Self {
		let earth_mass_kg: T = T::from(5.972e24f64);
		Mass{kg: earth_mass_kg * earth_mass}
	}
	fn from_solar_mass(sun_mass: T) -> Self {
		let sun_mass_kg: T = T::from(1.989e30f64);
		Mass{kg: sun_mass_kg * sun_mass}
	}
	fn from_g(g: T) -> Self {
		let c: T = T::from(1e-3f64);
		Mass{kg: c * g}
	}
}

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Distance<T: NumLike>{
	pub meters: T
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	fn from_au(au: T) -> Self{
		let au_m = T::from(1.495979e11f64);
		Distance{meters: au_m * au}
	}
}
// impl Distance<f32>  {
// 	const AU_M_f32: f32 = 1.495979e11 as f32;
// 	fn from_au(au: f32) -> Self{
// 		Distance{meters: Self::AU_M_f32 * au}
// 	}
// }

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Time<T: NumLike>{
	pub seconds: T
}
impl<T> Time<T> where T: NumLike+From<f64> {
	fn from_days(d: T) -> Self{
		let day_s = T::from(86400f64);
		Time{seconds: day_s * d}
	}
}

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Velocity<T: NumLike>{
	pub mps: T
}

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Acceleration<T: NumLike>{
	pub mps2: T
}

impl<T> std::ops::Div<Time<T>> for Distance<T> where T: NumLike {
	type Output = Velocity<T>;

	fn div(self, rhs: Time<T>) -> Self::Output {
		Velocity{mps: self.meters / rhs.seconds}
	}
}

impl<T> std::ops::Div<Time<T>> for Velocity<T> where T: NumLike {
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


struct LCGRand {
	seed: u64
}
impl LCGRand {
	fn rand_f64(&mut self) -> f64 {
		self.seed = (self.seed * 0x5DEECE66Du64 + 0xBu64) & 0xFFFFFFFFFFFFu64;
		return (self.seed & 0xFFFFFFFFu64) as f64 / 0xFFFFFFFFu64 as f64;
	}
}

struct MyFloat32 {
	x: f32
}
impl MyFloat32 {
	pub fn new(n: f32) -> Self{return Self{x: n}}
}
impl From<f64> for MyFloat32 {
	fn from(n: f64) -> Self {return Self::new(n as f32)}
}
impl std::ops::Add<Self> for MyFloat32 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {Self{ x: self.x + rhs.x }}
}
impl std::ops::Sub<Self> for MyFloat32 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {Self{ x: self.x - rhs.x }}
}
impl std::ops::Div<Self> for MyFloat32 {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {Self{ x: self.x / rhs.x}}
}
impl std::ops::Mul<Self> for MyFloat32 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {Self{ x: self.x * rhs.x }}
}
fn my_fn() -> Mass<MyFloat32>{
	let m = Mass::from_g(MyFloat32::new(1100_f32));
	return m * MyFloat32::new(0.5);
}

fn populate_system() -> Vec<MassPoint> {
	let mut prng = LCGRand{seed: 1234876};
	//
	let _unused = Distance{meters: 11.11f32};
	use num_bigfloat::BigFloat;
	let _unused2 = Distance{meters: BigFloat::from(123.456)};
	use num_complex::Complex;
	let _unused3 = Distance{meters: Complex::from(123.456)};
	// let _unused4 = Mass::from_earth_mass((prng.rand_f64() * 10.) as f32);
	let _unused4 = Mass::from_earth_mass(MyFloat32::new(123.456f32));
	let _unused5 = my_fn();
	//
	let mut system: Vec<MassPoint> = Vec::new();
	system.push(MassPoint{
		mass: Mass::from_solar_mass(prng.rand_f64() + 0.5),
		pos: [Distance{meters: 0.}, Distance{meters: 0.}],
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



#[test]
pub fn test_gravity_sim() {

	const G: f64 = 6.67408e-1; // m3 kg-1 s-2
	let timestep = Time::from_days(1.);
	let num_iters = 100;
	let system = populate_system();
	for _ in 0..num_iters {
		
	}
	todo!()
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