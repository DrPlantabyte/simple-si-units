use macrotest;
use simple_si_units_core::NumLike;
// expand macros with this command:
// rm .\tests\expand\*.expanded.rs; cargo +nightly test --test macro_tests


#[test]
pub fn expand_macros() {
	macrotest::expand("tests/expand/*.rs")
}

///// expanded output pasted here for compiler test
#[derive(Clone)]
pub struct MyUnit<DT: NumLike> {
	v: DT,
}
impl<DT: simple_si_units_core::NumLike> std::ops::Add<Self> for MyUnit<DT> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		return Self { v: self.v + rhs.v };
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::AddAssign for MyUnit<DT> {
	fn add_assign(&mut self, rhs: Self) {
		self.v += rhs.v;
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Sub<Self> for MyUnit<DT> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		return Self { v: self.v - rhs.v };
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::SubAssign for MyUnit<DT> {
	fn sub_assign(&mut self, rhs: Self) {
		self.v -= rhs.v;
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Div<Self> for MyUnit<DT> {
	type Output = DT;
	fn div(self, rhs: Self) -> Self::Output {
		return self.v / rhs.v;
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Div<DT> for MyUnit<DT> {
	type Output = Self;
	fn div(self, rhs: DT) -> Self::Output {
		return Self { v: self.v / rhs };
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::DivAssign<DT> for MyUnit<DT> {
	fn div_assign(&mut self, rhs: DT) {
		self.v /= rhs;
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Mul<DT> for MyUnit<DT> {
	type Output = Self;
	fn mul(self, rhs: DT) -> Self::Output {
		return Self { v: self.v * rhs };
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::MulAssign<DT> for MyUnit<DT> {
	fn mul_assign(&mut self, rhs: DT) {
		self.v *= rhs;
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for f64
	where
		DT: simple_si_units_core::NumLike + From<f64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for f32
	where
		DT: simple_si_units_core::NumLike + From<f32>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for u8
	where
		DT: simple_si_units_core::NumLike + From<u8>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for i8
	where
		DT: simple_si_units_core::NumLike + From<i8>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for u16
	where
		DT: simple_si_units_core::NumLike + From<u16>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for i16
	where
		DT: simple_si_units_core::NumLike + From<i16>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for u32
	where
		DT: simple_si_units_core::NumLike + From<u32>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for i32
	where
		DT: simple_si_units_core::NumLike + From<i32>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for u64
	where
		DT: simple_si_units_core::NumLike + From<u64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for i64
	where
		DT: simple_si_units_core::NumLike + From<i64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Neg for MyUnit<DT> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		return Self { v: self.v.neg() };
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Add<Self> for &MyUnit<DT> {
	type Output = MyUnit<DT>;
	fn add(self, rhs: Self) -> Self::Output {
		return Self::Output {
			v: self.v.clone() + rhs.v.clone(),
		};
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Sub<Self> for &MyUnit<DT> {
	type Output = MyUnit<DT>;
	fn sub(self, rhs: Self) -> Self::Output {
		return Self::Output {
			v: self.v.clone() - rhs.v.clone(),
		};
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Div<Self> for &MyUnit<DT> {
	type Output = DT;
	fn div(self, rhs: Self) -> Self::Output {
		return self.v.clone() / rhs.v.clone();
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Div<DT> for &MyUnit<DT> {
	type Output = MyUnit<DT>;
	fn div(self, rhs: DT) -> Self::Output {
		return Self::Output {
			v: self.v.clone() / rhs,
		};
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Mul<DT> for &MyUnit<DT> {
	type Output = MyUnit<DT>;
	fn mul(self, rhs: DT) -> Self::Output {
		return Self::Output {
			v: self.v.clone() * rhs,
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for f64
	where
		DT: simple_si_units_core::NumLike + From<f64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for f32
	where
		DT: simple_si_units_core::NumLike + From<f32>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for u8
	where
		DT: simple_si_units_core::NumLike + From<u8>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for i8
	where
		DT: simple_si_units_core::NumLike + From<i8>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for u16
	where
		DT: simple_si_units_core::NumLike + From<u16>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for i16
	where
		DT: simple_si_units_core::NumLike + From<i16>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for u32
	where
		DT: simple_si_units_core::NumLike + From<u32>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for i32
	where
		DT: simple_si_units_core::NumLike + From<i32>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for u64
	where
		DT: simple_si_units_core::NumLike + From<u64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT> std::ops::Mul<&MyUnit<DT>> for i64
	where
		DT: simple_si_units_core::NumLike + From<i64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: &MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v.clone(),
		};
	}
}
impl<DT: simple_si_units_core::NumLike> std::ops::Neg for &MyUnit<DT> {
	type Output = MyUnit<DT>;
	fn neg(self) -> Self::Output {
		return Self::Output {
			v: self.v.clone().neg(),
		};
	}
}
fn do_math<DT: NumLike>(a: MyUnit<DT>, b: MyUnit<DT>) -> DT {
	let x = (&a - &b) / (&a + &b);
	return x.clone() * x.clone();
}

