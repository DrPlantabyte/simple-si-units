
// TODO: documentation
pub trait NumLike: std::ops::Add<Output=Self>
+ std::ops::AddAssign
+ std::ops::Sub<Output=Self>
+ std::ops::SubAssign
+ std::ops::Mul<Output=Self>
+ std::ops::MulAssign
+ std::ops::Div<Output=Self>
+ std::ops::DivAssign
+ std::ops::Neg<Output=Self>
+ Sized
{}
impl<T> NumLike for T where T: std::ops::Add<Output=Self>
+ std::ops::AddAssign
+ std::ops::Sub<Output=Self>
+ std::ops::SubAssign
+ std::ops::Mul<Output=Self>
+ std::ops::MulAssign
+ std::ops::Div<Output=Self>
+ std::ops::DivAssign
+ std::ops::Neg<Output=Self>
+ Sized
{}
