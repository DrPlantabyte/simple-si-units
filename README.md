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
|                                 | Conductance (1/ohm, aka S)            | Solid Angle (sr) | Angular Momentum (kg.m^2.rad/s) | Radioactivity (1/s, aka Bq)    |
|                                 | Illuminance (lm/m^2, aka lux)         | Volume (m^3)     | Angular Velocity (rad/s)        |
|                                 | Inductance (Wb/A, aka H)              |                  | Area Density (kg.m^2)           |
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

## Quickstart guide
### Basic usage
To use **simple-si-units**, just add `simple-si-units = "1"` to the `[dependencies]` 
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
Note that **simple-si-units** structs all implement `std::ops::{Add,Sub,Mul,Div}` 
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


## Scope

### Organization
This package consists of 3 crates: 
[simple-si-units](https://crates.io/crates/simple-si-units), 
[simple-si-units-core](https://crates.io/crates/simple-si-units-core), and 
[simple-si-units-macros](https://crates.io/crates/simple-si-units-macros). 
Users only need to include 
[simple-si-units](https://crates.io/crates/simple-si-units), 
the other two crates exist only to support the procedural macro 
provided by this crate. 

Additional crates may come in the future to provided more unit types for 
specific applications, such as chemistry and astronomy.

### Units
This crate provides types for the following units. Other kinds of 
quantities not listed below (eg jolt) are beyond the scope of this crate.

#### Base SI units (and standard unit of measure):
* Distance (meters)
* Mass (kilogram)
* Time (seconds)
* Temperature (kelvin)
* Amount (moles)
* Current (amperes)
* Luminosity (candela)

#### Derived units:
* Angle (rad)
* Solid Angle (sr)
* Frequency (1/s, aka Hz)
* Area (m^2)
* Volume (m^3)
* Velocity (m/s)
* Acceleration (m/s^2)
* Force (kg.m/s^2, aka N)
* Pressure (N/m^2, aka Pa)
* Energy (kg.m^2/s^2, aka J)
* Coulomb (A.s, aka C)
* Watt (J/s, aka W)
* Voltage (W/A, aka V)
* Resistance (V/A, aka Ohm)
* Conductance (1/ohm, aka S)
* Capacitance (C/V)
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


### Operators
The above types implement basic arithmetic operators for conversion between 
types (eg dividing a Distance by a Time yields a Velocity), and also handle 
multiplication and division by standard number types (such as f64). 

Each type has a single public field, named for the reference unit of measure 
for that type (eg meters for Distance), and numerous to_* and from_* methods 
for converting to/from basic number types with a given unit of measure (eg 
`Distance::from_km(1.72)`).

### Roadmap
The version of this library will be incremented to reflect progress through the various milestones. The goal is to reach version 1.0 (API stable) as quickly as practical.

* **V0.1.0** - Finish README and claim [crates.io](https://crates.io/) namespace
* **V0.2.0** - Scope declaration
* **V0.3.0** - Design API
* **V0.4.0** - Unit and API tests
* **V0.5.0** - Base SI units (length, mass, time, temperature, amount, electric current, luminosity)
* **V0.6.0** - Common secondary units (velocity, acceleration, energy, etc.)
* **V0.7.0** - Full coverage of all types of units
* **V0.8.0** - Optional `Into` and `From` conversion to/from [uom](https://crates.io/crates/uom) types
* **V0.9.0** - Full documentation coverage
* **V1.0.0** - Done


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
`std::ops::*<Output=Self>+Clone+Debug+Display`, you could instead use the `Num`
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
It's higly recommended that you also implement the `std::ops::*` operators for 
all combinations of values and reference types (eg `X + X`, `X + &X`, `&X + X`, and `&X + &X`),
as this will make your number type much easier to use and integrate with **simple-si-units**.

## Developer notes
Note that the unit struct source files (excluding `lib.rs`), were all generated
by a Python program which performs dimensional analysis and other code generation 
activities, found in the [code-generator](https://github.com/DrPlantabyte/simple-si-units/tree/main/code-generator) 
folder.

If you wish to contribute, please start by adding the unit tests for your new 
feature and then modify the Python project to generate the Rust implementation
of the new feature. Thanks! 

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.
