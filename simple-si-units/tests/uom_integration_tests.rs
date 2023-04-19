
#[cfg(feature="uom")]
#[cfg(test)]
mod uom_integration {
	use uom::si;
	use simple_si_units;

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
			let velocity = length / time;
			println!("length {:?} / time {:?} = velocity {:?}", length, time, velocity);
			let x: f32 = velocity.value;
			println!("{}", x);
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

	#[test]
	fn uom_equivalence_test() {
		let x: f64 = 2.5;
		assert_eq!(simple_si_units::base::Amount{mol: x}.mol,
			uom::si::f64::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(x).value);
		assert_eq!(simple_si_units::base::Current{A: x}.A,
			uom::si::f64::ElectricCurrent::new::<uom::si::electric_current::ampere>(x).value);
		assert_eq!(simple_si_units::base::Distance{m: x}.m,
			uom::si::f64::Length::new::<uom::si::length::meter>(x).value);
		assert_eq!(simple_si_units::base::Luminosity{cd: x}.cd,
			uom::si::f64::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(x).value);
		assert_eq!(simple_si_units::base::Mass{kg: x}.kg,
			uom::si::f64::Mass::new::<uom::si::mass::kilogram>(x).value);
		assert_eq!(simple_si_units::base::Temperature{K: x}.K,
			uom::si::f64::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(x).value);
		assert_eq!(simple_si_units::base::Time{s: x}.s,
			uom::si::f64::Time::new::<uom::si::time::second>(x).value);
		assert_eq!(simple_si_units::chemical::CatalyticActivity{molps: x}.molps,
			uom::si::f64::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(x).value);
		assert_eq!(simple_si_units::chemical::Concentration{molpm3: x}.molpm3,
			uom::si::f64::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Capacitance{F: x}.F,
			uom::si::f64::Capacitance::new::<uom::si::capacitance::farad>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Charge{C: x}.C,
			uom::si::f64::ElectricCharge::new::<uom::si::electric_charge::coulomb>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Illuminance{lux: x}.lux,
			uom::si::f64::Luminance::new::<uom::si::luminance::candela_per_square_meter>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Inductance{H: x}.H,
			uom::si::f64::Inductance::new::<uom::si::inductance::henry>(x).value);
		assert_eq!(simple_si_units::electromagnetic::MagneticFlux{Wb: x}.Wb,
			uom::si::f64::MagneticFlux::new::<uom::si::magnetic_flux::weber>(x).value);
		assert_eq!(simple_si_units::electromagnetic::MagneticFluxDensity{T: x}.T,
			uom::si::f64::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Resistance{Ohm: x}.Ohm,
			uom::si::f64::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Voltage{V: x}.V,
			uom::si::f64::ElectricPotential::new::<uom::si::electric_potential::volt>(x).value);
		assert_eq!(simple_si_units::geometry::Angle{rad: x}.rad,
			uom::si::f64::Angle::new::<uom::si::angle::radian>(x).value);
		assert_eq!(simple_si_units::geometry::Area{m2: x}.m2,
			uom::si::f64::Area::new::<uom::si::area::square_meter>(x).value);
		assert_eq!(simple_si_units::geometry::Volume{m3: x}.m3,
			uom::si::f64::Volume::new::<uom::si::volume::cubic_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Acceleration{mps2: x}.mps2,
			uom::si::f64::Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(x).value);
		assert_eq!(simple_si_units::mechanical::AngularAcceleration{radps2: x}.radps2,
			uom::si::f64::AngularAcceleration::new::<uom::si::angular_acceleration::radian_per_second_squared>(x).value);
		assert_eq!(simple_si_units::mechanical::AngularVelocity{radps: x}.radps,
			uom::si::f64::AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(x).value);
		assert_eq!(simple_si_units::mechanical::AreaDensity{kgpm2: x}.kgpm2,
			uom::si::f64::ArealMassDensity::new::<uom::si::areal_mass_density::kilogram_per_square_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Density{kgpm3: x}.kgpm3,
			uom::si::f64::MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Energy{J: x}.J,
			uom::si::f64::Energy::new::<uom::si::energy::joule>(x).value);
		assert_eq!(simple_si_units::mechanical::Force{N: x}.N,
			uom::si::f64::Force::new::<uom::si::force::newton>(x).value);
		assert_eq!(simple_si_units::mechanical::Frequency{Hz: x}.Hz,
			uom::si::f64::Frequency::new::<uom::si::frequency::hertz>(x).value);
		assert_eq!(simple_si_units::mechanical::MomentOfInertia{kgm2: x}.kgm2,
			uom::si::f64::MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Momentum{kgmps: x}.kgmps,
			uom::si::f64::Momentum::new::<uom::si::momentum::kilogram_meter_per_second>(x).value);
		assert_eq!(simple_si_units::mechanical::Power{W: x}.W,
			uom::si::f64::Power::new::<uom::si::power::watt>(x).value);
		assert_eq!(simple_si_units::mechanical::Pressure{Pa: x}.Pa,
			uom::si::f64::Pressure::new::<uom::si::pressure::pascal>(x).value);
		assert_eq!(simple_si_units::mechanical::Torque{Nm: x}.Nm,
			uom::si::f64::Torque::new::<uom::si::torque::newton_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Velocity{mps: x}.mps,
			uom::si::f64::Velocity::new::<uom::si::velocity::meter_per_second>(x).value);
		assert_eq!(simple_si_units::nuclear::Radioactivity{Bq: x}.Bq,
			uom::si::f64::Radioactivity::new::<uom::si::radioactivity::becquerel>(x).value);
	}

	#[test]
	fn into_uom_test(){
		todo!()
	}
	#[test]
	fn from_uom_test(){
		todo!()
	}
}