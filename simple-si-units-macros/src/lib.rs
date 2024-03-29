#![no_std]
#![warn(missing_docs)]
#![ doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use quote::{quote};
use syn::*;

// Test with $ rm ./tests/expand/derive_tester.expanded.rs ; cargo +nightly test -- --nocapture

/// This macro uses the `NumLike` trait from
/// [simple-si-units-core](https://crates.io/crates/simple-si-units-core) to
/// derive all of the relevant mathematical operators for the derived struct,
/// so long as that struct contains only a single named field. For example:
/// 
/// ```rust
/// use simple_si_units_macros::UnitStruct;
/// use simple_si_units_core::NumLike;
/// 
/// #[derive(UnitStruct, Debug, Clone)]
/// struct HyperVelocity<T: NumLike>{
///   square_meters_per_second: T
/// }
/// 
/// fn weighted_hypervel_sum<T: NumLike>(a: HyperVelocity<T>, b: HyperVelocity<T>, weight: f64) -> HyperVelocity<T>
///   where T:NumLike + From<f64>
/// {
///   return weight*a + (1.-weight)*b;
/// }
/// ```
#[proc_macro_derive(UnitStruct)]
pub fn derive_unit(tokens: TokenStream) -> TokenStream {
	// convert the input tokens into an ast, specially from a derive
	let input: syn::DeriveInput = syn::parse(tokens).expect("syn::parse failed on proc macro \
	input for simple_si_units_macros::UnitStruct");

	// Build the trait implementation
	impl_derive_unit(&input)
}

