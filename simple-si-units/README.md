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
(import with `use simple_si_units::base::*;`)
* Distance, aka Length (meters)
* Mass (kilogram)
* Time (seconds)
* Temperature (kelvin)
* Amount, aka Quantity (moles)
* Current (amperes)
* Luminosity (candela)

#### Derived units:
| chemical                        | electromagnetic                       | geometry         | mechanical                      | nuclear                        |
|---------------------------------|---------------------------------------|------------------|---------------------------------|--------------------------------|
| Catalytic Activity (mol/s)      | Capacitance (C/V, aka F)              | Angle (rad)      | Acceleration (m/s^2)            | Absorbed Dose (J/kg, aka Gy)   |
| Concentration (mol/m^3, aka mM) | Charge, aka Coulomb (A.s, aka C)      | Area (m^2)       | Angular Acceleration (rad/s^2)  | Dose Equivalent (J/kg, aka Sv) |
| Molar Mass (kg/mol)             | Conductance (1/ohm, aka S)            | Solid Angle (sr) | Angular Momentum (kg.m^2.rad/s) | Radioactivity (1/s, aka Bq)    |
| Molality (mol/kg)               | Illuminance (lm/m^2, aka lux)         | Volume (m^3)     | Angular Velocity (rad/s)        |
| Specific Heat Capacity (J/kg.K) | Inductance (Wb/A, aka H)              |                  | Area Density (kg.m^2)           |
|                                 | Luminous Flux (cd.sr, aka lm)         |                  | Density (kg/L)                  |
|                                 | Magnetic Flux (V.s, aka Wb)           |                  | Energy (kg.m^2/s^2, aka J)      |
|                                 | Magnetic Flux Density (Wb/m^2, aka T) |                  | Force (kg.m/s^2, aka N)         |
|                                 | Resistance (V/A, aka Ohm)             |                  | Frequency (1/s, aka Hz)         |
|                                 | Voltage (W/A, aka V)                  |                  | Moment of Inertia (kg.m^2)      |
|                                 |                                       |                  | Momentum (kg.m/s)               |
|                                 |                                       |                  | Power, aka Watt (J/s, aka W)    |
|                                 |                                       |                  | Pressure (N/m^2, aka Pa)        |
|                                 |                                       |                  | Torque (kg.m^2/s^2, aka N.m)    |
|                                 |                                       |                  | Velocity (m/s)                  |

## What's NOT included?
* Not supporting dimensional analysis
* Not providing an exhaustive list of all possible unit types (but you can use 
  this library to implement your own)
* Not supporting integer number types (use at your own risk)

