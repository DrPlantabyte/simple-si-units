
// TODO: documentation
pub trait UnitData: std::ops::Add<Output=Self>
+ std::ops::Sub<Output=Self>
+ std::ops::Mul<Output=Self>
+ std::ops::Div<Output=Self>
+ Sized
{}
impl<T> UnitData for T where T: std::ops::Add<Output=Self>
+ std::ops::Sub<Output=Self>
+ std::ops::Mul<Output=Self>
+ std::ops::Div<Output=Self>
+ Sized
{}
