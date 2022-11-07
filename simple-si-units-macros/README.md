# simple-si-units-core

This crate exists to support the [simple-si-units](https://crates.io/crates/simple-si-units). Please go there for more information.

## Contents

Crate [simple-si-units-macros](https://crates.io/crates/simple-si-units-macros) exports the following derive macros:

#### Unit
This macro uses the `NumLike` trait from [simple-si-units-core](https://crates.io/crates/simple-si-units-core) to derive all of the relevant mathematical operators for the derived struct, so long as that struct contains only a single named field. For example:

```rust
use simple_si_units_macros::Unit;
use simple_si_units_core::NumLike;

#[derive(UnitStruct, Debug, Copy, Clone)]
struct Velocity<T: NumLike>{
	pub meters_per_second: T
}

fn weighted_sum<T: NumLike>(a: Velocity<T>, b: Velocity<T>, weight: f64) -> Velocity<T> where
	T:NumLike + From<f64>
{
	return weight*a + (1.-weight)*b;
}
```

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.
