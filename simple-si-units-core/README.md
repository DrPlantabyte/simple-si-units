# simple-si-units-core

This crate exists to support the [simple-si-units](https://crates.io/crates/simple-si-units). Please go there for more information.

## Contents

Crate [simple-si-units-core](https://crates.io/crates/simple-si-units-core) exports the following:

#### UnitData
This is an ergonomic trait bundle that combines the following:
* std::ops::Add
* std::ops::Sub
* std::ops::Mul
* std::ops::Div
* Sized

Thus you can use this trait as part of a struct or function template definition, like this:

```rust
use simple_si_units_core::UnitData;
pub struct MyUnit<DT: UnitData> {
    v: DT,
}

impl<DT: simple_si_units_core::UnitData> std::ops::Add<Self> for MyUnit<DT> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        return Self { v: self.v + rhs.v };
    }
}

impl<DT: simple_si_units_core::UnitData> std::ops::Sub<Self> for MyUnit<DT> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self { v: self.v - rhs.v };
    }
}
```

## License
This library is open source, licensed under the [Mozilla Public License version 2.0](https://www.mozilla.org/en-US/MPL/). In summary, you may include this source code *as-is* in both open-source and proprietary projects without requesting permission from me, but if you modify the source code from this library then you must make your modified version of this library available under an open-source license.