## Features
The **simple-si-units** crate has the following optional features which can be
enabled to provide additional compatibility:
* **serde** - Adds [serde](https://crates.io/crates/serde) serialization/deserialization compatibility
* **uom** - If enabled, then unit structs will implement the `Into` and `From` traits
to convert between **simple-si-units** and **[uom](https://crates.io/crates/uom)** types
* **num-bigfloat** - Adds `core::ops::Mul` and `core::ops::Div` implementations
  for multiplying and dividing unit structs by `num-bigfloat` scalar values
* **num-complex** - Adds `core::ops::Mul` and `core::ops::Div` implementations
  for multiplying and dividing unit structs by `num-complex` scalar values

## Quickstart guide
### Basic usage
To use **simple-si-units**, just add `simple-si-units = "1.0"` to the `[dependencies]` 
section of your `Cargo.toml` file, then import the units you need like this:
```rust
use simple_si_units::base::*;
use simple_si_units::geometry::*;
use simple_si_units::mechanical::*;

fn main() {
  let box_width = Distance::from_cm(33.5);
  let box_length = Distance::from_cm(45.0);
  let box_height = Distance::from_cm(23.5);
  let carboard_density = AreaDensity::from_grams_per_square_meter(300.);
  let box_volume = &box_width * &box_height * &box_length;
  println!("Your box holds a total volume of {:.2} liters", box_volume.to_L());
  let box_weight = (2. * &box_width * &box_length
    + 2. * &box_width * &box_height
    + 2. * &box_length * &box_height) * &carboard_density;
  println!("Your box has a weight of {}", box_weight);
}
```
Note that **simple-si-units** structs all implement `core::ops::{Add,Sub,Mul,Div}` 
for both values and references, which is useful for number type which do not 
implement the `Copy` trait.

### Making APIs
**simple-si-units** was designed specifically to help people create safer APIs
for libraries and functions that perform scientific calculations. 

For most applications, you can simply specify both the SI unit type and data 
type for each variable, like this:
```rust
use simple_si_units::base::Distance;
use simple_si_units::geometry::Volume;
use std::f64::consts::PI;

pub fn sphere_volume(radius: Distance<f64>) -> Volume<f64> {
   &radius * &radius * &radius *  4. / 3. * PI
}
```

However, if you want to support more than one data type, then you should use
a generic templated function. The **simple-si-units** crate provides the 
`NumLike` type to help simplify generic APIs, for example:
```rust

use simple_si_units::base::Distance;
use simple_si_units::geometry::Volume;
use std::f64::consts::PI;
use simple_si_units::NumLike;

pub fn sphere_volume<T>(radius: Distance<T>) -> Volume<T>
where T: NumLike + From<f64>
{
    &radius * &radius * &radius * T::from(4. / 3. * PI)
}
```
The above generic function will work for any number type which implements 
`From<f64>`, such as `Complex64` or `BigFloat` (from the 
[num-complex](https://crates.io/crates/num-complex) and
[num-bigfloat](https://crates.io/crates/num-bigfloat) crates respectively). 

## Why not use [uom](https://crates.io/crates/uom)?
You don't have to choose, you can use both! All **simple-si-units** types implement the
`Into` and `From` traits to convert to and from their 
**[uom](https://crates.io/crates/uom)** equivalent.

The **[uom](https://crates.io/crates/uom)** and **simple-si-units** crates were 
both designed to provide compile-time type checking for scientific units of 
measure to help developers catch math errors and write cleaner calculation 
APIs. The difference is that [uom](https://crates.io/crates/uom) also performs 
dimensional analysis but cannot handle custom data types, while 
**simple-si-units** handles any number-like data type but does not attempt to 
implement full compile-time dimensional analysis. **simple-si-units** also 
prioritizes developer ergonomics, adhering to a consistent `Struct::from_...()`
and `Struct.to_...()` syntax for simple and intuitive number conversions.
Whether **[uom](https://crates.io/crates/uom)** or **simple-si-units** better 
suits your application depends on your needs. 

Here's a table comparing **simple-si-units** v1.0 and 
**[uom](https://crates.io/crates/uom)** v0.34 to help you decide which to use:

| Feature                                                           | [simple-si-units](https://crates.io/crates/simple-si-units) | [uom](https://crates.io/crates/uom) |
|-------------------------------------------------------------------|-------------------------------------------------------------|-------------------------------------|
| Zero-cost measurement unit type safety                            | ✅                                                           | ✅                                   |
| All primary and secondary SI units as defined by NIST             | ✅                                                           | ✅                                   |
| Inverse (aka reciprical) of SI units                              | ✅                                                           | partial                             |
| Support for standard decimal types (eg f64)                       | ✅                                                           | ✅                                   |
| Support for standard integer types (eg i32)                       | partial**                                                   | partial**                           |
| Support for [num-bigfloat](https://crates.io/crates/num-bigfloat) | ✅                                                           | ❌                                   |
| Support for [num-complex](https://crates.io/crates/num-complex)   | ✅                                                           | ✅                                   |
| Support for [num-rational](https://crates.io/crates/num-rational) | partial**                                                   | ✅                                   |
| Support for user-defined and other number types                   | ✅                                                           | ❌                                   |
| Compile-time dimensional analysis                                 | ❌                                                           | ✅                                   |

** *integer types and int-based number types are not fully supported in simple-si-units*

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
`core::ops::*<Output=Self>+Clone+Debug+Display`, you could instead use the `Num`
trait from the [num-traits crate](https://crates.io/crates/num-traits) if you 
prefer):

```rust
use simple_si_units::{UnitStruct, NumLike};

#[derive(UnitStruct, Debug, Clone)]
struct HyperVelocity<T: NumLike>{
  square_meters_per_second: T
}

fn weighted_hypervel_sum<T: NumLike>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T>
  where T:NumLike + From<f64>
{
  return weight*a + (1.-weight)*b;
}
```
Note that the `UnitStruct` derive macro only works on structs that contain only a 
single member variable. Otherwise it will generate a compiler error.


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
traits: **Clone, Debug, Display, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, 
Div, DivAssign, Neg**

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
It's highly recommended that you also implement the `std::ops::*` operators for 
all combinations of values and reference types (eg `X + X`, `X + &X`, `&X + X`, and `&X + &X`),
as this will make your number type much easier to use and integrate with **simple-si-units**.

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.


## Developer notes
Note that the unit struct source files (excluding `lib.rs`), were all generated
by a Python program which performs dimensional analysis and other code generation 
activities, found in the [code-generator](https://github.com/DrPlantabyte/simple-si-units/tree/main/code-generator) 
folder of the [GitHub repository](https://github.com/DrPlantabyte/simple-si-units).

If you wish to contribute, please start by adding the unit tests for your new 
feature and then modify the Python project to generate the Rust implementation
of the new feature. Thanks! 
