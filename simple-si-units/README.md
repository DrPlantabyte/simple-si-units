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
* Optional, limited integration with [uom](https://crates.io/crates/uom)

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
* Not supporting unusual number types (eg integers)
* Not aiming for full integration with [uom](https://crates.io/crates/uom)

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
[num_bigfloat::BigFloat](https://crates.io/crates/num-bigfloat) (see caveat 
section below regarding primitive types other than `f64`).

## Adding your own units
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

## Caveats
There are a few limitations owing to the Rust compiler's lack of 
[type specialization](https://github.com/rust-lang/rust/issues/31844) in stable 
Rust, the most notable of which is that some functions won't work with `f32` as 
the input type.

### Primitive types other than f64 
Many of the member functions will only work with number types
that implement `From<f64>` (because they need to multiply by an internal 
coefficient of type `f64`), so while you can still instantiate these structs
with f32 and other primitive types (eg `Mass{kg: 1.1_f32}` will work), you will
have to wrap primitive types other than f64 to use the constructor functions
(eg `Mass::from_g(1100_f32)` will *not* work). Thus to use `f32` or other
primitives which are not convertible from `f64`, you will need to wrap them
with an implementation of `From<f64>`. For example:
```rust
// TODO: updated example

```

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.
