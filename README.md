# simple-si-units
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
* Not providing an exhaustive list of all possible types (but you can use thie library to implement them yourself)
* Not supporting unusual number types (eg Big-Decimal)
* Not aiming for full integration with [uom](https://crates.io/crates/uom)

## How it works
For each type of unit (eg Length), Simple SI Units provides a universal 
trait type and a generic struct which implements common type conversion. For 
example, here's how you could define a function that calculates the gravity 
at the surface of a planet based on is size and mass:
```rust
todo!();
```

## Examples
### Simulate a ball thrown from a slingshot
```rust
todo!();
```

### Represent the solar system and calculate orbital velocities
```rust
todo!();
```

### Calculate electrical current through a circuit
```rust
todo!();
```


## Roadmap
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
