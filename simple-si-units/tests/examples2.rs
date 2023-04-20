// use simple_si_units::base::Mass;
// #[derive(Debug, Clone)]
// struct MyFloat32 {
// 	x: f32
// }
// impl MyFloat32 {
// 	pub fn new(n: f32) -> Self{return Self{x: n}}
// }
// impl From<f64> for MyFloat32 {
// 	fn from(n: f64) -> Self {return Self::new(n as f32)}
// }
// impl std::ops::Add<Self> for MyFloat32 {
// 	type Output = Self;
// 	fn add(self, rhs: Self) -> Self::Output {Self{ x: self.x + rhs.x }}
// }
// impl std::ops::Sub<Self> for MyFloat32 {
// 	type Output = Self;
// 	fn sub(self, rhs: Self) -> Self::Output {Self{ x: self.x - rhs.x }}
// }
// impl std::ops::Div<Self> for MyFloat32 {
// 	type Output = Self;
// 	fn div(self, rhs: Self) -> Self::Output {Self{ x: self.x / rhs.x}}
// }
// impl std::ops::Mul<Self> for MyFloat32 {
// 	type Output = Self;
// 	fn mul(self, rhs: Self) -> Self::Output {Self{ x: self.x * rhs.x }}
// }
// fn my_fn() -> Mass<MyFloat32>{
// 	let m = Mass::from_g(MyFloat32::new(1100_f32));
// 	return m * MyFloat32::new(0.5);
// }