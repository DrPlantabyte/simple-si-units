MODULE_TEMPLATE='''
/// This module provides %(category)s SI units, such as %(example1)s 
/// and %(example2)s.
pub mod %(category)s {
%(content)s
}

'''

UNIT_STRUCT_DEFINITION_TEMPLATE='''
	/// The %(desc first name)s unit type, defined as %(unit name)s in SI units
	#[derive(UnitStruct, Debug, Clone)]
	pub struct %(code name)s<T: NumLike>{
		pub %(unit symbol)s: T
	}

	impl<T> %(code name)s<T> where T: NumLike {
	
		/// Returns a new %(desc name)s value from the given number of %(unit name)s
		///
		/// # Arguments
		/// * `%(unit symbol)s` - Any number-like type, representing a quantity of %(unit name)s
		pub fn from_%(unit symbol)s(%(unit symbol)s: T) -> Self {
			%(code name)s{%(unit symbol)s}
		}
		
		/// Returns this %(desc name)s value in %(unit name)s
		pub fn to_%(unit symbol)s(self) -> T {
			return self.%(unit symbol)s;
		}
	}

	impl<T> Distance<T> where T: NumLike+From<f64> {
		%(to-and-from)s
	}
'''

UNIT_CONVERSION_TEMPLATE='''
		// TODO: %(code left-side)s %(operator)s %(code right-side)s -> %(code result)s
'''
