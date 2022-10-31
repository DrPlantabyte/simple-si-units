use macrotest;

#[test]
pub fn expand_macros() {
	macrotest::expand("tests/expand/*.rs")
}

/////
pub struct MyUnit<DT: simple_si_units_core::UnitData> {
	v: DT,
}
impl<DT: simple_si_units_core::UnitData> std::ops::Add<Self> for MyUnit<DT> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		return Self { v: self.v + rhs.v };
	}
}
impl<DT: simple_si_units_core::UnitData> std::ops::Sub<Self> for MyUnit<DT> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		return Self { v: self.v - rhs.v };
	}
}
impl<DT: simple_si_units_core::UnitData> std::ops::Div<Self> for MyUnit<DT> {
	type Output = DT;
	fn div(self, rhs: Self) -> Self::Output {
		return self.v / rhs.v;
	}
}
impl<DT: simple_si_units_core::UnitData> std::ops::Mul<DT> for MyUnit<DT> {
	type Output = Self;
	fn mul(self, rhs: DT) -> Self::Output {
		return Self { v: self.v * rhs };
	}
}
impl<DT> std::ops::Mul<MyUnit<DT>> for f64
	where
		DT: simple_si_units_core::UnitData + From<f64>,
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
		DT: simple_si_units_core::UnitData + From<f32>,
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
		DT: simple_si_units_core::UnitData + From<u8>,
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
		DT: simple_si_units_core::UnitData + From<i8>,
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
		DT: simple_si_units_core::UnitData + From<u16>,
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
		DT: simple_si_units_core::UnitData + From<i16>,
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
		DT: simple_si_units_core::UnitData + From<u32>,
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
		DT: simple_si_units_core::UnitData + From<i32>,
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
		DT: simple_si_units_core::UnitData + From<u64>,
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
		DT: simple_si_units_core::UnitData + From<i64>,
{
	type Output = MyUnit<DT>;
	fn mul(self, rhs: MyUnit<DT>) -> Self::Output {
		return MyUnit {
			v: DT::from(self) * rhs.v,
		};
	}
}