use std::ops::*;
use std::fmt::{Display, Formatter, Result};
use simple_si_units::base::Mass;
use simple_si_units::geometry::Volume;
use simple_si_units::mechanical::Density;

#[derive(Debug, Copy, Clone)]
struct MyNumber(f32);

impl Display for MyNumber {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {std::fmt::Display::fmt(&self.0, f)}
}
impl Add for MyNumber {
	type Output = MyNumber;
	fn add(self, rhs: Self) -> Self::Output {MyNumber(self.0 + rhs.0)}
}
impl AddAssign for MyNumber {
	fn add_assign(&mut self, rhs: Self) {self.0 += rhs.0;}
}
impl Sub for MyNumber {
	type Output = MyNumber;
	fn sub(self, rhs: Self) -> Self::Output {MyNumber(self.0 - rhs.0)}
}
impl SubAssign for MyNumber {
	fn sub_assign(&mut self, rhs: Self) {self.0 -= rhs.0;}
}
impl Mul for MyNumber {
	type Output = MyNumber;
	fn mul(self, rhs: Self) -> Self::Output {MyNumber(self.0 * rhs.0)}
}
impl MulAssign for MyNumber {
	fn mul_assign(&mut self, rhs: Self) {self.0 *= rhs.0;}
}
impl Div for MyNumber {
	type Output = MyNumber;
	fn div(self, rhs: Self) -> Self::Output {MyNumber(self.0 / rhs.0)}
}
impl DivAssign for MyNumber {
	fn div_assign(&mut self, rhs: Self) {self.0 /= rhs.0;}
}
impl Neg for MyNumber {
	type Output = MyNumber;
	fn neg(self) -> Self::Output {MyNumber(-self.0)}
}
impl From<f64> for MyNumber{
	fn from(value: f64) -> Self {MyNumber(value as f32)}
}

fn my_fn() -> Density<MyNumber>{
	let m = Mass::from_g(MyNumber(1222.5_f32));
	let v = Volume::from_L(MyNumber(11.3_f32));
	return m / v;
}

#[test]
fn test_mynumber() {
	let _ = my_fn();
}