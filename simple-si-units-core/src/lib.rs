#![warn(missing_docs)]
#![ doc = include_str!("../README.md")]
use std::fmt::{Debug, Display};

/// The `NumLike` trait is just a shorthand definition for any "number-like" 
/// type in Rust. "Number-like" means that a type implements the traits for 
/// standard arithmatic (Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, 
/// DivAssign, and Neg), plus Clone, Debug, and Display. Most number types also
/// implement the Copy marker trait, but that is not required (for example, an 
/// arbitrary-precision number type must dynamically allocate memory and thus 
/// cannot implement Copy).
/// 
/// This trait is not meant to be implemented, just for making generic type 
/// templates more ergonomic. E.g.
/// ```rust
/// use simple_si_units_core::NumLike;
/// 
/// fn delta_squared<T>(a: T, b: T) -> T where T: NumLike {
///   let delta = b - a;
///   return delta.clone() * delta;
/// }
/// ```
pub trait NumLike: std::ops::Add<Output=Self>
+ std::ops::AddAssign
+ std::ops::Sub<Output=Self>
+ std::ops::SubAssign
+ std::ops::Mul<Output=Self>
+ std::ops::MulAssign
+ std::ops::Div<Output=Self>
+ std::ops::DivAssign
+ std::ops::Neg<Output=Self>
+ Clone
+ Debug
+ Display
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
+ Clone
+ Debug
+ Display
{}