fn impl_derive_unit(input: &syn::DeriveInput) -> TokenStream {
	let requirements_msg = "Derive macro simple_si_units::UnitStruct can only be applied to structs \
	with a single field, whose type implements core::ops::{Add, Sub, Div, Mul} (eg \
	<T: NumLike>). For \
	example:\n\nuse simple_si_units::{UnitStruct, NumLike};\n#[derive(UnitStruct, Debug, Copy, Clone)\
	]\nstruct MyNewUnitStruct<T: NumLike>{\n    pub x: \
	T\n}\n\nfn \
	weighted_sum<T: NumLike>(a: MyNewUnitStruct<T>, b: MyNewUnitStruct<T>, weight: f64) -> MyNewUnitStruct<T> where T:\
	 NumLike + From<f64>\n{\n    return weight*a + (1.-weight)*b;\n}\n";
	let name = &input.ident;
	let fields = match &input.data {
		Data::Struct(DataStruct {
						 fields: Fields::Named(fields),
						 ..
					 }) => &fields.named,
		_ => panic!("Only structs with named fields can derive simple_si_units::UnitStruct.\n\n{}",
					requirements_msg),
	};
	if fields.len() != 1 {
		panic!("proc macro simple_si_units::UnitStruct can only be used on structs with a single \
		 named field. \n\n{}", requirements_msg)
	}
	let data_name = &fields[0].ident.as_ref().unwrap();
	let data_type = &fields[0].ty;
	let gen = quote! {
		#[doc="This struct implements the Copy marker trait if it's member data type also has the \
		Copy trait"]
		impl<#data_type> core::marker::Copy for #name<#data_type>
			where #data_type: NumLike + core::marker::Copy { }
		#[doc="This struct implements the PartialEq trait if it's member data type also has the \
		PartialEq trait"]
		impl<#data_type> core::cmp::PartialEq for #name<#data_type>
			where #data_type: NumLike + core::cmp::PartialEq {
			fn eq(&self, other: &Self) -> bool {
				#data_type::eq(&self.#data_name, &other.#data_name)
			}
			fn ne(&self, other: &Self) -> bool {
				#data_type::ne(&self.#data_name, &other.#data_name)
			}
		}
		#[doc="This struct implements the core::cmp::Eq trait if it's member data type also has the \
		core::cmp::Eq trait"]
		impl<#data_type> core::cmp::Eq for #name<#data_type>
			where #data_type: NumLike + core::cmp::PartialEq + core::cmp::Eq {}
		#[doc="This struct implements the Hash trait if it's member data type also has the \
		Hash trait"]
		impl<#data_type> core::hash::Hash for #name<#data_type>
			where #data_type: NumLike + core::hash::Hash {
			fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
				#data_type::hash(&self.#data_name, hasher);
			}
		}
		#[doc="This struct implements the PartialOrd trait if it's member data type also has the \
		PartialOrd trait"]
		impl<#data_type> core::cmp::PartialOrd for #name<#data_type>
			where #data_type: NumLike + core::cmp::PartialOrd {
			fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
				#data_type::partial_cmp(&self.#data_name, &other.#data_name)
			}
		}
		#[doc="This struct implements the Ord trait if it's member data type also has the \
		Ord trait"]
		impl<#data_type> core::cmp::Ord for #name<#data_type>
			where #data_type: NumLike + core::cmp::Ord {
			fn cmp(&self, other: &Self) -> core::cmp::Ordering {
				#data_type::cmp(&self.#data_name, &other.#data_name)
			}
		}

		#[doc="Adding two unit values of the same type returns a new unit value of the same type"]
		impl<#data_type: NumLike> core::ops::Add<Self> for #name<#data_type> {
			type Output = Self;
			fn add(self, rhs: Self) -> Self::Output {
				return Self{#data_name: self.#data_name + rhs.#data_name}
			}
		}
		#[doc="Adds the given unit value to this unit value"]
		impl<#data_type: NumLike> core::ops::AddAssign for #name<#data_type> {
			fn add_assign(&mut self, rhs: Self){
				self.#data_name += rhs.#data_name;
			}
		}
		#[doc="Subtracting two unit values of the same type returns a new unit value of the same \
		type"]
		impl<#data_type: NumLike> core::ops::Sub<Self> for
		#name<#data_type> {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self::Output {
				return Self{#data_name: self.#data_name - rhs.#data_name}
			}
		}
		#[doc="Subtracts the given unit value from this unit value"]
		impl<#data_type: NumLike> core::ops::SubAssign for #name<#data_type> {
			fn sub_assign(&mut self, rhs: Self){
				self.#data_name -= rhs.#data_name;
			}
		}
		#[doc="Dividing a unit value by another of the same type returns a scalar value"]
		impl<#data_type: NumLike> core::ops::Div<Self> for
		#name<#data_type> {
			type Output = #data_type;
			fn div(self, rhs: Self) -> Self::Output {
				return self.#data_name / rhs.#data_name;
			}
		}
		#[doc="Dividing a unit value by a scalar value returns a unit value"]
		impl<#data_type: NumLike> core::ops::Div<#data_type> for
		#name<#data_type> {
			type Output = Self;
			fn div(self, rhs: #data_type) -> Self::Output {
				return Self{#data_name: self.#data_name / rhs}
			}
		}
		#[doc="Divides this unit value by a scalar"]
		impl<#data_type: NumLike> core::ops::DivAssign<#data_type> for #name<#data_type> {
			fn div_assign(&mut self, rhs: #data_type){
				self.#data_name /= rhs;
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type: NumLike> core::ops::Mul<#data_type> for
		#name<#data_type> {
			type Output = Self;
			fn mul(self, rhs: #data_type) -> Self::Output {
				return Self{#data_name: self.#data_name * rhs}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type> core::ops::Mul<&#data_type> for #name<#data_type>
			where #data_type: NumLike {
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#data_type) -> Self::Output {
				#name{#data_name: self.#data_name * rhs.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type> core::ops::Mul<&#data_type> for &#name<#data_type>
			where #data_type: NumLike {
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#data_type) -> Self::Output {
				#name{#data_name: self.#data_name.clone() * rhs.clone()}
			}
		}
		#[doc="Multiplies this unit value by a scalar"]
		impl<#data_type: NumLike> core::ops::MulAssign<#data_type> for #name<#data_type> {
			fn mul_assign(&mut self, rhs: #data_type){
				self.#data_name *= rhs;
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 f64 where #data_type: NumLike + From<f64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 f32 where #data_type: NumLike + From<f32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 u8 where #data_type: NumLike + From<u8>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 i8 where #data_type: NumLike + From<i8>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 u16 where #data_type: NumLike + From<u16>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 i16 where #data_type: NumLike + From<i16>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 u32 where #data_type: NumLike + From<u32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 i32 where #data_type: NumLike + From<i32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 u64 where #data_type: NumLike + From<u64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value"]
		impl<#data_type>
		core::ops::Mul<#name<#data_type>> for
		 i64 where #data_type: NumLike + From<i64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		#[doc="Flips the sign of this unit value"]
		impl<#data_type: NumLike> core::ops::Neg for
		#name<#data_type> {
			type Output = Self;
			fn neg(self) -> Self::Output {
				return Self{#data_name: self.#data_name.neg()}
			}
		}

		// ref operators automatically clone the referenced data for convenient ergonomics
		#[doc="Adding two unit values of the same type returns a new unit value of the same type \
		(automatically clones the referenced data for convenient ergonomics)"]
		impl<#data_type: NumLike> core::ops::Add<Self> for &#name<#data_type> {
			type Output = #name<#data_type>;
			fn add(self, rhs: Self) -> Self::Output {
				return Self::Output{#data_name: self.#data_name.clone() + rhs.#data_name.clone()}
			}
		}
		#[doc="Subtracting two unit values of the same type returns a new unit value of the same \
		type (automatically clones the referenced data for convenient ergonomics)"]
		impl<#data_type: NumLike> core::ops::Sub<Self> for
		&#name<#data_type> {
			type Output = #name<#data_type>;
			fn sub(self, rhs: Self) -> Self::Output {
				return Self::Output{#data_name: self.#data_name.clone() - rhs.#data_name.clone()}
			}
		}
		#[doc="Dividing a unit value by another of the same type returns a scalar value \
		(automatically clones the referenced data for convenient ergonomics)"]
		impl<#data_type: NumLike> core::ops::Div<Self> for
		&#name<#data_type> {
			type Output = #data_type;
			fn div(self, rhs: Self) -> Self::Output {
				return self.#data_name.clone() / rhs.#data_name.clone();
			}
		}
		#[doc="Dividing a unit value by a scalar value returns a unit value (automatically clones \
		the referenced data for convenient ergonomics)"]
		impl<#data_type: NumLike> core::ops::Div<#data_type> for
		&#name<#data_type> {
			type Output = #name<#data_type>;
			fn div(self, rhs: #data_type) -> Self::Output {
				return Self::Output{#data_name: self.#data_name.clone() / rhs}
			}
		}
		#[doc="Dividing a unit value by a scalar value returns a unit value"]
		impl<#data_type> core::ops::Div<&#data_type> for #name<#data_type>
			where #data_type: NumLike {
			type Output = #name<#data_type>;
			fn div(self, rhs: &#data_type) -> Self::Output {
				#name{#data_name: self.#data_name / rhs.clone()}
			}
		}
		#[doc="Dividing a unit value by a scalar value returns a unit value"]
		impl<#data_type> core::ops::Div<&#data_type> for &#name<#data_type>
			where #data_type: NumLike {
			type Output = #name<#data_type>;
			fn div(self, rhs: &#data_type) -> Self::Output {
				#name{#data_name: self.#data_name.clone() / rhs.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type: NumLike> core::ops::Mul<#data_type> for
		&#name<#data_type> {
			type Output = #name<#data_type>;
			fn mul(self, rhs: #data_type) -> Self::Output {
				return Self::Output{#data_name: self.#data_name.clone() * rhs}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 f64 where #data_type: NumLike + From<f64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 f32 where #data_type: NumLike + From<f32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 u8 where #data_type: NumLike + From<u8>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 i8 where #data_type: NumLike + From<i8>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 u16 where #data_type: NumLike + From<u16>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 i16 where #data_type: NumLike + From<i16>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 u32 where #data_type: NumLike + From<u32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 i32 where #data_type: NumLike + From<i32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 u64 where #data_type: NumLike + From<u64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Multiplying a unit value by a scalar value returns a unit value (automatically \
		clones the referenced data for convenient ergonomics)"]
		impl<#data_type>
		core::ops::Mul<&#name<#data_type>> for
		 i64 where #data_type: NumLike + From<i64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: &#name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name.clone()}
			}
		}
		#[doc="Flips the sign of this unit value (automatically clones the referenced data for \
		convenient ergonomics)"]
		impl<#data_type: NumLike> core::ops::Neg for
		&#name<#data_type> {
			type Output = #name<#data_type>;
			fn neg(self) -> Self::Output {
				return Self::Output{#data_name: self.#data_name.clone().neg()}
			}
		}
		// Mul DT by Self and Self by DT -> Self

        // impl #name {
        //     fn hello_macro() {
        //         println!("Hello, Macro! My name is {}!", stringify!(#name));
        //     }
		// 	fn my_attr(){
		// 		println!("X is {} of {}", stringify!(#data_name), stringify!(#data_type));
		// 	}
        // }
    };
	let output = gen.into();
	// uncomment next line to see macro output as compiler error
	//panic!("impl_derive_unit macro output: {}", output.to_string());
	return output;
}


// #[test]
// fn macro_test() {
// 	println!("Testing UnitStruct procedural macro...");
// 	use core::str::FromStr;
// 	use proc_macro2::TokenStream;
// 	//
// 	let test_struct = "\
// struct Temperature<NT> { \
// 	k : NT\
// }";
// 	let tokens = TokenStream::from_str(test_struct).unwrap();
// 	let ast: syn::DeriveInput = syn::parse2(tokens).unwrap();
// 	// let struct_type = ast.ident.to_string();
// 	// assert_eq!(struct_type, "Temperature");
// 	let output = impl_derive_unit(&ast);
// 	println!("{}", output.to_string());
// }
