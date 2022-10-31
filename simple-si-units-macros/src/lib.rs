use proc_macro::TokenStream;
use quote::{quote};
use syn::*;

// Test with $ rm .\tests\expand\derive_tester.expanded.rs ; cargo +nightly test -- --nocapture

#[proc_macro_derive(Unit)]
pub fn derive_unit(tokens: TokenStream) -> TokenStream {
	// convert the input tokens into an ast, specially from a derive
	let input: syn::DeriveInput = syn::parse(tokens).expect("syn::parse failed on proc macro \
	input for simple_si_units_macros::Unit");

	// Build the trait implementation
	impl_derive_unit(&input)
}

fn impl_derive_unit(input: &syn::DeriveInput) -> TokenStream {
	let requirements_msg = "Derive macro simple_si_units::Unit can only be applied to structs \
	with a single field, whose type implements std::ops::{Add, Sub, Div, Mul} (eg \
	<T: simple_si_units_core::UnitData>). For \
	example:\n\nuse simple_si_units::{Unit, UnitData};\n#[derive(Unit, Debug, Copy, Clone)\
	]\nstruct MyNewUnit<T: UnitData>{\n    pub x: \
	T\n}\n\nfn \
	weighted_sum<T: UnitData>(a: MyNewUnit<T>, b: MyNewUnit<T>, weight: f64) -> MyNewUnit<T> where T:\
	 UnitData + From<f64>\n{\n    return weight*a + (1.-weight)*b;\n}\n";
	let name = &input.ident;
	let fields = match &input.data {
		Data::Struct(DataStruct {
						 fields: Fields::Named(fields),
						 ..
					 }) => &fields.named,
		_ => panic!("Only structs with named fields can derive simple_si_units::Unit.\n\n{}",
					requirements_msg),
	};
	if fields.len() != 1 {
		panic!("proc macro simple_si_units::Unit can only be used on structs with a single \
		 named field. \n\n{}", requirements_msg)
	}
	let data_name = &fields[0].ident.as_ref().unwrap();
	let data_type = &fields[0].ty;
	let gen = quote! {
		impl<#data_type: simple_si_units_core::UnitData> std::ops::Add<Self> for #name<#data_type> {
			type Output = Self;
			fn add(self, rhs: Self) -> Self::Output {
				return Self{#data_name: self.#data_name + rhs.#data_name}
			}
		}
		impl<#data_type: simple_si_units_core::UnitData> std::ops::Sub<Self> for
		#name<#data_type> {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self::Output {
				return Self{#data_name: self.#data_name - rhs.#data_name}
			}
		}
		impl<#data_type: simple_si_units_core::UnitData> std::ops::Div<Self> for
		#name<#data_type> {
			type Output = #data_type;
			fn div(self, rhs: Self) -> Self::Output {
				return self.#data_name / rhs.#data_name;
			}
		}
		impl<#data_type: simple_si_units_core::UnitData> std::ops::Mul<#data_type> for
		#name<#data_type> {
			type Output = Self;
			fn mul(self, rhs: #data_type) -> Self::Output {
				return Self{#data_name: self.#data_name * rhs}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 f64 where #data_type: simple_si_units_core::UnitData + From<f64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 f32 where #data_type: simple_si_units_core::UnitData + From<f32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 u8 where #data_type: simple_si_units_core::UnitData + From<u8>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 i8 where #data_type: simple_si_units_core::UnitData + From<i8>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 u16 where #data_type: simple_si_units_core::UnitData + From<u16>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 i16 where #data_type: simple_si_units_core::UnitData + From<i16>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 u32 where #data_type: simple_si_units_core::UnitData + From<u32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 i32 where #data_type: simple_si_units_core::UnitData + From<i32>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 u64 where #data_type: simple_si_units_core::UnitData + From<u64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
			}
		}
		impl<#data_type>
		std::ops::Mul<#name<#data_type>> for
		 i64 where #data_type: simple_si_units_core::UnitData + From<i64>{
			type Output = #name<#data_type>;
			fn mul(self, rhs: #name<#data_type>) -> Self::Output {
				return #name{#data_name: #data_type::from(self) * rhs.#data_name}
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
// 	println!("Testing Unit procedural macro...");
// 	use std::str::FromStr;
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