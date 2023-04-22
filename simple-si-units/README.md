# simple-si-units
![GitHub Workflow Build Status](https://github.com/DrPlantabyte/simple-si-units/actions/workflows/build-main.yml/badge.svg) ![GitHub Workflow Test Status](https://github.com/DrPlantabyte/simple-si-units/actions/workflows/unit-test-main.yml/badge.svg) [![codecov](https://codecov.io/gh/DrPlantabyte/simple-si-units/branch/main/graph/badge.svg?token=PRSVR7M9A0)](https://codecov.io/gh/DrPlantabyte/simple-si-units) [![Crate.io](https://img.shields.io/crates/v/simple-si-units)](https://crates.io/crates/simple-si-units) [![Redistribution license](https://img.shields.io/github/license/DrPlantabyte/simple-si-units?color=green)](https://github.com/DrPlantabyte/simple-si-units/blob/main/simple-si-units/LICENSE)

This Rust library provides compiler-checked types for the standard set of SI 
units, as specified by the US [National Institute of Standards and Technology](https://www.nist.gov/pml/owm/metric-si/si-units) (this project is not officially endorsed by NIST).

## What's included?
* Official standard SI Units
* Common secondary units, such as velocity
* Implements operators to automatically convert between units with basic 
  arithmatic (eg distance / time = velocity)
* Units are templated so that you can choose whether to use `f32` or `f64` or other number-like type as your concrete number type.
* Compatible with [uom](https://crates.io/crates/uom)

### Units
This crate provides types for the following units. Other kinds of 
quantities not listed below (eg jolt) are beyond the scope of this crate.

#### Base SI units (and standard unit of measure):
* Distance, aka Length (meters)
* Mass (kilogram)
* Time (seconds)
* Temperature (kelvin)
* Amount, aka Quantity (moles)
* Current (amperes)
* Luminosity (candela)

#### Derived units:
* Angle (rad)
* Angular Velocity (rad/s)
* Angular Acceleration (rad/s^2)
* Moment of Inertia (kg.m^2)
* Angular Momentum (kg.m^2.rad/s)
* Torque (kg.m^2/s^2, aka N.m)
* Solid Angle (sr)
* Frequency (1/s, aka Hz)
* Area (m^2)
* Area Density (kg.m^2)
* Volume (m^3)
* Density (kg/L)
* Velocity (m/s)
* Acceleration (m/s^2)
* Momentum (kg.m/s)
* Force (kg.m/s^2, aka N)
* Pressure (N/m^2, aka Pa)
* Energy (kg.m^2/s^2, aka J)
* Charge, aka Coulomb (A.s, aka C)
* Power, aka Watt (J/s, aka W)
* Voltage (W/A, aka V)
* Resistance (V/A, aka Ohm)
* Conductance (1/ohm, aka S)
* Capacitance (C/V, aka F)
* Inductance (Wb/A, aka H)
* Magnetic Flux (V.s, aka Wb)
* Magnetic Flux Density (Wb/m^2, aka T)
* Catalytic Activity (mol/s)
* Concentration (mol/m^3)
* Luminous Flux (cd.sr, aka lm)
* Illuminance (lm/m^2, aka lux)
* Radioactivity (1/s, aka Bq)
* Absorbed Dose (J/kg, aka Gy)
* Dose Equivalent (J/kg, aka Sv)

## What's NOT included?
* Not supporting dimensional analysis
* Not providing an exhaustive list of all possible unit types (but you can use 
  this library to implement them yourself)
* Not supporting integer number types (use at your own risk)

## Features
The **simple-si-units** crate has the following optional features which can be
enabled to provide additional compatibility:
* **serde** - Adds [serde](https://crates.io/crates/serde) serialization/deserialization compatibility
* **uom** - If enabled, then unit structs will implement`Into` and `From` trait
to convert between **simple-si-units** and **[uom](https://crates.io/crates/uom)** types
* **num-complex** - Adds `std::ops::Mul` and `std::ops::Div` implementations for multiplying and dividing 

## Quickstart guide
TODO: quick tutorial

## Why not use [uom](https://crates.io/crates/uom)?
The [uom](https://crates.io/crates/uom) crate and **simple-si-units** crate were 
both designed to provide compile-time type checking for scientific units of 
measure, thus helping developers catch math errors and write cleaner simulation 
APIs. The difference is that [uom](https://crates.io/crates/uom) also performs 
dimensional analysis but cannot canndle custom data types while 
**simple-si-units** handles any number-like data type, but does not attempt to 
implement full compile-time dimensional analysis. **simple-si-units** also 
prioritizes developer ergonomics, adhering to a consistent `Struct::to_...()`
and `Struct::from_...()` syntax for simple and intuitive number conversions.
Whether [uom](https://crates.io/crates/uom) or **simple-si-units** better 
suits your needs depends on your needs. 

Here's a table comparing **simple-si-units** v1.0 and 
**[uom](https://crates.io/crates/uom)** v0.34 to help you decide:

| Feature                                                           | [simple-si-units](https://crates.io/crates/simple-si-units) | [uom](https://crates.io/crates/uom) |
|-------------------------------------------------------------------|---|---|
| Zero-cost measurement unit type safety                            | ✅ | ✅ |
| All primary and secondary SI units as defined by NIST             | ✅ | ✅ |
| Angular units (angular velocity, momentum, and intertia)          | ✅ | ❌ |
| Support for standard numnber types (eg f64)                       | ✅ | ✅ |
| Support for [num-rational](https://crates.io/crates/num-rational) | ✅ | ✅ |
| Support for [num-complex](https://crates.io/crates/num-complex)   | ✅ | ✅ |
| Support for user-defined and other number types                   | ✅ | ❌ |
| Compile-time dimensional analysis                                 | ❌ | ✅ |

To further demonstrate the similarities and differences between **simple-si-units** and
**[uom](https://crates.io/crates/uom)**, here's two different versions of the same 
gravity calculation function, one using **simple-si-units** and the 
other using [uom](https://crates.io/crates/uom):
```rust
// simple-si-units version
mod simple_si_version {
  use simple_si_units::base::{Distance, Mass};
  use simple_si_units::mechanical::{Acceleration};

  pub fn calc_gravity(mass: Mass<f64>, dist: Distance<f64>) -> Acceleration<f64> {
    const G: f64 = 6.67408e-11; // m3 kg-1 s-2
    let d_squared = dist * dist;
    return Acceleration::from_mps2(G * mass.to_kg() / d_squared.to_m2())
  }

  fn test_gravity1() {
    let radius = Distance::from_km(6378.1);
    let mass = Mass::from_earth_mass(1.0);
    println!("simple-si-units: Earth gravity at sea-level is {}", calc_gravity(mass, radius));
  }
}

// uom version
mod uom_version {
  use uom::si::f64::{Length, Mass, Acceleration};
  use uom::si::length::*;
  use uom::si::mass::*;
  use uom::si::acceleration::*;
  use uom::fmt::DisplayStyle::Abbreviation;

  pub fn calc_gravity(mass: Mass, dist: Length) -> Acceleration {
    const G: f64 = 6.67408e-11; // m3 kg-1 s-2
    let d_squared = dist * dist;
    return Acceleration::new::<meter_per_second_squared>(G * mass.value / d_squared.value)
  }

  fn test_gravity2() {
    let radius = Length::new::<kilometer>(6378.1);
    let mass = Mass::new::<kilogram>(5.972e24);
    println!("uom: Earth gravity at sea-level is {}",
             calc_gravity(mass, radius).into_format_args(meter_per_second_squared, Abbreviation));
  }
}
```

## Roadmap
The version of this library will be incremented to reflect progress through the various milestones. The goal is to reach version 1.0 (API stable) as quickly as practical.

* **V0.1.0 (Done!)** - Finish README and claim [crates.io](https://crates.io/) 
  namespace
* **V0.2.0 (Done!)** - Scope declaration
* **V0.3.0 (Done!)** - Design API
* **V0.4.0 (Done!)** - Unit and API tests
* **V0.5.0 (Done!)** - Base SI units (distance, mass, time, temperature, amount, 
  electric current, luminosity)
* **V0.6.0 (Done!)** - Common secondary units (velocity, acceleration, energy, etc.)
* **V0.7.0 (Done!)** - Full test coverage of all types of units
* **V0.8.0 (Done!)** - Optional `Into` and `From` conversion to/from [uom](https://crates.io/crates/uom) types
* **V0.9.0 (In progress...)** - Full documentation coverage
* **V1.0.0** - Done
* **V1.1.0** - Add inverse of all provided units that don't already have an 
  inverse equivalent (eg InverseArea = 1/Area)

## How it works
For each type of unit (eg Distance), Simple SI Units provides a generic struct 
to represent the unit and which implements common type conversion. For example, 
dividing a Distance by a Time results in a Velocity:
```rust
use simple_si_units::base::{Distance, Mass};
use simple_si_units::mechanical::{Acceleration};
pub fn calc_gravity(mass: Mass<f64>, dist: Distance<f64>) -> Acceleration<f64>{
	const G: f64 = 6.67408e-11; // m3 kg-1 s-2
	let d_squared = dist * dist;
	return Acceleration::from_mps2(G * mass.to_kg() / d_squared.to_m2())
}

fn main(){
	let a = calc_gravity(Mass::from_solar_mass(1.0), Distance::from_au(1.0));
	println!("Solar gravity at Earth orbital distance: {}", a);
}
```

Since these structs use generic type templates for the internal data type, you 
can use any number-like data type with these structs, including 
[num_complex::Complex](https://crates.io/crates/num-complex) and 
[num_bigfloat::BigFloat](https://crates.io/crates/num-bigfloat) (see limitations 
section below regarding types that do not implement `From<f64>`).

For example, the above function could be rewritten as follows to allow almost 
any number-like data type:
```rust
use simple_si_units::base::{Distance, Mass};
use simple_si_units::mechanical::{Acceleration};
use simple_si_units::NumLike;
pub fn calc_gravity_generic<T>(mass: Mass<T>, dist: Distance<T>) -> Acceleration<T> 
  where T: NumLike+From<f64> 
{
  const G: f64 = 6.67408e-11; // m3 kg-1 s-2
  let d_squared = &dist * &dist;
  return Acceleration::from_mps2(T::from(G) * mass.to_kg() / d_squared.to_m2())
}
```

## Adding Your Own Units
Simple SI Units does not provide an exhaustive list of possible units of 
measure. To create your own units, use the `UnitStruct` procedural macro and 
`NumLike` trait bundle (`NumLike` is just shorthand for 
`Sized+std::ops::*<Output=Self>`, you could instead use the `Num` trait from 
the 
[num-traits crate](https://crates.io/crates/num-traits) if you prefer):

```rust
use simple_si_units::{UnitStruct, NumLike};
#[derive(UnitStruct, Debug, Clone)]
struct HyperVelocity<T: NumLike>{
	square_meters_per_second: T
}

fn weighted_sum<T: NumLike>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T> where
	T:NumLike + From<f64>
{
	return weight*a + (1.-weight)*b;
}
```

## Limitations
Due to the Rust compiler's lack of 
[type specialization](https://github.com/rust-lang/rust/issues/31844) in stable 
Rust, some of the unit constructor functions (eg `Mass::from_g(...)`) only work 
with number types that implement `From<f64>`. This means that those functions 
will not work for Rust's built-in `f32` or integer types. You can still 
construct unit structs with their SI reference measurement using any number 
type (eg `Mass::from_kg(1f32)` will work).

## Custom number types
**simple-si-units** works with any "number-like" data type, including libraries 
such as [num-bigfloat](https://crates.io/crates/num-bigfloat), 
[num-complex](https://crates.io/crates/num-complex), and even number types you 
define yourself. A data type is "number-like" if it implements the following 
traits: Clone, Debug, Display, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, 
Div, DivAssign, Neg

For example, here's a snippet of code that defines and uses a number type that is 
like `f32` but also implements `From<f64>`:
```rust
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
```

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.


## Developer notes
Note that the soure files, other than `lib.rs`, were all generated by a Python
program which performs dimensional analysis and other code generation 
activities, found in the [code-generator](https://github.com/DrPlantabyte/simple-si-units/tree/main/code-generator) 
folder of the [GitHub repository](https://github.com/DrPlantabyte/simple-si-units).

If you wish to contribute, please start by adding the unit tests for your new 
feature and then modify the Python project to generate the Rust implementation
of the new feature. Thanks! 