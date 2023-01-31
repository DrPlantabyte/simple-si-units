use num_complex::ComplexFloat;
use simple_si_units::*;

/*
How it will work:
unit type structs will template <T: number-like>, but not impose and from<> restrictions
but the implemented methods which require multiplying by a coefficient (eg 1000*m for km)
will add +From<f64> to the where clause

Users who want f32 or int implementations will still be able to use the structs, but
will need to use the num crate or implement their own wrapper type that implements
From<f64>
*/



// ===== SI unit coverage tests ===== //
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
fn basic_si_unit_coverage_test() {
	// Distance
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
	// Mass
	todo!();
	// Time
	todo!();
	// Temperature
	todo!();
	// Amount
	todo!();
	// Current
	todo!();
	// Luminosity
	todo!();
}
#[test]
fn derived_si_unit_coverage_test() {
	// Angle (rad)
	todo!();
	// SolidAngle (sr)
	todo!();
	// Frequency (1/s)
	todo!();
	// Area (m^2)
	todo!();
	// Volume (m^3)
	todo!();
	// Velocity (m/s)
	todo!();
	// Acceleration (m/s^2)
	todo!();
	// Force (kg.m/s^2, aka N)
	todo!();
	// Pressure (N/m^2, aka Pa)
	todo!();
	// Energy (kg.m^2/s^2, aka J)
	todo!();
	// Coulomb (A.s, aka C)
	todo!();
	// Watt (J/s, aka W)
	todo!();
	// Voltage (W/A, aka V)
	todo!();
	// Resistance (V/A, aka Ohm)
	todo!();
	// Conductance (1/ohm, aka S)
	todo!();
	// Capacitance (C/V)
	todo!();
	// Inductance (Wb/A, aka H)
	todo!();
	// Magnetic Flux (V.s, aka Wb)
	todo!();
	// Magnetic Flux Density (Wb/m^2, aka T)
	todo!();
	// Catalytic Activity (mol/s)
	todo!();
	// Concentration (mol/m^3)
	todo!();
	// Luminous Flux (cd.sr, aka lm)
	todo!();
	// Illuminance (lm/m^2, aka lux)
	todo!();
	// Radioactivity (1/s, aka Bq)
	todo!();
	// Absorbed Dose (J/kg, aka Gy)
	todo!();
	// Dose Equivalent (J/kg, aka Sv)
	todo!();
}
// ===== end of SI unit coverage tests ===== //

// ===== operator testing with various number types ===== //
fn templated_op_test<T: NumLike+From<f64>>() -> T{
	let w = Distance::from_meters(T::from(1.4));
	let w2 = Distance::from_meters(T::from(0.4));
	let l = Distance::from_meters(T::from(2.5));
	let h = Distance::from_meters(T::from(3.6));
	let num_subdivs = T::from(3.0);
	let vol: Volume<T> = (w + w2) * l * h;
	let unit_vol = vol / num_subdivs;
	let _aspect: T = h / w;
	let repetitions = T::from(25.);
	let space_efficiency = T::from(0.65);
	let mut repeated_vol = unit_vol;
	repeated_vol *= repetitions;
	repeated_vol /= space_efficiency;
	return unit_vol;
}
#[test]
pub fn num_bigfloat_test() {
	use num_bigfloat::BigFloat;
	let _ = templated_op_test::<BigFloat>();
}
#[test]
pub fn num_complex_test() {
	use num_complex::Complex;
	let _ = templated_op_test::<Complex<f64>>();
}

#[test]
pub fn placeholder_test() {
	//  placeholder to ensure we fail the testing phase until tests are done
	assert_eq!(1, 2)
}
// ===== end of operator testing ===== //


// ===== custom unit type test ===== //
#[derive(UnitStruct, Debug, Copy, Clone)]
struct Bananas<DT>{
	pub count: DT
}
fn some_math<DT: simple_si_units_core::NumLike>(a: DT, b: DT) -> DT {
	return a + b;
}
#[test]
pub fn macro_reexport_test() {
	let b1 = Bananas{count: 3};
	let b2 = Bananas{count: 1};
	println!("b1 - b2 = {:?}", b1 - b2);
	println!("some_math(2.3, 1.0) = {:?}", some_math(2.3, 1.0));
}
// ===== end of custom unit type test ===== //

// ===== float32 wrapper test ===== //
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
impl std::ops::AddAssign<Self> for MyFloat32 {
	fn add_assign(&mut self, rhs: Self){*self = Self{ x: self.x + rhs.x }}
}
impl std::ops::Sub<Self> for MyFloat32 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {Self{ x: self.x - rhs.x }}
}
impl std::ops::SubAssign<Self> for MyFloat32 {
	fn sub_assign(&mut self, rhs: Self){*self = Self{ x: self.x - rhs.x }}
}
impl std::ops::Div<Self> for MyFloat32 {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {Self{ x: self.x / rhs.x}}
}
impl std::ops::Mul<Self> for MyFloat32 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {Self{ x: self.x * rhs.x }}
}impl std::ops::Neg for MyFloat32 {
	type Output = Self;
	fn neg(self) -> Self::Output {Self{ x: -self.x}}
}
impl std::ops::MulAssign for MyFloat32 {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
	}
}
impl std::ops::DivAssign for MyFloat32 {
	fn div_assign(&mut self, rhs: Self) {
		self.x /= rhs.x;
	}
}
impl<T> Bananas<T> where T: NumLike+From<f64> {
	fn from_bunch(b: T) -> Self{
		let bunch_size = T::from(8f64);
		Bananas{count: bunch_size * b}
	}
}
#[test]
fn float_wrapper_test(){
	let b = Bananas::from_bunch(MyFloat32::new(1.5f32));
	let b2 = Bananas::from_bunch(MyFloat32::new(0.75f32));
	let scale_factor = MyFloat32{x: 2.5};
	let _total = (b + b2) * scale_factor;
}
// ===== end of float wrapper test ===== //

