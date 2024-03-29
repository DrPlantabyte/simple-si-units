use std::fmt::{Debug, Display, Formatter};
use simple_si_units::base::*;
use simple_si_units::geometry::*;
use simple_si_units::{NumLike, UnitStruct};

/*
How it will work:
unit type structs will template <T: number-like>, but not impose and from<> restrictions
but the implemented methods which require multiplying by a coefficient (eg 1000*m for km)
will add +From<f64> to the where clause

Users who want f32 or int implementations will still be able to use the structs, but
will need to use the num crate or implement their own wrapper type that implements
From<f64>
*/


// ===== operator testing with various number types ===== //
fn templated_op_test<T: NumLike+From<f64>>() -> T{
	let w = Distance::from_meters(T::from(1.4));
	let w2 = Distance::from_meters(T::from(0.4));
	let l = Distance::from_meters(T::from(2.5));
	let h = Distance::from_meters(T::from(3.6));
	let num_subdivs = T::from(3.0);
	let vol: Volume<T> = (&w + &w2) * &l * &h;
	let unit_vol = vol / num_subdivs;
	let _aspect: T = &h / &w;
	let repetitions = T::from(25.);
	let space_efficiency = T::from(0.65);
	let mut repeated_vol = unit_vol;
	repeated_vol *= repetitions;
	repeated_vol /= space_efficiency;
	return repeated_vol.to_L();
}
#[test]
#[cfg(feature="num-bigfloat")]
pub fn num_bigfloat_test() {
	use num_bigfloat::BigFloat;
	let _ = templated_op_test::<BigFloat>();
}

// unfortunately, astro_float does not support by-value std ops, and therefore cannot be used
// #[test]
// pub fn num_astrofloat_test() {
// 	use astro_float::BigFloat;
// 	let _ = templated_op_test::<BigFloat>();
// }
#[test]
pub fn num_complex_test() {
	use num_complex::Complex;
	let _ = templated_op_test::<Complex<f64>>();
}

// ===== end of operator testing ===== //


// ===== custom unit type test ===== //
#[derive(UnitStruct, Debug, Clone)]
struct Bananas<DT>{
	pub count: DT
}
// sadly, the following is not supported by the Rust compiler:
// impl<S, D> std::convert::From<Bananas<S>> for Bananas<D>
// 	where S: NumLike, D: NumLike + std::convert::From<S> {
// 	fn from(value: Bananas<S>) -> Self {
// 		Bananas{count: D::from(value.count)}
// 	}
// }
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
#[derive(Debug,Clone)]
struct MyFloat32 {
	x: f32
}
impl MyFloat32 {
	pub fn new(n: f32) -> Self{return Self{x: n}}
}
impl Display for MyFloat32{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		return std::fmt::Display::fmt(&self.x, f);
	}
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

