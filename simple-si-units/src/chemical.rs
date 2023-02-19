
//! This module provides chemical SI units, such as catalytic activity 
//! and chemical concentration.

/// The catalytic activity unit type, defined as moles per second in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct CatalyticActivity<T: NumLike>{
	pub molps: T
}

impl<T> CatalyticActivity<T> where T: NumLike {

	/// Returns a new catalytic activity value from the given number of moles per second
	///
	/// # Arguments
	/// * `molps` - Any number-like type, representing a quantity of moles per second
	pub fn from_molps(molps: T) -> Self {
		CatalyticActivity{molps}
	}
	
	/// Returns this catalytic activity value in moles per second
	pub fn to_molps(self) -> T {
		return self.molps;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: CatalyticActivity * Time -> Amount

	// TODO: CatalyticActivity / Amount -> Frequency

	// TODO: CatalyticActivity / Frequency -> Amount

}

/// The chemical concentration unit type, defined as moles per cubic meter in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct Concentration<T: NumLike>{
	pub molpm3: T
}

impl<T> Concentration<T> where T: NumLike {

	/// Returns a new chemical concentration value from the given number of moles per cubic meter
	///
	/// # Arguments
	/// * `molpm3` - Any number-like type, representing a quantity of moles per cubic meter
	pub fn from_molpm3(molpm3: T) -> Self {
		Concentration{molpm3}
	}
	
	/// Returns this chemical concentration value in moles per cubic meter
	pub fn to_molpm3(self) -> T {
		return self.molpm3;
	}
}

impl<T> Distance<T> where T: NumLike+From<f64> {
	
	// TODO: Concentration * Volume -> Amount

}


