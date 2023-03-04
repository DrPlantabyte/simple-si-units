# simple-si-units
![GitHub Workflow Build Status](https://github.com/DrPlantabyte/simple-si-units/actions/workflows/build-main.yml/badge.svg) ![GitHub Workflow Test Status](https://github.com/DrPlantabyte/simple-si-units/actions/workflows/unit-test-main.yml/badge.svg) [![codecov](https://codecov.io/gh/DrPlantabyte/simple-si-units/branch/main/graph/badge.svg?token=PRSVR7M9A0)](https://codecov.io/gh/DrPlantabyte/simple-si-units) [![Crate.io](https://img.shields.io/crates/v/simple-si-units)](https://crates.io/crates/simple-si-units) [![Redistribution license](https://img.shields.io/github/license/DrPlantabyte/simple-si-units?color=green)](https://github.com/DrPlantabyte/simple-si-units/blob/main/simple-si-units/LICENSE)

This Rust library provides compiler-checked types for the standard set of SI 
units, as specified by the US [National Institute of Standards and Technology](https://www.nist.gov/pml/owm/metric-si/si-units) (this project is not officially endorsed by NIST).

## What's included?
* Official standard SI Units
* Common secondary units, such as velocity
* Implements operators to autimatically convert between units with basic arithmatic
* Units are templated so that you can choose whether to use `f32` or `f64` or other number-like type as your concrete number type.
* Optional, limited integration with [uom](https://crates.io/crates/uom)

## What's NOT included?
* Not supporting dimensional analysis
* Not providing an exhaustive list of all possible types (but you can use this library to implement them yourself)
* Not supporting unusual number types (eg Big-Decimal)
* Not aiming for full integration with [uom](https://crates.io/crates/uom)

## How it works
For each type of unit (eg Distance), Simple SI Units provides a struct to 
represent the unit and which implements common type conversion. For 
example, dividing a Distance by a Time results in a Velocity:
```rust
todo!();
```

## Adding your own units
Simple SI Units does not provide an exhaustive list of possible units of 
measure. To create your own units, use the `UnitStruct` procedural macro and 
`NumLike` trait bundle, like this:

```rust
use simple_si_units::{UnitStruct, NumLike};
#[derive(UnitStruct, Debug, Copy, Clone)]
struct HyperVelocity<T: NumLike>{
	square_meters_per_second: T
}

fn weighted_sum<T: NumLike>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T> where
	T:NumLike + From<f64>
{
	return weight*a + (1.-weight)*b;
}
```

## Examples

### Represent the solar system and calculate orbital velocities
```rust
use simple_si_units::base::{Distance, Mass, Time};
use simple_si_units::geometry::{Area};
use simple_si_units::mechanical::{Velocity, Acceleration};

#[derive(Debug, Clone)]
struct MassPoint {
	mass: Mass<f64>,
	pos: [Distance<f64>; 2],
	vel: [Velocity<f64>; 2],
	accel: [Acceleration<f64>; 2],
}

struct LCGRand {
	seed: u64
}
impl LCGRand {
	fn rand_f64(&mut self) -> f64 {
		self.seed = (self.seed.overflowing_mul(0x5DEECE66Du64).0.overflowing_add(0xBu64).0) & 0xFFFFFFFFFFFFu64;
		return (self.seed & 0xFFFFFFFFu64) as f64 / 0xFFFFFFFFu64 as f64;
	}
}

fn populate_system() -> Vec<MassPoint> {
	let mut prng = LCGRand{seed: 12348768};
	//
	let mut system: Vec<MassPoint> = Vec::new();
	system.push(MassPoint{
		mass: Mass::from_solar_mass(prng.rand_f64() + 0.5),
		pos: [Distance{m: 0.}, Distance{m: 0.}],
		vel: [Velocity{mps: 0.}, Velocity{mps: 0.}],
		accel: [Acceleration{mps2: 0.}, Acceleration{mps2: 0.}]
	});
	for _ in 0..8 {
		system.push(MassPoint{
			mass: Mass::from_earth_mass(prng.rand_f64() * 10.),
			pos: [Distance::from_au(prng.rand_f64() * 20. - 10.),
				Distance::from_au(prng.rand_f64() * 20. - 10.)],
			vel: [Velocity{mps: 0.}, Velocity{mps: 0.}],
			accel: [Acceleration{mps2: 0.}, Acceleration{mps2: 0.}]
		});
	}
	return system;
}

fn calc_gravity_at(mass: &MassPoint, masses: &[MassPoint]) -> [Acceleration<f64>; 2] {
	use std::ptr;
	const G: f64 = 6.67408e-11; // m3 kg-1 s-2
	let mut net_accel = [Acceleration{mps2: 0f64}, Acceleration{mps2: 0f64}];
	for mp in masses {
		if ptr::eq(mass, mp) {
			continue;
		}
		let pos = mass.pos;
		let mut dsqr = Area{m2: 0.};
		for i in 0..2 {
			let di = pos[i] - mp.pos[i];
			dsqr += di * di;
		}
		let d = Distance::from_m(dsqr.to_m2().sqrt());
		let nvec = [(mp.pos[0] - pos[0]).m / d.m, (mp.pos[1] - pos[1]).m / d.m];
		let A = Acceleration{mps2: G * mp.mass.kg / dsqr.m2};
		for i in 0..2 {
			net_accel[i] = net_accel[i] + nvec[i] * A;
		}
	}
	return net_accel;
}

fn print_system(masses: &[MassPoint]) {
	let mut rows: Vec<Vec<char>> = vec![vec!['.'; 20]; 20];
	let mut i = 0;
	for mp in masses {
		let textx = 10 + (mp.pos[0].to_au()) as i32;
		let texty = 10 + (mp.pos[1].to_au()) as i32;
		if textx > 0 && textx < 20 && texty > 0 && texty < 20 {
			rows[(19-texty) as usize][textx as usize] = match i { 0 => '*', _ => 'o'};
		}
		i += 1;
	}
	for row in rows {
		let row_str = std::string::String::from_iter(row.iter());
		println!("{}", row_str);
	}
	println!();
}

fn main() {
	// use std::thread;
	let timestep = Time::from_days(1.);
	let num_iters = 300;
	let mut system = populate_system();
	// print visualization
	println!("Before:");
	print_system(&system);
	for _ in 0..num_iters {
		// set accel
		for n in 0..system.len() {
			system[n].accel = calc_gravity_at(&system[n], &system);
		}
		// set velocity
		for n in 0..system.len() {
			for i in 0..2 {
				system[n].vel[i] = system[n].vel[i] + system[n].accel[i] * timestep;
			}
		}
		//print_system(&system);
		// set position
		for n in 0..system.len() {
			for i in 0..2 {
				system[n].pos[i] = system[n].pos[i] + system[n].vel[i] * timestep;
			}
		}
		//print_system(&system);
		// thread::sleep(Duration::from_millis(100));

	}
	// print visualization
	println!("");
	println!("After:");
	print_system(&system);
}
```

### Simulate a ball thrown from a slingshot
```rust
todo!();
```

### Calculate electrical current through a circuit
```rust
todo!();
```


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

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.
