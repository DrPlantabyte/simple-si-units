
#[cfg(feature="uom")]
#[cfg(test)]
mod uom_integration {
	use uom::si;

	#[test]
	fn spot_check_uom_library(){
		// test that the uom lib is working
		// example from uom docs:
		{
			use uom::si::f32::*;
			use uom::si::length::kilometer;
			use uom::si::time::second;
			let length = Length::new::<kilometer>(5.0);
			let time = Time::new::<second>(15.0);
			let _velocity = length / time;
		}
		// uom unit conversion spot check
		assert_eq!(
			si::f64::Length::new::<si::length::meter>(1000.),
			si::f64::Length::new::<si::length::kilometer>(1.),
			"uom assert fail: 1000 m per km");
		assert_eq!(
			si::f64::MolarConcentration::new::<si::molar_concentration::mole_per_cubic_meter>(1.),
			si::f64::MolarConcentration::new::<si::molar_concentration::millimole_per_liter>(1.),
			"uom assert fail: 1 mol/m3 = 1 mM");
		assert_eq!(
			si::f64::Velocity::new::<si::velocity::meter_per_second>(5.)
			/ si::f64::Length::new::<si::length::meter>(2.),
			si::f64::Frequency::new::<si::frequency::hertz>(2.5),
			"uom assert fail: 5 mps / 2 m = 2.5 Hz");
	}

}