# simple-si-units
![GitHub Workflow Build Status](https://github.com/DrPlantabyte/simple-si-units/actions/workflows/build-and-test.yml/badge.svg)

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

#### Base SI units (and example unit of measure):
* Length (meters)
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

## What's NOT included?
* Not supporting dimensional analysis
* Not providing an exhaustive list of all possible types (but you can use this library to implement them yourself)
* Not supporting unusual number types (eg Big-Decimal)
* Not aiming for full integration with [uom](https://crates.io/crates/uom)

## Roadmap
The version of this library will be incremented to reflect progress through the various milestones. The goal is to reach version 1.0 (API stable) as quickly as practical.

* **V0.1.0** - Finish README and claim [crates.io](https://crates.io/) namespace
* **V0.2.0** - Scope declaration
* **V0.3.0** - Design API
* **V0.4.0** - Unit and API tests
* **V0.5.0** - Base SI units (distance, mass, time, temperature, amount, 
  electric current, luminosity)
* **V0.6.0** - Common secondary units (velocity, acceleration, energy, etc.)
* **V0.7.0** - Full coverage of all types of units
* **V0.8.0** - Optional `Into` and `From` conversion to/from [uom](https://crates.io/crates/uom) types
* **V0.9.0** - Full documentation coverage
* **V1.0.0** - Done

## How it works
For each type of unit (eg Distance), Simple SI Units provides a struct to 
represent the unit and which implements common type conversion. For 
example, dividing a Distance by a Time results in a Velocity:
```rust
todo!();
```

## Adding your own units
Simple SI Units does not provide an exhaustive list of possible units of 
measure. To create your own units, use the `Unit` procedural macro and 
`UnitData` trait bundle, like this:

```rust
use simple_si_units::{Unit, UnitData};
#[derive(Unit, Debug, Copy, Clone)]
struct HyperVelocity<T: UnitData>{
	square_meters_per_second: T
}

fn weighted_sum<T: UnitData>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T> where
	T:UnitData + From<f64>
{
	return weight*a + (1.-weight)*b;
}
```

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.
